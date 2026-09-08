///|
async fn main {
  match @env.args() {
    [_] => @session.serve(@stdio.stdin, @stdio.stdout)
    [_, "--request", request] => @session.serve_request(request, @stdio.stdout)
    _ => fail("usage: mbtx [--request JSON]")
  }
}
