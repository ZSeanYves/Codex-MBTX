///|
pub struct Job {
  id : String
  priv request_id : String
  priv control : @process.Control
  priv mut state : @runtime.State
  priv chunks : Array[@runtime.Chunk]
  priv mut exit_code : Int?
  priv mut duration_ms : Int64?
  priv mut error : @runtime.JobFailure?
  priv mut settled : Bool
  priv changed : @cond_var.Cond
  priv output_lock : @async.Mutex
}

///|
/// Return at most 128 output chunks beginning at cursor, plus current status.
/// Use next_cursor for the next page; snapshots never expose mutable history.
pub fn Job::output(
  self : Job,
  cursor? : Int = 0,
) -> @runtime.Snapshot raise JobError {
  if cursor < 0 || cursor > self.chunks.length() {
    raise InvalidCursor
  }
  let next_cursor = Int::min(self.chunks.length(), cursor + 128)
  {
    state: self.state,
    chunks: self.chunks[cursor:next_cursor].to_owned(),
    next_cursor,
    exit_code: self.exit_code,
    duration_ms: self.duration_ms,
    error: self.error,
  }
}

///|
/// Idempotently request cancellation. A stopping acknowledgement is not proof
/// of termination; wait for the stopped event or `wait` before reclaiming work.
pub fn Job::stop(self : Job) -> @runtime.State {
  if self.state == Running {
    self.state = Stopping
    self.control.stop()
  }
  self.state
}

///|
/// Wait until cleanup finishes and the terminal event has been enqueued.
pub async fn Job::wait(self : Job) -> Unit {
  while !self.settled {
    self.changed.wait()
  }
}

///|
async fn Job::execute(
  self : Job,
  request : @runtime.Request,
  emit : async (@runtime.Event) -> Unit,
) -> Unit {
  let started = @async.now()
  let terminal : @runtime.Event = try
    @process.stream(request, self.control, (stream, data) => {
      self.output_lock.acquire()
      defer self.output_lock.release()
      let chunk : @runtime.Chunk = { seq: self.chunks.length(), stream, data, }
      self.chunks.push(chunk)
      emit(Output(id=self.request_id, job_id=self.id, chunk~))
    })
  catch {
    @process.Stopped => {
      self.state = Stopped
      Stopped(
        id=self.request_id,
        job_id=self.id,
        duration_ms=@async.now() - started,
      )
    }
    @process.TimedOut =>
      self.fail("timeout", "execution exceeded \{request.timeout_ms} ms")
    @process.OutputLimitExceeded =>
      self.fail("output_limit", "output exceeded 1 MiB or 4096 chunks")
    @process.ExecutionFailed(message) => self.fail("runner_error", message)
    error => raise error
  } noraise {
    (code, duration) => {
      self.state = Completed
      self.exit_code = Some(code)
      Completed(
        id=self.request_id,
        job_id=self.id,
        exit_code=code,
        duration_ms=duration,
      )
    }
  }
  self.duration_ms = Some(@async.now() - started)
  emit(terminal)
  self.settled = true
  self.changed.broadcast()
}

///|
fn Job::fail(self : Job, kind : String, message : String) -> @runtime.Event {
  self.state = Failed
  self.error = Some({ kind, message, })
  Failed(id=Some(self.request_id), job_id=Some(self.id), kind~, message~)
}
