use serde_json::{Value, json};
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Observer {
    directory: PathBuf,
    sequence: AtomicU64,
}

fn now_ns() -> Option<u64> {
    #[cfg(unix)]
    {
        let mut t = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut t) } != 0 {
            return None;
        }
        Some(t.tv_sec as u64 * 1_000_000_000 + t.tv_nsec as u64)
    }
    #[cfg(not(unix))]
    {
        None
    }
}

impl Observer {
    pub fn new(directory: PathBuf) -> io::Result<Self> {
        fs::create_dir_all(&directory)?;
        Ok(Self {
            directory,
            sequence: AtomicU64::new(1),
        })
    }

    pub fn request(self: &Arc<Self>) -> RequestTrace {
        let context = fs::read(self.directory.join("context.json"))
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
            .unwrap_or(json!({}));
        let id = self.sequence.fetch_add(1, Ordering::Relaxed);
        RequestTrace {
            observer: Arc::clone(self),
            context,
            id,
            sequence: AtomicU64::new(1),
        }
    }
}

pub struct RequestTrace {
    observer: Arc<Observer>,
    context: Value,
    id: u64,
    sequence: AtomicU64,
}

impl RequestTrace {
    pub fn emit(&self, event: &str, detail: Value) {
        let mut row = json!({"schema_version":2,"phase":"external","event":event,
            "request_id":self.id,"sequence":self.sequence.fetch_add(1, Ordering::Relaxed),
            "monotonic_ns":now_ns(),"clock_domain":"os-monotonic","parent_id":null,
            "pid":std::process::id(),"exit_code":null,"signal":null,"stdout_bytes":null,
            "stderr_bytes":null,"status":"unknown","failure_class":null,"confidence":"observed","detail":detail});
        if let Some(context) = self.context.as_object() {
            row.as_object_mut().unwrap().extend(context.clone());
        }
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(
            self.observer
                .directory
                .join(format!("request-{:06}.jsonl", self.id)),
        ) {
            let _ = writeln!(file, "{row}");
        }
    }
}

pub struct ObservedBody<R> {
    pub body: R,
    pub trace: Arc<RequestTrace>,
    pub first: bool,
    pub finished: bool,
    pub pending: Vec<u8>,
}

impl<R> ObservedBody<R> {
    fn observe_sse(&mut self, bytes: &[u8]) {
        self.pending.extend_from_slice(bytes);
        while let Some(end) = self.pending.iter().position(|b| *b == b'\n') {
            let line: Vec<_> = self.pending.drain(..=end).collect();
            let Some(data) = line.strip_prefix(b"data:") else {
                continue;
            };
            let Ok(value) = serde_json::from_slice::<Value>(data) else {
                continue;
            };
            if value["type"] == "response.output_item.done"
                && matches!(
                    value["item"]["type"].as_str(),
                    Some("function_call" | "custom_tool_call")
                )
            {
                self.trace.emit(
                    "tool_arguments_complete",
                    json!({"call_id":value["item"]["call_id"],"tool":value["item"]["name"]}),
                );
            }
            if value["type"] == "response.completed" {
                self.trace.emit("response_completed", json!({"response_id":value["response"]["id"],"usage":value["response"]["usage"]}));
            }
        }
        if self.pending.len() > 4 * 1024 * 1024 {
            self.pending.clear();
            self.trace.emit(
                "sse_observation_censored",
                json!({"reason":"line exceeds observation cap"}),
            );
        }
    }
}

impl<R: Read> Read for ObservedBody<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.body.read(buf) {
            Ok(n) => {
                if n > 0 && !self.first {
                    self.trace.emit("first_byte", json!({}));
                    self.first = true;
                }
                if n == 0 {
                    self.finished = true;
                }
                self.trace.emit(
                    if n == 0 { "stream_eof" } else { "stream_chunk" },
                    json!({"bytes":n}),
                );
                self.observe_sse(&buf[..n]);
                Ok(n)
            }
            Err(error) => {
                self.trace
                    .emit("stream_error", json!({"error":error.to_string()}));
                Err(error)
            }
        }
    }
}

impl<R> Drop for ObservedBody<R> {
    fn drop(&mut self) {
        if !self.finished {
            self.trace.emit("stream_censored", json!({}));
        }
    }
}
