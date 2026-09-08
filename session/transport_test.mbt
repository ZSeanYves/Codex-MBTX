///|
async test "a broken output transport cancels and reaps running scripts" {
  let directory = @fs.tmpdir(prefix="mbtx-disconnect-")
  defer @fs.rmdir(directory, recursive=true)
  let marker = "\{directory}/escaped"
  let source =
    #|import {
    #|  "moonbitlang/async", "moonbitlang/async/stdio",
    #|  "moonbitlang/async/fs", "moonbitlang/core/env",
    #|}
    #|async fn main {
    #|  @stdio.stdout.write("ready")
    #|  @async.sleep(100)
    #|  @stdio.stdout.write("after-disconnect")
    #|  @async.sleep(500)
    #|  @fs.write_file(@env.args()[1], "escaped")
    #|}
  @async.with_timeout(20000, () => {
    @async.with_task_group() <| group => {
      let (input, client) = @io.pipe()
      let (reader, output) = @io.pipe()
      defer input.close()
      defer client.close()
      defer reader.close()
      let server = group.spawn(allow_failure=true, () => {
        defer output.close()
        @session.serve(input, output)
      })
      let seen : Array[Json] = []
      send(client, {
        "id": "disconnect",
        "op": "run",
        "source": source.to_json(),
        "args": [marker.to_json()],
      })
      until_event(reader, seen, "disconnect", "stdout") |> ignore
      reader.close()
      try server.wait() catch {
        @io.PipeClosed => ()
        error => raise error
      } noraise {
        _ => fail("expected the transport failure to propagate")
      }
    }
  })
  @async.sleep(700)
  assert_false(@fs.exists(marker))
}

///|
async test "oversized and invalid UTF-8 records recover at the next line" {
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
        let block = @utf8.encode("x".repeat(4096))
        for _ in 0..<257 {
          client.write(block)
        }
        client.write(b"\n\xff\n")
        let request : Json = {
          "id": "after-large",
          "op": "run",
          "source": "fn main {}",
        }
        client.write(request.stringify())
      })
      let seen : Array[Json] = []
      while reader.read_until("\n") is Some(line) {
        seen.push(@json.parse(line))
      }
      assert_eq(seen.length(), 4)
      assert_true(
        seen[0]
        is { "event": "failed", "error": { "kind": "invalid_request", .. }, .. },
      )
      assert_true(
        seen[1]
        is { "event": "failed", "error": { "kind": "invalid_request", .. }, .. },
      )
      assert_true(
        seen[3]
        is { "event": "completed", "id": "after-large", "exit_code": 0, .. },
      )
    }
  })
}
