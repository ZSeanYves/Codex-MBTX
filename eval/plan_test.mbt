///|
test "continuations preserve the original task and backend order" {
  let full = @eval.plan("full", 0)
  assert_eq(full.planned_rollouts, 24)
  assert_eq(full.rollouts.length(), 24)
  for start in [1, 12, 18, 23] {
    let continuation = @eval.plan("full", start)
    assert_eq(continuation.planned_rollouts, full.planned_rollouts)
    assert_eq(continuation.start_rollout, start)
    assert_eq(
      continuation.rollouts.to_json(),
      full.rollouts[start:].to_owned().to_json(),
    )
  }
  assert_eq(
    @eval.plan("full", 18).rollouts.map(r => {
      "\{r.task}/\{r.backend}/\{r.repetition}"
    }),
    [
      "literal_argv/shell/2", "literal_argv/transparent/2", "repair_moon/transparent/2",
      "repair_moon/shell/2", "background_job/shell/2", "background_job/transparent/2",
    ],
  )
  let smoke = @eval.plan("smoke", 0)
  assert_eq(smoke.rollouts.length(), 2)
  assert_eq(smoke.rollouts.to_json(), full.rollouts[:2].to_owned().to_json())
}

///|
test "invalid continuation cannot silently yield an empty or unpaired run" {
  for pair in [("full", -1), ("full", 24), ("smoke", 1), ("other", 0)] {
    let (mode, start) = pair
    try @eval.plan(mode, start) catch {
      _ => ()
    } noraise {
      _ => fail("invalid plan was accepted")
    }
  }
}
