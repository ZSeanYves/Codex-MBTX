# MoonBit Reference

MoonBit and the `moonbitlang/async@0.21.2` package are installed. Script files
use `.mbtx`; run an existing script with `moon run path.mbtx ARG...`.
New scripts may have imports followed by `async fn main { ... }` (no `()` after
`main`). Async calls need no `await`. Use structured JSON and literal argv.

```moonbit
import {
  "moonbitlang/async",
  "moonbitlang/async/fs",
  "moonbitlang/async/shell",
  "moonbitlang/core/json",
}

async fn main {
  let text = @fs.read_file("input.json").text()
  let data = @json.parse(text)
  if data is Array(rows) {
    let mut sum = 0
    for row in rows {
      if row is { "value": Number(n, ..), .. } { sum += n.to_int() }
    }
    let result : Json = { "total": sum.to_json() }
    @fs.write_file("result.json", result.stringify())
  }
  let output = @shell.Cmd("moon", ["run", "existing.mbtx", "literal argument"]).output()
  println(output.stdout())
  guard output.exit_code() == 0 else { fail(output.stderr()) }
}
```

`@fs.read_file(path).text()` reads UTF-8. `text.split("\n")` returns string
views; `.to_owned()` makes a String. JSON variants include `Object(fields)`,
`Array(items)`, `String(s)`, `Number(n, ..)`, `True`, `False`, `Null`.
`Map[String, Int] = {}` supports `map.get(key).unwrap_or(0)` and `map[key] = n`.
`Array[Int] = []` supports `.push(n)`. Values have `.to_json()`; JSON has
`.stringify()`. Catch parse errors using `try { ... } catch { _ => ... }`.
Use `moon ide doc '@fs.*'` or `moon ide doc '@shell.*'` to discover installed APIs.

An MBTX run uses `{"op":"run","source":"...","background":false}`
or `{"op":"run","script_path":"path.mbtx"}`. Source is MoonBit, not shell.
For background work set `background:true`, retain the returned opaque `job_id`,
then use `{"op":"job_output","job_id":"...","wait_ms":1000}` until done.
For an external program in MBTX, invoke `@shell.Cmd(program, [literal, args])`
from a MoonBit script. `args` on an MBTX request are arguments to that script.
