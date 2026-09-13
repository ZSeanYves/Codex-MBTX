use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn now_ns() -> u64 {
    let mut time = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // All participating processes use the OS clock, not process-local Instant origins.
    assert_eq!(
        unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut time) },
        0
    );
    time.tv_sec as u64 * 1_000_000_000 + time.tv_nsec as u64
}

pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn write_new(path: &Path, data: &[u8]) -> io::Result<()> {
    let temporary = path.with_extension(format!("pending-{}-{}", std::process::id(), now_ns()));
    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(data)?;
        file.sync_all()?;
        // link publishes the completed inode atomically and refuses an existing target.
        fs::hard_link(&temporary, path)
    })();
    let _ = fs::remove_file(temporary);
    result
}

pub fn write_json(path: &Path, value: &Value) -> io::Result<()> {
    write_new(path, &serde_json::to_vec_pretty(value)?)
}

pub fn read_json(path: &Path) -> io::Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

pub fn lines(path: &Path) -> io::Result<(Vec<Value>, bool)> {
    let bytes = fs::read(path)?;
    let complete = bytes.is_empty() || bytes.ends_with(b"\n");
    let mut values = Vec::new();
    let chunks: Vec<_> = bytes.split(|c| *c == b'\n').collect();
    for (i, line) in chunks.iter().enumerate() {
        if line.is_empty() {
            continue;
        }
        if i == chunks.len() - 1 && !complete {
            break;
        }
        match serde_json::from_slice(line) {
            Ok(value) => values.push(value),
            Err(_) => return Ok((values, false)),
        }
    }
    Ok((values, complete))
}

pub fn normalize_trace(mut row: Value) -> Value {
    if let Some(detail) = row.get("detail").and_then(Value::as_object).cloned() {
        for (key, value) in detail {
            if key == "status" {
                row["http_status"] = value;
            } else {
                row[&key] = value;
            }
        }
    }
    row
}

#[derive(Clone)]
pub struct Trace {
    inner: Arc<Mutex<(File, u64)>>,
    context: Value,
    namespace: String,
}

impl Trace {
    pub fn create(path: &Path, context: Value) -> io::Result<Self> {
        let file = OpenOptions::new().write(true).create_new(true).open(path)?;
        Ok(Self {
            inner: Arc::new(Mutex::new((file, 0))),
            context,
            namespace: digest(path.to_string_lossy().as_bytes()),
        })
    }

    pub fn emit(&self, phase: &str, event: &str, detail: Value) -> io::Result<()> {
        self.at(now_ns(), phase, event, detail)
    }

    pub fn at(&self, timestamp: u64, phase: &str, event: &str, detail: Value) -> io::Result<()> {
        let mut row = json!({"schema_version":2,"monotonic_ns":timestamp,
            "clock_domain":"os-monotonic","phase":phase,"event":event,
            "pid":std::process::id(),"pgid":unsafe { libc::getpgrp() },
            "parent_id":null,"exit_code":null,"signal":null,"stdout_bytes":null,
            "stderr_bytes":null,"status":"unknown","failure_class":null,
            "confidence":"observed"});
        if let Some(context) = self.context.as_object() {
            row.as_object_mut().unwrap().extend(context.clone());
        }
        if let Some(fields) = detail.as_object() {
            row.as_object_mut().unwrap().extend(fields.clone());
        }
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| io::Error::other("trace lock poisoned"))?;
        inner.1 += 1;
        row["sequence"] = json!(inner.1);
        row["event_id"] = json!(format!(
            "{}:{}:{}",
            std::process::id(),
            self.namespace,
            inner.1
        ));
        serde_json::to_writer(&mut inner.0, &row)?;
        inner.0.write_all(b"\n")?;
        inner.0.flush()
    }
}

pub(crate) fn inventory(directory: &Path) -> io::Result<Vec<Value>> {
    fn entries(root: &Path, path: &Path, rows: &mut Vec<Value>) -> io::Result<()> {
        let mut children: Vec<_> = fs::read_dir(path)?.collect::<Result<_, _>>()?;
        children.sort_by_key(|entry| entry.file_name());
        for entry in children {
            let kind = entry.file_type()?;
            if kind.is_dir() {
                entries(root, &entry.path(), rows)?;
            } else if kind.is_file() && entry.file_name() != "seal.json" {
                rows.push(json!({"path":entry.path().strip_prefix(root).unwrap(),
                    "sha256":digest(&fs::read(entry.path())?)}));
            }
        }
        Ok(())
    }
    let mut rows = Vec::new();
    entries(directory, directory, &mut rows)?;
    Ok(rows)
}

pub fn seal(directory: &Path) -> io::Result<()> {
    let rows = inventory(directory)?;
    write_json(
        &directory.join("seal.json"),
        &json!({"schema_version":2,"files":rows}),
    )
}
