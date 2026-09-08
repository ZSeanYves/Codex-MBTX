///|
priv suberror CallbackFailed {
  CallbackFailed(Error)
}

///|
priv struct Capture {
  mut bytes : Int
  mut chunks : Int
  stdout : Decoder
  stderr : Decoder
  on_output : async (@runtime.Stream, String) -> Unit
}

///|
fn Capture::new(on_output : async (@runtime.Stream, String) -> Unit) -> Capture {
  {
    bytes: 0,
    chunks: 0,
    stdout: Decoder::new(),
    stderr: Decoder::new(),
    on_output,
  }
}

///|
async fn Capture::emit(
  self : Capture,
  channel : @runtime.Stream,
  data : String,
) -> Unit {
  if !data.is_empty() {
    if self.chunks >= 4096 {
      raise OutputLimitExceeded
    }
    self.chunks += 1
    (self.on_output)(channel, data) catch {
      error => raise CallbackFailed(error)
    }
  }
}

///|
async fn Capture::accept(
  self : Capture,
  channel : @runtime.Stream,
  bytes : Bytes,
) -> Unit {
  let remaining = 1024 * 1024 - self.bytes
  let count = Int::min(remaining, bytes.length())
  self.bytes += count
  let decoder = match channel {
    Stdout => self.stdout
    Stderr => self.stderr
  }
  self.emit(channel, decoder.push(bytes[:count]))
  if count < bytes.length() {
    raise OutputLimitExceeded
  }
}

///|
async fn Capture::read(
  self : Capture,
  channel : @runtime.Stream,
  reader : @child.ReadFromProcess,
) -> Unit {
  while reader.read_some(max_len=4096) is Some(bytes) {
    self.accept(channel, bytes)
  }
}

///|
async fn Capture::flush(self : Capture) -> Unit {
  self.emit(Stdout, self.stdout.finish())
  self.emit(Stderr, self.stderr.finish())
}

///|
priv struct Decoder {
  mut tail : Bytes
}

///|
fn Decoder::new() -> Decoder {
  { tail: b"", }
}

///|
fn Decoder::push(self : Decoder, bytes : BytesView) -> String {
  let buffer = @buffer.Buffer(size_hint=self.tail.length() + bytes.length())
  buffer.write_bytes(self.tail)
  buffer.write_bytesview(bytes)
  let data = buffer.to_bytes()
  let size = data.length()
  // Retain only a possibly incomplete UTF-8 suffix for the next chunk.
  let mut start = size
  while start > 0 &&
        size - start < 3 &&
        (data[start - 1].to_int() & 0xc0) == 0x80 {
    start -= 1
  }
  if start > 0 {
    let byte = data[start - 1].to_int()
    let width = if byte >= 0xc2 && byte <= 0xdf {
      2
    } else if byte >= 0xe0 && byte <= 0xef {
      3
    } else if byte >= 0xf0 && byte <= 0xf4 {
      4
    } else {
      0
    }
    if width > size - start + 1 {
      start -= 1
    } else {
      start = size
    }
  } else {
    start = size
  }
  self.tail = data[start:].to_owned()
  @utf8.decode_lossy(data[:start])
}

///|
fn Decoder::finish(self : Decoder) -> String {
  let data = self.tail
  self.tail = b""
  @utf8.decode_lossy(data)
}
