//! Small, evaluation-only launcher used to prove that the Transparent MBTX
//! path was actually selected by Codex.  It logs only bounded, non-secret
//! metadata and then forwards the exact argv to the real MBTX executable.

use serde_json::json;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn append_record(path: &str, record: serde_json::Value) {
    let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) else {
        return;
    };
    let _ = writeln!(file, "{record}");
}

fn unix_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos().min(u64::MAX as u128) as u64)
        .unwrap_or_default()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let started = Instant::now();
    let trace_path = std::env::var("M5_TRANSPARENT_TRACE_FILE").ok();
    let record_args: Vec<String> = args.iter().skip(1).take(64).cloned().collect();
    if let Some(path) = trace_path.as_deref() {
        append_record(
            path,
            json!({
                "event": "launcher_start",
                "argv": record_args,
                "cwd": std::env::current_dir().ok().and_then(|p| p.to_str().map(str::to_owned)).unwrap_or_default(),
                "started_unix_ns": unix_ns(),
            }),
        );
    }
    let Some(real_mbtx) = args.get(1) else {
        eprintln!("m5-trace-launcher requires the real MBTX path");
        std::process::exit(64);
    };
    let status = Command::new(real_mbtx).args(&args[2..]).status();
    let code = match status {
        Ok(status) => status.code().unwrap_or(-1),
        Err(error) => {
            eprintln!("m5-trace-launcher: {error}");
            -1
        }
    };
    if let Some(path) = trace_path.as_deref() {
        append_record(
            path,
            json!({
                "event": "launcher_exit",
                "exit_code": code,
                "elapsed_ms": started.elapsed().as_millis(),
                "finished_unix_ns": unix_ns(),
            }),
        );
    }
    std::process::exit(code);
}
