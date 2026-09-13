///|
async fn main {
  while @stdio.stdin.read_until("\n") is Some(line) {
    let request = @json.parse(line)
    if request is { "operation": "quit", .. } {
      break
    }
    @stdio.stdout.write(@evaluation.dispatch(request).stringify() + "\n")
  }
}
