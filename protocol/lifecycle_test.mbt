///|
test "lifecycle requests validate modes, integer limits and control fields" {
  let json =
    #|{"id":"x","op":"run","source":"fn main {}","background":true,"timeout_ms":17}
  let request = @protocol.decode_request(json)
  assert_true(request.background)
  assert_eq(request.timeout_ms, 17)
  let stop =
    #|{"id":"stop-1","op":"job_stop","job_id":"job-1"}
  assert_true(
    @protocol.decode_command(stop) is JobStop(id="stop-1", job_id="job-1"),
  )
  let query =
    #|{"id":"query","op":"job_output","job_id":"job-1","cursor":3}
  assert_true(
    @protocol.decode_command(query)
    is JobOutput(id="query", job_id="job-1", cursor=3),
  )
  for timeout in ["0", "-1", "1.5", "600001", "1e100", "null", "true", "\"30\""] {
    let line = "{\"id\":\"x\",\"op\":\"run\",\"source\":\"fn main {}\",\"timeout_ms\":" +
      timeout +
      "}"
    try @protocol.decode_command(line) catch {
      @runtime.InvalidRequest(_) => ()
    } noraise {
      _ => fail("invalid timeout was accepted: \{timeout}")
    }
  }
  for extra in ["\"cursor\":-1", "\"cursor\":1.5", "\"source\":\"no\""] {
    let line = "{\"id\":\"q\",\"op\":\"job_output\",\"job_id\":\"job-1\"," +
      extra +
      "}"
    try @protocol.decode_command(line) catch {
      @runtime.InvalidRequest(_) => ()
    } noraise {
      _ => fail("invalid control request was accepted")
    }
  }
}

///|
test "known IDs and optional results are scalars rather than option arrays" {
  let failed = @json.parse(
    @protocol.encode_event(
      Failed(
        id=Some("stop"),
        job_id=Some("job-1"),
        kind="unknown_job",
        message="missing",
      ),
    ),
  )
  assert_true(failed is { "id": "stop", "job_id": "job-1", .. })
  let snapshot = @json.parse(
    @protocol.encode_event(
      JobOutput(id="query", job_id="job-1", snapshot={
        state: Completed,
        chunks: [],
        next_cursor: 0,
        exit_code: Some(7),
        duration_ms: Some(123L),
        error: None,
      }),
    ),
  )
  assert_true(
    snapshot is { "exit_code": 7, "duration_ms": 123, "error": Null, .. },
  )
}
