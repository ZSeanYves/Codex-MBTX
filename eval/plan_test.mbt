///|
test "continuations preserve the original task and backend order" {
  let full = @eval.plan("full", 0)
  assert_eq(full.planned_rollouts, 48)
  assert_eq(full.rollouts.length(), 48)
  for start in [1, 12, 24, 42, 47] {
    let continuation = @eval.plan("full", start)
    assert_eq(continuation.planned_rollouts, full.planned_rollouts)
    assert_eq(continuation.start_rollout, start)
    assert_eq(
      continuation.rollouts.to_json(),
      full.rollouts[start:].to_owned().to_json(),
    )
  }
  assert_eq(
    @eval.plan("full", 42).rollouts.map(r => {
      "\{r.task}/\{r.backend}/\{r.repetition}"
    }),
    [
      "host_background/shell/2", "host_background/transparent/2", "host_large_output/transparent/2",
      "host_large_output/shell/2", "host_exit_recovery/shell/2", "host_exit_recovery/transparent/2",
    ],
  )
  let smoke = @eval.plan("smoke", 0)
  assert_eq(smoke.rollouts.length(), 2)
  assert_eq(smoke.rollouts.to_json(), full.rollouts[:2].to_owned().to_json())

  let backend = @eval.plan("backend", 0)
  assert_eq(backend.planned_rollouts, 24)
  assert_eq(backend.rollouts.length(), 24)
  assert_true(backend.rollouts.all(r => r.task.has_prefix("host_")))
}

///|
test "invalid continuation cannot silently yield an empty or unpaired run" {
  for
    pair in [
      ("full", -1),
      ("full", 48),
      ("backend", 24),
      ("smoke", 1),
      ("other", 0),
    ] {
    let (mode, start) = pair
    try @eval.plan(mode, start) catch {
      _ => ()
    } noraise {
      _ => fail("invalid plan was accepted")
    }
  }
}
