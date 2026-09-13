use serde_json::{Value, json};
use std::fs;
use std::io::Write;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::thread;
use std::time::{Duration, Instant};

fn scratch(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "mbtx-{name}-{}-{}",
        std::process::id(),
        codex_mbtx_contract::evidence::now_ns()
    ));
    fs::create_dir(&path).unwrap();
    path
}
fn json_file(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
struct KillOnDrop(std::process::Child);
impl Drop for KillOnDrop {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
#[ignore = "requires MBTX_TEST_PROXY; actual proxy with only a local HTTP server"]
fn proxy_serializes_entire_stream_and_honors_retry_after() {
    let proxy = std::env::var("MBTX_TEST_PROXY").unwrap();
    let root = scratch("proxy");
    let upstream = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let url = format!("http://{}/responses", upstream.server_addr());
    let active = Arc::new(AtomicUsize::new(0));
    let peak = Arc::new(AtomicUsize::new(0));
    let server = thread::spawn(move || {
        let mut starts = Vec::new();
        let mut handles = Vec::new();
        for index in 0..3 {
            let mut request = upstream
                .recv_timeout(Duration::from_secs(15))
                .unwrap()
                .expect("proxy request deadline");
            let mut body = String::new();
            request.as_reader().read_to_string(&mut body).unwrap();
            assert_eq!(body, "{}");
            assert!(
                request
                    .headers()
                    .iter()
                    .any(|h| h.field.equiv("Authorization")
                        && h.value.as_str() == "Bearer offline-test-key")
            );
            starts.push(Instant::now());
            let active = Arc::clone(&active);
            let peak = Arc::clone(&peak);
            handles.push(thread::spawn(move || {
                let count = active.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(count, Ordering::SeqCst);
                thread::sleep(Duration::from_millis(250));
                let mut response = tiny_http::Response::from_string("fixture-body")
                    .with_status_code(if index == 0 {
                        429
                    } else if index == 1 {
                        503
                    } else {
                        200
                    });
                if index == 0 {
                    response = response
                        .with_header(tiny_http::Header::from_bytes("Retry-After", "1").unwrap());
                }
                request.respond(response).unwrap();
                active.fetch_sub(1, Ordering::SeqCst);
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        (starts, peak.load(Ordering::SeqCst))
    });
    let info = root.join("server.json");
    let mut proxy = KillOnDrop(
        Command::new(proxy)
            .args(["--server-info"])
            .arg(&info)
            .args(["--dump-dir"])
            .arg(&root)
            .args([
                "--upstream-url",
                &url,
                "--min-request-interval-ms",
                "300",
                "--request-timeout-ms",
                "5000",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    writeln!(proxy.0.stdin.take().unwrap(), "offline-test-key").unwrap();
    let started = Instant::now();
    while !info.exists() {
        assert!(started.elapsed() < Duration::from_secs(5));
        thread::sleep(Duration::from_millis(10));
    }
    let port = json_file(&info)["port"].as_u64().unwrap();
    let clients: Vec<_> = (0..3)
        .map(|_| {
            thread::spawn(move || {
                Command::new("curl")
                    .args([
                        "--silent",
                        "--show-error",
                        "--max-time",
                        "15",
                        "-H",
                        "Content-Type: application/json",
                        "--data",
                        "{}",
                        "-w",
                        "%{http_code}",
                        &format!("http://127.0.0.1:{port}/v1/responses"),
                    ])
                    .output()
                    .unwrap()
            })
        })
        .collect();
    let mut statuses = Vec::new();
    for client in clients {
        let output = client.join().unwrap();
        assert!(output.status.success());
        let text = String::from_utf8(output.stdout).unwrap();
        statuses.push(text[text.len() - 3..].to_owned());
    }
    statuses.sort();
    assert_eq!(statuses, ["200", "429", "503"]);
    let (starts, peak) = server.join().unwrap();
    assert_eq!(peak, 1, "upstream responses must never overlap");
    assert!(
        starts[1].duration_since(starts[0]) >= Duration::from_millis(1200),
        "Retry-After includes response delay"
    );
    assert!(starts[2].duration_since(starts[1]) >= Duration::from_millis(290));
    drop(proxy);
    for entry in fs::read_dir(&root).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            assert!(
                !fs::read_to_string(path)
                    .unwrap()
                    .contains("offline-test-key"),
                "key must not be retained"
            );
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[ignore = "requires MBTX_TEST_CODEX, MBTX_TEST_LAUNCHER and MBTX_TEST_MODEL; runs real sandbox and fixed Responses"]
fn real_codex_scenarios_and_immutable_recovery() {
    let root = scratch("replay");
    let codex = std::env::var("MBTX_TEST_CODEX").unwrap();
    let launcher = std::env::var("MBTX_TEST_LAUNCHER").unwrap();
    let model = std::env::var("MBTX_TEST_MODEL").unwrap();
    for mode in ["direct", "code"] {
        let output = root.join(mode);
        let mut args = vec![
            "run".into(),
            "--suite".into(),
            "codex-replay".into(),
            "--codex".into(),
            codex.clone(),
            "--mbtx".into(),
            launcher.clone(),
            "--fixture".into(),
            env!("CARGO_BIN_EXE_mbtx-fixture").into(),
            "--evaluation-model".into(),
            model.clone(),
            "--output".into(),
            output.display().to_string(),
            "--pairs".into(),
            "1".into(),
            "--tool-mode".into(),
            mode.into(),
            "--timeout-ms".into(),
            "40000".into(),
        ];
        if let Ok(tasks) = std::env::var("MBTX_TEST_TASKS") {
            args.extend(["--tasks".into(), tasks]);
        }
        let run = Command::new(env!("CARGO_BIN_EXE_mbtx-eval"))
            .args(&args)
            .output()
            .unwrap();
        assert!(
            run.status.success(),
            "{}; evidence={}",
            String::from_utf8_lossy(&run.stderr),
            output.display()
        );
        let summary = json_file(output.join("summary.json"));
        assert_eq!(
            summary["valid_pairs"],
            summary["target_pairs"],
            "{}; evidence={}",
            summary,
            output.display()
        );
        let before = fs::read(output.join("events.jsonl")).unwrap();
        let run = Command::new(env!("CARGO_BIN_EXE_mbtx-eval"))
            .args(&args)
            .output()
            .unwrap();
        assert!(!run.status.success(), "must reject overwriting a run");
        args.push("--resume".into());
        let run = Command::new(env!("CARGO_BIN_EXE_mbtx-eval"))
            .args(&args)
            .output()
            .unwrap();
        assert!(
            run.status.success(),
            "{}",
            String::from_utf8_lossy(&run.stderr)
        );
        assert_eq!(fs::read(output.join("events.jsonl")).unwrap(), before);
        let report = codex_mbtx_contract::reporting::build(&output, Path::new(&model)).unwrap();
        assert_eq!(
            report["summary"]["strict_comparable_pairs"],
            summary["target_pairs"]
        );
        for arm in report["arms"].as_array().unwrap() {
            if arm["result"]["task_id"] == "argv_empty_unicode" {
                assert!(
                    arm["metrics"]["startup_to_ready_ns"].as_u64().is_some(),
                    "spawn-to-ready attribution must cross sandbox PID namespaces"
                );
            }
        }
        for format in ["md", "json", "csv", "html", "trace"] {
            assert!(
                !codex_mbtx_contract::reporting::render(&report, format)
                    .unwrap()
                    .is_empty()
            );
        }
        let manifest = json_file(output.join("manifest.json"));
        let replay_output = root.join(format!("{mode}-frozen"));
        let replay_args = [
            "replay".to_owned(),
            output.display().to_string(),
            "--output".into(),
            replay_output.display().to_string(),
            "--pairs".into(),
            "1".into(),
            "--tasks".into(),
            manifest["tasks"][0]["id"].as_str().unwrap().into(),
        ];
        let changed_environment = Command::new(env!("CARGO_BIN_EXE_mbtx-eval"))
            .args(&replay_args)
            .env("PATH", "/invalid/frozen-replay-path")
            .output()
            .unwrap();
        assert!(!changed_environment.status.success());
        assert!(
            String::from_utf8_lossy(&changed_environment.stderr)
                .contains("original platform and environment PATH")
        );
        assert!(!replay_output.join("manifest.json").exists());
        let replayed = Command::new(env!("CARGO_BIN_EXE_mbtx-eval"))
            .args(&replay_args)
            .output()
            .unwrap();
        assert!(
            replayed.status.success(),
            "{}",
            String::from_utf8_lossy(&replayed.stderr)
        );
        assert_eq!(
            json_file(replay_output.join("summary.json"))["valid_pairs"],
            1
        );
        assert_eq!(
            json_file(replay_output.join("source-manifest.json")),
            manifest
        );
        assert_eq!(fs::read(output.join("events.jsonl")).unwrap(), before);
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn interrupted_evidence_is_not_silently_completed() {
    let root = scratch("evidence");
    let path = root.join("events.jsonl");
    fs::write(&path, b"{\"status\":\"success\"}\n{\"status\":").unwrap();
    let (rows, complete) = codex_mbtx_contract::evidence::lines(&path).unwrap();
    assert_eq!(rows, vec![json!({"status":"success"})]);
    assert!(!complete);
    assert!(codex_mbtx_contract::evidence::write_new(&path, b"overwrite").is_err());
    assert!(fs::read_to_string(&path).unwrap().contains("success"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[ignore = "requires MBTX_TEST_LAUNCHER; observes real Unix wait status"]
fn launcher_preserves_numeric_and_signal_exit_status() {
    let launcher = std::env::var("MBTX_TEST_LAUNCHER").unwrap();
    for arguments in [vec!["exit", "143"], vec!["self-term"]] {
        let direct = Command::new(env!("CARGO_BIN_EXE_mbtx-fixture"))
            .args(&arguments)
            .output()
            .unwrap();
        let wrapped = Command::new(&launcher)
            .args(["exec", "--", env!("CARGO_BIN_EXE_mbtx-fixture")])
            .args(&arguments)
            .output()
            .unwrap();
        assert_eq!(wrapped.status.code(), direct.status.code());
        assert_eq!(wrapped.status.signal(), direct.status.signal());
        assert_eq!(wrapped.stdout, direct.stdout);
    }
    let killed = Command::new(&launcher)
        .args(["exec", "--", "/bin/sh", "-c", "kill -KILL $$"])
        .output()
        .unwrap();
    assert_eq!(killed.status.signal(), Some(9));
}

#[test]
#[ignore = "requires MBTX_TEST_LAUNCHER; real TERM grace and reap with PID/group delivery"]
fn launcher_escalates_ignored_term_and_reaps_child() {
    let launcher = std::env::var("MBTX_TEST_LAUNCHER").unwrap();
    for group_delivery in [false, true] {
        let root = scratch("cancel");
        let mut command = Command::new(&launcher);
        command
            .args([
                "exec",
                "--",
                env!("CARGO_BIN_EXE_mbtx-fixture"),
                "ignore-term",
            ])
            .env("MBTX_FIXTURE_DIR", &root)
            .process_group(0)
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        if group_delivery {
            command.env("MBTX_CANCEL_SCOPE", "caller-process-group");
        }
        let mut child = KillOnDrop(command.spawn().unwrap());
        let start = Instant::now();
        let fixture_pid = loop {
            let ready = fs::read_dir(&root)
                .unwrap()
                .filter_map(Result::ok)
                .find(|e| e.file_name().to_string_lossy().ends_with(".ready.json"));
            if let Some(file) = ready {
                break json_file(file.path())["pid"].as_i64().unwrap() as i32;
            }
            assert!(start.elapsed() < Duration::from_secs(5));
            thread::sleep(Duration::from_millis(10));
        };
        let target = child.0.id() as i32 * if group_delivery { -1 } else { 1 };
        let sent = Instant::now();
        assert_eq!(unsafe { libc::kill(target, libc::SIGTERM) }, 0);
        let status = loop {
            if let Some(status) = child.0.try_wait().unwrap() {
                break status;
            }
            if sent.elapsed() > Duration::from_secs(3) {
                unsafe {
                    libc::kill(-(child.0.id() as i32), libc::SIGKILL);
                }
                panic!("launcher failed to complete cancellation");
            }
            thread::sleep(Duration::from_millis(10));
        };
        assert_eq!(status.signal(), Some(9));
        assert!(sent.elapsed() >= Duration::from_millis(450));
        assert_eq!(
            unsafe { libc::kill(fixture_pid, 0) },
            -1,
            "wait must reap the child"
        );
        fs::remove_dir_all(root).unwrap();
    }
}

#[cfg(target_os = "linux")]
#[test]
#[ignore = "requires bubblewrap user/PID namespaces; checks host identity and repeated namespace PIDs"]
fn linux_fixture_namespaces_resolve_host_pids_and_keep_distinct_evidence() {
    use codex_mbtx_contract::process_identity::{Resolution, resolve};
    let root = scratch("namespace");
    let mut seen = std::collections::BTreeSet::new();
    for _ in 0..2 {
        let mut child = KillOnDrop(
            Command::new("bwrap")
                .args([
                    "--unshare-user",
                    "--unshare-pid",
                    "--new-session",
                    "--ro-bind",
                    "/",
                    "/",
                    "--bind",
                ])
                .arg(&root)
                .arg(&root)
                .args([
                    "--proc",
                    "/proc",
                    "--",
                    env!("CARGO_BIN_EXE_mbtx-fixture"),
                    "term",
                ])
                .env("MBTX_FIXTURE_DIR", &root)
                .stdout(Stdio::null())
                .stderr(Stdio::inherit())
                .process_group(0)
                .spawn()
                .unwrap(),
        );
        let deadline = Instant::now() + Duration::from_secs(5);
        let receipt = loop {
            if let Some(entry) = fs::read_dir(&root)
                .unwrap()
                .filter_map(Result::ok)
                .find(|e| {
                    e.file_name().to_string_lossy().ends_with(".ready.json")
                        && !seen.contains(&e.file_name())
                })
            {
                seen.insert(entry.file_name());
                break json_file(entry.path());
            }
            assert!(
                Instant::now() < deadline,
                "fixture never entered its namespace"
            );
            thread::sleep(Duration::from_millis(10));
        };
        let Resolution::Live { pid, identity } = resolve(&receipt) else {
            panic!("host mapping missing: {receipt}");
        };
        assert_ne!(pid as i64, receipt["pid"].as_i64().unwrap());
        assert_eq!(
            codex_mbtx_contract::process_identity::identity(pid),
            Some(identity)
        );
        assert_eq!(unsafe { libc::kill(pid, libc::SIGTERM) }, 0);
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if child.0.try_wait().unwrap().is_some() {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "namespace child did not terminate"
            );
            thread::sleep(Duration::from_millis(10));
        }
        assert!(matches!(resolve(&receipt), Resolution::Gone));
    }
    assert_eq!(
        fs::read_dir(&root)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|e| e.file_name().to_string_lossy().ends_with(".started.json"))
            .count(),
        2
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[ignore = "requires MBTX_TEST_CODEX, MBTX_TEST_LAUNCHER and MBTX_TEST_MODEL; offline HTTP failure classification"]
fn real_codex_external_faults_remain_reportable() {
    let root = scratch("faults");
    let model = std::env::var("MBTX_TEST_MODEL").unwrap();
    for fault in ["429", "503", "disconnect", "detached"] {
        let output = root.join(fault);
        let run = Command::new(env!("CARGO_BIN_EXE_mbtx-eval"))
            .args([
                "run",
                "--suite",
                "codex-replay",
                "--codex",
                &std::env::var("MBTX_TEST_CODEX").unwrap(),
                "--mbtx",
                &std::env::var("MBTX_TEST_LAUNCHER").unwrap(),
                "--fixture",
                env!("CARGO_BIN_EXE_mbtx-fixture"),
                "--evaluation-model",
                &model,
                "--pairs",
                "1",
                "--tasks",
                if fault == "detached" {
                    "detached_descendant"
                } else {
                    "argv_empty_unicode"
                },
                "--boundaries",
                "--tool-mode",
                "direct",
                "--timeout-ms",
                "20000",
                "--fault",
                if fault == "detached" { "none" } else { fault },
                "--output",
            ])
            .arg(&output)
            .output()
            .unwrap();
        assert!(
            run.status.success(),
            "{}",
            String::from_utf8_lossy(&run.stderr)
        );
        let report = codex_mbtx_contract::reporting::build(&output, Path::new(&model)).unwrap();
        if fault == "detached" {
            // A Linux PID namespace can contain a descendant that escaped its PGID.
            // Judge the state before fallback, rather than assuming platforms match.
            for arm in report["arms"].as_array().unwrap() {
                let result = &arm["result"];
                let facts = json_file(
                    output
                        .join(result["artifact_dir"].as_str().unwrap())
                        .join("facts.json"),
                );
                assert_eq!(result["actual_exits"], result["expected_exits"]);
                if facts["processes_clean"] == true {
                    assert_eq!(result["status"], "success");
                } else {
                    assert_eq!(facts["processes_clean"], false);
                    assert_eq!(result["status"], "backend_failure");
                    assert_eq!(result["failure_class"], "lifecycle_residual");
                }
            }
        } else {
            assert_eq!(
                report["summary"]["intention_to_treat"]["arm_statuses"]["relay_error"],
                2,
                "summary={}; evidence={}",
                report["summary"],
                output.display()
            );
            assert_eq!(report["summary"]["strict_comparable_pairs"], 0);
            assert_eq!(report["summary"]["partial"], true);
        }
        for format in ["md", "html", "csv", "json", "trace"] {
            assert!(
                !codex_mbtx_contract::reporting::render(&report, format)
                    .unwrap()
                    .is_empty()
            );
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[ignore = "requires MBTX_TEST_CODEX and MBTX_TEST_LAUNCHER; real policy denial and invalid transparent configuration"]
fn real_codex_policy_and_configuration_boundaries() {
    use codex_mbtx_contract::{collection, replay::Replay};
    let root = scratch("policy");
    let fixture = env!("CARGO_BIN_EXE_mbtx-fixture");
    let launcher = std::env::var("MBTX_TEST_LAUNCHER").unwrap();
    let replay = Replay::start(root.join("responses")).unwrap();
    for case in [
        "shell-denied",
        "transparent-denied",
        "missing-config",
        "relative-launcher",
        "invalid-backend",
    ] {
        let before: std::collections::BTreeSet<_> = fs::read_dir(root.join("responses"))
            .unwrap()
            .filter_map(Result::ok)
            .map(|e| e.file_name())
            .collect();
        let directory = root.join(case);
        let home = directory.join("home");
        let trace = directory.join("trace");
        fs::create_dir_all(home.join("rules")).unwrap();
        fs::create_dir(&trace).unwrap();
        let mut config = collection::config(
            "gpt-5.6-terra",
            &replay.url,
            (case != "shell-denied").then_some(Path::new(&launcher)),
            true,
            "direct",
        )
        .unwrap();
        if case == "missing-config" || case == "relative-launcher" || case == "invalid-backend" {
            let mut value: toml::Value = toml::from_str(&config).unwrap();
            if case == "missing-config" {
                value.as_table_mut().unwrap().remove("mbtx_command");
            } else if case == "invalid-backend" {
                value["mbtx_backend"] = toml::Value::String("invalid".into());
            } else {
                value["mbtx_command"] =
                    toml::Value::Array(vec![toml::Value::String("relative-mbtx".into())]);
            }
            config = toml::to_string(&value).unwrap();
        } else {
            fs::write(
                home.join("rules/default.rules"),
                format!(
                    "prefix_rule(pattern=[{}], decision=\"forbidden\")\n",
                    json!(fixture)
                ),
            )
            .unwrap();
        }
        fs::write(home.join("config.toml"), config).unwrap();
        *replay.active.lock().unwrap() = json!({"tool_mode":"direct","context":{"attempt_id":case},
            "task":{"steps":[{"tool":"exec_command","arguments":{"cmd":format!("'{fixture}' small"),"yield_time_ms":1000}}]}});
        let output = Command::new(std::env::var("MBTX_TEST_CODEX").unwrap())
            .args([
                "--json",
                "--strict-config",
                "--ephemeral",
                "--skip-git-repo-check",
                "--cd",
            ])
            .arg(&directory)
            .arg("Execute the specified command once.")
            .env("CODEX_HOME", &home)
            .env("HOME", &home)
            .env("OPENAI_API_KEY", "offline")
            .env("RUST_LOG", "codex_core::exec_policy=trace")
            .env("MBTX_TRACE_DIR", &trace)
            .env("MBTX_FIXTURE_DIR", &trace)
            .env("MBTX_LAUNCH_TRACE", trace.join("launcher.jsonl"))
            .output()
            .unwrap();
        let text = format!(
            "{} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        fs::write(directory.join("codex.log"), &text).unwrap();
        assert!(
            !fs::read_dir(&trace)
                .unwrap()
                .filter_map(Result::ok)
                .any(|e| e.file_name().to_string_lossy().ends_with(".started.json")),
            "{case}: denied command ran"
        );
        assert!(
            !trace.join("launcher.jsonl").exists(),
            "{case}: launcher must not bypass validation/policy"
        );
        let response_bodies = fs::read_dir(root.join("responses"))
            .unwrap()
            .filter_map(Result::ok)
            .filter(|e| !before.contains(&e.file_name()))
            .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
            .filter_map(|e| fs::read_to_string(e.path()).ok())
            .collect::<Vec<_>>()
            .join("\n");
        let errors = format!("{text}\n{response_bodies}");
        assert!(
            errors.contains("forbidden")
                || errors.contains("blocked")
                || errors.contains("mbtx_command")
                || errors.contains("mbtx_backend")
                || errors.contains("rejected"),
            "{case}: no explicit error: {text}"
        );
    }
    drop(replay);
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[ignore = "requires prebuilt Codex, launcher and model; kills a real collector and resumes immutable evidence"]
fn real_codex_collector_kill_timeout_and_recovery() {
    let root = scratch("interrupted");
    let model = std::env::var("MBTX_TEST_MODEL").unwrap();
    let args = |out: &Path, timeout: &str, task: &str, pairs: &str| {
        vec![
            "run".into(),
            "--suite".into(),
            "codex-replay".into(),
            "--codex".into(),
            std::env::var("MBTX_TEST_CODEX").unwrap(),
            "--mbtx".into(),
            std::env::var("MBTX_TEST_LAUNCHER").unwrap(),
            "--evaluation-model".into(),
            model.clone(),
            "--fixture".into(),
            env!("CARGO_BIN_EXE_mbtx-fixture").into(),
            "--output".into(),
            out.display().to_string(),
            "--tool-mode".into(),
            "direct".into(),
            "--pairs".into(),
            pairs.into(),
            "--tasks".into(),
            task.into(),
            "--timeout-ms".into(),
            timeout.into(),
        ]
    };
    let timeout = root.join("timeout");
    let run = Command::new(env!("CARGO_BIN_EXE_mbtx-eval"))
        .args(args(&timeout, "1", "argv_empty_unicode", "1"))
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let report = codex_mbtx_contract::reporting::build(&timeout, Path::new(&model)).unwrap();
    assert_eq!(report["summary"]["strict_comparable_pairs"], 0);
    assert_eq!(
        report["summary"]["intention_to_treat"]["arm_statuses"]["timeout"],
        2
    );

    let output = root.join("killed");
    let options = args(&output, "10000", "session_poll", "2");
    let mut collector = KillOnDrop(
        Command::new(env!("CARGO_BIN_EXE_mbtx-eval"))
            .args(&options)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let deadline = Instant::now() + Duration::from_secs(20);
    let completed = output.join("attempts/r1-session_poll-a00-shell");
    let interrupted = output.join("attempts/r1-session_poll-a00-transparent");
    loop {
        if completed.join("seal.json").exists() && interrupted.join("events.jsonl").exists() {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "collector failed before interruption point"
        );
        thread::sleep(Duration::from_millis(5));
    }
    collector.0.kill().unwrap();
    collector.0.wait().unwrap();
    let seal_before = fs::read(completed.join("seal.json")).unwrap();
    let partial_before = fs::read(interrupted.join("events.jsonl")).unwrap();
    // Only the Codex process launched by this interrupted attempt is signalled.
    let events = codex_mbtx_contract::evidence::lines(&interrupted.join("events.jsonl"))
        .unwrap()
        .0;
    for row in &events {
        if row["phase"] == "codex" && row["event"] == "spawn_return" {
            if let Some(pid) = row["child_pid"].as_i64() {
                unsafe {
                    libc::kill(-(pid as i32), libc::SIGTERM);
                }
            }
        }
    }
    let partial = codex_mbtx_contract::reporting::build(&output, Path::new(&model)).unwrap();
    assert_eq!(partial["summary"]["partial"], true);
    assert!(
        codex_mbtx_contract::reporting::render(&partial, "html")
            .unwrap()
            .contains("application/json")
    );
    let resumed = Command::new(env!("CARGO_BIN_EXE_mbtx-eval"))
        .args(&options)
        .arg("--resume")
        .output()
        .unwrap();
    assert!(
        resumed.status.success(),
        "{}",
        String::from_utf8_lossy(&resumed.stderr)
    );
    assert_eq!(fs::read(completed.join("seal.json")).unwrap(), seal_before);
    assert_eq!(
        fs::read(interrupted.join("events.jsonl")).unwrap(),
        partial_before
    );
    let after = codex_mbtx_contract::reporting::build(&output, Path::new(&model)).unwrap();
    assert_eq!(
        after["summary"]["intention_to_treat"]["arm_statuses"]["censored"],
        1
    );
    assert_eq!(after["summary"]["strict_comparable_pairs"], 1);
    fs::write(completed.join("injected.jsonl"), "{}\n").unwrap();
    let tampered = codex_mbtx_contract::reporting::build(&output, Path::new(&model)).unwrap();
    assert_eq!(
        tampered["summary"]["intention_to_treat"]["arm_statuses"]["harness_error"],
        1
    );
    fs::remove_dir_all(root).unwrap();
}
