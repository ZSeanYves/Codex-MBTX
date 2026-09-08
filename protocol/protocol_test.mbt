///|
test "source request defaults and file request metadata" {
  let inline_json =
    #|{"id":"inline","op":"run","source":"fn main {}"}
  let inline = @protocol.decode_request(inline_json)
  assert_eq(inline.script, @runtime.Script::Source("fn main {}"))
  assert_eq(inline.args(), [])
  assert_eq(inline.cwd, ".")
  let file_json =
    #|{"id":"file","op":"run","script_path":"hello world.mbtx","args":["","$(echo no)","你好"],"cwd":"some directory"}
  let file = @protocol.decode_request(file_json)
  assert_eq(file.script, @runtime.Script::Path("hello world.mbtx"))
  assert_eq(file.args(), ["", "$(echo no)", "你好"])
  assert_eq(file.cwd, "some directory")
}

///|
test "invalid requests never silently fall back to a different mode" {
  let invalid = [
    "{",
    "[]",
    (
      #|{"op":"run","source":"fn main {}"}
    ),
    (
      #|{"id":"x","op":"stop","source":"fn main {}"}
    ),
    (
      #|{"id":"x","op":"run"}
    ),
    (
      #|{"id":"x","op":"run","source":"fn main {}","script_path":"x.mbtx"}
    ),
    (
      #|{"id":"x","op":"run","source":null}
    ),
    (
      #|{"id":"x","op":"run","source":"fn main {}","args":[1]}
    ),
    (
      #|{"id":"x","op":"run","source":"fn main {}","cwd":false}
    ),
    (
      #|{"id":"x","op":"run","source":"fn main {}","background":"true"}
    ),
  ]
  for line in invalid {
    try @protocol.decode_request(line) catch {
      @runtime.InvalidRequest(_) => ()
    } noraise {
      _ => fail("expected invalid request: \{line}")
    }
  }
}

///|
test "output is escaped as one JSONL record" {
  let line = @protocol.encode_event(
    @runtime.Event::Output(id="quoted\"id", job_id="job-1", chunk={
      seq: 0,
      stream: Stdout,
      data: "line 1\nline 2\u{0000}你好",
    }),
  )
  assert_false(line.contains("\n"))
  assert_eq(
    @json.parse(line),
    @json.parse(
      (
        #|{"event":"stdout","id":"quoted\"id","job_id":"job-1","seq":0,"data":"line 1\nline 2\u0000你好"}
      ),
    ),
  )
  assert_eq(
    @json.parse(
      @protocol.encode_event(
        Completed(id="done", job_id="job-2", exit_code=7, duration_ms=123L),
      ),
    ),
    @json.parse(
      (
        #|{"event":"completed","id":"done","job_id":"job-2","exit_code":7,"duration_ms":123}
      ),
    ),
  )
  assert_eq(
    @json.parse(
      @protocol.encode_event(
        Failed(id=None, job_id=None, kind="invalid_request", message="bad"),
      ),
    ),
    @json.parse(
      (
        #|{"event":"failed","id":null,"job_id":null,"error":{"kind":"invalid_request","message":"bad"}}
      ),
    ),
  )
}
