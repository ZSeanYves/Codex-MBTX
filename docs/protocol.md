# M1 JSONL Protocol

Run `moon run --quiet cmd/mbtx` from a checkout. The runner reads UTF-8 JSONL
from stdin and reserves stdout for JSONL events. It executes one request at a
time until EOF. Blank lines are ignored; a final nonempty line without a newline
is processed. Invalid requests produce an error event and do not stop later
requests.

## Request

```json
{"id":"hello","op":"run","source":"fn main { println(42) }","args":[],"cwd":"."}
```

| Field | Contract |
| --- | --- |
| `id` | Required nonempty string, echoed in execution events. A correlation ID, not a persistent job handle. |
| `op` | Required string `run`. |
| `source` | Nonempty MoonBit script source. Exactly one of `source` and `script_path` is required. |
| `script_path` | Nonempty path ending in `.mbtx`, resolved by `moon` relative to `cwd`. |
| `args` | Optional array of strings; defaults to `[]`. Empty arguments and shell punctuation are passed literally. |
| `cwd` | Optional nonempty host directory; defaults to `.` relative to the runner's working directory. |

Unknown fields, wrong types, NUL bytes, and unsupported operations are rejected.
M1 does not accept `background`, `timeout_ms`, or policy fields. The runner does
not interpret shell commands or expand argument punctuation.

## Events

Each event is a JSON object followed by one newline. Embedded newlines and
control characters in captured text are escaped by the JSON encoder.

```json
{"event":"started","id":"hello"}
{"event":"stdout","id":"hello","data":"42\n"}
{"event":"completed","id":"hello","exit_code":0,"duration_ms":123}
```

- `started`: the request passed validation and execution is about to be attempted.
  This event alone does not prove the child was successfully spawned.
- `stdout` / `stderr`: captured text, emitted after the process has finished.
  Empty streams produce no event. M1 emits stdout before stderr and does not
  preserve ordering between the two original streams. Text uses lossy UTF-8.
- `completed`: the toolchain process exited. `exit_code` and `duration_ms` are
  JSON numbers. Duration covers process startup, compilation, and execution.
  Nonzero exits also use this event, retaining compiler or runtime diagnostics
  in stderr. The runner does not guess which stage failed.
- `failed`: the request could not be validated or the runner could not obtain
  a completed result.

```json
{"event":"failed","id":null,"error":{"kind":"invalid_request","message":"op must be run"}}
```

`invalid_request` has `id: null` because there is no accepted request. Errors
after acceptance echo the request ID and use `timeout` or `runner_error`.
`runner_error` includes spawn/IO failures and output-limit failures. Partial
output is unavailable when the underlying capture fails.

The CLI's exit status describes the protocol process itself, not the last
script: it exits normally after handling requests that include script failures.
Clients must inspect each request's terminal event. Fatal protocol transport
errors, such as a broken stdout pipe, can terminate the runner without a final
event.

## Execution environment and limits

The runner launches `moon` from `PATH`, forces its Wasm target, and inherits the
host environment. Inline source is sent using `moon run -`; file requests close
stdin. Interactive input is outside M1's contract.

Every invocation has a fixed 30,000 ms deadline and a 1,048,576 byte combined
stdout/stderr capture cap, including compiler diagnostics. These are execution
bounds rather than sandbox guarantees. The direct child is managed by
`moonbitlang/async/shell`; descendants require the host to enforce confinement.
`cwd` selects a directory and does not restrict filesystem access. The future
Codex adapter must provide host approvals and sandboxing before this is used
as an untrusted execution service.
