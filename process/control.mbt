///|
/// Cooperative cancellation that is safe even before process startup.
pub struct Control {
  priv mut stopped : Bool
  priv changed : @cond_var.Cond
}

///|
pub fn Control::new() -> Control {
  { stopped: false, changed: @cond_var.Cond(), }
}

///|
/// Request cancellation. `stream` returns only after its child has been reaped.
pub fn Control::stop(self : Control) -> Unit {
  self.stopped = true
  self.changed.broadcast()
}

///|
fn Control::is_stopped(self : Control) -> Bool {
  self.stopped
}

///|
async fn Control::wait(self : Control) -> Unit {
  while !self.stopped {
    self.changed.wait()
  }
}
