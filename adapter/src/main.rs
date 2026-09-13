use serde::{Deserialize, Serialize};
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::{env, fs, process::Command, time::Instant};

#[derive(Debug, Deserialize)]
struct Job {
    experiment_id: String,
    pair_id: String,
    attempt_id: String,
    task_id: String,
    backend: String,
    command: Vec<String>,
    cwd: Option<String>,
    artifact_dir: String,
    #[serde(default)]
    expected_exit_code: Option<i32>,
}

#[derive(Debug, Serialize)]
struct Event<'a> {
    schema_version: u32,
    experiment_id: &'a str,
    pair_id: &'a str,
    attempt_id: &'a str,
    task_id: &'a str,
    backend: &'a str,
    phase: &'static str,
    event: &'static str,
    parent_id: Option<&'a str>,
    monotonic_ns: u128,
    clock_domain: &'static str,
    exit_code: Option<i32>,
    signal: Option<i32>,
    stdout_bytes: Option<usize>,
    stderr_bytes: Option<usize>,
    status: &'static str,
    failure_class: Option<&'static str>,
    artifact_dir: Option<&'a str>,
}

fn emit(event: Event<'_>) {
    println!("{}", serde_json::to_string(&event).unwrap());
}

fn emit_trace(
    job: &Job,
    phase: &'static str,
    event: &'static str,
    started: u128,
    extra: serde_json::Value,
) {
    let mut row = serde_json::json!({
        "schema_version": 2,
        "experiment_id": job.experiment_id,
        "pair_id": job.pair_id,
        "attempt_id": job.attempt_id,
        "task_id": job.task_id,
        "backend": job.backend,
        "phase": phase,
        "event": event,
        "parent_id": serde_json::Value::Null,
        "monotonic_ns": started,
        "clock_domain": "observer-monotonic",
        "status": "unknown",
        "failure_class": serde_json::Value::Null,
        "confidence": "observed",
        "pid": std::process::id(),
    });
    if let (Some(target), Some(fields)) = (row.as_object_mut(), extra.as_object()) {
        target.extend(fields.clone());
    }
    println!("{}", serde_json::to_string(&row).unwrap());
}

fn run(job: &Job, origin: Instant) {
    let start = origin.elapsed().as_nanos();
    emit_trace(
        job,
        "observer",
        "spawn_begin",
        start,
        serde_json::json!({
            "command": job.command,
            "cwd": job.cwd,
        }),
    );
    emit(Event {
        schema_version: 2,
        experiment_id: &job.experiment_id,
        pair_id: &job.pair_id,
        attempt_id: &job.attempt_id,
        task_id: &job.task_id,
        backend: &job.backend,
        phase: "child",
        event: "start",
        parent_id: None,
        monotonic_ns: start,
        clock_domain: "observer-monotonic",
        exit_code: None,
        signal: None,
        stdout_bytes: None,
        stderr_bytes: None,
        status: "unknown",
        failure_class: None,
        artifact_dir: None,
    });
    let Some((program, args)) = job.command.split_first() else {
        emit(Event {
            schema_version: 2,
            experiment_id: &job.experiment_id,
            pair_id: &job.pair_id,
            attempt_id: &job.attempt_id,
            task_id: &job.task_id,
            backend: &job.backend,
            phase: "child",
            event: "exit",
            parent_id: None,
            monotonic_ns: origin.elapsed().as_nanos(),
            clock_domain: "observer-monotonic",
            exit_code: None,
            signal: None,
            stdout_bytes: None,
            stderr_bytes: None,
            status: "harness_error",
            failure_class: Some("invalid_command"),
            artifact_dir: None,
        });
        return;
    };
    let mut command = Command::new(program);
    command.args(args);
    if let Some(cwd) = &job.cwd {
        command.current_dir(cwd);
    }
    let result = command.output();
    let after_wait = origin.elapsed().as_nanos();
    emit_trace(
        job,
        "observer",
        "wait_return",
        after_wait,
        serde_json::json!({
            "wait_ns": after_wait.saturating_sub(start),
        }),
    );
    match result {
        Ok(output) => {
            let code = output.status.code();
            #[cfg(unix)]
            let signal = output.status.signal();
            #[cfg(not(unix))]
            let signal: Option<i32> = None;
            let observed_code = code.or_else(|| signal.map(|value| 128 + value));
            let status = if job.expected_exit_code == Some(observed_code.unwrap_or(-1))
                || (job.expected_exit_code.is_none() && code == Some(0))
            {
                "success"
            } else {
                "backend_failure"
            };
            let _ = fs::create_dir_all(&job.artifact_dir);
            let _ = fs::write(format!("{}/stdout", job.artifact_dir), &output.stdout);
            let _ = fs::write(format!("{}/stderr", job.artifact_dir), &output.stderr);
            let artifact_end = origin.elapsed().as_nanos();
            emit_trace(
                job,
                "observer",
                "artifact_write",
                artifact_end,
                serde_json::json!({
                    "artifact_write_ns": artifact_end.saturating_sub(after_wait),
                    "stdout_bytes": output.stdout.len(),
                    "stderr_bytes": output.stderr.len(),
                }),
            );
            emit(Event {
                schema_version: 2,
                experiment_id: &job.experiment_id,
                pair_id: &job.pair_id,
                attempt_id: &job.attempt_id,
                task_id: &job.task_id,
                backend: &job.backend,
                phase: "child",
                event: "exit",
                parent_id: None,
                monotonic_ns: origin.elapsed().as_nanos(),
                clock_domain: "observer-monotonic",
                exit_code: code,
                signal,
                stdout_bytes: Some(output.stdout.len()),
                stderr_bytes: Some(output.stderr.len()),
                status,
                failure_class: None,
                artifact_dir: Some(&job.artifact_dir),
            });
        }
        Err(_) => emit(Event {
            schema_version: 2,
            experiment_id: &job.experiment_id,
            pair_id: &job.pair_id,
            attempt_id: &job.attempt_id,
            task_id: &job.task_id,
            backend: &job.backend,
            phase: "child",
            event: "exit",
            parent_id: None,
            monotonic_ns: origin.elapsed().as_nanos(),
            clock_domain: "observer-monotonic",
            exit_code: None,
            signal: None,
            stdout_bytes: None,
            stderr_bytes: None,
            status: "harness_error",
            failure_class: Some("spawn"),
            artifact_dir: None,
        }),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(path) = args
        .iter()
        .position(|a| a == "--jobs")
        .and_then(|i| args.get(i + 1))
    else {
        eprintln!("usage: mbtx-observe --jobs JOBS.json");
        std::process::exit(2);
    };
    let data = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("cannot read jobs: {e}");
        std::process::exit(2)
    });
    let jobs: Vec<Job> = serde_json::from_str(&data).unwrap_or_else(|e| {
        eprintln!("invalid jobs: {e}");
        std::process::exit(2)
    });
    let origin = Instant::now();
    for job in &jobs {
        run(job, origin);
    }
}
