///|
/// Execute an already resolved argv while preserving the caller's streams.
/// The launcher never parses shell syntax and never compiles source code.
pub async fn run(command : String, args : Array[String], cwd? : String) -> Int {
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
          cancel_handler=@child.hard_cancel(),
        )
      None =>
        @child.spawn(
          group,
          command,
          args,
          stdin=@stdio.stdin,
          stdout=@stdio.stdout,
          stderr=@stdio.stderr,
          cancel_handler=@child.hard_cancel(),
        )
    }
    child.wait()
  }
}
