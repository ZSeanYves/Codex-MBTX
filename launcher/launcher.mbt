///|
/// Execute an already resolved argv while preserving the caller's streams.
/// The launcher never parses shell syntax and never compiles source code.
pub async fn run(command : String, args : Array[String], cwd? : String) -> Int {
  trace_event(0, 0, 0)
  prepare_cancellation()
  defer finish_cancellation()
  trace_event(1, 0, 0)
  @async.with_task_group() <| group => {
    let child = match cwd {
      Some(directory) =>
        @child.spawn(
          group,
          command,
          args,
          stdin=@stdio.stdin,
          stdout=@stdio.stdout,
          stderr=@stdio.stderr,
          cwd=directory,
          cancel_handler=@child.graceful_cancel(timeout=500),
        )
      None =>
        @child.spawn(
          group,
          command,
          args,
          stdin=@stdio.stdin,
          stdout=@stdio.stdout,
          stderr=@stdio.stderr,
          cancel_handler=@child.graceful_cancel(timeout=500),
        )
    }
    trace_event(2, 0, child.pid)
    group.spawn_bg(no_wait=true, () => monitor_cancellation(child))
    let code = child.wait()
    trace_event(3, code, child.pid)
    code
  }
}

///|
#cfg(target="native")
extern "C" fn trace_event(event : Int, code : Int, pid : Int) = "mbtx_launcher_trace"

///|
#cfg(target="wasm")
fn trace_event(_event : Int, _code : Int, _pid : Int) -> Unit {
  ()
}
