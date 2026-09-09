///|
async fn main {
  match @env.args() {
    [_] => @session.serve(@stdio.stdin, @stdio.stdout)
    [_, "--request", request] => @session.serve_request(request, @stdio.stdout)
    [_, "exec", "--", command, .. args] => {
      let code = @process.proxy(command, args.to_owned())
      process_exit(code)
    }
    _ => fail("usage: mbtx [--request JSON] | exec -- COMMAND [ARGUMENTS...]")
  }
}

///|
/// Preserve the proxied command's exit status for Codex's unified executor.
#cfg(target="native")
extern "C" fn process_exit(code : Int) = "exit"

///|
/// Preserve the proxied command's exit status for a Wasm CLI build.
#cfg(target="wasm")
fn process_exit(code : Int) = "moonbitlang/async" "runtime/exit"
