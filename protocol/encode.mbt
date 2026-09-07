///|
/// Encode an event as one JSON object, without a trailing newline.
/// Strings are JSON escaped so script output cannot create extra protocol lines.
pub fn encode_event(event : @runtime.Event) -> String {
  let json = match event {
    Started(id~) =>
      Json::object({ "event": "started".to_json(), "id": id.to_json() })
    Stdout(id~, data~) =>
      Json::object({
        "event": "stdout".to_json(),
        "id": id.to_json(),
        "data": data.to_json(),
      })
    Stderr(id~, data~) =>
      Json::object({
        "event": "stderr".to_json(),
        "id": id.to_json(),
        "data": data.to_json(),
      })
    Completed(id~, exit_code~, duration_ms~) =>
      Json::object({
        "event": "completed".to_json(),
        "id": id.to_json(),
        "exit_code": exit_code.to_json(),
        "duration_ms": Json::number(
          duration_ms.to_double(),
          repr=duration_ms.to_string(),
        ),
      })
    Failed(id~, kind~, message~) =>
      Json::object({
        "event": "failed".to_json(),
        "id": id.to_json(),
        "error": Json::object({
          "kind": kind.to_json(),
          "message": message.to_json(),
        }),
      })
  }
  json.stringify()
}
