use crate::codex_evidence::{classify_external_failure, parse_codex_output};
use crate::evidence::{Trace, digest, lines, now_ns, read_json, seal, write_json, write_new};
use crate::model::Model;
use crate::process_identity::{Resolution, identity, resolve};
use crate::replay::Replay;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::{
    fs::PermissionsExt,
    process::{CommandExt, ExitStatusExt},
};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
    mpsc,
};
use std::thread;
use std::time::{Duration, Instant};

pub fn option(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|v| v == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn required(args: &[String], key: &str) -> io::Result<String> {
    option(args, key).ok_or_else(|| io::Error::other(format!("missing {key}")))
}

fn number(args: &[String], key: &str, default: usize) -> io::Result<usize> {
    option(args, key)
        .map(|v| v.parse().map_err(io::Error::other))
        .unwrap_or(Ok(default))
}

struct Proxy {
    child: Child,
    url: String,
}
impl Drop for Proxy {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn start_proxy(
    path: &Path,
    directory: &Path,
    upstream: &str,
    interval: usize,
) -> io::Result<Proxy> {
    fs::create_dir_all(directory)?;
    let info = directory.join("server-info.json");
    let stderr = File::create(directory.join("proxy.stderr.log"))?;
    let child = Command::new(path)
        .arg("--server-info")
        .arg(&info)
        .arg("--dump-dir")
        .arg(directory)
        .args([
            "--upstream-url",
            &format!("{}/responses", upstream.trim_end_matches('/')),
        ])
        .args([
            "--min-request-interval-ms",
            &interval.to_string(),
            "--request-timeout-ms",
            "240000",
        ])
        .env_remove("OPENAI_API_KEY")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(stderr)
        .spawn()?;
    let mut proxy = Proxy {
        child,
        url: String::new(),
    };
    let key = std::env::var("OPENAI_API_KEY").map_err(io::Error::other)?;
    if key.trim().is_empty() {
        return Err(io::Error::other("OPENAI_API_KEY is empty"));
    }
    writeln!(proxy.child.stdin.take().unwrap(), "{key}")?;
    for _ in 0..250 {
        if let Ok(info) = read_json(&info) {
            if let Some(port) = info["port"].as_u64().filter(|port| *port > 0) {
                proxy.url = format!("http://127.0.0.1:{port}/v1");
                return Ok(proxy);
            }
        }
        if let Some(status) = proxy.child.try_wait()? {
            return Err(io::Error::other(format!("proxy exited {status}")));
        }
        thread::sleep(Duration::from_millis(20));
    }
    Err(io::Error::other("proxy startup timeout"))
}

pub fn config(
    model: &str,
    url: &str,
    launcher: Option<&Path>,
    direct: bool,
    tool_mode: &str,
) -> io::Result<String> {
    let mut value = json!({"model_provider":"evaluation","model":model,"review_model":model,
        "model_reasoning_effort":"xhigh","approval_policy":"never","sandbox_mode":"workspace-write",
        "model_providers":{"evaluation":{"name":"Evaluation Responses endpoint","base_url":url,
            "wire_api":"responses","env_key":"OPENAI_API_KEY","requires_openai_auth":false,
            "request_max_retries":0,"stream_max_retries":0}},
        "sandbox_workspace_write":{"network_access":true},
        "shell_environment_policy":{"inherit":"all"},
        "features":{"unified_exec":true,"code_mode_host":tool_mode == "code","plugins":false}});
    if let Some(launcher) = launcher {
        value["mbtx_backend"] = json!("transparent");
        value["mbtx_command"] = json!([launcher]);
    }
    if direct {
        value["features"]["unified_exec_zsh_fork"] = json!(false);
    }
    toml::to_string_pretty(&value).map_err(io::Error::other)
}

fn trace_rows(directory: &Path) -> io::Result<Vec<Value>> {
    let mut rows = Vec::new();
    if !directory.exists() {
        return Ok(rows);
    }
    for item in fs::read_dir(directory)? {
        let path = item?.path();
        if path.is_dir() {
            rows.extend(trace_rows(&path)?);
        } else if path.extension().is_some_and(|v| v == "jsonl") {
            rows.extend(
                lines(&path)?
                    .0
                    .into_iter()
                    .map(crate::evidence::normalize_trace),
            );
        }
    }
    rows.sort_by_key(|row| row["monotonic_ns"].as_u64());
    Ok(rows)
}

fn signal_owned(pid: i32, expected: &str, signal: i32, trace: &Trace) -> io::Result<bool> {
    if identity(pid).as_deref() != Some(expected) {
        return Ok(false);
    }
    let pgid = unsafe { libc::getpgid(pid) };
    if pgid <= 1 || pgid == unsafe { libc::getpgrp() } {
        return Ok(false);
    }
    let result = unsafe { libc::kill(-pgid, signal) };
    trace.emit(
        "cancellation",
        "signal_sent",
        json!({"target_pid":pid,"target_pgid":pgid,"signal":signal,"result":result}),
    )?;
    Ok(result == 0)
}

fn read_output(
    mut pipe: impl Read + AsRawFd + Send + 'static,
    path: PathBuf,
    name: &'static str,
    trace: Trace,
    stopped: Arc<AtomicU64>,
) -> thread::JoinHandle<io::Result<(Vec<u8>, Option<u64>)>> {
    thread::spawn(move || {
        let mut bytes = Vec::new();
        let mut chunk = [0; 16384];
        let mut eof = None;
        let flags = unsafe { libc::fcntl(pipe.as_raw_fd(), libc::F_GETFL) };
        if flags < 0
            || unsafe { libc::fcntl(pipe.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0
        {
            return Err(io::Error::last_os_error());
        }
        loop {
            let mut poll = libc::pollfd {
                fd: pipe.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            };
            unsafe {
                libc::poll(&mut poll, 1, 50);
            }
            let n = match pipe.read(&mut chunk) {
                Ok(n) => n,
                Err(error)
                    if matches!(
                        error.kind(),
                        io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                    ) =>
                {
                    let ended = stopped.load(Ordering::Relaxed);
                    if ended > 0 && now_ns().saturating_sub(ended) > 2_000_000_000 {
                        trace.emit(
                            "codex_io",
                            "censored",
                            json!({"stream":name,"bytes":bytes.len()}),
                        )?;
                        break;
                    }
                    continue;
                }
                Err(error) => return Err(error),
            };
            let observed = now_ns();
            if n == 0 {
                eof = Some(observed);
                trace.at(
                    observed,
                    "codex_io",
                    "eof",
                    json!({"stream":name,"bytes":bytes.len()}),
                )?;
                break;
            }
            bytes.extend_from_slice(&chunk[..n]);
        }
        let started = now_ns();
        write_new(&path, &bytes)?;
        trace.emit(
            "artifact",
            "write",
            json!({"stream":name,"duration_ns":now_ns()-started,"bytes":bytes.len()}),
        )?;
        Ok((bytes, eof))
    })
}

struct Engine {
    output: PathBuf,
    codex: PathBuf,
    mbtx: PathBuf,
    fixture: PathBuf,
    model: Model,
    model_name: String,
    url: String,
    tool_mode: String,
    direct: bool,
    timeout: Duration,
    replay: Option<Replay>,
    experiment: String,
    relay_directory: PathBuf,
    fault: String,
}

impl Engine {
    fn attempt(
        &mut self,
        task: &Value,
        pair: &str,
        backend: &str,
        round: usize,
        phase: &str,
    ) -> io::Result<Value> {
        let id = format!("{pair}-{backend}");
        let directory = self.output.join("attempts").join(&id);
        if directory.exists() {
            if directory.join("seal.json").exists() {
                if crate::reporting::verify_seal(&directory)?["status"] != "success" {
                    return Err(io::Error::other("sealed attempt was modified"));
                }
                let mut row = read_json(&directory.join("result.json"))?;
                row["reused"] = json!(true);
                return Ok(row);
            }
            return Ok(
                json!({"experiment_id":self.experiment,"pair_id":pair,"attempt_id":id,"task_id":task["id"],"backend":backend,
                "sample_phase":phase,"round":round,"status":"censored","strict_comparable":false,"failure_class":"interrupted_attempt_preserved","artifact_dir":format!("attempts/{id}")}),
            );
        }
        fs::create_dir(&directory)?;
        let workspace = directory.join("workspace");
        fs::create_dir(&workspace)?;
        let home = workspace.join("home");
        fs::create_dir(&home)?;
        let observation = workspace.join("observation");
        fs::create_dir(&observation)?;
        let fixture_dir = observation.join("fixture");
        fs::create_dir(&fixture_dir)?;
        let space = workspace.join("space \u{8def}\u{5f84}");
        fs::create_dir(&space)?;
        fs::create_dir(space.join("target"))?;
        std::os::unix::fs::symlink("target", space.join("link"))?;
        write_new(&workspace.join("not-executable"), b"exit 0\n")?;
        fs::set_permissions(
            workspace.join("not-executable"),
            fs::Permissions::from_mode(0o600),
        )?;
        let context = json!({"experiment_id":self.experiment,"pair_id":pair,"attempt_id":id,
            "task_id":task["id"],"category":task["category"],"backend":backend,"round":round,"sample_phase":phase,
            "platform":std::env::consts::OS,"tool_mode":self.tool_mode});
        let trace = Trace::create(&directory.join("events.jsonl"), context.clone())?;
        write_json(&directory.join("task.json"), task)?;
        let configuration = config(
            &self.model_name,
            &self.url,
            (backend == "transparent").then_some(&self.mbtx),
            self.direct,
            &self.tool_mode,
        )?;
        write_new(&home.join("config.toml"), configuration.as_bytes())?;
        write_new(&directory.join("config.toml"), configuration.as_bytes())?;
        let prompt = if self.tool_mode == "code" {
            format!(
                "Call exec exactly once using the following complete JavaScript program verbatim. It runs the required tools sequentially. Do not add, remove or rewrite any tool arguments. Nonzero exits are intentional; execute the entire program. Then reply DONE.\n{}",
                task["code_mode_program"]
                    .as_str()
                    .unwrap_or("text('probe');")
            )
        } else {
            format!(
                "Execute this exact ordered tool trajectory. Preserve every argument and every character of cmd. Do not run other commands. Replace $session only with the real session ID returned by the preceding exec_command. Use one tool call at a time, and wait for each result. A nonzero exit is expected in some steps: continue with the next step. After the last step reply DONE.\n{}",
                task["steps"]
            )
        };
        write_new(&directory.join("prompt.txt"), prompt.as_bytes())?;
        let external_before: BTreeSet<_> = fs::read_dir(&self.relay_directory)?
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|e| e.file_name())
            .collect();
        if let Some(replay) = &self.replay {
            *replay.active.lock().unwrap() = json!({"task":task,"context":context,"tool_mode":self.tool_mode,"fault":self.fault});
        } else {
            let tmp = self.relay_directory.join("context.tmp");
            fs::write(&tmp, serde_json::to_vec(&context)?)?;
            fs::rename(tmp, self.relay_directory.join("context.json"))?;
        }
        let mut command = Command::new(&self.codex);
        if self
            .codex
            .file_name()
            .is_none_or(|name| name != "codex-exec")
        {
            command.arg("exec");
        }
        command
            .args([
                "--json",
                "--strict-config",
                "--ephemeral",
                "--skip-git-repo-check",
                "--cd",
            ])
            .arg(&workspace)
            .args(["--sandbox", "workspace-write"])
            .arg(prompt)
            .current_dir(&workspace)
            .env("HOME", &home)
            .env("CODEX_HOME", &home)
            .env("MBTX_TRACE_DIR", &observation)
            .env("MBTX_LAUNCH_TRACE", observation.join("launcher.jsonl"))
            .env("MBTX_FIXTURE_DIR", &fixture_dir)
            .env("MBTX_EVAL_MARKER", "fixed-environment")
            .env_remove("MBTX_EMPTY")
            .env_remove("MBTX_ABSENT")
            .env("OPENAI_API_KEY", "evaluation-local-proxy")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .process_group(0);
        for (field, key) in [
            ("experiment_id", "MBTX_EXPERIMENT_ID"),
            ("pair_id", "MBTX_PAIR_ID"),
            ("attempt_id", "MBTX_ATTEMPT_ID"),
            ("task_id", "MBTX_TASK_ID"),
            ("backend", "MBTX_BACKEND"),
        ] {
            command.env(key, context[field].as_str().unwrap());
        }
        let begin = now_ns();
        trace.at(begin, "codex", "spawn_begin", json!({}))?;
        let spawned = command.spawn();
        let mut infrastructure = "none".to_owned();
        let mut identities = BTreeMap::<i32, String>::new();
        let mut controlled = BTreeSet::new();
        let mut inspected = BTreeSet::new();
        let mut fixture_pids = BTreeMap::new();
        let mut unresolved = BTreeMap::new();
        let mut kill_due = Vec::<(i32, String, Instant)>::new();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let mut exit_code = None;
        let mut signal = None;
        let mut wait_ns = None;
        let mut io_end = None;
        match spawned {
            Err(error) => {
                infrastructure = "harness_error".into();
                trace.emit("codex", "spawn_error", json!({"error":error.to_string()}))?;
            }
            Ok(mut child) => {
                trace.emit("codex", "spawn_return", json!({"child_pid":child.id()}))?;
                let child_pid = child.id() as i32;
                let child_identity = identity(child_pid);
                let stopped = Arc::new(AtomicU64::new(0));
                let out = read_output(
                    child.stdout.take().unwrap(),
                    directory.join("codex.stdout.jsonl"),
                    "stdout",
                    trace.clone(),
                    Arc::clone(&stopped),
                );
                let err = read_output(
                    child.stderr.take().unwrap(),
                    directory.join("codex.stderr.log"),
                    "stderr",
                    trace.clone(),
                    Arc::clone(&stopped),
                );
                // Only this thread owns wait/reap. Timestamp before collection and disk IO.
                let (finished, received) = mpsc::sync_channel(1);
                let wait_stopped = Arc::clone(&stopped);
                let waiter = thread::spawn(move || {
                    let status = child.wait();
                    let observed = now_ns();
                    wait_stopped.store(observed, Ordering::Relaxed);
                    let _ = finished.send(status.map(|status| (status, observed)));
                });
                let deadline = Instant::now() + self.timeout;
                let mut cancellation_deadline = None;
                loop {
                    for entry in fs::read_dir(&fixture_dir)? {
                        let path = entry?.path();
                        if path.extension().is_none_or(|v| v != "json") {
                            continue;
                        }
                        let Ok(receipt) = read_json(&path) else {
                            continue;
                        };
                        let Some(instance) = receipt["process_instance"].as_str() else {
                            continue;
                        };
                        if !inspected.contains(instance) {
                            match resolve(&receipt) {
                                Resolution::Live { pid, identity } => {
                                    trace.emit("process_identity", "resolved", json!({
                                        "process_instance":instance,"namespace_pid":receipt["pid"],
                                        "pid_namespace":receipt["pid_namespace"],"host_pid":pid,
                                        "host_pgid":unsafe {libc::getpgid(pid)},"identity":identity}))?;
                                    identities.insert(pid, identity);
                                    fixture_pids.insert(instance.to_owned(), pid);
                                    inspected.insert(instance.to_owned());
                                    unresolved.remove(instance);
                                }
                                Resolution::Gone => {
                                    inspected.insert(instance.to_owned());
                                    unresolved.remove(instance);
                                }
                                Resolution::Unknown => {
                                    unresolved.insert(instance.to_owned(), receipt.clone());
                                }
                            }
                        }
                        let Some(&pid) = fixture_pids.get(instance) else {
                            continue;
                        };
                        let control = task["control"].as_str().unwrap_or("none");
                        if control == "none"
                            || !path.to_string_lossy().ends_with(".ready.json")
                            || receipt["mode"]
                                != task.get("control_mode").unwrap_or(&task["check"]).clone()
                            || controlled.contains(&pid)
                        {
                            continue;
                        }
                        if let Some(proof) = identities.get(&pid) {
                            // Allow exec_command to publish its session before the controlled cancellation.
                            let ready = receipt["ready_ns"].as_u64().unwrap_or(begin);
                            if now_ns().saturating_sub(ready) < 1_250_000_000 {
                                continue;
                            }
                            signal_owned(pid, proof, libc::SIGTERM, &trace)?;
                            kill_due.push((
                                pid,
                                proof.clone(),
                                Instant::now() + Duration::from_millis(500),
                            ));
                            controlled.insert(pid);
                        }
                    }
                    for (pid, proof, due) in &kill_due {
                        if Instant::now() >= *due {
                            signal_owned(*pid, proof, libc::SIGKILL, &trace)?;
                        }
                    }
                    kill_due.retain(|(_, _, due)| Instant::now() < *due);
                    match received.recv_timeout(Duration::from_millis(20)) {
                        Ok(result) => {
                            let (status, observed) = result?;
                            exit_code = status.code();
                            signal = status.signal();
                            wait_ns = Some(observed);
                            trace.at(
                                wait_ns.unwrap(),
                                "codex",
                                "wait_return",
                                json!({"exit_code":exit_code,"signal":signal}),
                            )?;
                            break;
                        }
                        Err(mpsc::RecvTimeoutError::Timeout)
                            if Instant::now() >= deadline && cancellation_deadline.is_none() =>
                        {
                            infrastructure = "timeout".into();
                            trace.emit("cancellation", "deadline", json!({}))?;
                            if let Some(proof) = &child_identity {
                                signal_owned(child_pid, proof, libc::SIGTERM, &trace)?;
                            }
                            cancellation_deadline =
                                Some(Instant::now() + Duration::from_millis(500));
                        }
                        Err(mpsc::RecvTimeoutError::Timeout)
                            if cancellation_deadline.is_some_and(|t| Instant::now() >= t) =>
                        {
                            if let Some(proof) = &child_identity {
                                signal_owned(child_pid, proof, libc::SIGKILL, &trace)?;
                            }
                        }
                        Err(mpsc::RecvTimeoutError::Disconnected) => {
                            return Err(io::Error::other("Codex wait observer disconnected"));
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => {}
                    }
                }
                waiter
                    .join()
                    .map_err(|_| io::Error::other("Codex wait observer panicked"))?;
                let out = out
                    .join()
                    .map_err(|_| io::Error::other("stdout observer panicked"))??;
                let err = err
                    .join()
                    .map_err(|_| io::Error::other("stderr observer panicked"))??;
                io_end = out.1.zip(err.1).map(|(a, b)| a.max(b));
                stdout = out.0;
                stderr = err.0;
                if io_end.is_none() {
                    infrastructure = "censored".into();
                }
            }
        }
        let end = wait_ns.zip(io_end).map(|(wait, io)| wait.max(io));
        let mut rows = trace_rows(&observation)?;
        let mut unidentified_live = Vec::new();
        for receipt in unresolved.values() {
            match resolve(receipt) {
                Resolution::Live { pid, identity } => {
                    identities.insert(pid, identity);
                }
                Resolution::Gone => {}
                Resolution::Unknown => unidentified_live.push(receipt.clone()),
            }
        }
        let mut residual = Vec::new();
        for (pid, proof) in &identities {
            if identity(*pid).as_deref() == Some(proof) {
                residual.push(json!({"pid":pid,"identity":proof}));
            }
        }
        trace.emit(
            "cleanup",
            "before_fallback",
            json!({"residual":residual,"descendants_without_identity":"unknown"}),
        )?;
        for row in &residual {
            signal_owned(
                row["pid"].as_i64().unwrap() as i32,
                row["identity"].as_str().unwrap(),
                libc::SIGKILL,
                &trace,
            )?;
        }
        let parsed = parse_codex_output(&stdout);
        if infrastructure == "none" && (!parsed.turn_completed || exit_code != Some(0)) {
            infrastructure =
                classify_external_failure(&parsed, &String::from_utf8_lossy(&stderr)).0;
        }
        let mut commands = Vec::new();
        for line in String::from_utf8_lossy(&stdout).lines() {
            let Ok(value) = serde_json::from_str::<Value>(line) else {
                continue;
            };
            if value["type"] == "item.completed"
                && matches!(
                    value["item"]["type"].as_str(),
                    Some("command_execution" | "commandExecution")
                )
            {
                commands.push(value["item"].clone());
            }
        }
        let mut actual = Vec::new();
        let mut session_id = Value::Null;
        for row in &rows {
            if row["phase"] != "tool" {
                continue;
            }
            if row["event"] == "result_ready" && row["tool"] == "exec_command" {
                session_id = row["process_id"].clone();
            }
            if row["event"] == "arguments" {
                actual.push(json!({"tool":row["tool"],"arguments":row["arguments"],"expected_session_id":session_id}));
            }
        }
        let mut receipts = Vec::new();
        for entry in fs::read_dir(&fixture_dir)? {
            let path = entry?.path();
            if path.to_string_lossy().ends_with(".receipt.json") {
                receipts.push(read_json(&path)?);
            }
        }
        receipts.sort_by_key(|v| v["ready_ns"].as_u64());
        let mut streams = Vec::new();
        for entry in fs::read_dir(&observation)? {
            let path = entry?.path();
            if path.extension().is_some_and(|v| v == "bin") {
                let bytes = fs::read(&path)?;
                streams.push(json!({"name":path.file_name().and_then(|name|name.to_str()),"bytes":bytes.len(),"sha256":digest(&bytes),"text":String::from_utf8(bytes).ok()}));
            }
        }
        streams.sort_by_key(|v| v["name"].as_str().unwrap_or("").to_owned());
        // Wait outside the measured Codex interval for the proxy to close its
        // response dump before publishing an immutable attempt snapshot.
        let snapshot_deadline = Instant::now() + Duration::from_secs(2);
        let (external, relay_snapshot_complete) = loop {
            let external = trace_rows(&self.relay_directory)?
                .into_iter()
                .filter(|v| v["attempt_id"] == id)
                .collect::<Vec<_>>();
            let pending = self.replay.is_none()
                && external
                    .iter()
                    .filter(|r| r["event"] == "request_received")
                    .any(|request| {
                        !external.iter().any(|r| {
                            r["request_id"] == request["request_id"]
                                && matches!(
                                    r["event"].as_str(),
                                    Some("request_finished" | "transport_error")
                                )
                        })
                    });
            if !pending || Instant::now() >= snapshot_deadline {
                break (external, !pending);
            }
            thread::sleep(Duration::from_millis(10));
        };
        // The HTTP status is stronger evidence than a substring from stderr.
        for row in &external {
            if row["event"] == "harness_error" {
                infrastructure = "harness_error".into();
            }
            if matches!(
                row["event"].as_str(),
                Some("stream_error" | "transport_error")
            ) {
                infrastructure = "relay_error".into();
            }
            if let Some(code) = row["http_status"]
                .as_u64()
                .or_else(|| row["detail"]["status"].as_u64())
            {
                if code == 429 || code >= 500 {
                    infrastructure = "relay_error".into();
                } else if code >= 400 {
                    infrastructure = "provider_error".into();
                }
            }
        }
        rows.extend(external);
        let relay_copy = directory.join("relay");
        fs::create_dir(&relay_copy)?;
        for entry in fs::read_dir(&self.relay_directory)? {
            let entry = entry?;
            let name = entry.file_name();
            if entry.file_type()?.is_file()
                && !external_before.contains(&name)
                && name != "context.json"
                && name != "context.tmp"
            {
                write_new(&relay_copy.join(name), &fs::read(entry.path())?)?;
            }
        }
        let processes_clean = if !residual.is_empty() {
            Some(false)
        } else if unidentified_live.is_empty() {
            Some(true)
        } else {
            None
        };
        let tool_results: Vec<_> = rows
            .iter()
            .filter(|r| r["phase"] == "tool" && r["event"] == "result_ready")
            .cloned()
            .collect();
        let facts = json!({"task":task,"platform":std::env::consts::OS,
            "collector_pid_namespace":crate::process_identity::fixture_scope()["pid_namespace"],
            "actual_steps":actual,"commands":commands,"tool_results":tool_results,"receipts":receipts,"streams":streams,
            "trace":rows,"codex_reported_usage":parsed.usage,"relay_snapshot_complete":relay_snapshot_complete,"infrastructure":infrastructure,"turn_completed":parsed.turn_completed,
            "processes_clean":processes_clean,"cleanup_scope":"observed fixture processes; detached/unobserved descendants are not inferred reaped", "unidentified_live":unidentified_live,"workspace":workspace,"home":home,"fixture":self.fixture});
        let mut assessment = self
            .model
            .call(json!({"operation":"evaluate","facts":facts}))?;
        if assessment.get("status").is_none() {
            return Err(io::Error::other(format!(
                "evaluation model rejected facts: {assessment}"
            )));
        }
        assessment
            .as_object_mut()
            .unwrap()
            .extend(context.as_object().unwrap().clone());
        assessment["elapsed_ms"] = json!(end.map(|end| (end - begin) as f64 / 1e6));
        assessment["begin_ns"] = json!(begin);
        assessment["end_ns"] = json!(end);
        assessment["exit_code"] = json!(exit_code);
        assessment["signal"] = json!(signal);
        assessment["artifact_dir"] = json!(format!("attempts/{id}"));
        assessment["shell_modes"] = json!(
            rows.iter()
                .filter(|r| r["event"] == "execution_path")
                .map(|r| r["shell_mode"].clone())
                .collect::<Vec<_>>()
        );
        write_json(&directory.join("facts.json"), &facts)?;
        write_json(&directory.join("result.json"), &assessment)?;
        trace.emit("codex", "exit", assessment.clone())?;
        fs::rename(&observation, directory.join("trace"))?;
        // HOME databases and mutable scratch files are not experimental evidence.
        fs::remove_dir_all(&workspace)?;
        seal(&directory)?;
        Ok(assessment)
    }
}

fn unhealthy(window: &VecDeque<bool>) -> bool {
    (window.len() >= 5 && window.iter().rev().take(5).all(|v| *v))
        || (window.len() == 16 && window.iter().filter(|v| **v).count() >= 8)
}

pub fn run(args: &[String]) -> io::Result<()> {
    let output = PathBuf::from(required(args, "--output")?);
    fs::create_dir_all(&output)?;
    let output = fs::canonicalize(output)?;
    let suite_name = option(args, "--suite").unwrap_or("codex-relay".into());
    if !matches!(suite_name.as_str(), "codex-relay" | "codex-replay") {
        return Err(io::Error::other(
            "suite must be codex-relay or codex-replay",
        ));
    }
    let fixture = fs::canonicalize(required(args, "--fixture")?)?;
    let model_path = fs::canonicalize(required(args, "--evaluation-model")?)?;
    let mut model = Model::start(&model_path)?;
    let frozen = option(args, "--frozen-run")
        .map(|path| read_json(&Path::new(&path).join("manifest.json")))
        .transpose()?;
    if frozen.is_some() && suite_name != "codex-replay" {
        return Err(io::Error::other("frozen input requires offline replay"));
    }
    let protocol = if let Some(manifest) = &frozen {
        manifest.clone()
    } else {
        model.call(json!({"operation":"suite","fixture":fixture}))?
    };
    let mut tasks = protocol["tasks"]
        .as_array()
        .ok_or_else(|| io::Error::other("invalid suite"))?
        .clone();
    if args.iter().any(|v| v == "--boundaries") {
        if suite_name != "codex-replay" {
            return Err(io::Error::other(
                "--boundaries is only supported by offline replay",
            ));
        }
        if let Some(boundaries) = protocol["offline_boundaries"].as_array() {
            tasks.extend(boundaries.clone());
        }
    }
    if let Some(filter) = option(args, "--tasks") {
        tasks.retain(|task| filter.split(',').any(|v| task["id"] == v));
    }
    if tasks.is_empty() {
        return Err(io::Error::other("no matching scenarios"));
    }
    let replay_mode = suite_name == "codex-replay";
    let fault = option(args, "--fault").unwrap_or("none".into());
    if !matches!(fault.as_str(), "none" | "429" | "503" | "disconnect")
        || (!replay_mode && fault != "none")
    {
        return Err(io::Error::other(
            "--fault is only available for codex-replay: 429, 503 or disconnect",
        ));
    }
    let per_task = number(args, "--pairs", if replay_mode { 10 } else { 8 })?;
    if per_task == 0 {
        return Err(io::Error::other("--pairs must be positive"));
    }
    let rounds = if replay_mode || per_task < 2 { 1 } else { 2 };
    let purpose = if per_task < if replay_mode { 10 } else { 8 } || tasks.len() < 24 {
        "validation"
    } else {
        "formal"
    };
    let interval = number(args, "--min-interval-ms", 15000)?.max(6000);
    let codex = fs::canonicalize(required(args, "--codex")?)?;
    let mbtx = fs::canonicalize(required(args, "--mbtx")?)?;
    if let Some(prior) = &frozen {
        if prior["platform"] != std::env::consts::OS
            || prior["path"] != json!(std::env::var("PATH").ok())
        {
            return Err(io::Error::other(
                "frozen replay requires the original platform and environment PATH",
            ));
        }
        if prior["binaries"]["fixture"]["path"] != fixture.to_string_lossy().as_ref() {
            return Err(io::Error::other(
                "frozen argv requires the original fixture path; no command rewriting is performed",
            ));
        }
        for (name, path) in [
            ("codex", &codex),
            ("launcher", &mbtx),
            ("fixture", &fixture),
            ("evaluation_model", &model_path),
        ] {
            if prior["binaries"][name]["sha256"] != digest(&fs::read(path)?) {
                return Err(io::Error::other(format!(
                    "frozen replay {name} binary hash differs"
                )));
            }
        }
    }
    let tool_mode = option(args, "--tool-mode").unwrap_or("code".into());
    if !matches!(tool_mode.as_str(), "code" | "direct") {
        return Err(io::Error::other("--tool-mode must be code or direct"));
    }
    let model_name = option(args, "--model").unwrap_or("gpt-5.6-terra".into());
    let upstream = option(args, "--relay-base-url").unwrap_or("https://tokenadvent.com/v1".into());
    let proxy_path = if replay_mode {
        None
    } else {
        Some(fs::canonicalize(required(args, "--proxy")?)?)
    };
    let code_host = codex.parent().unwrap().join("codex-code-mode-host");
    if tool_mode == "code"
        && let Some(prior) = &frozen
    {
        if prior["binaries"]["code_mode_host"]["sha256"] != digest(&fs::read(&code_host)?) {
            return Err(io::Error::other(
                "frozen replay Code Mode host hash differs",
            ));
        }
    }
    let binary_info = |path: &Path| -> io::Result<Value> {
        Ok(json!({"path":path,"sha256":digest(&fs::read(path)?)}))
    };
    let experiment = format!(
        "{suite_name}-{}",
        digest(output.to_string_lossy().as_bytes())
    );
    let manifest = json!({"schema_version":2,"experiment_id":experiment,"suite":suite_name,"purpose":purpose,"target_pairs":tasks.len()*per_task,
        "pairs_per_task":per_task,"rounds":rounds,"seed":20260913,"tasks":tasks,
        "min_request_interval_ms":interval,"max_concurrent_requests":1,"tool_mode":tool_mode,"fault":fault,
        "direct_control":args.iter().any(|v| v == "--direct-shell"),"platform":std::env::consts::OS,
        "model":model_name,"upstream":if replay_mode {"fixed-responses"} else {&upstream},
        "timeout_ms":number(args, "--timeout-ms", 300000)?,"path":std::env::var("PATH").ok(),
        "binaries":{"codex":{"path":codex,"sha256":digest(&fs::read(&codex)?)},
            "launcher":{"path":mbtx,"sha256":digest(&fs::read(&mbtx)?)},"fixture":{"path":fixture,"sha256":digest(&fs::read(&fixture)?)},
            "evaluation_model":{"path":model_path,"sha256":digest(&fs::read(&model_path)?)},
            "proxy":proxy_path.as_deref().map(binary_info).transpose()?,
            "code_mode_host":if tool_mode == "code" {Some(binary_info(&code_host)?)} else {None}}});
    let resume = args.iter().any(|v| v == "--resume");
    if resume {
        let prior = read_json(&output.join("manifest.json"))?;
        if prior != manifest {
            return Err(io::Error::other(
                "resume requires identical protocol, binaries, platform and request interval",
            ));
        }
    } else {
        write_json(&output.join("manifest.json"), &manifest)?;
        if let Some(prior) = &frozen {
            write_json(&output.join("source-manifest.json"), prior)?;
        }
        fs::create_dir(output.join("attempts"))?;
    }
    let generation = now_ns();
    let events = Trace::create(
        &output.join(if resume {
            format!("events-resume-{generation}.jsonl")
        } else {
            "events.jsonl".into()
        }),
        json!({"experiment_id":experiment}),
    )?;
    let relay_directory = output
        .join("relay")
        .join(format!("generation-{generation}"));
    let replay = if replay_mode {
        Some(Replay::start(relay_directory.clone())?)
    } else {
        None
    };
    let proxy = if replay_mode {
        None
    } else {
        Some(start_proxy(
            proxy_path.as_deref().unwrap(),
            &relay_directory,
            &upstream,
            interval,
        )?)
    };
    let url = replay
        .as_ref()
        .map(|r| r.url.clone())
        .or_else(|| proxy.as_ref().map(|p| p.url.clone()))
        .unwrap();
    let mut engine = Engine {
        output: output.clone(),
        codex,
        mbtx,
        fixture,
        model,
        model_name,
        url,
        tool_mode,
        direct: args.iter().any(|v| v == "--direct-shell"),
        timeout: Duration::from_millis(number(args, "--timeout-ms", 300000)? as u64),
        replay,
        experiment,
        relay_directory,
        fault,
    };
    let mut window = VecDeque::new();
    let mut attempted = 0;
    let mut valid = 0;
    let mut failures = 0;
    let mut stop_reason = None;
    if !replay_mode && !resume {
        let probe = json!({"id":"relay_probe","category":"probe","steps":[],"expected_exits":[],"check":"probe","control":"none"});
        let mut successes = 0;
        for index in 0..3 {
            let row = engine.attempt(&probe, &format!("probe-{index}"), "shell", 0, "probe")?;
            successes += usize::from(row["status"] == "success");
            events.emit("probe", "exit", row)?;
        }
        if successes == 0 {
            stop_reason = Some("all three initial probes failed".to_owned());
        }
    }
    let started = Instant::now();
    let mut seed = 20260913u32;
    'rounds: for round in 0..rounds {
        let quota = per_task / rounds + usize::from(round < per_task % rounds);
        let limit = if replay_mode {
            quota
        } else {
            quota + quota.div_ceil(2)
        };
        let mut successes = vec![0usize; tasks.len()];
        for index in (1..tasks.len()).rev() {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            tasks.swap(index, seed as usize % (index + 1));
        }
        for sample in 0..limit {
            for (task_index, task) in tasks.iter().enumerate() {
                if stop_reason.is_some() {
                    break 'rounds;
                }
                if successes[task_index] >= quota {
                    continue;
                }
                let pair = format!(
                    "r{}-{}-a{sample:02}",
                    round + 1,
                    task["id"].as_str().unwrap()
                );
                let order = if attempted % 2 == 0 {
                    ["shell", "transparent"]
                } else {
                    ["transparent", "shell"]
                };
                let mut comparable = true;
                for backend in order {
                    events.emit("collection","attempt_start",json!({"pair_id":pair,"task_id":task["id"],"backend":backend,
                        "valid_pairs":valid,"target_pairs":tasks.len()*per_task,"failed_arms":failures}))?;
                    let row = match engine.attempt(task, &pair, backend, round + 1, "formal") {
                        Ok(row) => row,
                        Err(error) => {
                            stop_reason = Some(format!("collector failure: {error}"));
                            events.emit(
                                "collection",
                                "harness_error",
                                json!({"error":error.to_string(),"pair_id":pair,"backend":backend}),
                            )?;
                            break;
                        }
                    };
                    comparable &= row["strict_comparable"] == true;
                    let failed = row["status"] != "success";
                    failures += usize::from(failed);
                    if row["reused"] != true {
                        window.push_back(matches!(
                            row["status"].as_str(),
                            Some("relay_error" | "provider_error" | "harness_error")
                        ));
                    }
                    if window.len() > 16 {
                        window.pop_front();
                    }
                    events.emit("codex", "exit", row)?;
                }
                attempted += 1;
                if comparable && stop_reason.is_none() {
                    successes[task_index] += 1;
                    valid += 1;
                }
                if unhealthy(&window) {
                    stop_reason = Some("infrastructure failure threshold reached".into());
                }
                let remaining = tasks.len() * per_task - valid;
                let eta = (valid > 0)
                    .then(|| started.elapsed().as_secs_f64() * remaining as f64 / valid as f64);
                events.emit("collection", "progress", json!({"pair_id":pair,"task_id":task["id"],"valid_pairs":valid,
                    "target_pairs":tasks.len()*per_task,"failed_arms":failures,"remaining_target":remaining,"estimated_remaining_seconds":eta}))?;
                eprintln!(
                    "[collect] pairs={attempted} valid={valid}/{} failed_arms={failures} round={} scenario={} remaining_target={remaining} elapsed={}s",
                    tasks.len() * per_task,
                    round + 1,
                    task["id"],
                    started.elapsed().as_secs()
                );
            }
        }
    }
    drop(engine);
    drop(proxy);
    let summary = json!({"schema_version":2,"attempted_pairs":attempted,"valid_pairs":valid,"target_pairs":tasks.len()*per_task,
        "failed_arms":failures,"partial":valid<tasks.len()*per_task,"stop_reason":stop_reason,"elapsed_seconds":started.elapsed().as_secs_f64()});
    write_json(
        &output.join(if resume {
            format!("summary-resume-{generation}.json")
        } else {
            "summary.json".into()
        }),
        &summary,
    )?;
    events.emit("collection", "finished", summary.clone())?;
    eprintln!("{summary}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn external_failure_thresholds_do_not_count_expected_cancellation() {
        assert!(!unhealthy(&VecDeque::from([false; 16])));
        assert!(unhealthy(&VecDeque::from([true; 5])));
        assert!(!unhealthy(&VecDeque::from([true; 4])));
        assert!(unhealthy(&VecDeque::from_iter((0..16).map(|i| i % 2 == 0))));
    }
    #[test]
    fn config_uses_structured_toml_and_keeps_shell_default() {
        let text = config("a\"b", "http://localhost/v1", None, false, "code").unwrap();
        let parsed: toml::Value = toml::from_str(&text).unwrap();
        assert_eq!(parsed["model"].as_str(), Some("a\"b"));
        assert!(parsed.get("mbtx_backend").is_none());
        assert_eq!(
            parsed["model_providers"]["evaluation"]["request_max_retries"].as_integer(),
            Some(0)
        );
    }
}
