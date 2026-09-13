///|
fn bool_field(value : Json, key : String) -> Bool {
  match value {
    Object(fields) => fields.get(key) is Some(True)
    _ => false
  }
}

///|
pub fn assess_facts(facts : Json) -> Json {
  let (status, reason) = match facts {
    { "infrastructure": String(kind), .. } if kind != "none" =>
      (kind, "external_or_harness_failure")
    _ if !bool_field(facts, "turn_completed") => ("unknown", "incomplete_turn")
    _ if !bool_field(facts, "trajectory_match") =>
      ("unknown", "model_trajectory_deviation")
    _ if !bool_field(facts, "observation_complete") =>
      ("unknown", "missing_observation")
    { "oracle_match": Null, .. } => ("unknown", "missing_oracle_evidence")
    { "processes_clean": Null, .. } => ("unknown", "missing_lifecycle_evidence")
    _ if !bool_field(facts, "oracle_match") =>
      ("backend_failure", "behavior_mismatch")
    _ if !bool_field(facts, "processes_clean") =>
      ("backend_failure", "lifecycle_residual")
    _ => ("success", "none")
  }
  {
    "status": status.to_json(),
    "failure_class": if reason == "none" {
      Json::null()
    } else {
      reason.to_json()
    },
    "strict_comparable": (status == "success").to_json(),
    "confidence": "observed",
  }
}

///|
pub fn dispatch(request : Json) -> Json {
  match request {
    { "operation": "suite", "fixture": String(fixture), .. } => suite(fixture)
    { "operation": "assess", "facts": facts, .. } => assess_facts(facts)
    { "operation": "evaluate", "facts": facts, .. } => evaluate(facts)
    { "operation": "statistics", "pairs": Array(pairs), .. } =>
      paired_statistics(pairs)
    _ => { "error": "unknown evaluation model operation" }
  }
}
