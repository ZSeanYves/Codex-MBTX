///|
/// A script supplied inline or as a `.mbtx` path relative to the request's cwd.
pub(all) enum Script {
  Source(String)
  Path(String)
} derive(Debug, Eq)

///|
/// A validated execution request. Use `Request::new` to construct it.
pub struct Request {
  id : String
  script : Script
  cwd : String
  background : Bool
  timeout_ms : Int
  priv arguments : Array[String]
} derive(Debug)

///|
pub(all) suberror RequestError {
  InvalidRequest(String)
} derive(Debug, Eq)

///|
/// Construct a request without interpreting arguments as shell syntax.
/// `cwd` is a host working directory, not a sandbox boundary.
///
/// ```mbt check
/// test {
///   let request = @runtime.Request::new("example", Source("fn main {}"))
///   assert_eq(request.id, "example")
///   assert_eq(request.cwd, ".")
/// }
/// ```
pub fn Request::new(
  id : String,
  script : Script,
  args? : Array[String] = [],
  cwd? : String = ".",
  background? : Bool = false,
  timeout_ms? : Int = 30_000,
) -> Request raise RequestError {
  require_text(id, "id")
  require_text(cwd, "cwd")
  if timeout_ms < 1 || timeout_ms > 600_000 {
    raise InvalidRequest("timeout_ms must be an integer between 1 and 600000")
  }
  match script {
    Source(source) => require_text(source, "source")
    Path(path) => {
      require_text(path, "script_path")
      if !path.has_suffix(".mbtx") {
        raise InvalidRequest("script_path must end in .mbtx")
      }
    }
  }
  for arg in args {
    if arg.contains("\u{0000}") {
      raise InvalidRequest("args must not contain NUL bytes")
    }
  }
  { id, script, cwd, background, timeout_ms, arguments: args.copy(), }
}

///|
/// Return a copy so a caller cannot mutate a validated request's arguments.
pub fn Request::args(self : Request) -> Array[String] {
  self.arguments.copy()
}

///|
fn require_text(value : String, field : String) -> Unit raise RequestError {
  if value.trim().is_empty() {
    raise InvalidRequest("\{field} must not be empty")
  }
  if value.contains("\u{0000}") {
    raise InvalidRequest("\{field} must not contain NUL bytes")
  }
}
