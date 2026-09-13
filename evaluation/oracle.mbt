///|
fn array_field(value : Json, key : String) -> Array[Json] {
  match value {
    Object(fields) =>
      match fields.get(key) {
        Some(Array(items)) => items
        _ => []
      }
    _ => []
  }
}

///|
fn field(value : Json, key : String) -> Json {
  match value {
    Object(fields) => fields.get(key).unwrap_or(Json::null())
    _ => Json::null()
  }
}

///|
fn text_field(value : Json, key : String) -> String {
  match field(value, key) {
    String(text) => text
    _ => ""
  }
}

///|
fn trajectory_matches(expected : Array[Json], actual : Array[Json]) -> Bool {
  if expected.length() != actual.length() {
    return false
  }
  for index in 0..<expected.length() {
    let want = expected[index]
    let got = actual[index]
    if field(want, "tool") != field(got, "tool") {
      return false
    }
    let arguments = field(want, "arguments")
    let received = field(got, "arguments")
    match (arguments, received) {
      (Object(left), Object(right)) => {
        if left.length() != right.length() {
          return false
        }
        for key, value in left {
          if key == "session_id" && value == "$session" {
            guard right.get(key) is Some(Number(_)) else { return false }
            if field(got, "expected_session_id") != right[key] {
              return false
            }
          } else if right.get(key) != Some(value) {
            return false
          }
        }
      }
      _ => return false
    }
  }
  true
}

///|
fn receipt_for(facts : Json, mode : String) -> Json {
  array_field(facts, "receipts")
  .iter()
  .find_first(r => field(r, "mode") == mode.to_json())
  .unwrap_or(Json::null())
}

///|
fn output_stream(facts : Json, stream : String) -> String {
  array_field(facts, "streams")
  .filter(r => text_field(r, "name").has_suffix("-" + stream + ".bin"))
  .map(r => text_field(r, "text"))
  .join("")
}

///|
fn isolated_pid_namespace(facts : Json) -> Bool {
  guard field(facts, "collector_pid_namespace") is String(collector) else {
    return false
  }
  array_field(facts, "receipts").any(receipt => {
    match field(receipt, "pid_namespace") {
      String(child) => child != collector
      _ => false
    }
  })
}

///|
fn behavior_matches(facts : Json, check : String) -> Json {
  let receipt = receipt_for(facts, "argv")
  let stdin = receipt_for(facts, "stdin")
  let environment = receipt_for(facts, "environment")
  let stdout = output_stream(facts, "stdout")
  let stderr = output_stream(facts, "stderr")
  let commands = array_field(facts, "commands")
  let merged = commands.map(c => text_field(c, "aggregated_output")).join("")
  let required_receipt = match check {
    "argv_empty_unicode" | "argv_metacharacters" | "argv_long" => "argv"
    "stdin_eof" | "stdin_pty_utf8" | "stdin_backpressure" => "stdin"
    "cwd_symlink" | "environment_empty_unset" | "environment" => "environment"
    _ => ""
  }
  if required_receipt != "" && receipt_for(facts, required_receipt) is Null {
    return Json::null()
  }
  if check.has_prefix("output-") && array_field(facts, "streams").is_empty() {
    return Json::null()
  }
  let result = match check {
    "probe" => true
    "argv_empty_unicode" =>
      field(receipt, "arguments") == ["", "a b", "\u4e2d\u6587"].to_json()
    "argv_metacharacters" =>
      field(receipt, "arguments") ==
      ["'\"\\", "$() ${HOME}; * ? | & < >", "line1\nline2"].to_json()
    "argv_long" =>
      field(receipt, "arguments") ==
      (["abcd".repeat(1024)] + Array::makei(128, i => i.to_string())).to_json()
    "stdin_eof" => field(stdin, "input_bytes") == 0
    "stdin_pty_utf8" =>
      field(stdin, "input_utf8") ==
      "\u7b2c\u4e00\u884c\n\u6700\u540e\u4e00\u884c\n" &&
      field(stdin, "stdin_tty") == true
    "stdin_backpressure" =>
      field(stdin, "input_bytes") == 262144 &&
      field(stdin, "input_utf8") == "i".repeat(262144).to_json()
    "cwd_symlink" =>
      text_field(environment, "cwd") ==
      text_field(facts, "workspace") + "/space \u8def\u5f84/target"
    "environment_empty_unset" =>
      field(field(environment, "environment"), "MBTX_EMPTY") == "" &&
      field(field(environment, "environment"), "MBTX_ABSENT") is Null
    "environment" =>
      field(field(environment, "environment"), "HOME") == field(facts, "home") &&
      field(field(environment, "environment"), "MBTX_EVAL_MARKER") ==
      "fixed-environment" &&
      text_field(field(environment, "environment"), "PATH") != ""
    "output-large" => stdout == "x".repeat(65536) && stderr == "e".repeat(4096)
    "output-tail" =>
      stdout == "x".repeat(262144) + "TAIL-END\n" &&
      stderr == "e".repeat(4096) &&
      merged.contains("TAIL-END")
    "output-interleaved" => {
      let lines = Array::makei(32, i => {
        let stream = if i % 2 == 0 { "stdout" } else { "stderr" }
        stream + ":" + (if i < 10 { "0" } else { "" }) + i.to_string() + "\n"
      })
      stdout == lines.filter(line => line.has_prefix("stdout:")).join("") &&
      stderr == lines.filter(line => line.has_prefix("stderr:")).join("")
    }
    "pipe-tail" => {
      let tail_written = stdout.contains("descendant-tail") ||
        array_field(facts, "trace").any(r => {
          field(r, "event") == "write" &&
          field(r, "content_tag") == "descendant_tail"
        })
      // PID-namespace teardown may kill the descendant before its delayed write.
      // Any tail observed at the writer or reader must still reach Codex output.
      merged.contains("parent-done") &&
      (if isolated_pid_namespace(facts) && !tail_written {
        true
      } else {
        merged.contains("descendant-tail")
      })
    }
    "session" =>
      stdout.contains("session-ready") && stdout.contains("session-done")
    "exit_signal" => {
      let waits = array_field(facts, "trace").filter(r => {
        field(r, "phase") == "child" && field(r, "event") == "wait_return"
      })
      waits.any(r => field(r, "exit_code") == 143 && field(r, "signal") is Null) &&
      (
        isolated_pid_namespace(facts) ||
        waits.any(r => field(r, "signal") == 15)
      )
    }
    "sequence" =>
      array_field(facts, "receipts")
      .filter(r => field(r, "mode") == "small")
      .length() ==
      10
    "recovery" =>
      array_field(facts, "receipts")
      .filter(r => field(r, "mode") == "small")
      .length() ==
      1
    "resources" =>
      array_field(facts, "receipts")
      .filter(r => field(r, "mode") == "small")
      .length() ==
      6
    "exit"
    | "exec_errors"
    | "descendants"
    | "interrupt"
    | "term"
    | "ignore-term" => true
    _ => return Json::null()
  }
  result.to_json()
}

///|
pub fn evaluate(facts : Json) -> Json {
  let task = field(facts, "task")
  let expected = array_field(task, "steps")
  let actual = array_field(facts, "actual_steps")
  let exits = array_field(facts, "tool_results")
    .map(c => field(c, "exit_code"))
    .filter(v => !(v is Null))
  let check = text_field(task, "check")
  let behavior = behavior_matches(facts, check)
  let observation_complete = check == "probe" ||
    (
      actual.length() > 0 &&
      !exits.is_empty() &&
      exits.all(v => !(v is Null)) &&
      !(behavior is Null)
    )
  let assessment = assess_facts({
    "infrastructure": field(facts, "infrastructure"),
    "sandbox_denied": array_field(facts, "tool_results")
    .any(r => bool_field(r, "sandbox_denied"))
    .to_json(),
    "turn_completed": field(facts, "turn_completed"),
    "trajectory_match": trajectory_matches(expected, actual).to_json(),
    "observation_complete": observation_complete.to_json(),
    "oracle_match": if behavior is Null {
      Json::null()
    } else {
      (behavior == true && exits.to_json() == field(task, "expected_exits")).to_json()
    },
    "processes_clean": field(facts, "processes_clean"),
  })
  if assessment is Object(fields) {
    fields["actual_exits"] = exits.to_json()
    fields["expected_exits"] = field(task, "expected_exits")
    fields["trajectory_match"] = trajectory_matches(expected, actual).to_json()
    fields["behavior_match"] = behavior
    // Codex can print zero counters when a provider omitted usage. Keep those
    // separately from usage actually observed on the Responses stream.
    let request_usage = array_field(facts, "trace")
      .filter(r => {
        field(r, "phase") == "external" &&
        field(r, "event") == "response_completed"
      })
      .map(r => {
        (
          {
            "request_id": field(r, "request_id"),
            "response_id": field(r, "response_id"),
            "usage": field(r, "usage"),
          } : Json)
      })
    fields["codex_reported_usage"] = field(facts, "codex_reported_usage")
    fields["usage"] = if request_usage.is_empty() ||
      request_usage.all(r => field(r, "usage") is Null) {
      Json::null()
    } else {
      {
        "source": "responses_proxy",
        "requests": request_usage.to_json(),
        "complete": (bool_field(facts, "relay_snapshot_complete") &&
        request_usage.all(r => !(field(r, "usage") is Null)) &&
        request_usage.length() ==
        array_field(facts, "trace")
        .filter(r => {
          field(r, "phase") == "external" && field(r, "event") == "request_send"
        })
        .length()).to_json(),
      }
    }
    fields["boundary_notes"] = if isolated_pid_namespace(facts) &&
      check == "exit_signal" {
      [
        {
          "confidence": "unknown",
          "layer": "sandbox",
          "message": "The sandbox outer wait normalizes signal termination to an exit code. This Codex sample does not prove inner Shell signal identity; native launcher contracts check real wait status separately.",
        },
      ].to_json()
    } else if isolated_pid_namespace(facts) && check == "pipe-tail" {
      [
        {
          "confidence": "inferred",
          "layer": "sandbox",
          "message": "A separate PID namespace is observed. Sandbox teardown may terminate the descendant before its tail write; emitted tail bytes must still be present in Codex output.",
        },
      ].to_json()
    } else {
      ([] : Array[Json]).to_json()
    }
  }
  assessment
}
