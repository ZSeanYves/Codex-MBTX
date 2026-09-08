///|
async test "concurrent streams keep whole JSON frames and per-job sequence" {
  @async.with_timeout(30000, () => {
    @async.with_task_group() <| group => {
      let (input, client) = @io.pipe()
      let (reader, output) = @io.pipe()
      defer input.close()
      defer reader.close()
      group.spawn_bg(() => {
        defer output.close()
        @session.serve(input, output)
      })
      let source =
        #|import { "moonbitlang/async", "moonbitlang/async/stdio", "moonbitlang/core/env" }
        #|async fn main {
        #|  let label = @env.args()[1]
        #|  @async.sleep(50)
        #|  for _ in 0..<16 {
        #|    @stdio.stdout.write(label + "-out\n")
        #|    @stdio.stderr.write(label + "-err\n")
        #|  }
        #|}
      group.spawn_bg(() => {
        defer client.close()
        for id in ["a", "b", "c"] {
          send(client, {
            "id": id.to_json(),
            "op": "run",
            "source": source.to_json(),
            "args": [id.to_json()],
            "background": true,
          })
        }
      })
      let next : Map[String, Int] = Map([])
      let stdout : Map[String, String] = Map([])
      let stderr : Map[String, String] = Map([])
      let completed : Map[String, Bool] = Map([])
      let jobs : Map[String, String] = Map([])
      while reader.read_until("\n") is Some(line) {
        let event = @json.parse(line)
        match event {
          { "event": "started", "id": String(id), "job_id": String(job_id), .. } => {
            jobs[id] = job_id
            next[id] = 0
          }
          {
            "event": String(kind),
            "id": String(id),
            "seq": Number(seq, ..),
            "data": String(data),
            ..
          } => {
            assert_false(completed.get(id).unwrap_or(false))
            assert_eq(seq.to_int(), next.get(id).unwrap())
            next[id] = seq.to_int() + 1
            let stream = if kind == "stdout" { stdout } else { stderr }
            stream[id] = stream.get(id).unwrap_or("") + data
          }
          { "event": "completed", "id": String(id), "exit_code": 0, .. } => {
            assert_false(completed.get(id).unwrap_or(false))
            completed[id] = true
          }
          _ => fail("unexpected lifecycle event: \{event.stringify()}")
        }
      }
      for id in ["a", "b", "c"] {
        assert_true(completed.get(id).unwrap())
        assert_eq(stdout.get(id), Some((id + "-out\n").repeat(16)))
        assert_eq(stderr.get(id), Some((id + "-err\n").repeat(16)))
      }
      assert_true(
        jobs.get("a") != jobs.get("b") && jobs.get("b") != jobs.get("c"),
      )
    }
  })
}

///|
async test "a full run queue does not block cancellation controls" {
  @async.with_timeout(30000, () => {
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
        #|async fn main { @stdio.stdout.write("ready"); @async.sleep(60000) }
      send(client, { "id": "blocking", "op": "run", "source": source.to_json() })
      let job_id = started_job(until_event(reader, seen, "blocking", "started"))
      until_event(reader, seen, "blocking", "stdout") |> ignore
      group.spawn_bg(() => {
        defer client.close()
        for index in 0..<20 {
          send(client, {
            "id": "queued-\{index}".to_json(),
            "op": "run",
            "source": "fn main {}",
          })
        }
        send(client, {
          "id": "stop",
          "op": "job_stop",
          "job_id": job_id.to_json(),
        })
      })
      while reader.read_until("\n") is Some(line) {
        seen.push(@json.parse(line))
      }
      assert_eq(
        seen
        .iter()
        .filter(e => {
          e is { "event": "failed", "error": { "kind": "busy", .. }, .. }
        })
        .count(),
        4,
      )
      assert_eq(
        seen.iter().filter(e => e is { "event": "completed", .. }).count(),
        16,
      )
      assert_eq(
        seen
        .iter()
        .filter(e => e is { "event": "stopped", "id": "blocking", .. })
        .count(),
        1,
      )
      assert_eq(
        seen
        .iter()
        .filter(e => e is { "event": "job_stop", "id": "stop", .. })
        .count(),
        1,
      )
    }
  })
}
