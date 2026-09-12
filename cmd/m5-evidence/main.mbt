///|
fn tasks_for(mode : String) -> Array[@eval.M5Task] raise {
  if mode == "process" {
    @eval.m5_process_suite()
  } else if mode == "script" {
    @eval.m5_script_suite()
  } else if mode == "all" {
    @eval.m5_process_suite() + @eval.m5_script_suite()
  } else {
    fail("unknown M5 suite: " + mode)
  }
}

///|
fn integer_arg(text : String, name : String) -> Int raise {
  guard @json.parse(text) is Number(number, ..) &&
    number >= 0 &&
    number <= 2147483647.0 &&
    number == number.to_int().to_double() else {
    fail(name + " must be a nonnegative integer")
  }
  number.to_int()
}

///|
fn repetitions_for(mode : String, supplied : String?) -> Int raise {
  match supplied {
    Some(value) => integer_arg(value, "repetitions")
    None => if mode == "pilot" { 5 } else if mode == "formal" { 20 } else { 10 }
  }
}

///|
struct M5AuditInput {
  events : String
  dumps : Array[String]
  requests : Array[String]?
  transport_error : Bool
  stream_complete : Bool
  backend_observed : Bool
  backend_compliant : Bool
} derive(FromJson)

///|
async fn main {
  let args = @env.args()
  match args {
    [_, "suite"] => println(tasks_for("all").to_json().stringify())
    [_, "suite", mode] => println(tasks_for(mode).to_json().stringify())
    [_, "prompt", task_id] => {
      let mut task : @eval.M5Task? = None
      for candidate in tasks_for("all") {
        if candidate.id == task_id {
          task = Some(candidate)
        }
      }
      guard task is Some(task) else { fail("unknown M5 task: " + task_id) }
      println(@eval.m5_prompt(task))
    }
    [_, "fixture", task_id] => {
      let mut task : @eval.M5Task? = None
      for candidate in tasks_for("all") {
        if candidate.id == task_id {
          task = Some(candidate)
        }
      }
      guard task is Some(task) else { fail("unknown M5 task: " + task_id) }
      println(task.to_json().stringify())
    }
    [_, "plan", mode, seed_text, start_text] => {
      let seed = integer_arg(seed_text, "seed")
      let start = integer_arg(start_text, "start_block")
      let repetitions = repetitions_for(mode, None)
      let tasks = tasks_for(if mode == "script" { "script" } else { "process" }).map(t => {
          t.id
        },
      )
      println(
        @eval.m5_plan(mode, tasks, repetitions, seed, start)
        .to_json()
        .stringify(),
      )
    }
    [_, "plan", mode, seed_text, start_text, repetitions_text] => {
      let seed = integer_arg(seed_text, "seed")
      let start = integer_arg(start_text, "start_block")
      let repetitions = repetitions_for(mode, Some(repetitions_text))
      let tasks = tasks_for(if mode == "script" { "script" } else { "process" }).map(t => {
          t.id
        },
      )
      println(
        @eval.m5_plan(mode, tasks, repetitions, seed, start)
        .to_json()
        .stringify(),
      )
    }
    [_, "health"] => {
      let health : @eval.M5RelayHealth = @json.from_json(
        @json.parse(@stdio.stdin.read_all().text()),
      )
      let output : Json = {
        "health_valid": @eval.m5_relay_health_valid(health),
        "batch_valid": @eval.m5_relay_batch_valid(health),
        "health_usable": @eval.m5_relay_health_usable(health),
        "batch_usable": @eval.m5_relay_batch_usable(health),
      }
      println(output.stringify())
    }
    [_, "classify"] => {
      let input : @eval.M5ClassificationInput = @json.from_json(
        @json.parse(@stdio.stdin.read_all().text()),
      )
      let output : Json = { "failure_category": @eval.m5_classify(input) }
      println(output.stringify())
    }
    [_, "pair"] => {
      let input : @eval.M5PairInput = @json.from_json(
        @json.parse(@stdio.stdin.read_all().text()),
      )
      println(
        @eval.m5_pair_summary(input.shell_ms, input.transparent_ms)
        .to_json()
        .stringify(),
      )
    }
    [_, "reliability"] => {
      let observations : Array[@eval.M5RunObservation] = @json.from_json(
        @json.parse(@stdio.stdin.read_all().text()),
      )
      println(@eval.m5_reliability_summary(observations).to_json().stringify())
    }
    [_, "attempts"] => {
      let attempts : Array[@eval.M5Attempt] = @json.from_json(
        @json.parse(@stdio.stdin.read_all().text()),
      )
      println(@eval.m5_attempt_summary(attempts).to_json().stringify())
    }
    [_, "audit"] => {
      let input : M5AuditInput = @json.from_json(
        @json.parse(@stdio.stdin.read_all().text()),
      )
      let audit = @eval.Audit::new()
      audit.jsonl(input.events)
      for dump in input.dumps {
        audit.dump(dump)
      }
      for request in input.requests.unwrap_or([]) {
        audit.request(request)
      }
      let output : Json = {
        "relay_status": audit.m5_relay_status(input.transport_error),
        "observability": audit.m5_observability(input.stream_complete),
        "backend_observation": audit.m5_backend_observation(
          input.backend_observed,
          input.backend_compliant,
        ),
        "audit": audit.to_json(),
      }
      println(output.stringify())
    }
    [_, "decision"] => {
      let input : @eval.M5DecisionInput = @json.from_json(
        @json.parse(@stdio.stdin.read_all().text()),
      )
      println(@eval.m5_decide(input).to_json().stringify())
    }
    [_, "legacy"] => {
      let report = @json.parse(@stdio.stdin.read_all().text())
      guard report
        is { "schema_version": Number(version, ..), "results": Array(_), .. } &&
        version >= 1.0 &&
        version < 5.0 else {
        fail("legacy report is not a supported M4 schema (1-4)")
      }
      let output : Json = {
        "schema_version": match report {
          { "schema_version": Number(version, ..), .. } => version.to_int()
          _ => 0
        },
        "parseable": true,
      }
      println(output.stringify())
    }
    _ =>
      fail(
        "usage: m5-evidence suite [process|script|all] | prompt TASK | fixture TASK | plan MODE SEED START [REPETITIONS] | health < JSON | classify < JSON | pair < JSON | reliability < JSON | attempts < JSON | audit < JSON | decision < JSON | legacy < M4_JSON",
      )
  }
}
