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
    is { "at_least_ten_percent_faster": true, .. },
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
