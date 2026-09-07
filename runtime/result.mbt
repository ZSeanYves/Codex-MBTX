///|
/// Completed `moon run` output. Nonzero exit codes are results, not runner errors.
pub(all) struct RunResult {
  exit_code : Int
  stdout : String
  stderr : String
  duration_ms : Int64
} derive(Debug, Eq)

///|
/// M1 buffers stdout and stderr; output events are emitted after the child exits.
pub(all) enum Event {
  Started(id~ : String)
  Stdout(id~ : String, data~ : String)
  Stderr(id~ : String, data~ : String)
  Completed(id~ : String, exit_code~ : Int, duration_ms~ : Int64)
  Failed(id~ : String?, kind~ : String, message~ : String)
} derive(Debug, Eq)
