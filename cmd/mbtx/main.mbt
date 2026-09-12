///|
async fn main {
  match @env.args() {
    [_, "exec", "--", command, .. args] => {
      let code = @launcher.run(command, args.to_owned())
      process_exit(code)
    }
    _ => {
      @stdio.stderr.write("usage: mbtx exec -- COMMAND [ARGUMENTS...]\n")
      process_exit(2)
    }
  }
}

///|
/// Preserve the proxied command's exit status for Codex's unified executor.
#cfg(target="native")
extern "C" fn process_exit(code : Int) = "mbtx_process_exit"

///|
/// Preserve the proxied command's exit status for a Wasm CLI build.
#cfg(target="wasm")
fn process_exit(code : Int) = "moonbitlang/async" "runtime/exit"
