///|
async fn read_frames(
  input : &@io.Reader,
  on_line : async (String) -> Unit,
  on_invalid : async () -> Unit,
) -> Unit {
  let pending = @buffer.Buffer(size_hint=4096)
  let mut oversized = false
  while input.read_some(max_len=4096) is Some(bytes) {
    let mut start = 0
    for index in 0..<bytes.length() {
      if bytes[index] == b'\n' {
        if !oversized {
          if pending.length() + index - start > 1024 * 1024 {
            on_invalid()
          } else {
            pending.write_bytesview(bytes[start:index])
            deliver(pending.to_bytes(), on_line, on_invalid)
          }
        }
        pending.reset()
        oversized = false
        start = index + 1
      }
    }
    if !oversized {
      if pending.length() + bytes.length() - start > 1024 * 1024 {
        pending.reset()
        oversized = true
        on_invalid()
      } else {
        pending.write_bytesview(bytes[start:])
      }
    }
  }
  if !oversized && pending.length() > 0 {
    deliver(pending.to_bytes(), on_line, on_invalid)
  }
}

///|
async fn deliver(
  bytes : Bytes,
  on_line : async (String) -> Unit,
  on_invalid : async () -> Unit,
) -> Unit {
  let line = @utf8.decode(bytes) catch {
    _ => {
      on_invalid()
      return
    }
  }
  if !line.trim().is_empty() {
    on_line(line)
  }
}
