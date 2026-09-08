///|
async test "inline script receives literal and option-like arguments" {
  let args = ["", "two words", "$(echo no) | *", "--quiet", "你好", "a\nb"]
  let source =
    #|import {
    #|  "moonbitlang/core/env",
    #|  "moonbitlang/core/json",
    #|}
    #|fn main {
    #|  let values : Array[String] = []
    #|  for arg in @env.args()[1:] { values.push(arg) }
    #|  println(values.to_json().stringify())
    #|}
  let request = @runtime.Request::new("args", Source(source), args~)
  let result = @process.run(request)
  assert_eq(result.exit_code, 0)
  assert_eq(@json.parse(result.stdout), args.to_json())
  assert_true(result.duration_ms >= 0L)
}

///|
async test "a script path and cwd containing spaces are honored" {
  let directory = @fs.tmpdir(prefix="codex mbtx cwd ")
  defer @fs.rmdir(directory, recursive=true)
  let source =
    #|import { "moonbitlang/core/env" }
    #|fn main { println(@env.current_dir().unwrap()) }
  @fs.write_file("\{directory}/hello world.mbtx", source)
  let result = @process.run(
    @runtime.Request::new("cwd", Path("hello world.mbtx"), cwd=directory),
  )
  assert_eq(result.exit_code, 0)
  assert_eq(result.stdout.trim().to_owned(), @fs.realpath(directory))
}

///|
async test "stdout and stderr remain separate on a runtime failure" {
  let source =
    #|import {
    #|  "moonbitlang/async",
    #|  "moonbitlang/async/stdio",
    #|}
    #|async fn main {
    #|  @stdio.stdout.write("before failure\n")
    #|  @stdio.stderr.write("stderr marker\n")
    #|  abort("intentional failure")
    #|}
  let result = @process.run(
    @runtime.Request::new("runtime-failure", Source(source)),
  )
  assert_true(result.exit_code != 0)
  assert_eq(result.stdout, "before failure\n")
  assert_true(result.stderr.contains("stderr marker"))
  assert_false(result.stdout.contains("stderr marker"))
}

///|
async test "compile errors preserve the toolchain diagnostics" {
  let result = @process.run(
    @runtime.Request::new(
      "compile-failure",
      Source("fn main { missing_function() }"),
    ),
  )
  assert_true(result.exit_code != 0)
  assert_true(result.stderr.contains("missing_function"))
}

///|
async test "invalid cwd is a runner failure" {
  let directory = @fs.tmpdir(prefix="codex-mbtx-missing-cwd")
  @fs.rmdir(directory)
  try
    @process.run(
      @runtime.Request::new("bad-cwd", Source("fn main {}"), cwd=directory),
    )
  catch {
    @process.ExecutionFailed(message) => assert_false(message.is_empty())
    error => raise error
  } noraise {
    _ => fail("expected a missing working directory to fail")
  }
}

///|
async test "captured output cannot grow beyond the runner limit" {
  let source =
    #|fn main {
    #|  for _ in 0..<100_000 { println("0123456789abcdef") }
    #|}
  try
    @process.run(@runtime.Request::new("output-limit", Source(source)))
  catch {
    @process.OutputLimitExceeded => ()
    error => raise error
  } noraise {
    _ => fail("expected excessive output to stop the run")
  }
}
