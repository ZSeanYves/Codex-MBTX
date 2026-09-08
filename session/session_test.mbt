///|
async fn send(writer : &@io.Writer, value : Json) -> Unit {
  writer.write(value.stringify() + "\n")
}

///|
async fn read_event(reader : &@io.Reader, seen : Array[Json]) -> Json {
  let line = reader.read_until("\n").unwrap()
  let event = @json.parse(line)
  seen.push(event)
  event
}

///|
async fn until_event(
  reader : &@io.Reader,
  seen : Array[Json],
  id : String,
  kind : String,
) -> Json {
  for ;; {
    let event = read_event(reader, seen)
    if event is { "id": String(event_id), "event": String(event_kind), .. } &&
      event_id == id &&
      event_kind == kind {
      return event
    }
  }
}

///|
fn started_job(event : Json) -> String raise {
  guard event is { "job_id": String(job_id), .. } else {
    fail("missing job ID")
  }
  job_id
}

///|
async test "background, live output, foreground controls, cursor and stop lifecycle" {
  @async.with_timeout(20000, () => {
    @async.with_task_group() <| group => {
      let (input, client) = @io.pipe()
      let (reader, output) = @io.pipe()
      defer input.close()
      defer client.close()
      defer reader.close()
      group.spawn_bg(() => {
        defer output.close()
        @session.serve(input, output)
      })
      let seen : Array[Json] = []
      let source =
        #|import { "moonbitlang/async", "moonbitlang/async/stdio" }
        #|async fn main {
        #|  @stdio.stdout.write("ready-without-newline")
        #|  @async.sleep(60000)
        #|}
      send(client, {
        "id": "background",
        "op": "run",
        "source": source.to_json(),
        "background": true,
      })
      let job_id = started_job(
        until_event(reader, seen, "background", "started"),
      )
      let live = until_event(reader, seen, "background", "stdout")
      assert_true(live is { "data": "ready-without-newline", "seq": 0, .. })
      send(client, {
        "id": "query",
        "op": "job_output",
        "job_id": job_id.to_json(),
      })
      let snapshot = until_event(reader, seen, "query", "job_output")
      assert_true(
        snapshot
        is {
          "state": "running",
          "next_cursor": 1,
          "chunks": [{ "data": "ready-without-newline", .. }],
          ..
        },
      )
      send(client, {
        "id": "quick",
        "op": "run",
        "source": "fn main { println(7) }",
      })
      let quick = until_event(reader, seen, "quick", "completed")
      assert_true(quick is { "exit_code": 0, .. })
      send(client, {
        "id": "stop-bg",
        "op": "job_stop",
        "job_id": job_id.to_json(),
      })
      until_event(reader, seen, "stop-bg", "job_stop") |> ignore
      until_event(reader, seen, "background", "stopped") |> ignore
      send(client, {
        "id": "again",
        "op": "job_stop",
        "job_id": job_id.to_json(),
      })
      assert_true(
        until_event(reader, seen, "again", "job_stop")
        is { "state": "stopped", .. },
      )
      send(client, {
        "id": "tail",
        "op": "job_output",
        "job_id": job_id.to_json(),
        "cursor": 1,
      })
      assert_true(
        until_event(reader, seen, "tail", "job_output")
        is { "chunks": [], "state": "stopped", .. },
      )
      send(client, {
        "id": "foreground",
        "op": "run",
        "source": source.to_json(),
      })
      let foreground = started_job(
        until_event(reader, seen, "foreground", "started"),
      )
      until_event(reader, seen, "foreground", "stdout") |> ignore
      send(client, {
        "id": "stop-fg",
        "op": "job_stop",
        "job_id": foreground.to_json(),
      })
      until_event(reader, seen, "foreground", "stopped") |> ignore
      client.close()
      while reader.read_until("\n") is Some(line) {
        seen.push(@json.parse(line))
      }
      for id in ["background", "foreground", "quick"] {
        let terminal = seen
          .iter()
          .filter(event => {
            event is { "id": String(event_id), "event": String(kind), .. } &&
            event_id == id &&
            (kind == "completed" || kind == "stopped" || kind == "failed")
          })
          .count()
        assert_eq(terminal, 1)
      }
    }
  })
}

///|
async test "bad controls return correlated errors and EOF drains queued runs" {
  @async.with_timeout(20000, () => {
    @async.with_task_group() <| group => {
      let (input, client) = @io.pipe()
      let (reader, output) = @io.pipe()
      defer input.close()
      defer reader.close()
      group.spawn_bg(() => {
        defer output.close()
        @session.serve(input, output)
      })
      group.spawn_bg(() => {
        defer client.close()
        client.write("{bad}\r\n\n")
        send(client, {
          "id": "missing",
          "op": "job_stop",
          "job_id": "not-a-job",
        })
        send(client, {
          "id": "one",
          "op": "run",
          "source": "fn main { println(1) }",
        })
        client.write(
          "{\"id\":\"two\",\"op\":\"run\",\"source\":\"fn main { println(2) }\"}",
        )
      })
      let seen : Array[Json] = []
      while reader.read_until("\n") is Some(line) {
        seen.push(@json.parse(line))
      }
      assert_true(seen[0] is { "event": "failed", "id": Null, .. })
      assert_true(
        seen[1]
        is {
          "event": "failed",
          "id": "missing",
          "error": { "kind": "unknown_job", .. },
          ..
        },
      )
      let lifecycle = seen.filter(event => !(event is { "event": "stdout", .. }))
      assert_true(lifecycle[2] is { "event": "started", "id": "one", .. })
      assert_true(lifecycle[3] is { "event": "completed", "id": "one", .. })
      assert_true(lifecycle[4] is { "event": "started", "id": "two", .. })
      assert_true(lifecycle[5] is { "event": "completed", "id": "two", .. })
    }
  })
}
