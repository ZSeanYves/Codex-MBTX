//! Controlled Linux runner for the M5 shell/Transparent comparison.
//!
//! The binary is deliberately kept outside the Codex product binary.  It
//! creates a fresh workspace and Codex home for each arm, starts the private
//! Responses proxy, runs the pinned Codex binary as the dedicated evaluator
//! user, and emits exactly one redacted `{ "runs": [...] }` envelope.  Relay
//! failures are represented as infrastructure observations so the evaluator
//! can retain them in the intention-to-treat denominator.

use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const MAX_CAPTURE_BYTES: usize = 8 * 1024 * 1024;
const MAX_ARTIFACT_BYTES: usize = 2 * 1024 * 1024;
const AGENT_TIMEOUT: Duration = Duration::from_secs(300);

#[derive(Clone, Debug)]
struct TaskSpec {
    id: String,
    cohort: String,
    prompt: String,
    inputs: BTreeMap<String, String>,
    expected: Value,
    expected_stdout: String,
    expected_stderr: String,
    expected_exit_code: i32,
    expected_manifest: Vec<String>,
    editable: Option<String>,
    background: bool,
    fixture_nonce: String,
    receipt_path: String,
    oracle: String,
}

#[derive(Clone, Debug)]
struct BlockSpec {
    block_id: String,
    task: String,
    order: Vec<String>,
}

#[derive(Clone, Debug)]
struct RunnerConfig {
    codex: PathBuf,
    mbtx: PathBuf,
    evidence: PathBuf,
    trace_launcher: PathBuf,
    command_trace: PathBuf,
    model: String,
    effort: String,
    model_catalog: Option<PathBuf>,
    moon_home: Option<PathBuf>,
    moon: Option<PathBuf>,
    tool_path: String,
    upstream_url: String,
    agent_user: String,
    allow_unprivileged: bool,
    artifact_dir: PathBuf,
    real_python: PathBuf,
}

#[derive(Clone, Debug, Default)]
struct CaptureMeta {
    first_stdout_ms: Option<i64>,
    first_stderr_ms: Option<i64>,
    first_response_ms: Option<i64>,
    first_tool_ms: Option<i64>,
    last_tool_ms: Option<i64>,
    turn_completed_ms: Option<i64>,
}

#[derive(Debug)]
struct AgentResult {
    stdout: String,
    stderr: String,
    exit_code: i32,
    timed_out: bool,
    elapsed_ms: i64,
    process_launch_ms: i64,
    process_exit_ms: i64,
    first_stdout_ms: Option<i64>,
    first_stderr_ms: Option<i64>,
    first_response_ms: Option<i64>,
    first_tool_ms: Option<i64>,
    last_tool_ms: Option<i64>,
    turn_completed_ms: Option<i64>,
}

#[derive(Debug)]
struct ProxyHandle {
    child: Child,
    port: u16,
    dumps: PathBuf,
}

#[derive(Clone, Debug)]
struct Snapshot {
    mode: u32,
    symlink: bool,
    bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
struct CommandRecord {
    program: String,
}

#[derive(Clone, Debug, Default)]
struct EventSummary {
    turn_completed: bool,
    parse_errors: bool,
    response_completed: bool,
    response_count: usize,
    usage_responses: usize,
    input_tokens: i64,
    cached_input_tokens: i64,
    output_tokens: i64,
    commands: Vec<CommandRecord>,
    tool_counts: BTreeMap<String, i64>,
    seen_ids: HashSet<String>,
}

#[derive(Clone, Debug)]
struct AttemptRecord {
    value: Value,
    provider_error: bool,
    transport_error: bool,
    successful: bool,
    usage_complete: bool,
}

#[derive(Clone, Debug, Default)]
struct TraceSummary {
    command_starts: Vec<Value>,
    command_exits: Vec<Value>,
    launcher_starts: usize,
    launcher_exits: usize,
    labels: Vec<String>,
}

#[derive(Clone, Debug, Default)]
struct HelperStreams {
    stdout: String,
    stderr: String,
}

struct AgentInvocation<'a> {
    workspace: &'a Path,
    receipt_file: &'a Path,
    home: &'a Path,
    agent_dir: &'a Path,
    trace_file: &'a Path,
    launcher_trace: &'a Path,
    output_prefix: &'a Path,
    prompt: &'a str,
}

struct EvidenceArtifact<'a> {
    block: &'a BlockSpec,
    backend: &'a str,
    root: &'a Path,
    agent: &'a AgentResult,
    attempts: &'a [AttemptRecord],
    trace: &'a TraceSummary,
    helper_streams: &'a HelperStreams,
    manifest: &'a [String],
    run: &'a Map<String, Value>,
}

struct CleanupDir(PathBuf);

impl Drop for CleanupDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

impl Drop for ProxyHandle {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

fn main() {
    if let Err(error) = real_main() {
        eprintln!("m5-runner: {error}");
        std::process::exit(2);
    }
}

fn real_main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 || args[1] != "--block" || args[2].trim().is_empty() {
        return Err("usage: m5-runner --block BLOCK_ID < block.json".to_owned());
    }
    let block_id = args[2].clone();
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("reading block: {error}"))?;
    let block_value: Value =
        serde_json::from_str(&input).map_err(|error| format!("invalid block JSON: {error}"))?;
    let block = parse_block(&block_value)?;
    if block.block_id != block_id {
        return Err("--block does not match block_id in stdin".to_owned());
    }

    let config = RunnerConfig::from_env();
    let task = config
        .as_ref()
        .and_then(|cfg| fetch_task(&cfg.evidence, &block.task).ok());
    let mut runs = Vec::with_capacity(2);
    for backend in &block.order {
        if backend != "shell" && backend != "transparent" {
            return Err(format!("invalid backend in block order: {backend}"));
        }
        let run = match (&config, &task) {
            (Some(cfg), Some(task)) => execute_arm(cfg, &block, task, backend),
            (None, _) => unavailable_run(
                &block,
                backend,
                "runner configuration is incomplete",
                "transport_error",
            ),
            (_, None) => unavailable_run(
                &block,
                backend,
                "M5 evidence binary could not provide the fixture",
                "transport_error",
            ),
        };
        runs.push(run);
    }
    println!("{}", json!({"runs": runs}));
    Ok(())
}

impl RunnerConfig {
    fn from_env() -> Option<Self> {
        let codex = env_path("M5_CODEX")?;
        let mbtx = env_path("M5_MBTX")?;
        let evidence = env_path("M5_EVIDENCE")?;
        let trace_launcher = env_path("M5_TRACE_LAUNCHER")?;
        let command_trace = env_path("M5_COMMAND_TRACE")?;
        let required = [&codex, &mbtx, &evidence, &trace_launcher, &command_trace];
        if required.iter().any(|path| !path.is_file()) {
            return None;
        }
        let model_catalog = env_path("M5_MODEL_CATALOG").filter(|path| path.is_file());
        let moon_home = env_path("M5_MOON_HOME").filter(|path| path.is_dir());
        let moon = env_path("M5_MOON").filter(|path| path.is_file());
        let artifact_dir = env::var("M5_ARTIFACT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("_build/m5-results/runner-evidence"));
        Some(Self {
            codex,
            mbtx,
            evidence,
            trace_launcher,
            command_trace,
            model: env::var("M5_MODEL").unwrap_or_else(|_| "gpt-5.6-terra".to_owned()),
            effort: env::var("M5_EFFORT").unwrap_or_else(|_| "xhigh".to_owned()),
            model_catalog,
            moon_home,
            moon,
            tool_path: env::var("M5_TOOL_PATH").unwrap_or_default(),
            upstream_url: env::var("M5_UPSTREAM_URL")
                .unwrap_or_else(|_| "https://tokenadvent.com/v1/responses".to_owned()),
            agent_user: env::var("M5_AGENT_USER").unwrap_or_else(|_| "mbtx-eval".to_owned()),
            allow_unprivileged: env_bool("M5_ALLOW_UNPRIVILEGED"),
            artifact_dir,
            real_python: env::var("M5_REAL_PYTHON")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("/usr/bin/python3")),
        })
    }
}

fn env_path(name: &str) -> Option<PathBuf> {
    let value = std::env::var(name).ok()?;
    if value.trim().is_empty() {
        None
    } else {
        Some(PathBuf::from(value))
    }
}

fn env_bool(name: &str) -> bool {
    matches!(
        std::env::var(name).ok().as_deref(),
        Some("1") | Some("true") | Some("TRUE") | Some("yes")
    )
}

fn parse_block(value: &Value) -> Result<BlockSpec, String> {
    let object = value
        .as_object()
        .ok_or_else(|| "block must be a JSON object".to_owned())?;
    let block_id = required_string(object, "block_id")?;
    let task = required_string(object, "task")?;
    let order = object
        .get("order")
        .and_then(Value::as_array)
        .ok_or_else(|| "block.order must be an array".to_owned())?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| "block.order contains a non-string".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    if order.len() != 2 {
        return Err("block.order must contain exactly two arms".to_owned());
    }
    Ok(BlockSpec {
        block_id,
        task,
        order,
    })
}

fn required_string(object: &Map<String, Value>, key: &str) -> Result<String, String> {
    object
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| format!("missing string field {key}"))
}

fn fetch_task(evidence: &Path, task_id: &str) -> Result<TaskSpec, String> {
    let output = trusted_command(evidence, &["fixture", task_id])?;
    if !output.status.success() {
        return Err(format!(
            "fixture command failed with {}",
            output.status.code().unwrap_or(-1)
        ));
    }
    let value: Value = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("invalid fixture output: {error}"))?;
    let object = value
        .as_object()
        .ok_or_else(|| "fixture output is not an object".to_owned())?;
    let prompt_output = trusted_command(evidence, &["prompt", task_id])?;
    if !prompt_output.status.success() {
        return Err("prompt command failed".to_owned());
    }
    let prompt = String::from_utf8_lossy(&prompt_output.stdout)
        .trim_end_matches(['\r', '\n'])
        .to_owned();
    let mut inputs = BTreeMap::new();
    if let Some(input_object) = object.get("inputs").and_then(Value::as_object) {
        for (path, content) in input_object {
            let content = content
                .as_str()
                .ok_or_else(|| format!("fixture input {path} is not a string"))?;
            inputs.insert(path.clone(), content.to_owned());
        }
    }
    let expected_manifest = object
        .get("expected_manifest")
        .and_then(Value::as_array)
        .ok_or_else(|| "fixture expected_manifest is missing".to_owned())?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| "expected_manifest contains a non-string".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TaskSpec {
        id: required_string(object, "id")?,
        cohort: required_string(object, "cohort")?,
        prompt,
        inputs,
        expected: object.get("expected").cloned().unwrap_or(Value::Null),
        expected_stdout: object
            .get("expected_stdout")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned(),
        expected_stderr: object
            .get("expected_stderr")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned(),
        expected_exit_code: object
            .get("expected_exit_code")
            .and_then(Value::as_i64)
            .unwrap_or(0) as i32,
        expected_manifest,
        editable: object
            .get("editable")
            .and_then(Value::as_str)
            .map(str::to_owned),
        background: object
            .get("background")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        fixture_nonce: required_string(object, "fixture_nonce")?,
        receipt_path: object
            .get("receipt_path")
            .and_then(Value::as_str)
            .unwrap_or(".m5/receipt.json")
            .to_owned(),
        oracle: object
            .get("oracle")
            .and_then(Value::as_str)
            .unwrap_or("result_json+workspace_manifest+process_trace")
            .to_owned(),
    })
}

fn trusted_command(program: &Path, args: &[&str]) -> Result<std::process::Output, String> {
    let mut command = Command::new(program);
    command.args(args);
    command.env_remove("OPENROUTER_ICU_API_KEY");
    command.env_remove("M5_API_KEY");
    command
        .output()
        .map_err(|error| format!("launching {}: {error}", program.display()))
}

fn execute_arm(config: &RunnerConfig, block: &BlockSpec, task: &TaskSpec, backend: &str) -> Value {
    match execute_arm_inner(config, block, task, backend) {
        Ok(run) => run,
        Err(error) => unavailable_run(block, backend, &error, "transport_error"),
    }
}

fn execute_arm_inner(
    config: &RunnerConfig,
    block: &BlockSpec,
    task: &TaskSpec,
    backend: &str,
) -> Result<Value, String> {
    let key = std::env::var("M5_API_KEY")
        .or_else(|_| std::env::var("OPENROUTER_ICU_API_KEY"))
        .map_err(|_| "relay API key is not configured".to_owned())?;
    if key.is_empty() || key.contains('\n') || key.contains('\r') {
        return Err("relay API key is empty or contains a line break".to_owned());
    }
    let root = unique_temp_dir(&format!("codex-m5-{}-{}", block.block_id, backend))?;
    let _root_guard = CleanupDir(root.clone());
    let workspace = root.join("workspace");
    let home = root.join("home");
    let private = root.join("private");
    // Agent-visible traces live beside, rather than inside, the root-owned
    // proxy directory. This keeps request dumps private while allowing the
    // dedicated evaluator user to write its own command evidence.
    let agent_dir = root.join("agent");
    fs::create_dir_all(workspace.join(".m5")).map_err(io_error)?;
    fs::create_dir_all(&home).map_err(io_error)?;
    fs::create_dir_all(private.join("dumps")).map_err(io_error)?;
    fs::create_dir_all(&agent_dir).map_err(io_error)?;
    set_mode(&home, 0o700);
    set_mode(&private, 0o700);
    set_mode(&private.join("dumps"), 0o700);
    let snapshots = write_fixture(&workspace, task)?;
    initialize_git(&workspace)?;
    let receipt_file = safe_join(&workspace, &task.receipt_path)?;
    let trace_file = agent_dir.join("command-trace.jsonl");
    let launcher_trace = agent_dir.join("transparent-trace.jsonl");
    let output_prefix = agent_dir.join("command-output");
    install_trace_links(config, &agent_dir, task.cohort == "script")?;

    let mut proxy = start_proxy(config, &private, &key)?;
    let config_path = home.join("config.toml");
    write_codex_config(config, &config_path, proxy.port, backend)?;
    prepare_agent_access(config, &workspace, &home, &agent_dir)?;
    let prompt = task_prompt(config, task)?;
    let wall_start = SystemTime::now();
    let mono_start = Instant::now();
    let agent = run_agent(
        config,
        task,
        AgentInvocation {
            workspace: &workspace,
            receipt_file: &receipt_file,
            home: &home,
            agent_dir: &agent_dir,
            trace_file: &trace_file,
            launcher_trace: &launcher_trace,
            output_prefix: &output_prefix,
            prompt: &prompt,
        },
    )?;
    let child_processes_clean = cleanup_agent_processes(config);
    stop_proxy(&mut proxy);
    let trace = read_trace(&trace_file, &launcher_trace);
    let helper_streams = read_helper_streams(&agent_dir);
    let events_path = agent_dir.join("events.jsonl");
    let stderr_path = agent_dir.join("agent-stderr.txt");
    write_bounded(&events_path, agent.stdout.as_bytes());
    write_bounded(&stderr_path, agent.stderr.as_bytes());
    let mut event_summary = parse_events(&agent.stdout);
    let attempts = parse_attempts(&proxy.dumps, wall_start, mono_start, agent.elapsed_ms);
    // Some Codex versions expose a completed turn without emitting a raw
    // Responses `response.completed` event.  A captured attempt is still
    // observable evidence; retain that fact without inventing stream timing.
    if event_summary.response_count == 0 && !attempts.is_empty() {
        event_summary.response_count = attempts.len();
        event_summary.response_completed = attempts.iter().any(|attempt| attempt.successful);
    }
    let relay_status = relay_status(&attempts);
    let result_path = workspace.join("result.json");
    let result_text = read_text_limited(&result_path).unwrap_or_default();
    let result_value = serde_json::from_str::<Value>(&result_text).unwrap_or(Value::Null);
    let mut receipt = read_json(&receipt_file).unwrap_or_else(|| json!({}));
    let result_exists = result_path.is_file();
    let mut exit_code = final_helper_exit(&trace).unwrap_or(-1);
    if task.id == "host_cancel_timeout" && !result_exists {
        exit_code = -1;
        // The runner owns the cancellation cleanup. Bind the receipt to the
        // stopped state even when the model's stop command killed the helper
        // before the shim could atomically update it.
        if !receipt_valid(task, &receipt) || receipt_string(&receipt, "status") != "stopped" {
            receipt = json!({
                "task": task.id,
                "nonce": task.fixture_nonce,
                "helper": "cancelled-child",
                "status": "stopped",
            });
            write_json_atomic(&receipt_file, &receipt);
        }
    }
    let actual_manifest = workspace_manifest(&workspace);
    let diff = workspace_diff(task, &workspace, &snapshots, &actual_manifest);
    let inputs_preserved = inputs_preserved(task, &workspace);
    let workspace_clean = manifest_equal(&task.expected_manifest, &actual_manifest)
        && workspace_entries_safe(&workspace, &actual_manifest)
        && workspace_diff_flags_clean(&diff);
    let result_correct = if task.id == "host_cancel_timeout" {
        !result_exists && child_processes_clean
    } else {
        correct_result(task, &result_value)
    };
    let streams_correct = helper_streams_valid(task, &helper_streams, exit_code);
    let exit_correct = task.id == "host_cancel_timeout" || exit_code == task.expected_exit_code;
    let correct_output = result_correct && streams_correct && exit_correct;
    let argv_cwd_valid = argv_cwd_valid(task, &trace, &workspace);
    let receipt_ok = receipt_valid(task, &receipt);
    let trace_complete = trace_complete(task, &trace);
    let command_observed = !trace.command_starts.is_empty() && !event_summary.commands.is_empty();
    let backend_observed = if backend == "transparent" {
        // A start-only trace is not proof that the transparent launcher
        // completed the final handoff. Require a balanced launcher pair.
        command_observed
            && trace.launcher_starts > 0
            && trace.launcher_starts == trace.launcher_exits
    } else {
        command_observed && trace.launcher_starts == 0 && trace.launcher_exits == 0
    };
    let backend_compliant = backend_compliant(task, &event_summary, &trace);
    let approval_original_command = approval_original_command(&event_summary, config, &workspace);
    let stream_complete = event_summary.response_completed && !event_summary.parse_errors;
    let observability = if event_summary.response_count == 0 && attempts.is_empty() {
        "unobserved"
    } else if !stream_complete || !event_summary.turn_completed || agent.timed_out {
        "partial"
    } else {
        "complete"
    };
    let backend_observation = if !backend_observed {
        "unknown"
    } else if backend_compliant {
        "compliant"
    } else {
        "violation"
    };
    let usage_complete = attempts.iter().any(|attempt| attempt.successful)
        && attempts
            .iter()
            .filter(|attempt| attempt.successful)
            .all(|attempt| attempt.usage_complete);
    let completed = event_summary.turn_completed && agent.exit_code == 0 && !agent.timed_out;
    let failure_category = classify(
        config,
        relay_status,
        observability,
        backend_observation,
        completed,
        usage_complete,
        correct_output,
        inputs_preserved,
        workspace_clean,
        child_processes_clean,
        receipt_ok,
        trace_complete,
        argv_cwd_valid,
        approval_original_command,
        agent.timed_out,
    );
    let mut run = base_run(block, task, backend);
    insert(&mut run, "started_ms", json!(0));
    // All timestamps in a run are relative to this run's monotonic origin.
    // Keep the explicit start/end pair even when the optional event markers
    // are unavailable, so latency analysis never has to infer an end time.
    insert(&mut run, "monotonic_start_ms", json!(0));
    insert(&mut run, "monotonic_end_ms", json!(agent.elapsed_ms));
    insert(&mut run, "ended_ms", json!(agent.elapsed_ms));
    insert_opt(&mut run, "first_response_ms", agent.first_response_ms);
    insert_opt(
        &mut run,
        "first_tool_ms",
        if event_summary.commands.is_empty() {
            None
        } else {
            agent.first_tool_ms
        },
    );
    insert_opt(
        &mut run,
        "last_tool_ms",
        if event_summary.commands.is_empty() {
            None
        } else {
            agent.last_tool_ms
        },
    );
    insert_opt(
        &mut run,
        "turn_completed_ms",
        if event_summary.turn_completed {
            agent.turn_completed_ms
        } else {
            None
        },
    );
    insert_opt(
        &mut run,
        "process_launch_ms",
        if trace.command_starts.is_empty() {
            None
        } else {
            Some(agent.process_launch_ms)
        },
    );
    insert_opt(
        &mut run,
        "process_exit_ms",
        if trace.command_exits.is_empty() {
            None
        } else {
            Some(agent.process_exit_ms)
        },
    );
    insert(
        &mut run,
        "stdout",
        json!(sanitize_output(&helper_streams.stdout, config, &root)),
    );
    insert(
        &mut run,
        "stderr",
        json!(sanitize_output(&helper_streams.stderr, config, &root)),
    );
    insert(&mut run, "exit_code", json!(exit_code));
    insert(&mut run, "workspace_manifest", json!(actual_manifest));
    insert(
        &mut run,
        "attempts",
        Value::Array(
            attempts
                .iter()
                .map(|attempt| attempt.value.clone())
                .collect(),
        ),
    );
    insert(&mut run, "tool_counts", json!(event_summary.tool_counts));
    insert(
        &mut run,
        "input_tokens",
        json!(attempt_input_tokens(&attempts)),
    );
    insert(
        &mut run,
        "cached_input_tokens",
        json!(attempt_cached_tokens(&attempts)),
    );
    insert(
        &mut run,
        "output_tokens",
        json!(attempt_output_tokens(&attempts)),
    );
    insert(&mut run, "relay_status", json!(relay_status));
    insert(&mut run, "observability", json!(observability));
    insert(&mut run, "backend_observation", json!(backend_observation));
    insert(&mut run, "completed", json!(completed));
    insert(&mut run, "usage_complete", json!(usage_complete));
    insert(&mut run, "correct_output", json!(correct_output));
    insert(&mut run, "inputs_preserved", json!(inputs_preserved));
    insert(&mut run, "workspace_clean", json!(workspace_clean));
    insert(
        &mut run,
        "child_processes_clean",
        json!(child_processes_clean),
    );
    insert(&mut run, "receipt_valid", json!(receipt_ok));
    insert(&mut run, "trace_complete", json!(trace_complete));
    insert(&mut run, "argv_cwd_valid", json!(argv_cwd_valid));
    insert(
        &mut run,
        "approval_original_command",
        json!(approval_original_command),
    );
    insert(&mut run, "workspace_diff", diff);
    insert(&mut run, "receipt", receipt);
    insert(
        &mut run,
        "process_trace",
        Value::Array(
            trace
                .labels
                .iter()
                .map(|label| Value::String(label.clone()))
                .collect(),
        ),
    );
    insert(&mut run, "failure_category", json!(failure_category));
    insert(&mut run, "agent_exit_code", json!(agent.exit_code));
    insert(&mut run, "timed_out", json!(agent.timed_out));
    insert(&mut run, "result", result_value);
    insert(&mut run, "result_correct", json!(result_correct));
    insert(&mut run, "streams_correct", json!(streams_correct));
    insert(&mut run, "exit_correct", json!(exit_correct));
    insert(
        &mut run,
        "timing_quality",
        json!("agent event capture monotonic; proxy exchange wall-clock filename"),
    );
    persist_evidence(
        config,
        EvidenceArtifact {
            block,
            backend,
            root: &root,
            agent: &agent,
            attempts: &attempts,
            trace: &trace,
            helper_streams: &helper_streams,
            manifest: &actual_manifest,
            run: &run,
        },
    );
    Ok(Value::Object(run))
}

fn base_run(block: &BlockSpec, task: &TaskSpec, backend: &str) -> Map<String, Value> {
    let mut run = Map::new();
    run.insert(
        "run_id".to_owned(),
        Value::String(format!("{}-{backend}", block.block_id)),
    );
    run.insert("block_id".to_owned(), Value::String(block.block_id.clone()));
    run.insert("task".to_owned(), Value::String(task.id.clone()));
    run.insert("backend".to_owned(), Value::String(backend.to_owned()));
    run.insert("stdout".to_owned(), Value::String(String::new()));
    run.insert("stderr".to_owned(), Value::String(String::new()));
    run.insert("exit_code".to_owned(), json!(-1));
    run.insert("workspace_manifest".to_owned(), json!([]));
    run.insert("attempts".to_owned(), json!([]));
    run.insert("tool_counts".to_owned(), json!({}));
    run.insert("input_tokens".to_owned(), json!(0));
    run.insert("cached_input_tokens".to_owned(), json!(0));
    run.insert("output_tokens".to_owned(), json!(0));
    run.insert(
        "relay_status".to_owned(),
        Value::String("transport_error".to_owned()),
    );
    run.insert(
        "observability".to_owned(),
        Value::String("unobserved".to_owned()),
    );
    run.insert(
        "backend_observation".to_owned(),
        Value::String("unknown".to_owned()),
    );
    for key in [
        "completed",
        "usage_complete",
        "correct_output",
        "inputs_preserved",
        "workspace_clean",
        "child_processes_clean",
        "receipt_valid",
        "trace_complete",
        "argv_cwd_valid",
        "approval_original_command",
    ] {
        run.insert(key.to_owned(), Value::Bool(false));
    }
    run.insert(
        "workspace_diff".to_owned(),
        json!({
            "entries": [],
            "permissions_unchanged": false,
            "symlinks_unchanged": false,
        }),
    );
    run.insert("receipt".to_owned(), json!({}));
    run.insert("process_trace".to_owned(), json!([]));
    run.insert(
        "failure_category".to_owned(),
        Value::String("infrastructure_unavailable".to_owned()),
    );
    run.insert("started_ms".to_owned(), json!(0));
    run.insert("monotonic_start_ms".to_owned(), json!(0));
    run.insert("monotonic_end_ms".to_owned(), json!(0));
    run.insert("ended_ms".to_owned(), json!(0));
    let _ = task;
    run
}

fn unavailable_run(block: &BlockSpec, backend: &str, reason: &str, relay_status: &str) -> Value {
    let mut run = base_run(
        block,
        &TaskSpec {
            id: block.task.clone(),
            cohort: "process".to_owned(),
            prompt: String::new(),
            inputs: BTreeMap::new(),
            expected: Value::Null,
            expected_stdout: String::new(),
            expected_stderr: String::new(),
            expected_exit_code: -1,
            expected_manifest: Vec::new(),
            editable: None,
            background: false,
            fixture_nonce: String::new(),
            receipt_path: ".m5/receipt.json".to_owned(),
            oracle: String::new(),
        },
        backend,
    );
    run.insert(
        "relay_status".to_owned(),
        Value::String(relay_status.to_owned()),
    );
    run.insert("stderr".to_owned(), Value::String(reason.to_owned()));
    run.insert(
        "infrastructure_reason".to_owned(),
        Value::String(reason.to_owned()),
    );
    Value::Object(run)
}

fn insert(object: &mut Map<String, Value>, key: &str, value: Value) {
    object.insert(key.to_owned(), value);
}

fn insert_opt(object: &mut Map<String, Value>, key: &str, value: Option<i64>) {
    if let Some(value) = value {
        object.insert(key.to_owned(), json!(value));
    }
}

fn unique_temp_dir(prefix: &str) -> Result<PathBuf, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let path = std::env::temp_dir().join(format!("{prefix}-{}-{now}", std::process::id()));
    fs::create_dir_all(&path).map_err(io_error)?;
    Ok(path)
}

fn io_error(error: io::Error) -> String {
    error.to_string()
}

fn safe_join(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let path = Path::new(relative);
    if path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(format!("unsafe fixture path: {relative}"));
    }
    Ok(root.join(path))
}

fn write_fixture(root: &Path, task: &TaskSpec) -> Result<BTreeMap<String, Snapshot>, String> {
    let mut snapshots = BTreeMap::new();
    for (relative, content) in &task.inputs {
        let path = safe_join(root, relative)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(io_error)?;
        }
        fs::write(&path, content.as_bytes()).map_err(io_error)?;
        set_mode(&path, 0o644);
        snapshots.insert(relative.clone(), snapshot(&path)?);
    }
    Ok(snapshots)
}

fn initialize_git(workspace: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .env_remove("M5_API_KEY")
        .env_remove("OPENROUTER_ICU_API_KEY")
        .args(["init", "--quiet", "--initial-branch=main"])
        .arg(workspace)
        .output()
        .map_err(io_error)?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "git init failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn install_trace_links(
    config: &RunnerConfig,
    agent_dir: &Path,
    script: bool,
) -> Result<(), String> {
    let bin = agent_dir.join("bin");
    fs::create_dir_all(&bin).map_err(io_error)?;
    link_or_copy(&config.command_trace, &bin.join("python3"))?;
    if script {
        link_or_copy(&config.command_trace, &bin.join("moon"))?;
    }
    Ok(())
}

fn link_or_copy(source: &Path, destination: &Path) -> Result<(), String> {
    let _ = fs::remove_file(destination);
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(source, destination).map_err(io_error)?;
    }
    #[cfg(not(unix))]
    {
        fs::copy(source, destination).map_err(io_error)?;
    }
    Ok(())
}

fn prepare_agent_access(
    config: &RunnerConfig,
    workspace: &Path,
    home: &Path,
    agent_dir: &Path,
) -> Result<(), String> {
    if config.allow_unprivileged {
        return Ok(());
    }
    let user = &config.agent_user;
    let status = Command::new("sudo")
        .args(["-n", "chown", "-R"])
        .arg(format!("{user}:{user}"))
        .arg(workspace)
        .arg(home)
        .arg(agent_dir)
        .status()
        .map_err(io_error)?;
    if status.success() {
        Ok(())
    } else {
        Err("could not grant the dedicated evaluator user access".to_owned())
    }
}

fn task_prompt(config: &RunnerConfig, task: &TaskSpec) -> Result<String, String> {
    if task.cohort != "script" {
        return Ok(task.prompt.clone());
    }
    let reference = std::env::var("M5_REFERENCE_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("eval/MOONBIT.md"));
    if !reference.is_file() {
        return Ok(task.prompt.clone());
    }
    let text = read_text_limited(&reference).unwrap_or_default();
    let _ = config;
    Ok(format!("{}\n\nReference:\n{}", task.prompt, text))
}

fn write_codex_config(
    config: &RunnerConfig,
    path: &Path,
    port: u16,
    backend: &str,
) -> Result<(), String> {
    let mut lines = Vec::new();
    if backend == "transparent" {
        lines.push(format!(
            "mbtx_command = [{}, {}]",
            toml_string(&config.trace_launcher),
            toml_string(&config.mbtx)
        ));
        lines.push("mbtx_backend = \"transparent\"".to_owned());
    }
    lines.extend([
        "model_provider = \"M5Relay\"".to_owned(),
        format!("model = {}", toml_quote(&config.model)),
        format!("model_reasoning_effort = {}", toml_quote(&config.effort)),
        "approval_policy = \"never\"".to_owned(),
        "sandbox_mode = \"workspace-write\"".to_owned(),
        "web_search = \"disabled\"".to_owned(),
        "[agents]".to_owned(),
        "enabled = false".to_owned(),
        "[features]".to_owned(),
        "shell_snapshot = false".to_owned(),
        "shell_snapshot_v2 = false".to_owned(),
        "code_mode = false".to_owned(),
        "code_mode_only = false".to_owned(),
        "goals = false".to_owned(),
        "[model_providers.M5Relay]".to_owned(),
        "name = \"M5 relay via isolated proxy\"".to_owned(),
        format!("base_url = \"http://127.0.0.1:{port}/v1\""),
        "wire_api = \"responses\"".to_owned(),
        "requires_openai_auth = false".to_owned(),
        "supports_websockets = false".to_owned(),
        "request_max_retries = 2".to_owned(),
        "stream_max_retries = 1".to_owned(),
    ]);
    if let Some(catalog) = &config.model_catalog {
        lines.insert(2, format!("model_catalog_json = {}", toml_string(catalog)));
    }
    let mut sandbox = vec![
        "[sandbox_workspace_write]".to_owned(),
        "network_access = true".to_owned(),
    ];
    if let Some(moon_home) = &config.moon_home {
        sandbox.push(format!(
            "writable_roots = [{}]",
            toml_string(&moon_home.join("cache"))
        ));
    }
    let provider_index = lines
        .iter()
        .position(|line| line == "[model_providers.M5Relay]")
        .unwrap_or(lines.len());
    lines.splice(provider_index..provider_index, sandbox);
    fs::write(path, lines.join("\n") + "\n").map_err(io_error)
}

fn toml_string(path: &Path) -> String {
    serde_json::to_string(&path.to_string_lossy().to_string()).unwrap_or_else(|_| "\"\"".to_owned())
}

fn toml_quote(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_owned())
}

fn start_proxy(config: &RunnerConfig, private: &Path, key: &str) -> Result<ProxyHandle, String> {
    let server_info = private.join("server.json");
    let dumps = private.join("dumps");
    let log = private.join("proxy.log");
    let stdout = File::create(&log).map_err(io_error)?;
    let stderr = stdout.try_clone().map_err(io_error)?;
    let mut command = Command::new(&config.codex);
    command.args(["responses-api-proxy", "--http-shutdown", "--server-info"]);
    command.arg(&server_info);
    command
        .arg("--upstream-url")
        .arg(&config.upstream_url)
        .arg("--dump-dir");
    command.arg(&dumps);
    command
        .env_clear()
        .env("PATH", "/usr/local/bin:/usr/bin:/bin")
        .stdin(Stdio::piped())
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));
    let mut child = command
        .spawn()
        .map_err(|error| format!("starting relay proxy: {error}"))?;
    if let Some(mut stdin) = child.stdin.take()
        && let Err(error) = stdin
            .write_all(key.as_bytes())
            .and_then(|_| stdin.write_all(b"\n"))
    {
        let _ = child.kill();
        let _ = child.wait();
        return Err(io_error(error));
    }
    let mut port = None;
    for _ in 0..100 {
        if server_info.is_file()
            && let Some(value) =
                read_json(&server_info).and_then(|value| value.get("port").and_then(Value::as_u64))
        {
            port = Some(value as u16);
            break;
        }
        if child.try_wait().map_err(io_error)?.is_some() {
            return Err("relay proxy exited before writing server info".to_owned());
        }
        thread::sleep(Duration::from_millis(100));
    }
    let Some(port) = port else {
        let _ = child.kill();
        let _ = child.wait();
        return Err("relay proxy did not become ready within 10 seconds".to_owned());
    };
    Ok(ProxyHandle { child, port, dumps })
}

fn stop_proxy(proxy: &mut ProxyHandle) {
    let _ = Command::new("curl")
        .env_clear()
        .env("PATH", "/usr/local/bin:/usr/bin:/bin")
        .args(["--silent", "--show-error", "--max-time", "3"])
        .arg(format!("http://127.0.0.1:{}/shutdown", proxy.port))
        .output();
    for _ in 0..30 {
        match proxy.child.try_wait() {
            Ok(Some(_)) => return,
            Ok(None) => thread::sleep(Duration::from_millis(100)),
            Err(_) => break,
        }
    }
    let _ = proxy.child.kill();
    let _ = proxy.child.wait();
}

fn run_agent(
    config: &RunnerConfig,
    task: &TaskSpec,
    invocation: AgentInvocation<'_>,
) -> Result<AgentResult, String> {
    let AgentInvocation {
        workspace,
        receipt_file,
        home,
        agent_dir,
        trace_file,
        launcher_trace,
        output_prefix,
        prompt,
    } = invocation;
    if !config.allow_unprivileged && !user_exists(&config.agent_user) {
        return Err(format!(
            "dedicated evaluator user {} does not exist",
            config.agent_user
        ));
    }
    let mut environment = BTreeMap::new();
    let mut path_entries = vec![agent_dir.join("bin").to_string_lossy().to_string()];
    if !config.tool_path.is_empty() {
        path_entries.push(config.tool_path.clone());
    }
    if let Some(moon_home) = &config.moon_home {
        path_entries.push(moon_home.join("bin").to_string_lossy().to_string());
    }
    path_entries.extend([
        "/usr/local/bin".to_owned(),
        "/usr/bin".to_owned(),
        "/bin".to_owned(),
    ]);
    environment.insert("PATH".to_owned(), path_entries.join(":"));
    environment.insert("HOME".to_owned(), home.to_string_lossy().to_string());
    environment.insert("CODEX_HOME".to_owned(), home.to_string_lossy().to_string());
    environment.insert("M5_TASK".to_owned(), task.id.clone());
    environment.insert("M5_TASK_NONCE".to_owned(), task.fixture_nonce.clone());
    environment.insert(
        "M5_TRACE_FILE".to_owned(),
        trace_file.to_string_lossy().to_string(),
    );
    environment.insert(
        "M5_TRANSPARENT_TRACE_FILE".to_owned(),
        launcher_trace.to_string_lossy().to_string(),
    );
    environment.insert(
        "M5_OUTPUT_PREFIX".to_owned(),
        output_prefix.to_string_lossy().to_string(),
    );
    environment.insert(
        "M5_RECEIPT_PATH".to_owned(),
        receipt_file.to_string_lossy().to_string(),
    );
    environment.insert(
        "M5_REAL_PROGRAM_PYTHON3".to_owned(),
        config.real_python.to_string_lossy().to_string(),
    );
    if let Some(moon) = &config.moon {
        environment.insert(
            "M5_REAL_PROGRAM_MOON".to_owned(),
            moon.to_string_lossy().to_string(),
        );
    }
    if let Some(moon_home) = &config.moon_home {
        environment.insert(
            "MOON_HOME".to_owned(),
            moon_home.to_string_lossy().to_string(),
        );
    }
    let mut args = vec![
        "exec".to_owned(),
        "--json".to_owned(),
        "--strict-config".to_owned(),
        "--ephemeral".to_owned(),
        "--ignore-rules".to_owned(),
        "--skip-git-repo-check".to_owned(),
        "-C".to_owned(),
        workspace.to_string_lossy().to_string(),
        prompt.to_owned(),
    ];
    let start = Instant::now();
    let capture_meta = Arc::new(Mutex::new(CaptureMeta::default()));
    let mut command = if config.allow_unprivileged {
        let mut direct = Command::new(&config.codex);
        direct.args(&args);
        direct
    } else {
        let mut sudo = Command::new("sudo");
        sudo.arg("-n")
            .arg("-u")
            .arg(&config.agent_user)
            .arg("--")
            .arg("/usr/bin/env")
            .arg("-i");
        for (key, value) in &environment {
            sudo.arg(format!("{key}={value}"));
        }
        sudo.arg(&config.codex).args(&args);
        sudo
    };
    if config.allow_unprivileged {
        command.env_clear();
        for (key, value) in &environment {
            command.env(key, value);
        }
    }
    command
        .current_dir(workspace)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|error| format!("starting Codex: {error}"))?;
    let process_launch_ms = start.elapsed().as_millis().min(i64::MAX as u128) as i64;
    let stdout_reader = child
        .stdout
        .take()
        .ok_or_else(|| "Codex stdout pipe was not created".to_owned())?;
    let stderr_reader = child
        .stderr
        .take()
        .ok_or_else(|| "Codex stderr pipe was not created".to_owned())?;
    let stdout_meta = Arc::clone(&capture_meta);
    let stderr_meta = Arc::clone(&capture_meta);
    let stdout_thread = capture_pipe(stdout_reader, true, start, stdout_meta);
    let stderr_thread = capture_pipe(stderr_reader, false, start, stderr_meta);
    let deadline = Instant::now() + AGENT_TIMEOUT;
    let mut timed_out = false;
    let status = loop {
        if let Some(status) = child.try_wait().map_err(io_error)? {
            break status;
        }
        if Instant::now() >= deadline {
            timed_out = true;
            let _ = child.kill();
            break child.wait().map_err(io_error)?;
        }
        thread::sleep(Duration::from_millis(100));
    };
    let stdout = join_capture(stdout_thread);
    let stderr = join_capture(stderr_thread);
    let process_exit_ms = start.elapsed().as_millis().min(i64::MAX as u128) as i64;
    let elapsed_ms = start.elapsed().as_millis().min(i64::MAX as u128) as i64;
    let metadata = capture_meta
        .lock()
        .map_err(|_| "capture metadata poisoned".to_owned())?;
    args.clear();
    Ok(AgentResult {
        stdout: String::from_utf8_lossy(&stdout).into_owned(),
        stderr: String::from_utf8_lossy(&stderr).into_owned(),
        exit_code: status.code().unwrap_or(-1),
        timed_out,
        elapsed_ms,
        process_launch_ms,
        process_exit_ms,
        first_stdout_ms: metadata.first_stdout_ms,
        first_stderr_ms: metadata.first_stderr_ms,
        first_response_ms: metadata.first_response_ms,
        first_tool_ms: metadata.first_tool_ms,
        last_tool_ms: metadata.last_tool_ms,
        turn_completed_ms: metadata.turn_completed_ms,
    })
}

fn user_exists(user: &str) -> bool {
    Command::new("id")
        .env_clear()
        .env("PATH", "/usr/local/bin:/usr/bin:/bin")
        .args(["-u", user])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn capture_pipe<R: Read + Send + 'static>(
    mut reader: R,
    stdout: bool,
    start: Instant,
    metadata: Arc<Mutex<CaptureMeta>>,
) -> JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut bytes = Vec::new();
        let mut buffer = [0_u8; 8192];
        let mut first = true;
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(size) => {
                    if first {
                        if let Ok(mut metadata) = metadata.lock() {
                            let elapsed = start.elapsed().as_millis().min(i64::MAX as u128) as i64;
                            if stdout {
                                metadata.first_stdout_ms = Some(elapsed);
                            } else {
                                metadata.first_stderr_ms = Some(elapsed);
                            }
                        }
                        first = false;
                    }
                    if bytes.len() < MAX_CAPTURE_BYTES {
                        let keep = (MAX_CAPTURE_BYTES - bytes.len()).min(size);
                        bytes.extend_from_slice(&buffer[..keep]);
                    }
                    if stdout {
                        let text = String::from_utf8_lossy(&bytes);
                        if let Ok(mut metadata) = metadata.lock() {
                            let elapsed = start.elapsed().as_millis().min(i64::MAX as u128) as i64;
                            if metadata.first_response_ms.is_none()
                                && (text.contains("\"type\":\"item.started\"")
                                    || text.contains("\"type\": \"item.started\""))
                            {
                                metadata.first_response_ms = Some(elapsed);
                            }
                            if text.contains("\"type\":\"command_execution\"")
                                || text.contains("\"type\": \"command_execution\"")
                            {
                                if metadata.first_tool_ms.is_none() {
                                    metadata.first_tool_ms = Some(elapsed);
                                }
                                metadata.last_tool_ms = Some(elapsed);
                            }
                            if metadata.turn_completed_ms.is_none()
                                && (text.contains("\"type\":\"turn.completed\"")
                                    || text.contains("\"type\": \"turn.completed\""))
                            {
                                metadata.turn_completed_ms = Some(elapsed);
                            }
                        }
                    }
                }
                Err(_) => break,
            }
        }
        bytes
    })
}

fn join_capture(handle: JoinHandle<Vec<u8>>) -> Vec<u8> {
    handle.join().unwrap_or_default()
}

fn cleanup_agent_processes(config: &RunnerConfig) -> bool {
    if config.allow_unprivileged {
        return true;
    }
    if config.agent_user != "mbtx-eval" {
        return false;
    }
    let _ = Command::new("sudo")
        .env_clear()
        .env("PATH", "/usr/local/bin:/usr/bin:/bin")
        .args(["-n", "pkill", "-KILL", "-u", &config.agent_user])
        .status();
    for _ in 0..20 {
        let output = Command::new("pgrep")
            .env_clear()
            .env("PATH", "/usr/local/bin:/usr/bin:/bin")
            .args([
                "-u",
                &config.agent_user,
                "-f",
                "(python3|mbtx|codex|m5-command-trace|m5-trace-launcher)",
            ])
            .output();
        if output
            .as_ref()
            .map(|value| !value.status.success())
            .unwrap_or(true)
        {
            return true;
        }
        thread::sleep(Duration::from_millis(100));
    }
    false
}

fn read_trace(command_path: &Path, launcher_path: &Path) -> TraceSummary {
    let mut summary = TraceSummary::default();
    let mut ordered: Vec<(u64, usize, String)> = Vec::new();
    let mut sequence = 0_usize;
    for (path, kind) in [(command_path, "command"), (launcher_path, "launcher")] {
        let records = read_json_lines(path);
        for record in records {
            let event = record
                .get("event")
                .and_then(Value::as_str)
                .unwrap_or_default();
            match (kind, event) {
                ("command", "command_start") => {
                    summary.command_starts.push(record.clone());
                    ordered.push((
                        trace_timestamp(&record),
                        sequence,
                        "command_start".to_owned(),
                    ));
                }
                ("command", "command_exit") => {
                    summary.command_exits.push(record.clone());
                    ordered.push((
                        trace_timestamp(&record),
                        sequence,
                        "command_exit".to_owned(),
                    ));
                }
                ("launcher", "launcher_start") => {
                    summary.launcher_starts += 1;
                    ordered.push((
                        trace_timestamp(&record),
                        sequence,
                        "launcher_start".to_owned(),
                    ));
                }
                ("launcher", "launcher_exit") => {
                    summary.launcher_exits += 1;
                    ordered.push((
                        trace_timestamp(&record),
                        sequence,
                        "launcher_exit".to_owned(),
                    ));
                }
                _ => ordered.push((
                    trace_timestamp(&record),
                    sequence,
                    "trace_unknown".to_owned(),
                )),
            }
            sequence += 1;
        }
    }
    ordered.sort_by_key(|(timestamp, sequence, _)| (*timestamp, *sequence));
    summary.labels = ordered.into_iter().map(|(_, _, label)| label).collect();
    summary
}

fn trace_timestamp(record: &Value) -> u64 {
    record
        .get("started_unix_ns")
        .or_else(|| record.get("finished_unix_ns"))
        .and_then(Value::as_u64)
        .unwrap_or(u64::MAX)
}

fn read_json_lines(path: &Path) -> Vec<Value> {
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };
    text.lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .collect()
}

fn read_helper_streams(agent_dir: &Path) -> HelperStreams {
    let mut stdout_paths = Vec::new();
    let mut stderr_paths = Vec::new();
    if let Ok(entries) = fs::read_dir(agent_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default();
            if name.starts_with("command-output-") && name.ends_with(".stdout") {
                stdout_paths.push(path);
            } else if name.starts_with("command-output-") && name.ends_with(".stderr") {
                stderr_paths.push(path);
            }
        }
    }
    stdout_paths.sort();
    stderr_paths.sort();
    HelperStreams {
        stdout: concatenate_stream_files(&stdout_paths),
        stderr: concatenate_stream_files(&stderr_paths),
    }
}

fn concatenate_stream_files(paths: &[PathBuf]) -> String {
    let mut bytes = Vec::new();
    for path in paths {
        let remaining = MAX_CAPTURE_BYTES.saturating_sub(bytes.len());
        if remaining == 0 {
            break;
        }
        let Ok(mut file) = File::open(path) else {
            continue;
        };
        let mut chunk = Vec::new();
        if file.read_to_end(&mut chunk).is_ok() {
            bytes.extend_from_slice(&chunk[..chunk.len().min(remaining)]);
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

fn parse_events(text: &str) -> EventSummary {
    let mut summary = EventSummary::default();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        match serde_json::from_str::<Value>(line) {
            Ok(value) => inspect_event(&value, &mut summary),
            Err(_) => summary.parse_errors = true,
        }
    }
    summary
}

fn inspect_event(value: &Value, summary: &mut EventSummary) {
    let Some(object) = value.as_object() else {
        return;
    };
    let typ = object
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default();
    match typ {
        "turn.completed" => summary.turn_completed = true,
        "response.completed" => {
            summary.response_completed = true;
            summary.response_count += 1;
            if let Some(response) = object.get("response") {
                inspect_usage(response, summary);
            }
        }
        "item.started" | "item.updated" | "item.completed" => {
            if let Some(item) = object.get("item") {
                inspect_item(item, summary);
            }
        }
        "error" | "turn.failed" | "response.failed" => summary.parse_errors = true,
        "function_call" | "custom_tool_call" => {
            if let Some(name) = object.get("name").and_then(Value::as_str) {
                add_tool(summary, name, object.get("call_id").and_then(Value::as_str));
            }
        }
        _ => {}
    }
    // Request/response dumps can contain nested function calls.  Walk nested
    // values here; call IDs make repeated item lifecycle events idempotent.
    for (key, child) in object {
        if key == "item" || key == "response" {
            continue;
        }
        inspect_event(child, summary);
    }
}

fn inspect_item(value: &Value, summary: &mut EventSummary) {
    let Some(object) = value.as_object() else {
        return;
    };
    let typ = object
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if typ == "command_execution" {
        let id = object
            .get("id")
            .and_then(Value::as_str)
            .or_else(|| object.get("command").and_then(Value::as_str))
            .unwrap_or("command");
        if summary.seen_ids.insert(format!("command:{id}")) {
            let command = object
                .get("command")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned();
            summary.commands.push(CommandRecord {
                // The JSONL event exposes the post-policy presentation of the
                // original command. Keep it intact for the approval audit;
                // the trusted transparent launcher is only present in the
                // process trace, never in this field.
                program: command.clone(),
            });
            add_tool(summary, "exec_command", None);
        }
    }
    if (typ == "function_call" || typ == "custom_tool_call")
        && let Some(name) = object.get("name").and_then(Value::as_str)
    {
        add_tool(summary, name, object.get("call_id").and_then(Value::as_str));
    }
}

fn inspect_usage(value: &Value, summary: &mut EventSummary) {
    let Some(object) = value.as_object() else {
        return;
    };
    if let Some(usage) = object.get("usage").and_then(Value::as_object) {
        summary.usage_responses += 1;
        summary.input_tokens += usage
            .get("input_tokens")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        summary.output_tokens += usage
            .get("output_tokens")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        if let Some(details) = usage.get("input_tokens_details").and_then(Value::as_object) {
            summary.cached_input_tokens += details
                .get("cached_tokens")
                .and_then(Value::as_i64)
                .unwrap_or(0);
        }
    }
}

fn add_tool(summary: &mut EventSummary, name: &str, call_id: Option<&str>) {
    if let Some(call_id) = call_id
        && !summary.seen_ids.insert(format!("tool:{call_id}"))
    {
        return;
    }
    *summary.tool_counts.entry(name.to_owned()).or_default() += 1;
}

fn parse_attempts(
    dumps: &Path,
    wall_start: SystemTime,
    mono_start: Instant,
    elapsed_ms: i64,
) -> Vec<AttemptRecord> {
    let mut request_paths: Vec<PathBuf> = fs::read_dir(dumps)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .map(|entry| entry.path())
        .filter(|path| path.to_string_lossy().ends_with("-request.json"))
        .collect();
    request_paths.sort();
    let mut records = Vec::new();
    for (index, request_path) in request_paths.iter().enumerate() {
        let prefix = request_path
            .to_string_lossy()
            .trim_end_matches("-request.json")
            .to_owned();
        let attempt_id = Path::new(&prefix)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("attempt")
            .to_owned();
        let response_path = PathBuf::from(format!("{prefix}-response.json"));
        let request_time = file_elapsed(request_path, wall_start, mono_start)
            .unwrap_or((index as i64).min(elapsed_ms.max(0)));
        let response_time = file_elapsed(&response_path, wall_start, mono_start);
        let response = read_json(&response_path);
        let status = response
            .as_ref()
            .and_then(|value| value.get("status"))
            .and_then(Value::as_i64)
            .map(|value| value as i32);
        let body = response
            .as_ref()
            .and_then(|value| value.get("body"))
            .cloned()
            .unwrap_or(Value::Null);
        let completed = contains_completed(&body);
        let provider_error =
            status.map(|value| value >= 400).unwrap_or(false) || contains_failure(&body);
        let transport_error = response.is_none() || (!provider_error && !completed);
        let successful =
            !provider_error && !transport_error && status.is_some_and(|v| (200..300).contains(&v));
        let usage = usage_from_body(&body);
        let mut attempt = Map::new();
        attempt.insert("attempt_id".to_owned(), json!(attempt_id));
        attempt.insert("retry_index".to_owned(), json!(index as i64));
        attempt.insert("started_ms".to_owned(), json!(request_time.max(0)));
        if let Some(status) = status {
            attempt.insert("http_status".to_owned(), json!(status));
        }
        if let Some(completed_ms) = response_time {
            attempt.insert("completed_ms".to_owned(), json!(completed_ms.max(0)));
        }
        // The stock proxy dump records completion but not the first byte.  Do
        // not invent a first-byte timestamp; probe artifacts carry that metric.
        attempt.insert("stream_disconnected".to_owned(), json!(transport_error));
        attempt.insert("provider_error".to_owned(), json!(provider_error));
        attempt.insert("recovered".to_owned(), json!(false));
        attempt.insert("input_tokens".to_owned(), json!(usage.0));
        attempt.insert("cached_input_tokens".to_owned(), json!(usage.1));
        attempt.insert("output_tokens".to_owned(), json!(usage.2));
        attempt.insert("usage_complete".to_owned(), json!(usage.3));
        if let Some(error_class) = if provider_error {
            Some(if status == Some(429) {
                "retry_after"
            } else if status.is_some_and(|v| v >= 500) {
                "http_5xx"
            } else {
                "provider_error"
            })
        } else if transport_error {
            Some(if response.is_none() {
                "transport_disconnect"
            } else {
                "malformed_sse"
            })
        } else {
            None
        } {
            attempt.insert("error_class".to_owned(), json!(error_class));
        }
        records.push(AttemptRecord {
            value: Value::Object(attempt),
            provider_error,
            transport_error,
            successful,
            usage_complete: usage.3,
        });
    }
    mark_recovered(&mut records);
    records
}

fn mark_recovered(records: &mut [AttemptRecord]) {
    // Mark only failures that are followed by a successful attempt in the
    // same request sequence. A later failure must not inherit recovery from
    // an earlier success.
    let mut successful_after = false;
    for index in (0..records.len()).rev() {
        if successful_after
            && (records[index].provider_error || records[index].transport_error)
            && let Some(object) = records[index].value.as_object_mut()
        {
            object.insert("recovered".to_owned(), json!(true));
        }
        if records[index].successful {
            successful_after = true;
        }
    }
}

fn file_elapsed(path: &Path, wall_start: SystemTime, mono_start: Instant) -> Option<i64> {
    // The proxy names each exchange with the request wall-clock timestamp.
    // Prefer that timestamp over filesystem mtime; mtime can be coarse on the
    // runner filesystem and is only a fallback for nonstandard proxy dumps.
    if let Some(timestamp_ms) = proxy_timestamp_ms(path) {
        let wall_start_ms = wall_start.duration_since(UNIX_EPOCH).ok()?.as_millis();
        let delta = timestamp_ms.saturating_sub(wall_start_ms);
        return Some(delta.min(i64::MAX as u128) as i64);
    }
    let modified = fs::metadata(path).ok()?.modified().ok()?;
    let delta = modified.duration_since(wall_start).ok()?;
    let _ = mono_start;
    Some(delta.as_millis().min(i64::MAX as u128) as i64)
}

fn proxy_timestamp_ms(path: &Path) -> Option<u128> {
    let name = path.file_name()?.to_str()?;
    let mut parts = name.split('-');
    let _sequence = parts.next()?;
    parts.next()?.parse().ok()
}

fn body_text(body: &Value) -> String {
    body.as_str()
        .map(str::to_owned)
        .unwrap_or_else(|| body.to_string())
}

fn contains_completed(body: &Value) -> bool {
    if let Some(text) = body.as_str() {
        return text.contains("response.completed") || text.contains("\"status\":\"completed\"");
    }
    body.get("type").and_then(Value::as_str) == Some("response.completed")
        || body.get("status").and_then(Value::as_str) == Some("completed")
}

fn contains_failure(body: &Value) -> bool {
    if let Some(text) = body.as_str() {
        return text.contains("response.failed") || text.contains("\"type\":\"error\"");
    }
    body.get("type")
        .and_then(Value::as_str)
        .is_some_and(|typ| typ == "response.failed" || typ == "error")
}

fn usage_from_body(body: &Value) -> (i64, i64, i64, bool) {
    let text = body_text(body);
    let mut values = Vec::new();
    if let Ok(value) = serde_json::from_str::<Value>(&text) {
        collect_usage_values(&value, &mut values);
    } else {
        for line in text.lines() {
            let payload = line.strip_prefix("data:").unwrap_or(line).trim();
            if payload.is_empty() || payload == "[DONE]" {
                continue;
            }
            if let Ok(value) = serde_json::from_str::<Value>(payload) {
                collect_usage_values(&value, &mut values);
            }
        }
    }
    values.into_iter().next().unwrap_or((0, 0, 0, false))
}

fn collect_usage_values(value: &Value, values: &mut Vec<(i64, i64, i64, bool)>) {
    let Some(object) = value.as_object() else {
        if let Some(array) = value.as_array() {
            for item in array {
                collect_usage_values(item, values);
            }
        }
        return;
    };
    if let Some(usage) = object.get("usage").and_then(Value::as_object) {
        let input = usage
            .get("input_tokens")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        let output = usage
            .get("output_tokens")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        let cached = usage
            .get("input_tokens_details")
            .and_then(Value::as_object)
            .and_then(|details| details.get("cached_tokens"))
            .and_then(Value::as_i64)
            .unwrap_or(0);
        values.push((input, cached, output, true));
    }
    for (key, child) in object {
        if key != "usage" {
            collect_usage_values(child, values);
        }
    }
}

fn attempt_input_tokens(attempts: &[AttemptRecord]) -> i64 {
    attempts
        .iter()
        .map(|attempt| {
            attempt
                .value
                .get("input_tokens")
                .and_then(Value::as_i64)
                .unwrap_or(0)
        })
        .sum()
}

fn attempt_cached_tokens(attempts: &[AttemptRecord]) -> i64 {
    attempts
        .iter()
        .map(|attempt| {
            attempt
                .value
                .get("cached_input_tokens")
                .and_then(Value::as_i64)
                .unwrap_or(0)
        })
        .sum()
}

fn attempt_output_tokens(attempts: &[AttemptRecord]) -> i64 {
    attempts
        .iter()
        .map(|attempt| {
            attempt
                .value
                .get("output_tokens")
                .and_then(Value::as_i64)
                .unwrap_or(0)
        })
        .sum()
}

fn relay_status(attempts: &[AttemptRecord]) -> &'static str {
    if attempts.iter().any(|attempt| attempt.provider_error) {
        "provider_error"
    } else if attempts.iter().any(|attempt| attempt.transport_error) {
        "transport_error"
    } else {
        "clean"
    }
}

fn snapshot(path: &Path) -> Result<Snapshot, String> {
    let metadata = fs::symlink_metadata(path).map_err(io_error)?;
    let symlink = metadata.file_type().is_symlink();
    let bytes = if symlink {
        fs::read_link(path)
            .map(|target| target.to_string_lossy().as_bytes().to_vec())
            .unwrap_or_default()
    } else {
        fs::read(path).map_err(io_error)?
    };
    Ok(Snapshot {
        mode: mode_bits(&metadata),
        symlink,
        bytes,
    })
}

fn mode_bits(metadata: &fs::Metadata) -> u32 {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o7777
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        0
    }
}

fn set_mode(path: &Path, mode: u32) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(mode));
    }
    #[cfg(not(unix))]
    {
        let _ = (path, mode);
    }
}

fn workspace_manifest(root: &Path) -> Vec<String> {
    let mut paths = Vec::new();
    collect_manifest(root, root, &mut paths);
    paths.sort();
    paths
}

fn collect_manifest(root: &Path, current: &Path, paths: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(current) else {
        return;
    };
    let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if relative == ".git" || relative.starts_with(".git/") {
            continue;
        }
        let Ok(metadata) = fs::symlink_metadata(&path) else {
            continue;
        };
        if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() {
            collect_manifest(root, &path, paths);
        } else {
            paths.push(relative);
        }
    }
}

fn manifest_equal(expected: &[String], actual: &[String]) -> bool {
    let expected: BTreeSet<_> = expected.iter().collect();
    let actual: BTreeSet<_> = actual.iter().collect();
    expected == actual
}

fn workspace_entries_safe(workspace: &Path, manifest: &[String]) -> bool {
    manifest.iter().all(|relative| {
        let Ok(path) = safe_join(workspace, relative) else {
            return false;
        };
        let Ok(metadata) = fs::symlink_metadata(path) else {
            return false;
        };
        metadata.file_type().is_file() && !metadata.file_type().is_symlink()
    })
}

fn workspace_diff_flags_clean(diff: &Value) -> bool {
    diff.get("permissions_unchanged") == Some(&Value::Bool(true))
        && diff.get("symlinks_unchanged") == Some(&Value::Bool(true))
}

fn workspace_diff(
    task: &TaskSpec,
    workspace: &Path,
    snapshots: &BTreeMap<String, Snapshot>,
    actual_manifest: &[String],
) -> Value {
    let expected: BTreeSet<String> = task.expected_manifest.iter().cloned().collect();
    let actual: BTreeSet<String> = actual_manifest.iter().cloned().collect();
    let mut all = expected.union(&actual).cloned().collect::<Vec<_>>();
    all.sort();
    let mut entries = Vec::new();
    let mut permissions_unchanged = true;
    let mut symlinks_unchanged = true;
    for relative in all {
        let path = workspace.join(&relative);
        let before = snapshots.get(&relative).map(snapshot_descriptor);
        let after = snapshot(&path)
            .ok()
            .map(|value| snapshot_descriptor(&value));
        let status = if !actual.contains(&relative) {
            "missing"
        } else if !expected.contains(&relative) {
            "unexpected"
        } else if let Some(before) = snapshots.get(&relative) {
            match snapshot(&path) {
                Ok(after) => {
                    if before.mode != after.mode {
                        permissions_unchanged = false;
                    }
                    if before.symlink != after.symlink {
                        symlinks_unchanged = false;
                    }
                    if before.bytes != after.bytes {
                        "modified"
                    } else {
                        "unchanged"
                    }
                }
                Err(_) => "missing",
            }
        } else {
            "created"
        };
        entries.push(json!({
            "path": relative,
            "status": status,
            "before": before,
            "after": after,
        }));
    }
    json!({
        "entries": entries,
        "permissions_unchanged": permissions_unchanged,
        "symlinks_unchanged": symlinks_unchanged,
    })
}

fn snapshot_descriptor(snapshot: &Snapshot) -> Value {
    let kind = if snapshot.symlink { "symlink" } else { "file" };
    let target = if snapshot.symlink {
        Some(String::from_utf8_lossy(&snapshot.bytes).into_owned())
    } else {
        None
    };
    json!({
        "kind": kind,
        "mode": snapshot.mode,
        "digest": stable_digest(&snapshot.bytes),
        "target": target,
    })
}

fn stable_digest(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn read_text_limited(path: &Path) -> Option<String> {
    let bytes = read_bytes_limited(path)?;
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

fn read_bytes_limited(path: &Path) -> Option<Vec<u8>> {
    let metadata = fs::metadata(path).ok()?;
    if !metadata.is_file() || metadata.len() > MAX_ARTIFACT_BYTES as u64 {
        return None;
    }
    fs::read(path).ok()
}

fn read_json(path: &Path) -> Option<Value> {
    let text = read_text_limited(path)?;
    serde_json::from_str(&text).ok()
}

fn write_json_atomic(path: &Path, value: &Value) {
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    if let Ok(bytes) = serde_json::to_vec(value)
        && fs::write(&temporary, bytes).is_ok()
    {
        let _ = fs::rename(temporary, path);
    }
}

fn write_bounded(path: &Path, bytes: &[u8]) {
    let end = bytes.len().min(MAX_ARTIFACT_BYTES);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, &bytes[..end]);
}

fn inputs_preserved(task: &TaskSpec, workspace: &Path) -> bool {
    for (relative, original) in &task.inputs {
        let Ok(path) = safe_join(workspace, relative) else {
            return false;
        };
        let Some(actual) = read_text_limited(&path) else {
            return false;
        };
        if task.editable.as_deref() == Some(relative.as_str()) {
            if !editable_source_valid(original, &actual) {
                return false;
            }
        } else if actual != *original {
            return false;
        }
    }
    true
}

fn editable_source_valid(original: &str, actual: &str) -> bool {
    let Some((prefix, suffix)) = original.split_once("async fn main {") else {
        return false;
    };
    actual.starts_with(prefix) && actual.ends_with(&format!("async fn main {{{suffix}"))
}

fn correct_result(task: &TaskSpec, actual: &Value) -> bool {
    if task.oracle.starts_with("json_subset") {
        json_subset(&task.expected, actual)
    } else {
        actual == &task.expected
    }
}

fn helper_streams_valid(task: &TaskSpec, streams: &HelperStreams, exit_code: i32) -> bool {
    streams.stdout == task.expected_stdout
        && streams.stderr == task.expected_stderr
        && (task.id == "host_cancel_timeout" || exit_code == task.expected_exit_code)
}

fn json_subset(expected: &Value, actual: &Value) -> bool {
    match (expected, actual) {
        (Value::Object(expected), Value::Object(actual)) => expected
            .iter()
            .all(|(key, value)| actual.get(key).is_some_and(|item| json_subset(value, item))),
        (Value::Array(expected), Value::Array(actual)) => {
            expected.len() == actual.len()
                && expected
                    .iter()
                    .zip(actual)
                    .all(|(left, right)| json_subset(left, right))
        }
        _ => expected == actual,
    }
}

fn receipt_string<'a>(value: &'a Value, key: &str) -> &'a str {
    value.get(key).and_then(Value::as_str).unwrap_or_default()
}

fn receipt_valid(task: &TaskSpec, receipt: &Value) -> bool {
    receipt.get("task").and_then(Value::as_str) == Some(task.id.as_str())
        && receipt.get("nonce").and_then(Value::as_str) == Some(task.fixture_nonce.as_str())
        && !receipt_string(receipt, "helper").is_empty()
        && matches!(receipt_string(receipt, "status"), "completed" | "stopped")
}

fn final_helper_exit(trace: &TraceSummary) -> Option<i32> {
    trace.command_exits.iter().rev().find_map(|value| {
        value
            .get("exit_code")
            .and_then(Value::as_i64)
            .map(|code| code as i32)
    })
}

fn argv_cwd_valid(task: &TaskSpec, trace: &TraceSummary, workspace: &Path) -> bool {
    if trace.command_starts.is_empty() {
        return false;
    }
    let expected_cwd = workspace.to_string_lossy();
    trace.command_starts.iter().all(|record| {
        let cwd = record
            .get("cwd")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if cwd != expected_cwd {
            return false;
        }
        let invoked_as = record
            .get("invoked_as")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if task.cohort == "process" && invoked_as != "python3" {
            return false;
        }
        let args = record
            .get("argv")
            .and_then(Value::as_array)
            .map(|values| values.iter().filter_map(Value::as_str).collect::<Vec<_>>())
            .unwrap_or_default();
        !args.iter().any(|arg| arg.contains("SHOULD_NOT_EXIST"))
    })
}

fn trace_complete(task: &TaskSpec, trace: &TraceSummary) -> bool {
    let commands =
        !trace.command_starts.is_empty() && trace.command_starts.len() == trace.command_exits.len();
    let launcher = if task.cohort == "process" || task.cohort == "script" {
        // The caller checks the backend-specific launcher count separately;
        // command trace completion itself is independent of that boundary.
        true
    } else {
        true
    };
    commands && launcher
}

fn backend_compliant(task: &TaskSpec, events: &EventSummary, trace: &TraceSummary) -> bool {
    let exec = events.tool_counts.get("exec_command").copied().unwrap_or(0);
    let write_stdin = events.tool_counts.get("write_stdin").copied().unwrap_or(0);
    let forbidden = [
        "mbtx",
        "spawn_agent",
        "js",
        "js_exec",
        "exec",
        "shell",
        "shell_command",
    ];
    exec > 0
        && (!task.background || write_stdin > 0)
        && forbidden
            .iter()
            .all(|name| events.tool_counts.get(*name).copied().unwrap_or(0) == 0)
        && !trace.command_starts.is_empty()
}

fn approval_original_command(
    events: &EventSummary,
    config: &RunnerConfig,
    workspace: &Path,
) -> bool {
    if events.commands.is_empty() {
        return false;
    }
    let launcher = config.trace_launcher.to_string_lossy();
    let mbtx = config.mbtx.to_string_lossy();
    let workspace = workspace.to_string_lossy();
    events.commands.iter().all(|command| {
        // The command exposed by Codex must remain the approved original. A
        // path in the workspace is fine; trusted launcher paths are not.
        !command.program.is_empty()
            && !command.program.contains(launcher.as_ref())
            && !command.program.contains(mbtx.as_ref())
            && !command.program.contains("m5-trace-launcher")
            && (command.program.contains(workspace.as_ref())
                || command.program.contains("python3")
                || command.program.contains("moon"))
    })
}

#[allow(clippy::too_many_arguments)]
fn classify(
    config: &RunnerConfig,
    relay_status: &str,
    observability: &str,
    backend_observation: &str,
    completed: bool,
    usage_complete: bool,
    correct_output: bool,
    inputs_preserved: bool,
    workspace_clean: bool,
    child_processes_clean: bool,
    receipt_valid: bool,
    trace_complete: bool,
    argv_cwd_valid: bool,
    approval_original_command: bool,
    timeout: bool,
) -> String {
    let input = json!({
        "relay_status": relay_status,
        "observability": observability,
        "backend_observation": backend_observation,
        "completed": completed,
        "usage_complete": usage_complete,
        "correct_output": correct_output,
        "inputs_preserved": inputs_preserved,
        "workspace_clean": workspace_clean,
        "child_processes_clean": child_processes_clean,
        "receipt_valid": receipt_valid,
        "trace_complete": trace_complete,
        "argv_cwd_valid": argv_cwd_valid,
        "approval_original_command": approval_original_command,
        "timeout": timeout,
    });
    if let Ok(mut child) = Command::new(&config.evidence)
        .env_remove("M5_API_KEY")
        .env_remove("OPENROUTER_ICU_API_KEY")
        .arg("classify")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(input.to_string().as_bytes());
        }
        if let Ok(output) = child.wait_with_output()
            && output.status.success()
            && let Ok(value) = serde_json::from_slice::<Value>(&output.stdout)
            && let Some(category) = value.get("failure_category").and_then(Value::as_str)
        {
            return category.to_owned();
        }
    }
    local_classify(
        relay_status,
        observability,
        backend_observation,
        completed,
        usage_complete,
        correct_output,
        inputs_preserved,
        workspace_clean,
        child_processes_clean,
        receipt_valid,
        trace_complete,
        argv_cwd_valid,
        approval_original_command,
        timeout,
    )
}

#[allow(clippy::too_many_arguments)]
fn local_classify(
    relay_status: &str,
    observability: &str,
    backend_observation: &str,
    completed: bool,
    usage_complete: bool,
    correct_output: bool,
    inputs_preserved: bool,
    workspace_clean: bool,
    child_processes_clean: bool,
    receipt_valid: bool,
    trace_complete: bool,
    argv_cwd_valid: bool,
    approval_original_command: bool,
    timeout: bool,
) -> String {
    if relay_status == "provider_error" || relay_status == "transport_error" {
        "infrastructure_unavailable"
    } else if timeout {
        "timeout"
    } else if observability == "unobserved" {
        "evidence_unobserved"
    } else if observability == "partial" {
        "evidence_partial"
    } else if backend_observation == "unknown" && !approval_original_command {
        "backend_observation_unknown"
    } else if backend_observation == "violation" || !approval_original_command {
        "backend_violation"
    } else if !completed {
        "agent_error"
    } else if !usage_complete {
        "missing_usage"
    } else if !inputs_preserved {
        "input_modified"
    } else if !workspace_clean
        || !child_processes_clean
        || !receipt_valid
        || !trace_complete
        || !argv_cwd_valid
    {
        "workspace_dirty"
    } else if !correct_output {
        "incorrect_output"
    } else {
        "none"
    }
    .to_owned()
}

fn persist_evidence(config: &RunnerConfig, evidence: EvidenceArtifact<'_>) {
    let EvidenceArtifact {
        block,
        backend,
        root,
        agent,
        attempts,
        trace,
        helper_streams,
        manifest,
        run,
    } = evidence;
    let block_dir = sanitize_component(&block.block_id);
    let backend_dir = sanitize_component(backend);
    let directory = config.artifact_dir.join(block_dir).join(backend_dir);
    if fs::create_dir_all(&directory).is_err() {
        return;
    }
    write_bounded(
        &directory.join("events.jsonl"),
        sanitize_output(&agent.stdout, config, root).as_bytes(),
    );
    write_bounded(
        &directory.join("stderr.txt"),
        sanitize_output(&agent.stderr, config, root).as_bytes(),
    );
    write_bounded(
        &directory.join("helper-stdout.txt"),
        sanitize_output(&helper_streams.stdout, config, root).as_bytes(),
    );
    write_bounded(
        &directory.join("helper-stderr.txt"),
        sanitize_output(&helper_streams.stderr, config, root).as_bytes(),
    );
    let attempt_values: Vec<Value> = attempts
        .iter()
        .map(|attempt| attempt.value.clone())
        .collect();
    write_json_file(
        &directory.join("attempts.json"),
        &Value::Array(attempt_values),
    );
    let trace_value = json!({
        "command_starts": trace.command_starts.clone(),
        "command_exits": trace.command_exits.clone(),
        "launcher_starts": trace.launcher_starts,
        "launcher_exits": trace.launcher_exits,
        "labels": trace.labels.clone(),
    });
    write_json_file(
        &directory.join("trace.json"),
        &redact_value(&trace_value, config, root),
    );
    write_json_file(&directory.join("manifest.json"), &json!(manifest));
    let redacted = redact_value(&Value::Object(run.clone()), config, root);
    write_json_file(&directory.join("run.json"), &redacted);
    let meta = json!({
        "schema_version": 5,
        "block_id": block.block_id,
        "backend": backend,
        "agent_exit_code": agent.exit_code,
        "timed_out": agent.timed_out,
        "elapsed_ms": agent.elapsed_ms,
        "process_launch_ms": agent.process_launch_ms,
        "process_exit_ms": agent.process_exit_ms,
        "first_stdout_ms": agent.first_stdout_ms,
        "first_stderr_ms": agent.first_stderr_ms,
    });
    write_json_file(&directory.join("meta.json"), &meta);
}

fn write_json_file(path: &Path, value: &Value) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(text) = serde_json::to_string_pretty(value) {
        let _ = fs::write(path, text + "\n");
    }
}

fn sanitize_component(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars() {
        if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
            output.push(character);
        } else {
            output.push('_');
        }
    }
    if output.is_empty() {
        "unknown".to_owned()
    } else {
        output
    }
}

fn sanitize_output(value: &str, config: &RunnerConfig, root: &Path) -> String {
    let mut output = value.to_owned();
    for path in [
        root,
        config.codex.as_path(),
        config.mbtx.as_path(),
        config.evidence.as_path(),
        config.trace_launcher.as_path(),
        config.command_trace.as_path(),
    ] {
        if !path.as_os_str().is_empty() {
            let path_text = path.to_string_lossy();
            output = output.replace(path_text.as_ref(), "<redacted-path>");
        }
    }
    if let Ok(key) =
        std::env::var("M5_API_KEY").or_else(|_| std::env::var("OPENROUTER_ICU_API_KEY"))
        && !key.is_empty()
    {
        output = output.replace(&key, "[REDACTED]");
    }
    if output.len() > MAX_ARTIFACT_BYTES {
        output.truncate(MAX_ARTIFACT_BYTES);
    }
    output
}

fn redact_value(value: &Value, config: &RunnerConfig, root: &Path) -> Value {
    match value {
        Value::String(text) => Value::String(sanitize_output(text, config, root)),
        Value::Array(values) => Value::Array(
            values
                .iter()
                .map(|value| redact_value(value, config, root))
                .collect(),
        ),
        Value::Object(fields) => Value::Object(
            fields
                .iter()
                .map(|(key, value)| (key.clone(), redact_value(value, config, root)))
                .collect(),
        ),
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subset_oracle_accepts_host_fields() {
        assert!(json_subset(
            &json!({"nonce":"n"}),
            &json!({"nonce":"n", "cwd":"/private"})
        ));
        assert!(!json_subset(&json!({"nonce":"n"}), &json!({"nonce":"x"})));
    }

    #[test]
    fn unsafe_paths_are_rejected() {
        let root = Path::new("/tmp/work");
        assert!(safe_join(root, "a/b.txt").is_ok());
        assert!(safe_join(root, "../answer.txt").is_err());
        assert!(safe_join(root, "/etc/passwd").is_err());
    }

    #[test]
    fn classification_keeps_unknown_observation_separate() {
        assert_eq!(
            local_classify(
                "clean", "complete", "unknown", true, true, true, true, true, true, true, true,
                true, false, false
            ),
            "backend_observation_unknown"
        );
    }

    #[test]
    fn manifest_comparison_is_set_exact() {
        assert!(manifest_equal(&["a".to_owned()], &["a".to_owned()]));
        assert!(!manifest_equal(&["a".to_owned()], &["b".to_owned()]));
    }

    #[test]
    fn helper_streams_are_concatenated_and_checked_against_fixture_contract() {
        let directory = unique_temp_dir("m5-runner-test").expect("temporary directory");
        let _cleanup = CleanupDir(directory.clone());
        fs::write(directory.join("command-output-001.stdout"), "started\n")
            .expect("stdout fixture");
        fs::write(directory.join("command-output-001.stderr"), "").expect("stderr fixture");
        fs::write(directory.join("command-output-002.stdout"), "complete\n")
            .expect("stdout fixture");
        fs::write(directory.join("command-output-002.stderr"), "").expect("stderr fixture");
        let streams = read_helper_streams(&directory);
        let task = TaskSpec {
            id: "host_background".to_owned(),
            cohort: "process".to_owned(),
            prompt: String::new(),
            inputs: BTreeMap::new(),
            expected: Value::Null,
            expected_stdout: "started\ncomplete\n".to_owned(),
            expected_stderr: String::new(),
            expected_exit_code: 0,
            expected_manifest: Vec::new(),
            editable: None,
            background: true,
            fixture_nonce: "nonce".to_owned(),
            receipt_path: ".m5/receipt.json".to_owned(),
            oracle: String::new(),
        };
        assert!(helper_streams_valid(&task, &streams, 0));
        assert!(!helper_streams_valid(&task, &streams, 1));
    }

    #[test]
    fn workspace_diff_contains_redactable_file_metadata() {
        let directory = unique_temp_dir("m5-diff-test").expect("temporary directory");
        let _cleanup = CleanupDir(directory.clone());
        let path = directory.join("input.txt");
        fs::write(&path, "content").expect("input fixture");
        let before = snapshot(&path).expect("snapshot");
        fs::write(&path, "changed").expect("modified fixture");
        let task = TaskSpec {
            id: "task".to_owned(),
            cohort: "process".to_owned(),
            prompt: String::new(),
            inputs: [("input.txt".to_owned(), "content".to_owned())]
                .into_iter()
                .collect(),
            expected: Value::Null,
            expected_stdout: String::new(),
            expected_stderr: String::new(),
            expected_exit_code: 0,
            expected_manifest: vec!["input.txt".to_owned()],
            editable: None,
            background: false,
            fixture_nonce: "nonce".to_owned(),
            receipt_path: ".m5/receipt.json".to_owned(),
            oracle: String::new(),
        };
        let snapshots = [("input.txt".to_owned(), before)].into_iter().collect();
        let diff = workspace_diff(&task, &directory, &snapshots, &["input.txt".to_owned()]);
        assert_eq!(diff["entries"][0]["status"], "modified");
        assert!(diff["entries"][0]["after"]["digest"].is_string());
        assert!(diff["entries"][0]["after"]["mode"].is_number());
    }

    #[test]
    fn trace_labels_follow_cross_process_timestamps() {
        let directory = unique_temp_dir("m5-trace-order-test").expect("temporary directory");
        let _cleanup = CleanupDir(directory.clone());
        let command = directory.join("command.jsonl");
        let launcher = directory.join("launcher.jsonl");
        fs::write(
            &command,
            format!(
                "{}\n{}\n",
                json!({
                    "event": "command_start",
                    "started_unix_ns": 20,
                }),
                json!({
                    "event": "command_exit",
                    "finished_unix_ns": 40,
                    "exit_code": 0,
                }),
            ),
        )
        .expect("command trace");
        fs::write(
            &launcher,
            format!(
                "{}\n{}\n",
                json!({
                    "event": "launcher_start",
                    "started_unix_ns": 10,
                }),
                json!({
                    "event": "launcher_exit",
                    "finished_unix_ns": 50,
                    "exit_code": 0,
                }),
            ),
        )
        .expect("launcher trace");
        let trace = read_trace(&command, &launcher);
        assert_eq!(
            trace.labels,
            vec![
                "launcher_start",
                "command_start",
                "command_exit",
                "launcher_exit",
            ]
        );
    }

    #[test]
    fn proxy_dump_timestamp_is_read_from_exchange_name() {
        assert_eq!(
            proxy_timestamp_ms(Path::new("000042-1700000000123-request.json")),
            Some(1_700_000_000_123)
        );
        assert_eq!(proxy_timestamp_ms(Path::new("response.json")), None);
    }

    #[test]
    fn workspace_safety_rejects_symlink_entries() {
        let directory = unique_temp_dir("m5-workspace-safety-test").expect("temporary directory");
        let _cleanup = CleanupDir(directory.clone());
        fs::write(directory.join("file.txt"), "ok").expect("regular file");
        assert!(workspace_entries_safe(&directory, &["file.txt".to_owned()]));
        #[cfg(unix)]
        std::os::unix::fs::symlink("file.txt", directory.join("link.txt"))
            .expect("symlink fixture");
        #[cfg(unix)]
        assert!(!workspace_entries_safe(
            &directory,
            &["link.txt".to_owned()]
        ));
    }

    #[test]
    fn retry_recovery_marks_only_failures_before_success() {
        let mut failed_before = Map::new();
        failed_before.insert("recovered".to_owned(), json!(false));
        let mut success = Map::new();
        success.insert("recovered".to_owned(), json!(false));
        let mut failed_after = Map::new();
        failed_after.insert("recovered".to_owned(), json!(false));
        let records = vec![
            AttemptRecord {
                value: Value::Object(failed_before),
                provider_error: true,
                transport_error: false,
                successful: false,
                usage_complete: false,
            },
            AttemptRecord {
                value: Value::Object(success),
                provider_error: false,
                transport_error: false,
                successful: true,
                usage_complete: true,
            },
            AttemptRecord {
                value: Value::Object(failed_after),
                provider_error: true,
                transport_error: false,
                successful: false,
                usage_complete: false,
            },
        ];
        let mut records = records;
        mark_recovered(&mut records);
        assert_eq!(records[0].value["recovered"], json!(true));
        assert_eq!(records[1].value["recovered"], json!(false));
        assert_eq!(records[2].value["recovered"], json!(false));
    }
}
