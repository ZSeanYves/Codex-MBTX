///|
/// Run a host command while keeping the caller's standard streams attached.
///
/// This is the transparent execution path used by Codex. The command has
/// already been resolved to literal argv by the host adapter, so MBTX does not
/// parse shell syntax and does not compile a generated MoonBit script.
pub async fn proxy(
  command : String,
  args : Array[String],
  cwd? : String,
  cancel_handler? : @child.CancellationHandler = @child.hard_cancel(),
) -> Int {
  @async.with_task_group() <| group => {
    let child = match cwd {
      Some(cwd) =>
        @child.spawn(
          group,
          command,
          args,
          stdin=@stdio.stdin,
          stdout=@stdio.stdout,
          stderr=@stdio.stderr,
          cwd~,
          cancel_handler~,
        )
      None =>
        @child.spawn(
          group,
          command,
          args,
          stdin=@stdio.stdin,
          stdout=@stdio.stdout,
          stderr=@stdio.stderr,
          cancel_handler~,
        )
    }
    child.wait()
  }
}
