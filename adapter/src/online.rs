use serde::Serialize;
use serde_json::{Value, json};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(unix)]
use std::os::unix::process::{CommandExt, ExitStatusExt};

const EXPERIMENT_ID: &str = "codex-relay-online-v1";
const DEFAULT_MODEL: &str = "gpt-5.6-terra";
const DEFAULT_RELAY_BASE_URL: &str = "https://tokenadvent.com/v1";
const DEFAULT_TIMEOUT_MS: u64 = 300_000;

#[derive(Clone, Copy)]
struct Task {
    id: &'static str,
    command: &'static str,
    expected_exit: i32,
    marker: &'static str,
    timeout_ms: u64,
    forbidden_path: Option<&'static str>,
}

fn tasks() -> [Task; 8] {
    [
        Task {
            id: "literal_argv",
            command: "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'",
            expected_exit: 0,
            marker: "SHOULD_NOT_EXIST",
            timeout_ms: DEFAULT_TIMEOUT_MS,
            forbidden_path: Some("SHOULD_NOT_EXIST"),
        },
        Task {
            id: "stdin_utf8",
            command: "printf '第一行\\n\\n最后一行\\n' | python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'",
            expected_exit: 0,
            marker: "最后一行",
            timeout_ms: DEFAULT_TIMEOUT_MS,
            forbidden_path: None,
        },
        Task {
            id: "cwd_environment",
            command: "printf '%s|%s\\n' \"$PWD\" \"$MBTX_EVAL_TOKEN\"",
            expected_exit: 0,
            marker: "online-fixture",
            timeout_ms: DEFAULT_TIMEOUT_MS,
            forbidden_path: None,
        },
        Task {
            id: "large_output",
            command: "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'",
            expected_exit: 0,
            marker: "xxxxxxxx",
            timeout_ms: DEFAULT_TIMEOUT_MS,
            forbidden_path: None,
        },
        Task {
            id: "nonzero_exit",
            command: "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'",
            expected_exit: 7,
            marker: "expected failure",
            timeout_ms: DEFAULT_TIMEOUT_MS,
            forbidden_path: None,
        },
        Task {
            id: "background_cleanup",
            command: "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'",
            expected_exit: 0,
            marker: "child-clean",
            timeout_ms: DEFAULT_TIMEOUT_MS,
            forbidden_path: None,
        },
        Task {
            id: "signal_exit",
            command: "python3 -c 'import subprocess,time; subprocess.Popen([\"sh\",\"-c\",\"trap \\\"\\\" TERM; sleep 10\"]); print(\"term-start\", flush=True); time.sleep(10)'",
            expected_exit: 143,
            marker: "term-start",
            timeout_ms: 250,
            forbidden_path: None,
        },
        Task {
            id: "repeated_recovery",
            command: "python3 -c 'print(\"recovery-ok\")'",
            expected_exit: 0,
            marker: "recovery-ok",
            timeout_ms: DEFAULT_TIMEOUT_MS,
            forbidden_path: None,
        },
    ]
}

#[derive(Debug, Serialize)]
struct AttemptRecord {
    pair_id: String,
    attempt_id: String,
    task_id: String,
    backend: String,
    status: String,
    failure_class: Option<String>,
    codex_exit_code: Option<i32>,
    codex_signal: Option<i32>,
    command_exit_code: Option<i32>,
    command_status: Option<String>,
    elapsed_ms: u128,
    stdout_bytes: usize,
    stderr_bytes: usize,
    json_lines: usize,
    parse_errors: usize,
    artifact_dir: String,
}

#[derive(Debug)]
struct ParsedCodex {
    command: Option<String>,
    aggregated_output: String,
    command_exit_code: Option<i32>,
    command_status: Option<String>,
    messages: Vec<String>,
    json_lines: usize,
    parse_errors: usize,
}

#[derive(Debug)]
struct ProcessOutput {
    status: Option<ExitStatus>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    elapsed_ms: u128,
    timed_out: bool,
    spawn_error: Option<String>,
}

fn usage() -> ! {
    eprintln!(
        "usage: mbtx-online run --codex CODEX --mbtx MBTX --output DIR [--model MODEL] [--relay-base-url URL] [--pairs N] [--timeout-ms N]"
    );
    std::process::exit(2)
}

fn option(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == name)
        .and_then(|index| args.get(index + 1))
        .cloned()
}

fn required_option(args: &[String], name: &str) -> String {
    option(args, name).unwrap_or_else(|| {
        eprintln!("missing required option {name}");
        usage()
    })
}

fn parse_positive(args: &[String], name: &str, default: u64) -> u64 {
    option(args, name)
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn toml_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn config_text(model: &str, relay_base_url: &str, mbtx: Option<&Path>) -> String {
    let launcher = mbtx.map(|path| {
        format!(
            "\n# Transparent launcher is trusted configuration, outside project-local control.\nmbtx_backend = \"transparent\"\nmbtx_command = [\"{}\"]\n",
            toml_string(&path.to_string_lossy())
        )
    });
    format!(
        "model_provider = \"OpenrouterICU\"\nmodel = \"{}\"\nreview_model = \"{}\"\nmodel_reasoning_effort = \"xhigh\"\napproval_policy = \"never\"\nsandbox_mode = \"workspace-write\"\nexperimental_use_unified_exec_tool = true{}\n\n[model_providers.OpenrouterICU]\nname = \"OpenRouter ICU\"\nbase_url = \"{}\"\nwire_api = \"responses\"\nrequires_openai_auth = true\nrequest_max_retries = 0\nstream_max_retries = 0\n\n[sandbox_workspace_write]\nnetwork_access = true\n\n[features]\ngoals = true\n",
        toml_string(model),
        toml_string(model),
        launcher.unwrap_or_default(),
        toml_string(relay_base_url),
    )
}

fn create_unique_dir(parent: &Path, base: &str) -> std::io::Result<PathBuf> {
    for suffix in 0..1000_u32 {
        let name = if suffix == 0 {
            base.to_string()
        } else {
            format!("{base}-{suffix}")
        };
        let path = parent.join(name);
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        "could not allocate an immutable attempt directory",
    ))
}

fn monotonic_ns(origin: &Instant) -> u128 {
    origin.elapsed().as_nanos()
}

fn event_base(
    pair_id: &str,
    attempt_id: &str,
    task_id: &str,
    backend: &str,
    phase: &str,
    event: &str,
    origin: &Instant,
) -> Value {
    json!({
        "schema_version": 1,
        "experiment_id": EXPERIMENT_ID,
        "pair_id": pair_id,
        "attempt_id": attempt_id,
        "task_id": task_id,
        "backend": backend,
        "phase": phase,
        "event": event,
        "parent_id": Value::Null,
        "monotonic_ns": monotonic_ns(origin),
        "clock_domain": "observer-monotonic",
        "exit_code": Value::Null,
        "signal": Value::Null,
        "stdout_bytes": Value::Null,
        "stderr_bytes": Value::Null,
        "status": "unknown",
        "failure_class": Value::Null,
    })
}

fn write_event(file: &mut File, event: Value) -> std::io::Result<()> {
    if !event.is_object() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "evidence event must be an object",
        ));
    }
    let line = serde_json::to_string(&event).map_err(std::io::Error::other)?;
    file.write_all(line.as_bytes())?;
    file.write_all(b"\n")?;
    file.flush()
}

fn parse_codex_output(stdout: &[u8]) -> ParsedCodex {
    let mut parsed = ParsedCodex {
        command: None,
        aggregated_output: String::new(),
        command_exit_code: None,
        command_status: None,
        messages: Vec::new(),
        json_lines: 0,
        parse_errors: 0,
    };
    for line in String::from_utf8_lossy(stdout).lines() {
        if line.trim().is_empty() {
            continue;
        }
        let value: Value = match serde_json::from_str(line) {
            Ok(value) => {
                parsed.json_lines += 1;
                value
            }
            Err(_) => {
                parsed.parse_errors += 1;
                continue;
            }
        };
        let event_type = value.get("type").and_then(Value::as_str).unwrap_or("");
        if event_type == "error" {
            if let Some(message) = value.get("message").and_then(Value::as_str) {
                parsed.messages.push(message.to_string());
            }
        }
        if event_type == "turn.failed" {
            if let Some(message) = value
                .get("error")
                .and_then(|error| error.get("message"))
                .and_then(Value::as_str)
            {
                parsed.messages.push(message.to_string());
            }
        }
        if event_type != "item.completed" {
            continue;
        }
        let Some(item) = value.get("item") else {
            continue;
        };
        let item_type = item.get("type").and_then(Value::as_str).unwrap_or("");
        if item_type != "command_execution" && item_type != "commandExecution" {
            continue;
        }
        parsed.command = item
            .get("command")
            .and_then(Value::as_str)
            .map(str::to_string);
        parsed.aggregated_output = item
            .get("aggregated_output")
            .or_else(|| item.get("aggregatedOutput"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        parsed.command_exit_code = item
            .get("exit_code")
            .or_else(|| item.get("exitCode"))
            .and_then(Value::as_i64)
            .and_then(|code| i32::try_from(code).ok());
        parsed.command_status = item
            .get("status")
            .and_then(Value::as_str)
            .map(str::to_string);
    }
    parsed
}

fn text_has_any(text: &str, needles: &[&str]) -> bool {
    let lower = text.to_ascii_lowercase();
    needles.iter().any(|needle| lower.contains(needle))
}

fn classify_external_failure(parsed: &ParsedCodex, stderr: &str) -> (String, String) {
    let messages = parsed.messages.join(" ");
    let combined = format!("{messages} {stderr}");
    if text_has_any(
        &combined,
        &[
            "429",
            "502",
            "503",
            "504",
            "disconnect",
            "connection reset",
            "connection refused",
            "upstream",
            "gateway",
            "tokenadvent.com",
        ],
    ) {
        ("relay_error".to_string(), "relay_or_transport".to_string())
    } else if text_has_any(
        &combined,
        &[
            "401",
            "403",
            "404",
            "invalid api key",
            "unauthorized",
            "quota",
            "model not found",
            "provider",
        ],
    ) {
        (
            "provider_error".to_string(),
            "provider_response".to_string(),
        )
    } else if parsed.parse_errors > 0 {
        (
            "harness_error".to_string(),
            "invalid_codex_jsonl".to_string(),
        )
    } else {
        ("unknown".to_string(), "missing_command_event".to_string())
    }
}

fn kill_process_group(child: &mut std::process::Child) {
    #[cfg(unix)]
    {
        let group = format!("-{}", child.id());
        let _ = Command::new("kill")
            .args(["-TERM", "--", &group])
            .stderr(Stdio::null())
            .status();
        thread::sleep(Duration::from_millis(500));
        let _ = Command::new("kill")
            .args(["-KILL", "--", &group])
            .stderr(Stdio::null())
            .status();
    }
    #[cfg(not(unix))]
    {
        let _ = child.kill();
    }
}

fn read_pipe<T: Read>(mut pipe: T) -> Vec<u8> {
    let mut bytes = Vec::new();
    let _ = pipe.read_to_end(&mut bytes);
    bytes
}

fn run_process(
    program: &Path,
    args: &[String],
    cwd: &Path,
    codex_home: &Path,
    timeout: Duration,
    env_token: &str,
) -> ProcessOutput {
    let started = Instant::now();
    let mut command = Command::new(program);
    command
        .args(args)
        .current_dir(cwd)
        .env("CODEX_HOME", codex_home)
        .env("MBTX_EVAL_TOKEN", env_token)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    command.process_group(0);
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            return ProcessOutput {
                status: None,
                stdout: Vec::new(),
                stderr: Vec::new(),
                elapsed_ms: started.elapsed().as_millis(),
                timed_out: false,
                spawn_error: Some(error.to_string()),
            };
        }
    };
    let stdout = child
        .stdout
        .take()
        .map(|pipe| thread::spawn(|| read_pipe(pipe)));
    let stderr = child
        .stderr
        .take()
        .map(|pipe| thread::spawn(|| read_pipe(pipe)));
    let deadline = Instant::now() + timeout;
    let mut timed_out = false;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if Instant::now() >= deadline => {
                timed_out = true;
                kill_process_group(&mut child);
                break child.wait().ok();
            }
            Ok(None) => thread::sleep(Duration::from_millis(25)),
            Err(_) => break child.wait().ok(),
        }
    };
    let stdout = stdout
        .map(|reader| reader.join().unwrap_or_default())
        .unwrap_or_default();
    let stderr = stderr
        .map(|reader| reader.join().unwrap_or_default())
        .unwrap_or_default();
    ProcessOutput {
        status,
        stdout,
        stderr,
        elapsed_ms: started.elapsed().as_millis(),
        timed_out,
        spawn_error: None,
    }
}

fn status_code(status: Option<&ExitStatus>) -> Option<i32> {
    status.and_then(ExitStatus::code)
}

fn status_signal(status: Option<&ExitStatus>) -> Option<i32> {
    #[cfg(unix)]
    {
        status.and_then(ExitStatusExt::signal)
    }
    #[cfg(not(unix))]
    {
        let _ = status;
        None
    }
}

fn run_attempt(
    codex: &Path,
    mbtx: &Path,
    output: &Path,
    events: &mut File,
    task: Task,
    pair_id: &str,
    backend: &str,
    pair_index: usize,
    model: &str,
    relay_base_url: &str,
    timeout: Duration,
    origin: &Instant,
) -> AttemptRecord {
    let attempt_id = format!(
        "attempt-{pair_index:02}-{task_id}-{backend}",
        task_id = task.id
    );
    let pair_dir = output.join("attempts").join(pair_id);
    fs::create_dir_all(&pair_dir).expect("create pair evidence directory");
    let artifact_dir = create_unique_dir(&pair_dir, backend)
        .unwrap_or_else(|error| panic!("cannot create immutable artifact directory: {error}"));
    let artifact_relative = artifact_dir
        .strip_prefix(output)
        .unwrap_or(&artifact_dir)
        .to_string_lossy()
        .replace('\\', "/");
    let prompt = format!(
        "Use the exec_command tool exactly once. Set its cmd field to this exact shell command, preserving every character. Do not rewrite it, split it, or call any other tool. Wait for the command to finish, then reply exactly DONE.\n\n{}",
        task.command
    );
    fs::write(artifact_dir.join("prompt.txt"), &prompt).expect("write prompt evidence");
    let home = output.join("homes").join(&attempt_id);
    fs::create_dir_all(&home).expect("create isolated Codex home");
    fs::write(
        home.join("config.toml"),
        config_text(
            model,
            relay_base_url,
            (backend == "transparent").then_some(mbtx),
        ),
    )
    .expect("write isolated Codex config");
    let workspace = output.join("workspaces").join(&attempt_id);
    fs::create_dir_all(&workspace).expect("create isolated arm workspace");
    let args = vec![
        "exec".to_string(),
        "--json".to_string(),
        "--strict-config".to_string(),
        "--ephemeral".to_string(),
        "--ignore-rules".to_string(),
        "--skip-git-repo-check".to_string(),
        "--cd".to_string(),
        workspace.to_string_lossy().into_owned(),
        "--sandbox".to_string(),
        "workspace-write".to_string(),
        "--model".to_string(),
        model.to_string(),
        prompt,
    ];
    let mut start_event = event_base(
        pair_id,
        &attempt_id,
        task.id,
        backend,
        "codex",
        "start",
        origin,
    );
    start_event["artifact_dir"] = json!(artifact_relative);
    start_event["command"] = json!(task.command);
    write_event(events, start_event).expect("write start event");
    let task_timeout = timeout.min(Duration::from_millis(task.timeout_ms));
    let process = run_process(
        codex,
        &args,
        &workspace,
        &home,
        task_timeout,
        "online-fixture",
    );
    fs::write(artifact_dir.join("codex.stdout.jsonl"), &process.stdout)
        .expect("write stdout evidence");
    fs::write(artifact_dir.join("codex.stderr.log"), &process.stderr)
        .expect("write stderr evidence");
    fs::write(
        artifact_dir.join("launcher.txt"),
        if backend == "transparent" {
            format!("{} exec -- <Codex resolved command>\n", mbtx.display())
        } else {
            "Codex default shell launcher\n".to_string()
        },
    )
    .expect("write launcher evidence");
    let parsed = parse_codex_output(&process.stdout);
    let codex_exit_code = status_code(process.status.as_ref());
    let codex_signal = status_signal(process.status.as_ref());
    let stderr = String::from_utf8_lossy(&process.stderr);
    let forbidden_path_exists = task
        .forbidden_path
        .map(|path| workspace.join(path).exists())
        .unwrap_or(false);
    let (status, failure_class) = if process.spawn_error.is_some() {
        ("harness_error".to_string(), Some("codex_spawn".to_string()))
    } else if process.timed_out {
        ("timeout".to_string(), Some("codex_timeout".to_string()))
    } else if parsed.command_exit_code.is_none() {
        let (status, class) = classify_external_failure(&parsed, &stderr);
        (status, Some(class))
    } else if parsed.command_exit_code != Some(task.expected_exit) {
        (
            "backend_failure".to_string(),
            Some("command_exit_mismatch".to_string()),
        )
    } else if forbidden_path_exists {
        (
            "backend_failure".to_string(),
            Some("unexpected_side_effect".to_string()),
        )
    } else if !task.marker.is_empty() && !parsed.aggregated_output.contains(task.marker) {
        (
            "backend_failure".to_string(),
            Some("command_output_mismatch".to_string()),
        )
    } else {
        ("success".to_string(), None)
    };
    let mut exit_event = event_base(
        pair_id,
        &attempt_id,
        task.id,
        backend,
        "codex",
        "exit",
        origin,
    );
    exit_event["exit_code"] = codex_exit_code.map_or(Value::Null, |code| json!(code));
    exit_event["signal"] = codex_signal.map_or(Value::Null, |signal| json!(signal));
    exit_event["stdout_bytes"] = json!(process.stdout.len());
    exit_event["stderr_bytes"] = json!(process.stderr.len());
    exit_event["status"] = json!(status);
    exit_event["failure_class"] = failure_class
        .clone()
        .map_or(Value::Null, |class| json!(class));
    exit_event["artifact_dir"] = json!(artifact_relative);
    exit_event["elapsed_ms"] = json!(process.elapsed_ms);
    exit_event["command"] = json!(parsed.command);
    exit_event["command_exit_code"] = parsed
        .command_exit_code
        .map_or(Value::Null, |code| json!(code));
    exit_event["command_status"] = parsed
        .command_status
        .clone()
        .map_or(Value::Null, |value| json!(value));
    exit_event["aggregated_output_bytes"] = json!(parsed.aggregated_output.len());
    write_event(events, exit_event).expect("write exit event");
    let mut reap_event = event_base(
        pair_id,
        &attempt_id,
        task.id,
        backend,
        "codex",
        "reap",
        origin,
    );
    reap_event["exit_code"] = codex_exit_code.map_or(Value::Null, |code| json!(code));
    reap_event["signal"] = codex_signal.map_or(Value::Null, |signal| json!(signal));
    reap_event["status"] = json!(status);
    reap_event["failure_class"] = failure_class
        .clone()
        .map_or(Value::Null, |class| json!(class));
    reap_event["artifact_dir"] = json!(artifact_relative);
    write_event(events, reap_event).expect("write reap event");
    let mut drain_event = event_base(
        pair_id,
        &attempt_id,
        task.id,
        backend,
        "io",
        "drain",
        origin,
    );
    drain_event["stdout_bytes"] = json!(process.stdout.len());
    drain_event["stderr_bytes"] = json!(process.stderr.len());
    drain_event["status"] = json!(status);
    drain_event["failure_class"] = failure_class
        .clone()
        .map_or(Value::Null, |class| json!(class));
    drain_event["artifact_dir"] = json!(artifact_relative);
    write_event(events, drain_event).expect("write IO drain event");
    if parsed.command.is_some() {
        let mut child_event = event_base(
            pair_id,
            &attempt_id,
            task.id,
            backend,
            "child",
            "exit",
            origin,
        );
        child_event["exit_code"] = parsed
            .command_exit_code
            .map_or(Value::Null, |code| json!(code));
        child_event["stdout_bytes"] = json!(parsed.aggregated_output.len());
        child_event["status"] = json!(status);
        child_event["failure_class"] = failure_class
            .clone()
            .map_or(Value::Null, |class| json!(class));
        child_event["artifact_dir"] = json!(artifact_relative);
        child_event["command"] = json!(parsed.command);
        child_event["aggregated_output_bytes"] = json!(parsed.aggregated_output.len());
        write_event(events, child_event).expect("write child event");
    }
    if status == "relay_error" || status == "provider_error" {
        let mut relay_event = event_base(
            pair_id,
            &attempt_id,
            task.id,
            backend,
            "relay",
            "error",
            origin,
        );
        relay_event["status"] = json!(status);
        relay_event["failure_class"] = failure_class
            .clone()
            .map_or(Value::Null, |class| json!(class));
        relay_event["artifact_dir"] = json!(artifact_relative);
        relay_event["error_messages"] = json!(parsed.messages);
        write_event(events, relay_event).expect("write relay event");
    }
    let metadata = json!({
        "experiment_id": EXPERIMENT_ID,
        "pair_id": pair_id,
        "attempt_id": attempt_id,
        "task_id": task.id,
        "backend": backend,
        "expected_command_exit": task.expected_exit,
        "expected_marker": task.marker,
        "forbidden_path": task.forbidden_path,
        "forbidden_path_exists": forbidden_path_exists,
        "timeout_ms": task.timeout_ms,
        "model": model,
        "codex_home": home,
        "workspace": workspace,
        "artifact_dir": artifact_relative,
        "status": status,
        "failure_class": failure_class,
        "codex_exit_code": codex_exit_code,
        "codex_signal": codex_signal,
        "command": parsed.command,
        "command_exit_code": parsed.command_exit_code,
        "command_status": parsed.command_status,
        "aggregated_output": parsed.aggregated_output,
        "json_lines": parsed.json_lines,
        "parse_errors": parsed.parse_errors,
        "elapsed_ms": process.elapsed_ms,
    });
    fs::write(
        artifact_dir.join("metadata.json"),
        serde_json::to_vec_pretty(&metadata).expect("serialize attempt metadata"),
    )
    .expect("write attempt metadata");
    AttemptRecord {
        pair_id: pair_id.to_string(),
        attempt_id,
        task_id: task.id.to_string(),
        backend: backend.to_string(),
        status,
        failure_class,
        codex_exit_code,
        codex_signal,
        command_exit_code: parsed.command_exit_code,
        command_status: parsed.command_status,
        elapsed_ms: process.elapsed_ms,
        stdout_bytes: process.stdout.len(),
        stderr_bytes: process.stderr.len(),
        json_lines: parsed.json_lines,
        parse_errors: parsed.parse_errors,
        artifact_dir: artifact_relative,
    }
}

fn write_summary(
    output: &Path,
    records: &[AttemptRecord],
    planned_pairs: usize,
    stop_reason: Option<&str>,
) {
    let successful_arms = records
        .iter()
        .filter(|record| record.status == "success")
        .count();
    let complete_pairs = records
        .chunks(2)
        .filter(|pair| pair.len() == 2 && pair.iter().all(|record| record.status == "success"))
        .count();
    let summary = json!({
        "schema_version": 1,
        "experiment_id": EXPERIMENT_ID,
        "planned_pairs": planned_pairs,
        "attempts": records.len(),
        "successful_arms": successful_arms,
        "complete_pairs": complete_pairs,
        "partial": complete_pairs < planned_pairs,
        "stop_reason": stop_reason,
        "records": records,
    });
    fs::write(
        output.join("summary.json"),
        serde_json::to_vec_pretty(&summary).expect("serialize summary"),
    )
    .expect("write summary");
    let mut markdown = String::from("# Codex relay online collection\n\n");
    markdown.push_str(&format!(
        "- Planned pairs: {}\n- Recorded arms: {}\n- Successful arms: {}\n- Complete pairs: {}\n- Partial: {}\n",
        planned_pairs,
        records.len(),
        successful_arms,
        complete_pairs,
        complete_pairs < planned_pairs
    ));
    if let Some(reason) = stop_reason {
        markdown.push_str(&format!("- Stop reason: `{reason}`\n"));
    }
    markdown.push_str("\n| Pair | Task | Backend | Status | Failure | Command exit | Evidence |\n|---|---|---|---|---|---:|---|\n");
    for record in records {
        let relative = Path::new(&record.artifact_dir)
            .strip_prefix(output)
            .unwrap_or_else(|_| Path::new(&record.artifact_dir));
        markdown.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            record.pair_id,
            record.task_id,
            record.backend,
            record.status,
            record.failure_class.as_deref().unwrap_or(""),
            record
                .command_exit_code
                .map_or_else(|| "".to_string(), |code| code.to_string()),
            relative.display()
        ));
    }
    fs::write(output.join("online-summary.md"), markdown).expect("write markdown summary");
}

fn run(args: &[String]) {
    if env::var("OPENAI_API_KEY").map_or(true, |key| key.is_empty()) {
        eprintln!("OPENAI_API_KEY is required in the environment; it is never written to evidence");
        std::process::exit(2);
    }
    let codex = PathBuf::from(required_option(args, "--codex"));
    let mbtx = PathBuf::from(required_option(args, "--mbtx"));
    let output = PathBuf::from(required_option(args, "--output"));
    let model = option(args, "--model").unwrap_or_else(|| DEFAULT_MODEL.to_string());
    let relay_base_url =
        option(args, "--relay-base-url").unwrap_or_else(|| DEFAULT_RELAY_BASE_URL.to_string());
    let pairs = parse_positive(args, "--pairs", 4).clamp(1, 6) as usize;
    let timeout = Duration::from_millis(parse_positive(args, "--timeout-ms", DEFAULT_TIMEOUT_MS));
    if !codex.is_file() || !mbtx.is_file() {
        eprintln!("codex and mbtx must be existing files");
        std::process::exit(2);
    }
    if output.join("events.jsonl").exists() {
        eprintln!("refusing to append to existing evidence; choose a new output directory");
        std::process::exit(2);
    }
    fs::create_dir_all(&output).expect("create output directory");
    fs::create_dir_all(output.join("attempts")).expect("create attempts directory");
    let shell_home = output.join("config").join("shell");
    let transparent_home = output.join("config").join("transparent");
    fs::create_dir_all(&shell_home).expect("create shell config directory");
    fs::create_dir_all(&transparent_home).expect("create transparent config directory");
    fs::write(
        shell_home.join("config.toml"),
        config_text(&model, &relay_base_url, None),
    )
    .expect("write shell config");
    fs::write(
        transparent_home.join("config.toml"),
        config_text(&model, &relay_base_url, Some(&mbtx)),
    )
    .expect("write transparent config");
    fs::create_dir_all(output.join("homes")).expect("create homes directory");
    fs::create_dir_all(output.join("workspaces")).expect("create workspaces directory");
    let mut events = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(output.join("events.jsonl"))
        .expect("create immutable event log");
    let origin = Instant::now();
    let task_list = tasks();
    let planned_pairs = pairs * task_list.len();
    println!(
        "[online] plan={} pairs={} arms={} model={} relay={}",
        EXPERIMENT_ID,
        planned_pairs,
        planned_pairs * 2,
        model,
        relay_base_url
    );
    let mut records = Vec::new();
    let mut consecutive_infra_failures = 0_usize;
    let mut recent_statuses: Vec<String> = Vec::new();
    let mut stop_reason = None;
    'rounds: for pair_index in 0..pairs {
        for (task_index, task) in task_list.iter().copied().enumerate() {
            let pair_id = format!("pair-{pair_index:02}-{}", task.id);
            let first_transparent = (pair_index + task_index) % 2 == 1;
            let order = if first_transparent {
                ["transparent", "shell"]
            } else {
                ["shell", "transparent"]
            };
            for backend in order {
                println!("[online] pair={pair_id} backend={backend} task={}", task.id);
                let record = run_attempt(
                    &codex,
                    &mbtx,
                    &output,
                    &mut events,
                    task,
                    &pair_id,
                    backend,
                    pair_index,
                    &model,
                    &relay_base_url,
                    timeout,
                    &origin,
                );
                let infrastructure = matches!(
                    record.status.as_str(),
                    "relay_error" | "provider_error" | "harness_error" | "timeout"
                );
                if infrastructure {
                    consecutive_infra_failures += 1;
                } else {
                    consecutive_infra_failures = 0;
                }
                recent_statuses.push(record.status.clone());
                if recent_statuses.len() > 16 {
                    recent_statuses.remove(0);
                }
                records.push(record);
                if consecutive_infra_failures >= 5 {
                    stop_reason = Some("five consecutive infrastructure failures");
                    break 'rounds;
                }
                if recent_statuses.len() == 16
                    && recent_statuses
                        .iter()
                        .filter(|status| {
                            matches!(
                                status.as_str(),
                                "relay_error" | "provider_error" | "harness_error"
                            )
                        })
                        .count()
                        * 2
                        >= recent_statuses.len()
                {
                    stop_reason = Some(
                        "at least half of the latest 16 arms are external or harness failures",
                    );
                    break 'rounds;
                }
            }
        }
    }
    write_summary(&output, &records, planned_pairs, stop_reason);
    println!(
        "[online] complete recorded_arms={} complete_pairs={} output={}",
        records.len(),
        records
            .chunks(2)
            .filter(|pair| pair.len() == 2 && pair.iter().all(|record| record.status == "success"))
            .count(),
        output.display()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("run") {
        run(&args[2..]);
    } else {
        usage();
    }
}

#[cfg(test)]
mod tests {
    use super::{classify_external_failure, config_text, parse_codex_output};
    use std::path::Path;

    #[test]
    fn parses_command_execution_from_codex_jsonl() {
        let parsed = parse_codex_output(
            br#"{"type":"item.completed","item":{"type":"command_execution","command":"printf ok","aggregated_output":"ok","exit_code":0,"status":"completed"}}"#,
        );
        assert_eq!(parsed.json_lines, 1);
        assert_eq!(parsed.parse_errors, 0);
        assert_eq!(parsed.command.as_deref(), Some("printf ok"));
        assert_eq!(parsed.command_exit_code, Some(0));
        assert_eq!(parsed.aggregated_output, "ok");
    }

    #[test]
    fn classifies_relay_errors_without_calling_them_backend_failures() {
        let parsed =
            parse_codex_output(br#"{"type":"error","message":"upstream returned HTTP 502"}"#);
        let (status, failure_class) = classify_external_failure(&parsed, "");
        assert_eq!(status, "relay_error");
        assert_eq!(failure_class, "relay_or_transport");
    }

    #[test]
    fn transparent_config_contains_only_the_launcher_path() {
        let config = config_text(
            "gpt-test",
            "https://relay.example/v1",
            Some(Path::new("/opt/mbtx")),
        );
        assert!(config.contains("mbtx_backend = \"transparent\""));
        assert!(config.contains("mbtx_command = [\"/opt/mbtx\"]"));
        assert!(config.find("mbtx_backend").unwrap() < config.find("[model_providers").unwrap());
        assert!(!config.contains("OPENAI_API_KEY"));
    }

    #[cfg(unix)]
    #[test]
    fn timeout_reaps_the_process_group_and_drains_pipes() {
        let output = super::run_process(
            Path::new("/bin/sh"),
            &["-c".to_string(), "sleep 2".to_string()],
            Path::new("."),
            Path::new("/tmp"),
            std::time::Duration::from_millis(50),
            "test",
        );
        assert!(output.timed_out);
        assert!(output.spawn_error.is_none());
        assert!(output.stdout.is_empty());
        assert!(output.stderr.is_empty());
    }
}
