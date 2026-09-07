///|
/// Decode one JSONL request. Unknown fields and unsupported operations fail
/// explicitly so a caller cannot accidentally request unsupported behavior.
///
/// ```mbt check
/// test {
///   let line =
///     #|{"id":"demo","op":"run","source":"fn main {}"}
///   let request = @protocol.decode_request(line)
///   assert_eq(request.id, "demo")
/// }
/// ```
pub fn decode_request(
  line : String,
) -> @runtime.Request raise @runtime.RequestError {
  let json = @json.parse(line) catch {
    error => raise @runtime.InvalidRequest("invalid JSON: \{error}")
  }
  guard json is Object(fields) else {
    raise @runtime.InvalidRequest("request must be a JSON object")
  }
  for key, _ in fields {
    match key {
      "id" | "op" | "source" | "script_path" | "args" | "cwd" => ()
      _ => raise @runtime.InvalidRequest("unsupported field: \{key}")
    }
  }
  let id = string_field(fields, "id")
  if string_field(fields, "op") != "run" {
    raise @runtime.InvalidRequest("op must be run")
  }
  let script = match (fields.get("source"), fields.get("script_path")) {
    (Some(String(source)), None) => @runtime.Script::Source(source)
    (None, Some(String(path))) => @runtime.Script::Path(path)
    _ =>
      raise @runtime.InvalidRequest(
        "provide exactly one string field: source or script_path",
      )
  }
  let args = match fields.get("args") {
    None => []
    Some(Array(values)) => {
      let args : Array[String] = []
      for value in values {
        guard value is String(arg) else {
          raise @runtime.InvalidRequest("args must be an array of strings")
        }
        args.push(arg)
      }
      args
    }
    _ => raise @runtime.InvalidRequest("args must be an array of strings")
  }
  let cwd = match fields.get("cwd") {
    None => "."
    Some(String(cwd)) => cwd
    _ => raise @runtime.InvalidRequest("cwd must be a string")
  }
  @runtime.Request::new(id, script, args~, cwd~)
}

///|
fn string_field(
  fields : Map[String, Json],
  name : String,
) -> String raise @runtime.RequestError {
  match fields.get(name) {
    Some(String(value)) => value
    _ => raise @runtime.InvalidRequest("\{name} must be a string")
  }
}
