///|
pub struct Rollout {
  index : Int
  task : String
  backend : String
  repetition : Int
} derive(ToJson)

///|
pub struct Plan {
  planned_rollouts : Int
  start_rollout : Int
  rollouts : Array[Rollout]
} derive(ToJson)

///|
/// Continue at an absolute zero-based rollout index without changing pair order.
pub fn plan(mode : String, start_rollout : Int) -> Plan raise {
  guard mode == "smoke" || mode == "full" || mode == "backend" else {
    fail("invalid evaluation mode")
  }
  let cases = if mode == "backend" {
    suite().filter(c => c.workload == "process")
  } else {
    suite()
  }
  let count = if mode == "smoke" { 1 } else { cases.length() }
  let repetitions = if mode == "smoke" { 1 } else { 2 }
  let planned_rollouts = count * 2 * repetitions
  guard start_rollout >= 0 && start_rollout < planned_rollouts else {
    fail("start_rollout is outside the evaluation plan")
  }
  guard mode != "smoke" || start_rollout == 0 else {
    fail("smoke must run both backends")
  }
  let rollouts : Array[Rollout] = []
  let mut index = 0
  for repetition in 1..<=repetitions {
    for i in 0..<count {
      let backends = if (i + repetition) % 2 == 1 {
        ["shell", "transparent"]
      } else {
        ["transparent", "shell"]
      }
      for backend in backends {
        if index >= start_rollout {
          rollouts.push({ index, task: cases[i].id, backend, repetition, })
        }
        index += 1
      }
    }
  }
  { planned_rollouts, start_rollout, rollouts, }
}
