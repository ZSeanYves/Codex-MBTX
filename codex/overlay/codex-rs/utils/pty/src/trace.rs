//! Opt-in evaluation trace. It observes existing waits and stream reads without owning them.
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

static SEQUENCE: AtomicU64 = AtomicU64::new(1);
static FILE: OnceLock<Option<Mutex<File>>> = OnceLock::new();

pub fn now_ns() -> Option<u64> {
    #[cfg(unix)]
    {
        let mut time = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut time) } != 0 {
            return None;
        }
        Some(time.tv_sec as u64 * 1_000_000_000 + time.tv_nsec as u64)
    }
    #[cfg(not(unix))]
    {
        None
    }
}

pub fn enabled() -> bool {
    std::env::var_os("MBTX_TRACE_DIR").is_some()
}

pub fn next_id() -> u64 {
    SEQUENCE.fetch_add(1, Ordering::Relaxed)
}

pub struct Span {
    phase: &'static str,
    detail: String,
}
impl Drop for Span {
    fn drop(&mut self) {
        emit(self.phase, "return", &self.detail);
    }
}
pub fn span(phase: &'static str, detail: String) -> Span {
    emit(phase, "begin", &detail);
    Span { phase, detail }
}

pub fn resources(call_id: &str) {
    if !enabled() {
        return;
    }
    let directory = if cfg!(target_os = "linux") {
        "/proc/self/fd"
    } else {
        "/dev/fd"
    };
    let count = std::fs::read_dir(directory)
        .ok()
        .map(|entries| entries.filter_map(Result::ok).count());
    emit(
        "resources",
        "fd_snapshot",
        &format!(
            "{{\"call_id\":{},\"open_fds\":{}}}",
            escape_json(call_id),
            count.map_or("null".into(), |n| n.to_string())
        ),
    );
}

fn json_string(name: &str) -> String {
    let Some(value) = std::env::var_os(name) else {
        return "null".to_owned();
    };
    escape_json(&value.to_string_lossy())
}

fn escape_json(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('"');
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_control() => {
                escaped.push_str(&format!("\\u{:04x}", character as u32))
            }
            character => escaped.push(character),
        }
    }
    escaped.push('"');
    escaped
}

pub fn emit(phase: &str, event: &str, detail: &str) {
    if !enabled() {
        return;
    }
    let Some(timestamp) = now_ns() else {
        return;
    };
    let file = FILE.get_or_init(|| {
        let path = PathBuf::from(std::env::var_os("MBTX_TRACE_DIR")?);
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path.join(format!("process-{}.jsonl", std::process::id())))
            .ok()
            .map(Mutex::new)
    });
    if let Some(file) = file
        && let Ok(mut file) = file.lock()
    {
        let id = next_id();
        let row = format!(
            "{{\"schema_version\":2,\"experiment_id\":{},\"pair_id\":{},\"attempt_id\":{},\"task_id\":{},\"backend\":{},\"phase\":\"{phase}\",\"event\":\"{event}\",\"parent_id\":null,\"monotonic_ns\":{timestamp},\"clock_domain\":\"os-monotonic\",\"pid\":{},\"sequence\":{id},\"exit_code\":null,\"signal\":null,\"stdout_bytes\":null,\"stderr_bytes\":null,\"status\":\"unknown\",\"failure_class\":null,\"confidence\":\"observed\",\"detail\":{detail}}}\n",
            json_string("MBTX_EXPERIMENT_ID"),
            json_string("MBTX_PAIR_ID"),
            json_string("MBTX_ATTEMPT_ID"),
            json_string("MBTX_TASK_ID"),
            json_string("MBTX_BACKEND"),
            std::process::id()
        );
        let _ = file.write_all(row.as_bytes());
    }
}

pub fn stream(id: u64, name: &str, bytes: &[u8]) {
    if !enabled() {
        return;
    }
    let Some(path) = std::env::var_os("MBTX_TRACE_DIR").map(PathBuf::from) else {
        return;
    };
    let filename = format!("stream-{}-{id}-{name}.bin", std::process::id());
    emit(
        "io",
        if bytes.is_empty() { "eof" } else { "read" },
        &format!(
            "{{\"stream_id\":{id},\"stream\":\"{name}\",\"bytes\":{},\"file\":\"{filename}\"}}",
            bytes.len()
        ),
    );
    if !bytes.is_empty()
        && let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path.join(filename))
    {
        let _ = file.write_all(bytes);
    }
}
