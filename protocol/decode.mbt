///|
/// Decode one run request. The M1 API remains available to library callers.
///
/// ```mbt check
/// test {
///   let line =
///     #|{"id":"demo","op":"run","source":"fn main {}"}
///   assert_eq(@protocol.decode_request(line).id, "demo")
/// }
/// ```
pub fn decode_request(
  line : String,
) -> @runtime.Request raise @runtime.RequestError {
  match decode_command(line) {
    Run(request) => request
    _ => raise @runtime.InvalidRequest("expected a run request")
  }
}

///|
/// Decode run, job_output, or job_stop, rejecting unknown fields and coercions.
pub fn decode_command(
  line : String,
) -> @runtime.Command raise @runtime.RequestError {
  let json = @json.parse(line) catch {
    error => raise @runtime.InvalidRequest("invalid JSON: \{error}")
  }
  guard json is Object(fields) else {
    raise @runtime.InvalidRequest("request must be a JSON object")
  }
  let id = string_field(fields, "id")
  let op = string_field(fields, "op")
  match op {
    "run" => {
      check_fields(fields, [
        "id", "op", "source", "script_path", "args", "cwd", "background", "timeout_ms",
      ])
      Run(decode_run(id, fields))
    }
    "job_output" => {
      check_fields(fields, ["id", "op", "job_id", "cursor"])
      let job_id = string_field(fields, "job_id")
      let cursor = integer_field(fields, "cursor", default=0, min=0, max=4096)
      JobOutput(id~, job_id~, cursor~)
    }
    "job_stop" => {
      check_fields(fields, ["id", "op", "job_id"])
      JobStop(id~, job_id=string_field(fields, "job_id"))
    }
    _ =>
      raise @runtime.InvalidRequest("op must be run, job_output, or job_stop")
  }
}

///|
fn decode_run(
  id : String,
  fields : Map[String, Json],
) -> @runtime.Request raise @runtime.RequestError {
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
  let cwd = if fields.contains("cwd") {
    string_field(fields, "cwd")
  } else {
    "."
  }
  let background = match fields.get("background") {
    None | Some(False) => false
    Some(True) => true
    _ => raise @runtime.InvalidRequest("background must be a boolean")
  }
  let timeout_ms = integer_field(
    fields,
    "timeout_ms",
    default=30_000,
    min=1,
    max=600_000,
  )
  @runtime.Request::new(id, script, args~, cwd~, background~, timeout_ms~)
}

///|
fn string_field(
  fields : Map[String, Json],
  name : String,
) -> String raise @runtime.RequestError {
  match fields.get(name) {
    Some(String(value)) if !value.trim().is_empty() &&
      !value.contains("\u{0000}") => value
    _ =>
      raise @runtime.InvalidRequest(
        "\{name} must be a nonempty string without NUL",
      )
  }
}

///|
fn integer_field(
  fields : Map[String, Json],
  name : String,
  default~ : Int,
  min~ : Int,
  max~ : Int,
) -> Int raise @runtime.RequestError {
  match fields.get(name) {
    None => default
    Some(Number(value, ..)) if value >= min.to_double() &&
      value <= max.to_double() &&
      value.to_int().to_double() == value => value.to_int()
    _ =>
      raise @runtime.InvalidRequest(
        "\{name} must be an integer between \{min} and \{max}",
      )
  }
}

///|
fn check_fields(
  fields : Map[String, Json],
  allowed : Array[String],
) -> Unit raise @runtime.RequestError {
  for key, _ in fields {
    if !allowed.contains(key) {
      raise @runtime.InvalidRequest("unsupported field: \{key}")
    }
  }
}
