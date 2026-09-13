use serde_json::Value;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

/// One prebuilt MoonBit model process per collection/report, never per arm.
pub struct Model {
    child: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
}

impl Model {
    pub fn start(path: &Path) -> io::Result<Self> {
        let mut child = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(Self {
            input: child.stdin.take().unwrap(),
            output: BufReader::new(child.stdout.take().unwrap()),
            child,
        })
    }

    pub fn call(&mut self, request: Value) -> io::Result<Value> {
        serde_json::to_writer(&mut self.input, &request)?;
        self.input.write_all(b"\n")?;
        self.input.flush()?;
        let mut line = String::new();
        if self.output.read_line(&mut line)? == 0 {
            return Err(io::Error::other("evaluation model exited"));
        }
        let value: Value = serde_json::from_str(&line)?;
        if value.get("error").is_some() {
            return Err(io::Error::other(value.to_string()));
        }
        Ok(value)
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        let _ = self.input.write_all(b"{\"operation\":\"quit\"}\n");
        let _ = self.input.flush();
        let _ = self.child.wait();
    }
}
