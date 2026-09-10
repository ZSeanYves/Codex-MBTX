///|
test "M5 suite separates process and script cohorts" {
  let process = @eval.m5_process_suite()
  let scripts = @eval.m5_script_suite()
  assert_eq(process.length(), 8)
  assert_eq(scripts.length(), 6)
  assert_true(process.all(t => t.cohort == "process"))
  assert_true(scripts.all(t => t.cohort == "script-capability"))
  assert_true(process.all(t => t.oracle.contains("process_trace")))
  assert_true(!process[0].prompt.contains("MOONBIT.md"))
  assert_true(scripts[0].prompt.contains("MoonBit"))
}

///|
test "M5 task metadata has unique ids and receipts" {
  let tasks = @eval.m5_process_suite() + @eval.m5_script_suite()
  let ids : Array[String] = []
  for task in tasks {
    assert_false(ids.contains(task.id))
    ids.push(task.id)
    assert_true(task.fixture_nonce.has_prefix("m5-"))
    assert_eq(task.receipt_path, ".m5/receipt.json")
    assert_true(task.expected_manifest.length() >= task.inputs.length() + 1)
    assert_true(task.expected_manifest.contains(".m5/receipt.json"))
  }
}

///|
test "M5 fixtures expose stream, exit and exact manifest contracts" {
  let tasks = @eval.m5_process_suite()
  let literal = tasks.filter(t => t.id == "host_literal_argv")[0]
  assert_true(@eval.m5_streams_valid(literal, "", "", 0))
  assert_true(
    @eval.m5_manifest_valid(literal, [
      ".m5/receipt.json", "capture_args.py", "result.json",
    ]),
  )
  assert_false(
    @eval.m5_manifest_valid(literal, [
      ".m5/receipt.json", "capture_args.py", "result.json", "extra",
    ]),
  )
  let cancelled = tasks.filter(t => t.id == "host_cancel_timeout")[0]
  assert_eq(cancelled.expected_exit_code, -1)
  assert_true(@eval.m5_streams_valid(cancelled, "", "", -1))
  let background = tasks.filter(t => t.id == "host_background")[0]
  assert_true(@eval.m5_streams_valid(background, "started\ncomplete\n", "", 0))
  let recovery = tasks.filter(t => t.id == "host_exit_recovery")[0]
  assert_true(
    @eval.m5_streams_valid(recovery, "", "confirmation required\n", 0),
  )
}

///|
test "M5 oracle supports host-dependent cwd subsets without weakening exact tasks" {
  let tasks = @eval.m5_process_suite()
  let cwd = tasks.filter(t => t.id == "host_cwd_environment")[0]
  assert_true(
    @eval.m5_correct_output(
      cwd, "{\"cwd\":\"/isolated/workspace\",\"nonce\":\"m5-host_cwd_environment-v1\"}",
    ),
  )
  assert_false(@eval.m5_correct_output(cwd, "{\"nonce\":\"wrong\"}"))
  let literal = tasks.filter(t => t.id == "host_literal_argv")[0]
  assert_false(@eval.m5_correct_output(literal, "[]"))
}

///|
test "M5 receipt binds helper, task and fixture nonce" {
  let task = @eval.m5_process_suite()[0]
  assert_true(
    @eval.m5_receipt_valid(task, {
      "task": task.id,
      "nonce": task.fixture_nonce,
      "helper": "capture_args.py",
      "status": "completed",
    }),
  )
  assert_true(
    @eval.m5_receipt_valid(task, {
      "task": task.id,
      "nonce": task.fixture_nonce,
      "helper": "sleeper.py",
      "status": "stopped",
    }),
  )
  assert_false(
    @eval.m5_receipt_valid(task, {
      "task": task.id,
      "nonce": "wrong",
      "helper": "x",
      "status": "completed",
    }),
  )
}

///|
test "M5 cancellation oracle requires absence of a late result and cleanup" {
  let task = @eval.m5_process_suite().filter(t => t.id == "host_cancel_timeout")[0]
  assert_true(@eval.m5_cancellation_valid(task, false, true))
  assert_false(@eval.m5_cancellation_valid(task, true, true))
  assert_false(@eval.m5_cancellation_valid(task, false, false))
}
