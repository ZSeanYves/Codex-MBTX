///|
struct Evidence {
  task : String
  backend : String
  events : String
  dumps : Array[String]
  correct : Bool
  inputs_ok : Bool
  timeout : Bool
  exit_code : Int
} derive(FromJson)

///|
async fn main {
  match @env.args() {
    [_, "suite"] => println(@eval.suite().to_json().stringify())
    [_, "prompt", task, backend] => {
      guard @eval.suite().filter(c => c.id == task).get(0) is Some(case) else {
        fail("unknown task")
      }
      println(@eval.prompt(case, backend))
    }
    [_, "audit"] => {
      let evidence : Evidence = @json.from_json(
        @json.parse(@stdio.stdin.read_all().text()),
      )
      guard @eval.suite().filter(c => c.id == evidence.task).get(0)
        is Some(case) else {
        fail("unknown task")
      }
      let audit = @eval.Audit::new()
      audit.jsonl(evidence.events)
      for dump in evidence.dumps {
        audit.dump(dump)
      }
      let result : Json = {
        "audit": audit.to_json(),
        "backend_compliant": audit
        .backend_ok(evidence.backend, case.background)
        .to_json(),
        "failure_category": @eval.failure_category(
          audit,
          evidence.backend,
          case.background,
          evidence.correct,
          evidence.inputs_ok,
          evidence.timeout,
          evidence.exit_code,
        ).to_json(),
      }
      println(result.stringify())
    }
    _ =>
      fail(
        "usage: m4-evidence suite | prompt TASK BACKEND | audit < EVIDENCE_JSON",
      )
  }
}
