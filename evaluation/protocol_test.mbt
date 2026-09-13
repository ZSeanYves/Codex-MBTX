///|
test "suite contains twenty four distinct scenarios with lifecycle steps" {
  let suite = @evaluation.suite("/fixture path/mbtx-fixture")
  guard suite is { "tasks": Array(tasks), .. } else {
    fail("suite missing tasks")
  }
  assert_eq(tasks.length(), 24)
  let ids = tasks.map(t => {
    match t {
      { "id": String(id), .. } => id
      _ => ""
    }
  })
  for i in 0..<ids.length() {
    assert_true(ids[i] != "")
    for j in (i + 1)..<ids.length() {
      assert_true(ids[i] != ids[j])
    }
  }
}

///|
test "external failures and incomplete observation never become backend failures" {
  assert_true(
    @evaluation.assess_facts({ "infrastructure": "relay_error" })
    is { "status": "relay_error", .. },
  )
  assert_true(
    @evaluation.assess_facts({
      "infrastructure": "none",
      "turn_completed": true,
      "trajectory_match": false,
    })
    is {
      "status": "unknown",
      "failure_class": "model_trajectory_deviation",
      ..
    },
  )
}

///|
test "paired startup ratio and unavailable values" {
  let values = Array::make(20, ({ "shell": 100, "transparent": 80 } : Json))
  assert_true(
    @evaluation.paired_statistics(values)
    is { "ratio_ci_upper_at_most_0_90": true, .. },
  )
  assert_true(
    @evaluation.paired_statistics([{ "shell": Json::null(), "transparent": 80 }])
    is { "ratio": Null, .. },
  )
}

///|
test "oracle keeps external errors above a completed child and preserves unknowns" {
  let facts : Json = {
    "task": { "steps": [], "expected_exits": [], "check": "probe" },
    "infrastructure": "relay_error",
    "turn_completed": true,
    "processes_clean": true,
  }
  assert_true(
    @evaluation.evaluate(facts)
    is { "status": "relay_error", "strict_comparable": false, .. },
  )
  assert_true(
    @evaluation.assess_facts({
      "infrastructure": "none",
      "turn_completed": true,
      "trajectory_match": true,
      "observation_complete": true,
      "oracle_match": Json::null(),
      "processes_clean": true,
    })
    is { "status": "unknown", .. },
  )
}

///|
test "a completed turn with sandbox denial identifies the boundary without blaming a backend" {
  assert_true(
    @evaluation.evaluate({
      "task": {
        "steps": [],
        "expected_exits": [0],
        "check": "argv_empty_unicode",
      },
      "infrastructure": "none",
      "turn_completed": true,
      "tool_results": [{ "exit_code": 1, "sandbox_denied": true }],
      "receipts": [],
    })
    is {
      "status": "unknown",
      "failure_class": "codex_sandbox_denied",
      "strict_comparable": false,
      ..
    },
  )
  let step : Json = {
    "tool": "exec_command",
    "arguments": { "cmd": "./non-executable" },
  }
  assert_true(
    @evaluation.evaluate({
      "task": {
        "steps": [step],
        "expected_exits": [126],
        "check": "exec_errors",
      },
      "actual_steps": [step],
      "infrastructure": "none",
      "turn_completed": true,
      "tool_results": [{ "exit_code": 126, "sandbox_denied": true }],
      "processes_clean": true,
    })
    is { "status": "success", "strict_comparable": true, .. },
  )
}

///|
test "task arguments are compared without silently normalizing commands" {
  let facts : Json = {
    "task": {
      "steps": [{ "tool": "exec_command", "arguments": { "cmd": "literal" } }],
      "expected_exits": [0],
      "check": "exit",
    },
    "actual_steps": [
      { "tool": "exec_command", "arguments": { "cmd": "literal " } },
    ],
    "commands": [{ "exit_code": 0 }],
    "infrastructure": "none",
    "turn_completed": true,
    "processes_clean": true,
  }
  assert_true(
    @evaluation.evaluate(facts)
    is {
      "status": "unknown",
      "failure_class": "model_trajectory_deviation",
      ..
    },
  )
}

///|
fn tail_facts(isolated : Bool, tail_written : Bool) -> Json {
  let step : Json = {
    "tool": "exec_command",
    "arguments": { "cmd": "fixture pipe-tail" },
  }
  {
    "task": { "steps": [step], "expected_exits": [0], "check": "pipe-tail" },
    "actual_steps": [step],
    "tool_results": [{ "exit_code": 0 }],
    "infrastructure": "none",
    "turn_completed": true,
    "processes_clean": true,
    "collector_pid_namespace": "pid:[host]",
    "receipts": [
      { "pid_namespace": if isolated { "pid:[child]" } else { "pid:[host]" } },
    ],
    "commands": [{ "aggregated_output": "parent-done\n" }],
    "streams": [
      {
        "name": "stream-stdout.bin",
        "text": if tail_written {
          "parent-done\ndescendant-tail\n"
        } else {
          "parent-done\n"
        },
      },
    ],
  }
}

///|
test "namespace containment does not excuse loss of observed tail bytes" {
  assert_true(
    @evaluation.evaluate(tail_facts(true, false)) is { "status": "success", .. },
  )
  assert_true(
    @evaluation.evaluate(tail_facts(true, true))
    is { "status": "backend_failure", .. },
  )
  assert_true(
    @evaluation.evaluate(tail_facts(false, false))
    is { "status": "backend_failure", .. },
  )
}

///|
test "normalized sandbox exits retain the missing inner signal boundary" {
  let step : Json = {
    "tool": "exec_command",
    "arguments": { "cmd": "fixture self-term" },
  }
  let result = @evaluation.evaluate({
    "task": { "steps": [step], "expected_exits": [143], "check": "exit_signal" },
    "actual_steps": [step],
    "tool_results": [{ "exit_code": 143 }],
    "infrastructure": "none",
    "turn_completed": true,
    "processes_clean": true,
    "collector_pid_namespace": "pid:[host]",
    "receipts": [{ "pid_namespace": "pid:[child]" }],
    "trace": [
      {
        "phase": "child",
        "event": "wait_return",
        "exit_code": 143,
        "signal": Json::null(),
      },
    ],
  })
  assert_true(
    result
    is {
      "status": "success",
      "boundary_notes": [{ "confidence": "unknown", .. }],
      ..
    },
  )
}

///|
test "Codex zero counters cannot replace missing provider usage" {
  assert_true(
    @evaluation.evaluate({
      "codex_reported_usage": { "input_tokens": 0, "output_tokens": 0 },
      "trace": [
        {
          "phase": "external",
          "event": "response_completed",
          "usage": Json::null(),
        },
      ],
    })
    is { "usage": Null, "codex_reported_usage": { "input_tokens": 0, .. }, .. },
  )
  assert_true(
    @evaluation.evaluate({
      "relay_snapshot_complete": true,
      "trace": [
        { "phase": "external", "event": "request_send" },
        {
          "phase": "external",
          "event": "response_completed",
          "usage": { "input_tokens": 0, "output_tokens": 0 },
        },
      ],
    })
    is {
      "usage": {
        "source": "responses_proxy",
        "complete": true,
        "requests": [{ "usage": { "input_tokens": 0, .. }, .. }],
        ..
      },
      ..
    },
  )
}
