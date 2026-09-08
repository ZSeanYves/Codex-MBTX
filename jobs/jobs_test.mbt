///|
async test "admission bounds, cancellation and oldest completed eviction" {
  let manager = @jobs.Manager::new(max_active=1, max_retained=2)
  let events : Array[@runtime.Event] = []
  @async.with_task_group() <| group => {
    let request = @runtime.Request::new(
      "repeat",
      Source("fn main { println(42) }"),
    )
    let first = manager.start(group, request, event => events.push(event))
    try manager.start(group, request, event => events.push(event)) catch {
      @jobs.Busy => ()
      error => raise error
    } noraise {
      _ => fail("active job limit was ignored")
    }
    assert_eq(first.stop(), @runtime.State::Stopping)
    assert_eq(first.stop(), @runtime.State::Stopping)
    first.wait()
    assert_eq(first.output().state, @runtime.State::Stopped)
    assert_eq(first.stop(), @runtime.State::Stopped)
    let second = manager.start(group, request, event => events.push(event))
    second.wait()
    assert_eq(second.output().exit_code, Some(0))
    let third = manager.start(group, request, event => events.push(event))
    assert_true(first.id != second.id && second.id != third.id)
    try manager.get(first.id) catch {
      @jobs.UnknownJob(id) => assert_eq(id, first.id)
      error => raise error
    } noraise {
      _ => fail("oldest completed job should be evicted")
    }
    assert_eq(manager.get(second.id).id, second.id)
    third.wait()
    assert_eq(events.iter().filter(event => event is Stopped(..)).count(), 1)
    assert_eq(events.iter().filter(event => event is Completed(..)).count(), 2)
  }
}

///|
async test "terminal failure and output snapshot are retained" {
  let manager = @jobs.Manager::new()
  @async.with_task_group() <| group => {
    let timed = manager.start(
      group,
      @runtime.Request::new("timeout", Source("fn main {}"), timeout_ms=1),
      _ => (),
    )
    timed.wait()
    let failed = timed.output()
    assert_eq(failed.state, @runtime.State::Failed)
    assert_eq(failed.error.unwrap().kind, "timeout")
    let job = manager.start(
      group,
      @runtime.Request::new("output", Source("fn main { println(42) }")),
      _ => (),
    )
    job.wait()
    let page = job.output()
    assert_eq(page.state, @runtime.State::Completed)
    assert_eq(page.chunks.map(c => c.data).join(""), "42\n")
    let next = job.output(cursor=page.next_cursor)
    assert_eq(next.chunks, [])
    page.chunks.clear()
    assert_false(job.output().chunks.is_empty())
    try job.output(cursor=next.next_cursor + 1) catch {
      @jobs.InvalidCursor => ()
      error => raise error
    } noraise {
      _ => fail("a future output cursor should be rejected")
    }
  }
}
