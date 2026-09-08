///|
/// Completed script output. Nonzero exit codes are results, not runner errors.
pub(all) struct RunResult {
  exit_code : Int
  stdout : String
  stderr : String
  duration_ms : Int64
} derive(Debug, Eq)

///|
pub(all) enum Stream {
  Stdout
  Stderr
} derive(Debug, Eq)

///|
pub(all) struct Chunk {
  seq : Int
  stream : Stream
  data : String
} derive(Debug, Eq)

///|
pub(all) enum State {
  Running
  Stopping
  Completed
  Failed
  Stopped
} derive(Debug, Eq)

///|
pub fn State::is_terminal(self : State) -> Bool {
  match self {
    Completed | Failed | Stopped => true
    Running | Stopping => false
  }
}

///|
pub(all) struct JobFailure {
  kind : String
  message : String
} derive(Debug, Eq)

///|
pub(all) struct Snapshot {
  state : State
  chunks : Array[Chunk]
  next_cursor : Int
  exit_code : Int?
  duration_ms : Int64?
  error : JobFailure?
} derive(Debug, Eq)

///|
/// Job events retain the originating run ID; control replies use their own ID.
pub(all) enum Event {
  Started(id~ : String, job_id~ : String)
  Output(id~ : String, job_id~ : String, chunk~ : Chunk)
  Completed(
    id~ : String,
    job_id~ : String,
    exit_code~ : Int,
    duration_ms~ : Int64
  )
  Stopped(id~ : String, job_id~ : String, duration_ms~ : Int64)
  Failed(id~ : String?, job_id~ : String?, kind~ : String, message~ : String)
  JobOutput(id~ : String, job_id~ : String, snapshot~ : Snapshot)
  JobStop(id~ : String, job_id~ : String, state~ : State)
} derive(Debug, Eq)
