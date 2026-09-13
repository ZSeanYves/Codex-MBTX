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
    "pipe-tail" =>
      merged.contains("parent-done") && merged.contains("descendant-tail")
    "session" =>
      stdout.contains("session-ready") && stdout.contains("session-done")
    "exit_signal" => {
      let waits = array_field(facts, "trace").filter(r => {
        field(r, "phase") == "child" && field(r, "event") == "wait_return"
      })
      waits.any(r => field(r, "exit_code") == 143 && field(r, "signal") is Null) &&
      waits.any(r => field(r, "signal") == 15)
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
  }
  assessment
}
