///|
async test "output paging preserves sequence without exposing history" {
  let manager = Manager::new()
  @async.with_task_group() <| group => {
    let job = manager.start(
      group,
      @runtime.Request::new("paging", Source("fn main {}")),
      _ => (),
    )
    job.wait()
    for seq in 0..<260 {
      job.chunks.push({ seq, stream: Stdout, data: "chunk-\{seq}", })
    }
    assert_eq(job.output().next_cursor, 128)
    assert_eq(job.output(cursor=128).next_cursor, 256)
    let last = job.output(cursor=256)
    assert_eq(last.next_cursor, 260)
    assert_eq(last.chunks.map(c => c.seq), [256, 257, 258, 259])
    assert_eq(job.output(cursor=260).chunks, [])
  }
}
