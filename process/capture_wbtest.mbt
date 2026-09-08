///|
test "UTF-8 is identical across every possible chunk boundary" {
  let cases = [
    b"a\xe4\xbd\xa0\xe5\xa5\xbd\xf0\x9f\x98\x80z", b"\xef\xbb\xbfhello\x00\n", b"\xe0\x80\x80\xed\xa0\x80\xff\xf4\x90\x80\x80",
    b"\xf0\x91\x92", b"\x80\x80\xc2\xe1\x80z",
  ]
  for data in cases {
    for boundary in 0..<=data.length() {
      let decoder = Decoder::new()
      let text = decoder.push(data[:boundary]) +
        decoder.push(data[boundary:]) +
        decoder.finish()
      assert_eq(text, @utf8.decode_lossy(data))
    }
    let decoder = Decoder::new()
    let result = StringBuilder()
    for index in 0..<data.length() {
      result.write_string(decoder.push(data[index:index + 1]))
    }
    result.write_string(decoder.finish())
    assert_eq(result.to_string(), @utf8.decode_lossy(data))
  }
}

///|
async test "both streams share the byte and chunk bounds" {
  let capture = Capture::new((_, _) => ())
  capture.bytes = 1024 * 1024 - 2
  capture.accept(Stdout, b"a")
  capture.accept(Stderr, b"b")
  try capture.accept(Stdout, b"c") catch {
    OutputLimitExceeded => ()
    error => raise error
  } noraise {
    _ => fail("expected a shared byte limit")
  }
  let capture = Capture::new((_, _) => ())
  capture.chunks = 4096
  try capture.accept(Stderr, b"x") catch {
    OutputLimitExceeded => ()
    error => raise error
  } noraise {
    _ => fail("expected a bounded number of retained chunks")
  }
}
