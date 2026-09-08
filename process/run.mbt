///|
pub(all) suberror RunnerError {
  TimedOut
  Stopped
  OutputLimitExceeded
  ExecutionFailed(String)
} derive(Debug)

///|
/// Run and collect output using the same lifecycle as the streaming runner.
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
  let stdout = StringBuilder()
  let stderr = StringBuilder()
  let (exit_code, duration_ms) = stream(request, Control::new(), (channel, data) => {
    match channel {
      Stdout => stdout.write_string(data)
      Stderr => stderr.write_string(data)
    }
  })
  {
    exit_code,
    duration_ms,
    stdout: stdout.to_string(),
    stderr: stderr.to_string(),
  }
}

///|
/// Compile in a private build directory, then stream a directly managed VM.
/// The deadline covers compilation and execution; callbacks apply backpressure.
/// Callback failures and parent cancellation propagate after child cleanup.
pub async fn stream(
  request : @runtime.Request,
  control : Control,
  on_output : async (@runtime.Stream, String) -> Unit,
) -> (Int, Int64) {
  let started = @async.now()
  let capture = Capture::new(on_output)
  let outcome = try
    @async.with_task_group() <| group => {
      group.spawn_bg(no_wait=true, () => {
        control.wait()
        raise Stopped
      })
      group.spawn_bg(no_wait=true, () => {
        @async.sleep(request.timeout_ms)
        raise TimedOut
      })
      if control.is_stopped() {
        raise Stopped
      }
      execute(request, capture)
    }
  catch {
    error => Err(error)
  } noraise {
    code => Ok(code)
  }
  match outcome {
    Err(CallbackFailed(error)) => raise error
    Err(error) if @async.is_being_cancelled() ||
      @async.is_cancellation_error(error) => raise error
    _ => ()
  }
  capture.flush() catch {
    CallbackFailed(error) => raise error
    OutputLimitExceeded if outcome is Err(_) => ()
    error => raise error
  }
  match outcome {
    Ok(code) => (code, @async.now() - started)
    Err(CallbackFailed(error)) => raise error
    Err(error) => {
      if @async.is_being_cancelled() || @async.is_cancellation_error(error) {
        raise error
      }
      match error {
        TimedOut | Stopped | OutputLimitExceeded | ExecutionFailed(_) =>
          raise error
        _ => raise ExecutionFailed(error.to_string())
      }
    }
  }
}

///|
async fn execute(request : @runtime.Request, capture : Capture) -> Int {
  let directory = @fs.tmpdir(prefix="codex-mbtx-job-")
  defer @async.protect_from_cancel(() => @fs.rmdir(directory, recursive=true))
  let arguments = [
    "run", "--build-only", "--output-json", "--target", "wasm", "--target-dir", directory,
    "--",
  ]
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
  let build = @shell.Cmd("moon", arguments, cwd=request.cwd, stdin?).output(
    max_output_bytes=1024 * 1024,
  )
  if !build.stderr().is_empty() {
    capture.accept(Stderr, @utf8.encode(build.stderr()))
  }
  if build.exit_code() != 0 {
    if !build.stdout().is_empty() {
      capture.accept(Stderr, @utf8.encode(build.stdout()))
    }
    return build.exit_code()
  }
  let artifact = match @json.parse(build.stdout()) {
    { "artifacts_path": [String(path)], .. } => path
    _ => raise ExecutionFailed("moon did not return exactly one Wasm artifact")
  }
  let args = ["--", artifact]
  for arg in request.args() {
    args.push(arg)
  }
  @async.with_task_group() <| group => {
    let (stdout, out) = @child.read_from_process()
    defer stdout.close()
    defer out.close()
    let (stderr, err) = @child.read_from_process()
    defer stderr.close()
    defer err.close()
    let (input, writer) = @child.write_to_process()
    defer close_input(input)
    writer.close()
    let child = @child.spawn(
      group,
      "moonrun",
      args,
      cwd=request.cwd[:],
      stdin=input,
      stdout=out,
      stderr=err,
      cancel_handler=@child.hard_cancel(),
    )
    let stdout_task = group.spawn(() => capture.read(Stdout, stdout))
    let stderr_task = group.spawn(() => capture.read(Stderr, stderr))
    let code = child.wait()
    stdout_task.wait()
    stderr_task.wait()
    code
  }
}

///|
#warnings("-deprecated")
fn close_input(input : &@child.ProcessInput) -> Unit {
  input.close()
}
