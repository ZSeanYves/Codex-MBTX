///|
test "request owns its literal arguments" {
  let args = ["", "two words", "$(echo no) | *", "--quiet"]
  let request = @runtime.Request::new("literal", Source("fn main {}"), args~)
  args[0] = "changed"
  let copy = request.args()
  copy[1] = "also changed"
  assert_eq(request.args(), ["", "two words", "$(echo no) | *", "--quiet"])
}

///|
test "invalid execution metadata is rejected at construction" {
  let cases = [
    ("", @runtime.Script::Source("fn main {}"), ["ok"], "."),
    ("id", Source(" "), [], "."),
    ("id", Path("script.sh"), [], "."),
    ("id", Path("script.mbtx"), [], ""),
    ("id", Source("fn main {}"), ["bad\u{0000}arg"], "."),
    ("id", Path("bad\u{0000}.mbtx"), [], "."),
  ]
  for case in cases {
    let (id, script, args, cwd) = case
    try @runtime.Request::new(id, script, args~, cwd~) catch {
      @runtime.InvalidRequest(_) => ()
    } noraise {
      _ => fail("expected invalid execution metadata to be rejected")
    }
  }
}
