///|
#cfg(target="native")
extern "C" fn cancellation_start() -> Int = "mbtx_cancellation_start"

///|
#cfg(target="native")
extern "C" fn cancellation_child(pid : Int) = "mbtx_cancellation_child"

///|
#cfg(target="native")
extern "C" fn cancellation_signal() -> Int = "mbtx_cancellation_signal"

///|
#cfg(target="native")
extern "C" fn cancellation_kill(pid : Int) = "mbtx_cancellation_kill"

///|
#cfg(target="native")
extern "C" fn cancellation_finish() = "mbtx_cancellation_finish"

///|
#cfg(target="native")
fn prepare_cancellation() -> Unit raise {
  let code = cancellation_start()
  if code != 0 {
    fail("cannot install launcher cancellation handler: errno=\{code}")
  }
  @signal.set_global_cancellation_signals([])
}

///|
#cfg(target="native")
fn finish_cancellation() -> Unit {
  cancellation_finish()
  @signal.set_global_cancellation_signals([SIGINT, SIGTERM, SIGHUP])
}

///|
#cfg(target="native")
async fn monitor_cancellation(child : @child.Process) -> Unit {
  cancellation_child(child.pid)
  while cancellation_signal() == 0 {
    @async.sleep(5)
  }
  trace_event(4, cancellation_signal(), child.pid)
  @async.sleep(500)
  if child.try_wait() is None {
    trace_event(5, 9, child.pid)
    cancellation_kill(child.pid)
  }
}

///|
#cfg(target="wasm")
fn prepare_cancellation() -> Unit {
  ()
}

///|
#cfg(target="wasm")
fn finish_cancellation() -> Unit {
  ()
}

///|
#cfg(target="wasm")
async fn monitor_cancellation(_child : @child.Process) -> Unit {
  ()
}
