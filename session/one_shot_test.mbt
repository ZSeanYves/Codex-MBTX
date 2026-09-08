///|
async fn request_events(request : String) -> Array[Json] {
  let reader = @io.MemoryReader(writer => {
    @session.serve_request(request, writer)
  })
  defer reader.close()
  let result = []
  while reader.read_until("\n") is Some(line) {
    result.push(@json.parse(line))
  }
  result
}

///|
async test "one-shot accepts formatted JSON and drains its job" {
  let events = request_events(
    "{\n\"id\":\"once\",\n\"op\":\"run\",\n\"source\":\"fn main { println(42) }\"\n}",
  )
  assert_true(events[0] is { "event": "started", "id": "once", .. })
  assert_true(
    events.last().unwrap() is { "event": "completed", "exit_code": 0, .. },
  )
  assert_eq(
    events
    .iter()
    .filter_map(event => {
      match event {
        { "event": "stdout", "data": String(data), .. } => Some(data)
        _ => None
      }
    })
    .collect()
    .join(""),
    "42\n",
  )
}

///|
async test "one-shot rejects multiple requests and control-only launches" {
  for
    request in [
      "{\"id\":\"x\",\"op\":\"run\",\"source\":\"fn main {}\"}\n{\"id\":\"y\",\"op\":\"run\",\"source\":\"fn main {}\"}",
      "{\"id\":\"stop\",\"op\":\"job_stop\",\"job_id\":\"job-1\"}",
    ] {
    let events = request_events(request)
    assert_eq(events.length(), 1)
    assert_true(
      events[0]
      is { "event": "failed", "error": { "kind": "invalid_request", .. }, .. },
    )
  }
}
