# M2 JSONL Protocol

Run `moon run --quiet cmd/mbtx` from a checkout. Stdin and stdout carry UTF-8
JSONL. Blank lines are ignored; CRLF and a final record without a newline are
accepted. Each request line is limited to 1 MiB. Oversized records are discarded
through the next newline; malformed JSON or UTF-8 produces an error and does
not prevent later requests from being handled.

The optional CLI form `mbtx --request JSON` accepts exactly one run request as a
literal argument, allows formatted JSON, and drains its events without reading
stdin. It rejects multiple records and control-only launches. This is the
launch interface used by the Codex adapter; stdin JSONL mode remains available.

The transparent CLI form `mbtx exec -- COMMAND ARGUMENT...` is a separate host
launcher used by Codex's transparent backend. It forwards the literal argv and
the caller's standard streams to the child, then exits with the child's status.
It does not parse shell syntax, generate MoonBit source, invoke `moon run`, or
use the JSONL script-job protocol. The caller has already performed command
resolution, approval, and sandbox planning before this form is launched.

## Scheduling and lifetime

A run defaults to foreground execution. Run submissions are processed in order;
a foreground run holds subsequent run submissions until its terminal event has
been enqueued. A background run releases that submission queue immediately.
Both modes allow `job_output` and `job_stop` while the script is running.

The session admits at most four active jobs and queues at most 16 run requests.
Excess requests receive `failed` with kind `busy`; they have no job ID. A full
run queue does not block control requests. A job's deadline starts when execution
begins, after admission; time in the pending run queue is not included.

EOF stops admission, drains queued runs, and waits for active jobs to finish
under their deadlines. Background does not mean detached: jobs belong to this
runner process. A protocol transport error cancels the session's structured task
group and cleans up its directly managed children before propagating.

Job IDs are monotonically increasing, session-local strings such as `job-1`.
They are not PIDs and do not survive a runner restart. The registry retains up to
32 jobs, including active jobs. When a new admission needs room, the oldest
settled job in admission order is evicted. Active jobs are never evicted.
Clients must wait for `started` before sending controls for a job.

## Run request

```json
{"id":"hello","op":"run","source":"fn main { println(42) }","args":[],"cwd":".","background":false,"timeout_ms":30000}
```

| Field | Contract |
| --- | --- |
| `id` | Required nonempty string, echoed in events. Use a distinct ID for each run/control request. |
| `op` | Required string `run`. |
| `source` | Nonempty script source. Exactly one of `source` and `script_path` is required. |
| `script_path` | Nonempty path ending in `.mbtx`, resolved relative to `cwd`. |
| `args` | Optional array of strings; default `[]`. Empty and option-like arguments and shell punctuation are literal. |
| `cwd` | Optional nonempty host directory; default `.` relative to the runner. |
| `background` | Optional boolean; default `false`. |
| `timeout_ms` | Optional integer from 1 to 600,000; default 30,000. Includes compilation, execution, output backpressure, and cleanup. |

Unknown fields, wrong types, NUL bytes, fractional numbers, and unsupported
operations are rejected. There are no implicit string/number conversions.

## Output and stop requests

```json
{"id":"query-1","op":"job_output","job_id":"job-1","cursor":0}
{"id":"stop-1","op":"job_stop","job_id":"job-1"}
```

Both require nonempty `id` and `job_id` strings. `job_output` takes an optional
integer cursor (default 0), which is the next output sequence number to read.
It returns up to 128 chunks and a `next_cursor` for pagination. A cursor beyond
current history is rejected with `invalid_cursor`. No polling operation consumes
or mutates history.

```json
{"event":"job_output","id":"query-1","job_id":"job-1","state":"running","chunks":[{"seq":0,"stream":"stdout","data":"42\n"}],"next_cursor":1,"exit_code":null,"duration_ms":null,"error":null}
{"event":"job_stop","id":"stop-1","job_id":"job-1","state":"stopping"}
```

Repeated stops are idempotent. A stop reply is an acknowledgement, not proof of
termination. Wait for the run's terminal event or a terminal snapshot before
reclaiming its resources. A previously decided completion, timeout, or failure
can win a race with a stop. Unknown or evicted job IDs return `unknown_job`.

## Run events

```json
{"event":"started","id":"hello","job_id":"job-1"}
{"event":"stdout","id":"hello","job_id":"job-1","seq":0,"data":"42\n"}
{"event":"completed","id":"hello","job_id":"job-1","exit_code":0,"duration_ms":123}
```

- `started`: validation and admission succeeded; compilation is about to be
  attempted. It does not guarantee successful process creation.
- `stdout` / `stderr`: live output chunks, with a shared, zero-based sequence
  number per job. Chunk boundaries are arbitrary; clients concatenate data by
  stream and must not assume one chunk per line. Newlines are not required for
  delivery. UTF-8 split across reads is preserved; malformed/incomplete bytes
  decode lossily. Order within each stream is preserved; relative timing across
  the two OS pipes is not guaranteed.
- `completed`: compilation or script execution exited, readers drained, and
  cleanup finished. Nonzero exit codes also use this event and preserve
  diagnostics. The runner does not guess compilation/runtime error categories
  from diagnostic text.
- `stopped`: cancellation finished and the direct child was reaped.
- `failed`: validation, admission, timeout, output limit, or execution failure.

Each admitted job emits exactly one terminal event during a healthy session:
`completed`, `stopped`, or `failed`. No output event follows its terminal event.
Independent jobs may interleave; one writer serializes complete JSON frames.
Control responses carry the control request ID, while live and terminal events
retain the original run ID.

```json
{"event":"failed","id":"hello","job_id":"job-1","error":{"kind":"timeout","message":"execution exceeded 30000 ms"}}
{"event":"failed","id":null,"job_id":null,"error":{"kind":"invalid_request","message":"request must be a JSON object"}}
```

Validation errors have null IDs because no request was accepted. Admission
errors echo the run ID and use a null job ID. Control errors echo both IDs.
`runner_error` covers toolchain, spawn, and IO errors. `output_limit` covers
script output bounds. Earlier captured output remains queryable on failure or
stop. Under output backpressure, cancellation may prevent some already retained
chunks from reaching the live stream; `job_output` can retrieve that prefix.

Snapshots use states `running`, `stopping`, `completed`, `failed`, and
`stopped`. Terminal snapshots contain numeric `duration_ms`, and either an
exit code or error when applicable. JSON nullable fields are scalars or null.

An incomplete UTF-8 suffix that has not yet been emitted may be discarded on
timeout or cancellation; normal EOF flushes it using lossy decoding.

The CLI exits normally after processing requests even when scripts fail.
Clients inspect per-request terminal events, not just the CLI exit code.
A fatal transport error can terminate the runner without delivering final
events; it propagates after cancellation and cleanup.

## Execution environment and bounds

The runner invokes `moon run --build-only --output-json --target wasm` with a
private temporary target directory per job. It parses the toolchain's structured
artifact result, then starts `moonrun` directly. This makes the script VM a
directly cancellable child and prevents concurrent builds from sharing artifacts.
Inline source is supplied to the compiler on stdin; the VM's stdin is closed.
Compiler diagnostics are buffered until compilation finishes; script stdout
and stderr stream as they become available. Temporary builds are removed before
the job reaches its terminal state.

There is a combined 1,048,576-byte script output limit (including delivered
compiler diagnostics) and a 4,096-chunk limit per job. Internal compiler capture
is separately limited to 1 MiB. Hitting an output bound stops the job and retains
only the accepted prefix. Output snapshots are paginated, event delivery is
backpressured through a 64-event queue, and request/history counts are bounded.

The runner itself supports Wasm and native backends; scripts always run on Wasm.
The toolchain is resolved from PATH and inherits the host environment. `cwd`
selects a directory without restricting filesystem access. The runner is not
a sandbox: direct VM cancellation does not guarantee termination of arbitrary
descendant processes launched by scripts, nor cleanup after killing the runner
itself. Host confinement and approvals belong to the future Codex adapter.
Interactive stdin, detached/persistent jobs, and host sandbox policy are outside
M2.
