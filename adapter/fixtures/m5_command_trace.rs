//! Evaluation-only command shim.  The runner places this binary at the front
//! of PATH as `python3`, so both arms get identical helper instrumentation.
//! It forwards stdout/stderr byte-for-byte while recording bounded trace data.

use serde_json::json;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn append_record(path: &str, record: serde_json::Value) {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "{record}");
    }
}

fn helper_name(args: &[String]) -> String {
    args.iter()
        .find(|arg| arg.ends_with(".py") || arg.ends_with(".mbtx"))
        .and_then(|arg| arg.rsplit('/').next())
        .unwrap_or("python3")
        .to_owned()
}

fn write_receipt(path: &str, task: &str, nonce: &str, helper: &str, status: &str) {
    if path.is_empty() {
        return;
    }
    let receipt = json!({
        "task": task,
        "nonce": nonce,
        "helper": helper,
        "status": status,
    });
    let temporary = format!("{path}.tmp-{}", std::process::id());
    if fs::write(&temporary, receipt.to_string()).is_ok() {
        let _ = fs::rename(temporary, path);
    }
}

fn unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

fn unix_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos().min(u64::MAX as u128) as u64)
        .unwrap_or_default()
}

fn stream(
    mut input: impl Read + Send + 'static,
    mut output: impl Write + Send + 'static,
    path: String,
) -> thread::JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut data = Vec::new();
        let mut buffer = [0_u8; 8192];
        loop {
            match input.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    let _ = output.write_all(&buffer[..n]);
                    let _ = output.flush();
                    if data.len() < 1_048_576 {
                        let keep = (1_048_576 - data.len()).min(n);
                        data.extend_from_slice(&buffer[..keep]);
                    }
                }
                Err(_) => break,
            }
        }
        if !path.is_empty() {
            let _ = fs::write(path, &data);
        }
        data
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let invoked_as = std::env::args()
        .next()
        .and_then(|path| {
            std::path::Path::new(&path)
                .file_name()
                .and_then(|name| name.to_str())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "python3".to_owned());
    let program_key = format!("M5_REAL_PROGRAM_{}", invoked_as.to_ascii_uppercase());
    let real_program = std::env::var(&program_key)
        .or_else(|_| std::env::var("M5_REAL_PROGRAM"))
        .unwrap_or_else(|_| {
            if invoked_as == "moon" {
                "/usr/local/bin/moon".to_owned()
            } else {
                "/usr/bin/python3".to_owned()
            }
        });
    let trace_path = std::env::var("M5_TRACE_FILE").unwrap_or_default();
    let output_prefix = std::env::var("M5_OUTPUT_PREFIX").unwrap_or_default();
    let receipt_path = std::env::var("M5_RECEIPT_PATH").unwrap_or_default();
    let task = std::env::var("M5_TASK").unwrap_or_default();
    let nonce = std::env::var("M5_TASK_NONCE").unwrap_or_default();
    let started = Instant::now();
    let cwd = std::env::current_dir()
        .ok()
        .and_then(|p| p.to_str().map(str::to_owned))
        .unwrap_or_default();
    let helper = helper_name(&args);
    if !trace_path.is_empty() {
        append_record(
            &trace_path,
            json!({
                "event": "command_start",
                "program": real_program,
                "invoked_as": invoked_as,
                "argv": args.iter().take(64).collect::<Vec<_>>(),
                "cwd": cwd,
                "helper": helper,
                "started_unix_ms": unix_ms(),
                "started_unix_ns": unix_ns(),
            }),
        );
    }
    let child_result = Command::new(&real_program)
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let Ok(mut child) = child_result else {
        eprintln!("m5-command-trace: unable to launch {real_program}");
        if !trace_path.is_empty() {
            append_record(
                &trace_path,
                json!({"event":"command_exit","exit_code":-1,"helper":helper,"finished_unix_ns":unix_ns()}),
            );
        }
        std::process::exit(127);
    };
    // The PID keeps repeated helper invocations from overwriting one another.
    let unique_prefix = if output_prefix.is_empty() {
        String::new()
    } else {
        format!("{output_prefix}-{}-{}", unix_ms(), std::process::id())
    };
    let stdout_path = if unique_prefix.is_empty() {
        String::new()
    } else {
        format!("{unique_prefix}.stdout")
    };
    let stderr_path = if unique_prefix.is_empty() {
        String::new()
    } else {
        format!("{unique_prefix}.stderr")
    };
    let stdout_thread = child
        .stdout
        .take()
        .map(|pipe| stream(pipe, std::io::stdout(), stdout_path));
    let stderr_thread = child
        .stderr
        .take()
        .map(|pipe| stream(pipe, std::io::stderr(), stderr_path));
    let status = child.wait();
    let stdout = stdout_thread
        .and_then(|thread| thread.join().ok())
        .unwrap_or_default();
    let stderr = stderr_thread
        .and_then(|thread| thread.join().ok())
        .unwrap_or_default();
    let code = status.ok().and_then(|status| status.code()).unwrap_or(-1);
    if !trace_path.is_empty() {
        append_record(
            &trace_path,
            json!({
                "event": "command_exit",
                "exit_code": code,
                "elapsed_ms": started.elapsed().as_millis(),
                "stdout_bytes": stdout.len(),
                "stderr_bytes": stderr.len(),
                "helper": helper,
                "finished_unix_ms": unix_ms(),
                "finished_unix_ns": unix_ns(),
            }),
        );
    }
    let receipt_status = if code == 143 || code == 124 {
        "stopped"
    } else {
        "completed"
    };
    write_receipt(&receipt_path, &task, &nonce, &helper, receipt_status);
    std::process::exit(code);
}
