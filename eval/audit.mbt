///|
pub struct Audit {
  mut completed : Bool
  mut input_tokens : Int
  mut cached_input_tokens : Int
  mut output_tokens : Int
  mut reasoning_output_tokens : Int
  mut usage_responses : Int
  mut responses : Int
  mut http_errors : Int
  mut provider_errors : Int
  mut parse_errors : Int
  mut approval_requests : Int
  mut background_runs : Int
  mut tool_argument_errors : Int
  mut command_failures : Int
  mbtx_failures : Map[String, Int]
  mbtx_diagnostics : Array[String]
  tools : Map[String, Int]
  operations : Map[String, Int]
  models : Array[String]
  response_ids : Array[String]
  call_ids : Array[String]
  output_ids : Array[String]
} derive(ToJson)

///|
pub fn Audit::new() -> Audit {
  {
    completed: false,
    input_tokens: 0,
    cached_input_tokens: 0,
    output_tokens: 0,
    reasoning_output_tokens: 0,
    usage_responses: 0,
    responses: 0,
    http_errors: 0,
    provider_errors: 0,
    parse_errors: 0,
    approval_requests: 0,
    background_runs: 0,
    tool_argument_errors: 0,
    command_failures: 0,
    mbtx_failures: {},
    mbtx_diagnostics: [],
    tools: {},
    operations: {},
    models: [],
    response_ids: [],
    call_ids: [],
    output_ids: [],
  }
}

///|
fn integer(value : Json, key : String) -> Int {
  match value {
    Object(fields) =>
      match fields.get(key) {
        Some(Number(n, ..)) => n.to_int()
        _ => 0
      }
    _ => 0
  }
}

///|
fn Audit::item(self : Audit, item : Json) -> Unit {
  if item
    is {
      "type": "function_call"
      | "custom_tool_call",
      "name": String(name),
      "call_id": String(id),
      ..
    } {
    if self.call_ids.contains(id) {
      return
    }
    self.call_ids.push(id)
    self.tools[name] = self.tools.get(name).unwrap_or(0) + 1
    if name == "request_user_input" {
      self.approval_requests += 1
    }
    if name == "mbtx" {
      if item is { "arguments": String(args), .. } {
        try {
          let args = @json.parse(args)
          if args is { "op": String(op), .. } {
            self.operations[op] = self.operations.get(op).unwrap_or(0) + 1
          }
          if args is { "op": "run", "background": true, .. } {
            self.background_runs += 1
          }
        } catch {
          _ => self.tool_argument_errors += 1
        }
      }
    }
  }
}

///|
pub fn Audit::event(self : Audit, event : Json) -> Unit {
  if event is { "type": "response.failed" | "error", .. } {
    self.provider_errors += 1
  }
  if event is { "type": "response.output_item.done", "item": item, .. } {
    self.item(item)
  }
  if event is { "type": "response.completed", "response": response, .. } {
    if response is { "id": String(id), .. } {
      if self.response_ids.contains(id) {
        return
      }
      self.response_ids.push(id)
    }
    self.responses += 1
    if response is { "model": String(model), .. } {
      if !self.models.contains(model) {
        self.models.push(model)
      }
    }
    if response is { "output": Array(items), .. } {
      for item in items {
        self.item(item)
      }
    }
    if response
      is {
        "usage": { "input_tokens": Number(_), "output_tokens": Number(_), .. } as usage,
        ..
      } {
      self.usage_responses += 1
      self.input_tokens += integer(usage, "input_tokens")
      self.output_tokens += integer(usage, "output_tokens")
      if usage is { "input_tokens_details": details, .. } {
        self.cached_input_tokens += integer(details, "cached_tokens")
      }
      if usage is { "output_tokens_details": details, .. } {
        self.reasoning_output_tokens += integer(details, "reasoning_tokens")
      }
    }
  }
}

///|
pub fn Audit::sse(self : Audit, text : String) -> Unit {
  // SSE permits multiple data fields per event and either LF or CRLF framing.
  let data : Array[String] = []
  fn flush() {
    if data.is_empty() {
      return
    }
    let payload = data.join("\n")
    data.clear()
    if payload == "[DONE]" {
      return
    }
    self.event(@json.parse(payload)) catch {
      _ => self.parse_errors += 1
    }
  }
  for line in text.split("\n") {
    let line = line.trim_end(chars="\r")
    if line.is_empty() {
      flush()
    } else if line.has_prefix("data:") {
      let value = line[5:]
      data.push(
        (if value.has_prefix(" ") { value[1:] } else { value }).to_owned(),
      )
    }
  }
  flush()
}

///|
pub fn Audit::dump(self : Audit, text : String) -> Unit {
  try {
    let parsed = @json.parse(text)
    if parsed is { "status": Number(status, ..), "body": body, .. } {
      if status < 200 || status >= 300 {
        self.http_errors += 1
      }
      match body {
        String(s) => self.sse(s)
        _ => self.event(body)
      }
    } else {
      self.parse_errors += 1
    }
  } catch {
    _ => self.parse_errors += 1
  }
}

///|
pub fn Audit::jsonl(self : Audit, text : String) -> Unit {
  for line in text.split("\n") {
    if line.trim().is_empty() {
      continue
    }
    try {
      let event = @json.parse(line)
      if event is { "type": "turn.completed", .. } {
        self.completed = true
      }
      if event is { "item": { "status": "declined", .. }, .. } {
        self.approval_requests += 1
      }
      if event
        is {
          "type": "item.completed",
          "item": {
            "type": "command_execution",
            "exit_code": Number(code, ..),
            ..
          },
          ..
        } {
        if code != 0 {
          self.command_failures += 1
        }
      }
    } catch {
      _ => self.parse_errors += 1
    }
  }
}

///|
pub fn Audit::request(self : Audit, text : String) -> Unit {
  try {
    if @json.parse(text) is { "body": { "input": Array(items), .. }, .. } {
      for item in items {
        if item
          is {
            "type": "function_call_output",
            "call_id": String(id),
            "output": String(output),
            ..
          } {
          if self.output_ids.contains(id) {
            continue
          }
          self.output_ids.push(id)
          // Shell output is plain text; only MBTX's structured terminal is counted here.
          let terminal = @json.parse(output) catch { _ => continue }
          let kind = match terminal {
            { "state": "failed", "error": { "kind": String(kind), .. }, .. } =>
              kind
            { "state": "completed", "exit_code": Number(code, ..), .. } if code !=
              0 => "script_exit"
            _ => continue
          }
          self.mbtx_failures[kind] = self.mbtx_failures.get(kind).unwrap_or(0) +
            1
          if self.mbtx_diagnostics.length() < 4 {
            let detail = match terminal {
              { "stderr": String(stderr), .. } if !stderr.is_empty() => stderr
              { "error": { "message": String(message), .. }, .. } => message
              _ => ""
            }
            let detail = String::from_iter(detail.iter().take(2048))
            if !detail.is_empty() && !self.mbtx_diagnostics.contains(detail) {
              self.mbtx_diagnostics.push(detail)
            }
          }
        }
      }
    }
  } catch {
    _ => self.parse_errors += 1
  }
}

///|
pub fn Audit::backend_ok(
  self : Audit,
  backend : String,
  background : Bool,
) -> Bool {
  if backend == "mbtx" {
    if self.tools.get("mbtx").unwrap_or(0) == 0 {
      return false
    }
    for
      name in [
        "exec_command", "write_stdin", "shell", "shell_command", "js", "js_exec",
        "exec", "spawn_agent",
      ] {
      if self.tools.get(name).unwrap_or(0) > 0 {
        return false
      }
    }
    !background ||
    (
      self.background_runs > 0 &&
      self.operations.get("job_output").unwrap_or(0) > 0
    )
  } else {
    if backend != "shell" {
      return false
    }
    for
      name in [
        "mbtx", "spawn_agent", "js", "js_exec", "exec", "shell", "shell_command",
      ] {
      if self.tools.get(name).unwrap_or(0) > 0 {
        return false
      }
    }
    self.tools.get("exec_command").unwrap_or(0) > 0 &&
    (!background || self.tools.get("write_stdin").unwrap_or(0) > 0)
  }
}

///|
pub fn failure_category(
  audit : Audit,
  backend : String,
  background : Bool,
  correct : Bool,
  inputs_ok : Bool,
  timeout : Bool,
  exit_code : Int,
) -> String {
  if timeout {
    "timeout"
  } else if audit.parse_errors > 0 {
    "audit_error"
  } else if !audit.completed || exit_code != 0 {
    if audit.http_errors > 0 || audit.provider_errors > 0 {
      "provider_error"
    } else {
      "agent_error"
    }
  } else if audit.responses == 0 || audit.usage_responses != audit.responses {
    "missing_usage"
  } else if audit.approval_requests > 0 {
    "intervention_required"
  } else if !audit.backend_ok(backend, background) {
    "backend_violation"
  } else if !inputs_ok {
    "input_modified"
  } else if !correct {
    "incorrect_output"
  } else {
    "none"
  }
}
