//! The MBTX wire contract, independent of the Codex agent and process launcher.
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;

const MAX_REQUEST_BYTES: usize = 32 * 1024;
const MAX_FRAME_BYTES: usize = 1024 * 1024;
const MAX_BATCH_BYTES: usize = 8192;

#[derive(Debug, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Arguments {
    Run(RunArguments),
    JobOutput { job_id: String },
    JobStop { job_id: String },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunArguments {
    pub source: Option<String>,
    pub script_path: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
    pub cwd: Option<String>,
    #[serde(default)]
    pub background: bool,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

fn default_timeout() -> u64 {
    30_000
}

impl RunArguments {
    /// Build one canonical JSONL run request. The host owns background scheduling.
    pub fn request(&self, id: &str, cwd: &str) -> Result<String, String> {
        if !(1..=600_000).contains(&self.timeout_ms) {
            return Err("timeout_ms must be between 1 and 600000".into());
        }
        for text in std::iter::once(id)
            .chain(std::iter::once(cwd))
            .chain(self.source.as_deref())
            .chain(self.script_path.as_deref())
        {
            if text.trim().is_empty() || text.contains('\0') {
                return Err("id, cwd and script must be nonempty and contain no NUL".into());
            }
        }
        if self.args.iter().any(|arg| arg.contains('\0')) {
            return Err("args must not contain NUL".into());
        }
        let mut request = json!({
            "id": id, "op": "run", "args": self.args, "cwd": cwd,
            "background": false, "timeout_ms": self.timeout_ms
        });
        match (&self.source, &self.script_path) {
            (Some(source), None) => request["source"] = json!(source),
            (None, Some(path)) if path.ends_with(".mbtx") => request["script_path"] = json!(path),
            _ => return Err("provide exactly one source or .mbtx script_path".into()),
        }
        let encoded = request.to_string();
        if encoded.len() > MAX_REQUEST_BYTES {
            return Err(
                "MBTX tool request exceeds 32 KiB; use script_path for larger scripts".into(),
            );
        }
        Ok(encoded)
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
enum Event {
    Started {
        id: String,
        job_id: String,
    },
    Stdout {
        id: String,
        job_id: String,
        seq: usize,
        data: String,
    },
    Stderr {
        id: String,
        job_id: String,
        seq: usize,
        data: String,
    },
    Completed {
        id: String,
        job_id: String,
        exit_code: i32,
        duration_ms: u64,
    },
    Stopped {
        id: String,
        job_id: String,
        duration_ms: u64,
    },
    Failed {
        id: String,
        job_id: String,
        error: Failure,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Failure {
    pub kind: String,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum Terminal {
    Completed { exit_code: i32, duration_ms: u64 },
    Stopped { duration_ms: u64 },
    Failed { error: Failure },
}

#[derive(Debug, Default, Serialize)]
pub struct Batch {
    pub stdout: String,
    pub stderr: String,
    pub omitted_bytes: usize,
}

impl Batch {
    fn append(&mut self, data: &str, stderr: bool) {
        let available = MAX_BATCH_BYTES.saturating_sub(self.stdout.len() + self.stderr.len());
        let end = data.floor_char_boundary(available.min(data.len()));
        if stderr {
            self.stderr.push_str(&data[..end]);
        } else {
            self.stdout.push_str(&data[..end]);
        }
        self.omitted_bytes += data.len() - end;
    }
}

#[derive(Debug)]
pub struct Progress {
    id: String,
    job_id: Option<String>,
    next_seq: usize,
    pending: Vec<u8>,
    pub terminal: Option<Terminal>,
}

impl Progress {
    pub fn new(id: String) -> Self {
        Self {
            id,
            job_id: None,
            next_seq: 0,
            pending: Vec::new(),
            terminal: None,
        }
    }

    /// Decode arbitrary pipe fragments, enforcing correlation and lifecycle order.
    pub fn ingest(&mut self, bytes: &[u8]) -> Result<Batch, String> {
        let mut batch = Batch::default();
        for segment in bytes.split_inclusive(|byte| *byte == b'\n') {
            if self.pending.len() + segment.len() > MAX_FRAME_BYTES {
                return Err("runner JSONL frame exceeds 1 MiB".into());
            }
            self.pending.extend_from_slice(segment);
            if self.pending.last() != Some(&b'\n') {
                continue;
            }
            let event: Event = serde_json::from_slice(&self.pending)
                .map_err(|error| format!("invalid runner JSONL: {error}"))?;
            self.pending.clear();
            if self.terminal.is_some() {
                return Err("runner emitted an event after its terminal event".into());
            }
            match event {
                Event::Started { id, job_id } => {
                    if id != self.id || job_id.is_empty() || self.job_id.is_some() {
                        return Err("invalid or duplicate runner started event".into());
                    }
                    self.job_id = Some(job_id);
                }
                Event::Stdout {
                    id,
                    job_id,
                    seq,
                    data,
                } => {
                    self.output_event(&id, &job_id, seq)?;
                    batch.append(&data, false);
                }
                Event::Stderr {
                    id,
                    job_id,
                    seq,
                    data,
                } => {
                    self.output_event(&id, &job_id, seq)?;
                    batch.append(&data, true);
                }
                Event::Completed {
                    id,
                    job_id,
                    exit_code,
                    duration_ms,
                } => {
                    self.correlate(&id, &job_id)?;
                    self.terminal = Some(Terminal::Completed {
                        exit_code,
                        duration_ms,
                    });
                }
                Event::Stopped {
                    id,
                    job_id,
                    duration_ms,
                } => {
                    self.correlate(&id, &job_id)?;
                    self.terminal = Some(Terminal::Stopped { duration_ms });
                }
                Event::Failed {
                    id,
                    job_id,
                    mut error,
                } => {
                    self.correlate(&id, &job_id)?;
                    error.kind.truncate(error.kind.floor_char_boundary(64));
                    error
                        .message
                        .truncate(error.message.floor_char_boundary(512));
                    self.terminal = Some(Terminal::Failed { error });
                }
            }
        }
        Ok(batch)
    }

    pub fn finish(&self, runner_exit_code: i32) -> Result<(), String> {
        if runner_exit_code != 0 {
            return Err(format!("runner exited with code {runner_exit_code}"));
        }
        if !self.pending.is_empty() || self.terminal.is_none() {
            return Err("runner exited without a complete terminal event".into());
        }
        Ok(())
    }

    fn correlate(&self, id: &str, job_id: &str) -> Result<(), String> {
        if id == self.id && self.job_id.as_deref() == Some(job_id) {
            Ok(())
        } else {
            Err("runner event does not match the admitted request and job".into())
        }
    }

    fn output_event(&mut self, id: &str, job_id: &str, seq: usize) -> Result<(), String> {
        self.correlate(id, job_id)?;
        if seq != self.next_seq || seq >= 4096 {
            return Err("runner output sequence is missing, duplicated, or over limit".into());
        }
        self.next_seq += 1;
        Ok(())
    }
}

/// Bound serialized model context, including JSON escaping, while keeping valid JSON.
pub fn reply(job_id: Option<&str>, terminal: Option<&Terminal>, mut batch: Batch) -> Value {
    let mut value = terminal.map_or_else(|| json!({"state": "running"}), |state| json!(state));
    value["job_id"] = json!(job_id);
    loop {
        value["stdout"] = json!(batch.stdout);
        value["stderr"] = json!(batch.stderr);
        value["omitted_bytes"] = json!(batch.omitted_bytes);
        if value.to_string().len() <= 8000 {
            return value;
        }
        let text = if batch.stdout.len() >= batch.stderr.len() {
            &mut batch.stdout
        } else {
            &mut batch.stderr
        };
        let end = text.floor_char_boundary(text.len() / 2);
        batch.omitted_bytes += text.len() - end;
        text.truncate(end);
    }
}
