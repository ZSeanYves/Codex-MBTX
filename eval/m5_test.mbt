///|
test "M5 planner is reproducible and preserves paired blocks" {
  let first = @eval.m5_plan(
    "pilot",
    ["host_literal_argv", "host_background", "host_timeout"],
    3,
    17,
    0,
  )
  let second = @eval.m5_plan(
    "pilot",
    ["host_literal_argv", "host_background", "host_timeout"],
    3,
    17,
    0,
  )
  assert_eq(first, second)
  assert_eq(first.schema_version, 5)
  assert_eq(first.planned_blocks, 9)
  assert_eq(first.blocks.length(), 9)
  for block in first.blocks {
    assert_eq(block.order.length(), 2)
    assert_true(block.time_window >= 1 && block.time_window <= 3)
    assert_true(
      (block.order[0] == "shell" && block.order[1] == "transparent") ||
      (block.order[0] == "transparent" && block.order[1] == "shell"),
    )
    assert_eq(block.relay_health, Json::empty_object())
    assert_true(block.runs.is_empty())
  }
}

///|
test "M5 continuation starts at a block boundary" {
  let full = @eval.m5_plan("formal", ["a", "b"], 3, 99, 0)
  let continuation = @eval.m5_plan("formal", ["a", "b"], 3, 99, 2)
  assert_eq(continuation.blocks, full.blocks[2:].to_owned())
  assert_eq(continuation.start_block, 2)
  try @eval.m5_plan("formal", ["a"], 1, 1, 1) catch {
    _ => ()
  } noraise {
    _ => fail("out-of-range continuation was accepted")
  }
}

///|
test "M5 planner balances AB and BA per task" {
  let plan = @eval.m5_plan("formal", ["a", "b", "c"], 20, 123, 0)
  for task in ["a", "b", "c"] {
    let blocks = plan.blocks.filter(b => b.task == task)
    let ab = blocks.filter(b => b.order[0] == "shell").length()
    let ba = blocks.filter(b => b.order[0] == "transparent").length()
    assert_eq(ab, 10)
    assert_eq(ba, 10)
  }
  assert_eq(plan.blocks.filter(b => b.time_window == 1).length(), 21)
  assert_eq(plan.blocks.filter(b => b.time_window == 2).length(), 21)
  assert_eq(plan.blocks.filter(b => b.time_window == 3).length(), 18)
  assert_true(plan.blocks[:21].all(b => b.time_window == 1))
  assert_true(plan.blocks[21:42].all(b => b.time_window == 2))
  assert_true(plan.blocks[42:].all(b => b.time_window == 3))
}

///|
test "M5 planner seed changes task scheduling without changing the cohort" {
  let first = @eval.m5_plan("formal", ["a", "b", "c", "d"], 4, 1, 0)
  let second = @eval.m5_plan("formal", ["a", "b", "c", "d"], 4, 2, 0)
  assert_true(first.blocks.map(b => b.task) != second.blocks.map(b => b.task))
  assert_eq(first.planned_blocks, second.planned_blocks)
  assert_true(first.blocks.all(b => b.time_window >= 1 && b.time_window <= 3))
}

///|
test "M5 relay health applies conservative validity gates" {
  let healthy : @eval.M5RelayHealth = {
    probes: 10,
    successful_probes: 10,
    provider_errors: 0,
    transport_errors: 0,
    max_consecutive_failures: 0,
    first_byte_p95_ms: 1_200,
    arm_failure_rate_basis_points: 0,
    time_windows: 3,
  }
  assert_true(@eval.m5_relay_health_valid(healthy))
  assert_true(@eval.m5_relay_batch_valid(healthy))
  // The preflight probe gate allows one failed probe; the stricter 5% rule is
  // evaluated against the collected batch, not used to reject this probe set.
  assert_true(
    @eval.m5_relay_health_valid({
      ..healthy,
      successful_probes: 9,
      provider_errors: 1,
      max_consecutive_failures: 1,
      arm_failure_rate_basis_points: 2_000,
    }),
  )
  assert_false(
    @eval.m5_relay_batch_valid({
      ..healthy,
      successful_probes: 9,
      provider_errors: 1,
      max_consecutive_failures: 1,
      arm_failure_rate_basis_points: 2_000,
    }),
  )
  assert_false(
    @eval.m5_relay_health_valid({ ..healthy, successful_probes: 8, }),
  )
  assert_false(
    @eval.m5_relay_health_valid({ ..healthy, max_consecutive_failures: 2, }),
  )
  assert_false(
    @eval.m5_relay_batch_valid({ ..healthy, provider_errors: 1, probes: 10, }),
  )
  assert_true(@eval.m5_relay_health_valid({ ..healthy, time_windows: 1, }))
  assert_false(@eval.m5_relay_health_valid({ ..healthy, time_windows: 0, }))
}

///|
test "M5 failure categories preserve infrastructure and observability boundaries" {
  assert_eq(
    @eval.m5_failure_category(
      "provider_error", "complete", "compliant", true, true, true, true, true, true,
      true, true, true, true, false,
    ),
    "infrastructure_unavailable",
  )
  assert_eq(
    @eval.m5_failure_category(
      "clean", "unobserved", "unknown", false, false, false, true, true, true, true,
      true, true, true, false,
    ),
    "evidence_unobserved",
  )
  assert_eq(
    @eval.m5_failure_category(
      "clean", "complete", "violation", true, true, true, true, true, true, true,
      true, true, true, false,
    ),
    "backend_violation",
  )
  assert_eq(
    @eval.m5_failure_category(
      "clean", "complete", "unknown", true, true, true, true, true, true, true, true,
      true, true, false,
    ),
    "none",
  )
  assert_eq(
    @eval.m5_failure_category(
      "clean", "complete", "unknown", false, false, false, true, true, true, false,
      false, false, false, false,
    ),
    "backend_observation_unknown",
  )
}

///|
test "M5 paired summary reports robust quantiles and wins" {
  let summary = @eval.m5_pair_summary([10, 20, 30, 40], [12, 18, 30, 50])
  assert_eq(summary.count, 4)
  assert_eq(summary.shell_median_ms, 20)
  assert_eq(summary.transparent_median_ms, 18)
  assert_eq(summary.median_delta_ms, -2)
  assert_eq(summary.shell_wins, 2)
  assert_eq(summary.transparent_wins, 1)
  assert_eq(summary.ties, 1)
  assert_eq(@eval.m5_percentile([40, 10, 20, 30], 100), 40)
}

///|
test "M5 reliability summary counts unknown separately from violations" {
  let observations : Array[@eval.M5RunObservation] = [
    {
      run_id: "r1",
      block_id: "b1",
      task: "a",
      backend: "shell",
      fixture_nonce: "nonce-r1",
      started_ms: 0,
      monotonic_start_ms: None,
      monotonic_end_ms: None,
      ended_ms: None,
      first_response_ms: None,
      first_tool_ms: None,
      last_tool_ms: None,
      turn_completed_ms: None,
      process_launch_ms: None,
      process_exit_ms: None,
      stdout: "",
      stderr: "",
      exit_code: 0,
      workspace_manifest: [],
      attempts: [],
      tool_counts: Map([]),
      input_tokens: 0,
      cached_input_tokens: 0,
      output_tokens: 0,
      relay_status: "clean",
      observability: "complete",
      backend_observation: "unknown",
      completed: true,
      usage_complete: true,
      correct_output: true,
      inputs_preserved: true,
      workspace_clean: true,
      child_processes_clean: true,
      receipt_valid: true,
      trace_complete: true,
      argv_cwd_valid: true,
      approval_original_command: true,
      workspace_diff: Json::empty_object(),
      receipt: Json::empty_object(),
      process_trace: Json::array([]),
      failure_category: "none",
    },
    {
      run_id: "r2",
      block_id: "b2",
      task: "a",
      backend: "transparent",
      fixture_nonce: "nonce-r2",
      started_ms: 0,
      monotonic_start_ms: None,
      monotonic_end_ms: None,
      ended_ms: None,
      first_response_ms: None,
      first_tool_ms: None,
      last_tool_ms: None,
      turn_completed_ms: None,
      process_launch_ms: None,
      process_exit_ms: None,
      stdout: "",
      stderr: "",
      exit_code: -1,
      workspace_manifest: [],
      attempts: [],
      tool_counts: Map([]),
      input_tokens: 0,
      cached_input_tokens: 0,
      output_tokens: 0,
      relay_status: "provider_error",
      observability: "unobserved",
      backend_observation: "unknown",
      completed: false,
      usage_complete: false,
      correct_output: false,
      inputs_preserved: true,
      workspace_clean: true,
      child_processes_clean: true,
      receipt_valid: false,
      trace_complete: false,
      argv_cwd_valid: false,
      approval_original_command: true,
      workspace_diff: Json::empty_object(),
      receipt: Json::empty_object(),
      process_trace: Json::array([]),
      failure_category: "infrastructure_unavailable",
    },
    {
      run_id: "r3",
      block_id: "b3",
      task: "a",
      backend: "shell",
      fixture_nonce: "nonce-r3",
      started_ms: 0,
      monotonic_start_ms: None,
      monotonic_end_ms: None,
      ended_ms: None,
      first_response_ms: None,
      first_tool_ms: None,
      last_tool_ms: None,
      turn_completed_ms: None,
      process_launch_ms: None,
      process_exit_ms: None,
      stdout: "",
      stderr: "",
      exit_code: 0,
      workspace_manifest: [],
      attempts: [],
      tool_counts: Map([]),
      input_tokens: 0,
      cached_input_tokens: 0,
      output_tokens: 0,
      relay_status: "clean",
      observability: "complete",
      backend_observation: "violation",
      completed: true,
      usage_complete: true,
      correct_output: true,
      inputs_preserved: true,
      workspace_clean: true,
      child_processes_clean: true,
      receipt_valid: true,
      trace_complete: true,
      argv_cwd_valid: true,
      approval_original_command: false,
      workspace_diff: Json::empty_object(),
      receipt: Json::empty_object(),
      process_trace: Json::array([]),
      failure_category: "backend_violation",
    },
  ]
  let summary = @eval.m5_reliability_summary(observations)
  assert_eq(summary.planned, 3)
  assert_eq(summary.relay_clean, 2)
  assert_eq(summary.successes, 1)
  assert_eq(summary.attributable_successes, 0)
  assert_eq(summary.provider_failures, 1)
  assert_eq(summary.unknown_backend_observations, 2)
  assert_eq(summary.backend_violations, 1)
}

///|
test "M5 attempt audit keeps retry and transport failures distinct" {
  let summary = @eval.m5_attempt_summary([
    {
      attempt_id: "a1",
      retry_index: 0,
      http_status: Some(502),
      started_ms: 0,
      first_byte_ms: None,
      completed_ms: Some(10),
      stream_disconnected: false,
      provider_error: true,
      recovered: false,
      input_tokens: 0,
      cached_input_tokens: 0,
      output_tokens: 0,
      usage_complete: false,
      error_class: Some("502"),
    },
    {
      attempt_id: "a2",
      retry_index: 1,
      http_status: Some(200),
      started_ms: 20,
      first_byte_ms: Some(22),
      completed_ms: Some(40),
      stream_disconnected: true,
      provider_error: false,
      recovered: true,
      input_tokens: 10,
      cached_input_tokens: 3,
      output_tokens: 4,
      usage_complete: true,
      error_class: None,
    },
    {
      attempt_id: "a3",
      retry_index: 2,
      http_status: Some(200),
      started_ms: 50,
      first_byte_ms: Some(52),
      completed_ms: None,
      stream_disconnected: false,
      provider_error: false,
      recovered: false,
      input_tokens: 10,
      cached_input_tokens: 0,
      output_tokens: 0,
      usage_complete: false,
      error_class: Some("malformed_sse"),
    },
  ])
  assert_eq(summary.attempts, 3)
  assert_eq(summary.retry_count, 2)
  assert_eq(summary.provider_errors, 1)
  assert_eq(summary.transport_errors, 1)
  assert_eq(summary.incomplete_attempts, 1)
  assert_eq(summary.recovered_attempts, 1)
  assert_eq(summary.missing_usage, 2)
  assert_eq(summary.input_tokens, 20)
  assert_eq(summary.retry_input_tokens, 20)
  assert_eq(summary.retry_cached_input_tokens, 3)
  assert_eq(summary.retry_output_tokens, 4)
  assert_eq(summary.retry_elapsed_ms, 20)
}

///|
test "M5 decision treats a key task regression as NO_GO" {
  let decision = @eval.m5_decide({
    semantic_zero_regression: true,
    safety_compatibility_ok: true,
    success_lower_bound_basis_points: 0,
    p95_upper_bound_basis_points: 0,
    key_task_regression: true,
    evidence_complete: true,
    backend_observable: true,
    artifacts_rebuildable: true,
  })
  assert_eq(decision.status, "NO_GO")
}
