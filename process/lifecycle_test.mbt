///|
async test "streaming starts before exit and stop reaps the actual VM" {
  let directory = @fs.tmpdir(prefix="mbtx-stop-test-")
  defer @fs.rmdir(directory, recursive=true)
  let source =
    #|import {
    #|  "moonbitlang/async",
    #|  "moonbitlang/async/stdio",
    #|  "moonbitlang/async/fs",
    #|  "moonbitlang/core/env",
    #|}
    #|async fn main {
    #|  @stdio.stdout.write("ready")
    #|  @async.sleep(500)
    #|  @fs.write_file(@env.args()[1], "should not run")
    #|}
  let marker = "\{directory}/after-stop"
  let control = @process.Control::new()
  let seen = StringBuilder()
  try
    @process.stream(
      @runtime.Request::new("cancel", Source(source), args=[marker]),
      control,
      (stream, data) => {
        if stream == Stdout {
          seen.write_string(data)
          if seen.to_string() == "ready" {
            control.stop()
          }
        }
      },
    )
  catch {
    @process.Stopped => ()
    error => raise error
  } noraise {
    _ => fail("stop must interrupt the script before it completes")
  }
  assert_eq(seen.to_string(), "ready")
  @async.sleep(600)
  assert_false(@fs.exists(marker))
}

///|
async test "timeout interrupts execution and retains earlier output" {
  let source =
    #|import { "moonbitlang/async", "moonbitlang/async/stdio" }
    #|async fn main {
    #|  @stdio.stdout.write("ready")
    #|  @async.sleep(60000)
    #|}
  let seen = StringBuilder()
  let started = @async.now()
  try
    @process.stream(
      @runtime.Request::new("timeout", Source(source), timeout_ms=2000),
      @process.Control::new(),
      (stream, data) => if stream == Stdout { seen.write_string(data) },
    )
  catch {
    @process.TimedOut => ()
    error => raise error
  } noraise {
    _ => fail("expected deadline to stop execution")
  }
  assert_eq(seen.to_string(), "ready")
  assert_true(@async.now() - started < 10000L)
}

///|
async test "stop before startup never executes source" {
  let control = @process.Control::new()
  control.stop()
  control.stop()
  try
    @process.stream(
      @runtime.Request::new("pre-stopped", Source("fn main { println(42) }")),
      control,
      (_, _) => fail("pre-stopped job must have no output"),
    )
  catch {
    @process.Stopped => ()
    error => raise error
  } noraise {
    _ => fail("expected pre-start cancellation")
  }
}

///|
async test "completion drains all buffered stdout and stderr" {
  let source =
    #|import { "moonbitlang/async", "moonbitlang/async/stdio" }
    #|async fn main {
    #|  let text = StringBuilder()
    #|  for _ in 0..<8192 { text.write_string("0123456789abcdef") }
    #|  @stdio.stdout.write(text.to_string())
    #|  @stdio.stderr.write(text.to_string())
    #|}
  let result = @process.run(@runtime.Request::new("drain", Source(source)))
  assert_eq(result.exit_code, 0)
  assert_eq(result.stdout.length(), 131072)
  assert_eq(result.stderr.length(), 131072)
  assert_true(result.stdout.has_suffix("0123456789abcdef"))
  assert_true(result.stderr.has_suffix("0123456789abcdef"))
}
