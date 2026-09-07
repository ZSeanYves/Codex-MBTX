///|
async fn emit(event : @runtime.Event) -> Unit {
  @stdio.stdout.write(@protocol.encode_event(event) + "\n")
}

///|
async fn handle(line : String) -> Unit {
  let request = @protocol.decode_request(line) catch {
    @runtime.InvalidRequest(message) => {
      emit(Failed(id=None, kind="invalid_request", message~))
      return
    }
  }
  let id = request.id
  emit(Started(id~))
  let result = @process.run(request) catch {
    @process.TimedOut => {
      emit(
        Failed(
          id=Some(id),
          kind="timeout",
          message="execution exceeded 30000 ms",
        ),
      )
      return
    }
    @process.ExecutionFailed(message) => {
      emit(Failed(id=Some(id), kind="runner_error", message~))
      return
    }
    error => raise error
  }
  if !result.stdout.is_empty() {
    emit(Stdout(id~, data=result.stdout))
  }
  if !result.stderr.is_empty() {
    emit(Stderr(id~, data=result.stderr))
  }
  emit(
    Completed(id~, exit_code=result.exit_code, duration_ms=result.duration_ms),
  )
}

///|
async fn main {
  while @stdio.stdin.read_until("\n") is Some(line) {
    if !line.trim().is_empty() {
      handle(line)
    }
  }
}
