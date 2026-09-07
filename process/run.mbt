///|
pub(all) suberror RunnerError {
  TimedOut
  ExecutionFailed(String)
} derive(Debug)

///|
/// Run a validated request with the installed MoonBit toolchain on Wasm.
/// Source travels over stdin; the executable and arguments stay separate.
/// The direct child has a 30-second deadline and a 1 MiB aggregate capture cap.
/// These limits do not provide a sandbox for the script or its descendants.
///
/// ```mbt check
/// async test {
///   let request = @runtime.Request::new(
///     "example",
///     Source("fn main { println(42) }"),
///   )
///   let result = @process.run(request)
///   assert_eq(result.exit_code, 0)
///   assert_eq(result.stdout, "42\n")
/// }
/// ```
pub async fn run(request : @runtime.Request) -> @runtime.RunResult {
  let arguments = ["run", "--quiet", "--target", "wasm", "--"]
  let stdin = match request.script {
    Source(source) => {
      arguments.push("-")
      Some(@shell.Stdin::Text(source))
    }
    Path(path) => {
      arguments.push(path)
      None
    }
  }
  for argument in request.args() {
    arguments.push(argument)
  }
  let started = @async.now()
  let output = @shell.Cmd("moon", arguments, cwd=request.cwd, stdin?).output(
    timeout_ms=30_000,
    max_output_bytes=1024 * 1024,
  ) catch {
    @async.TimeoutError => raise TimedOut
    error => {
      if @async.is_cancellation_error(error) {
        raise error
      }
      raise ExecutionFailed(error.to_string())
    }
  }
  let elapsed = @async.now() - started
  {
    exit_code: output.exit_code(),
    stdout: output.stdout(),
    stderr: output.stderr(),
    duration_ms: if elapsed < 0L {
      0L
    } else {
      elapsed
    },
  }
}
