# M3 Codex adapter

## Build and run

M3 pins OpenAI Codex `rust-v0.153.4` at
`3d2ee51ca2d5db578f328aa75e20aa22c0197c9a`. The source is fetched into
`_build/codex-upstream`; `codex/integration.patch` updates existing files and
`codex/overlay/` supplies the adapter and integration tests. Preparation verifies
the commit and rejects conflicting patches. This repository does not publish
a second copy of the complete upstream history.

With MoonBit, Git, Rust and `just` installed:

```bash
moon run scripts/m3_build.mbtx
```

This creates `_build/m3-dist/codex` and `_build/m3-dist/mbtx` and prints the
configuration line containing the absolute runner path. Add that line to your
user Codex configuration, or pass it through Codex's `-c` option:

```toml
mbtx_command = ["/absolute/path/to/mbtx"]
```

Run the built `codex` executable with your usual workspace, model login and
permission settings. MoonBit's `moon` and `moonrun` must remain on its PATH.
Removing `mbtx_command` disables the new tool. The installed Codex desktop
application is not modified.

`mbtx_command` is an argv array, not a shell string. Its first element must be an
absolute executable path. Trusted prefix arguments are supported, for example
a `moonrun` binary followed by a compiled Wasm runner artifact. Only user,
system, managed or runtime configuration may select this command; project-local
configuration is explicitly excluded.

## Tool contract

The model sees one `mbtx` function alongside the existing `exec_command` and
`write_stdin` tools:

```json
{"op":"run","source":"fn main { println(42) }","args":[],"background":false,"timeout_ms":30000}
{"op":"run","script_path":"examples/hello.mbtx","args":["literal;argument"],"background":true}
{"op":"job_output","job_id":"mbtx-..."}
{"op":"job_stop","job_id":"mbtx-..."}
```

Exactly one script form is required. `cwd` defaults to Codex's selected workspace;
relative paths resolve against that workspace. Arguments are passed literally.
The model cannot select an executable, change environment variables, or request
an implicit sandbox bypass through this tool. Inline requests are limited to
32 KiB; larger scripts can use `script_path`. Timeouts range from 1 to 600,000 ms.

Results are JSON with `job_id`, `state`, `stdout`, `stderr`, and `omitted_bytes`.
Completion includes `exit_code` and `duration_ms`; failure includes `error.kind`
and `error.message`. A nonzero script exit is a completed execution with a failed
tool result. A malformed, truncated, mismatched or incomplete runner transcript
is a runner error, never a successful script execution.

Adapter job IDs are opaque and owned by one Codex session. At most four
unresolved jobs and 32 retained results are allowed. Polling drains new output;
the last reply for a finished job remains available and stops are idempotent.
Poll completed background jobs to release their admission slots. Restarting
Codex invalidates the job IDs.

These controls use Codex's process ownership and differ from the standalone
M2 JSONL controls: they do not expose M2's history cursor or its runner-local
`job-1` identifiers. Each adapter invocation creates an isolated runner using
`--request JSON`; the runner consumes exactly one run and closes its input.
Foreground waits for completion, while background returns a Codex-managed job.
The runner still enforces the M2 execution deadline. Codex adds a two-second
grace period before terminating an unresponsive runner. Waiting for an execution
approval does not consume the execution deadline. Explicit stop terminates
the managed process through Codex's existing process manager. An in-flight
output drain can delay host timeout cleanup by Codex's bounded poll interval.
Admission and reads serialize within this backend; a foreground run cannot
hold up another job's watchdog. Completed jobs cancel their watchdog timers.

## Host integration

The adapter constructs a direct argv request for Codex's unified executor.
The full canonical MBTX request is present in the approval action before
execution. Existing execution policy, approval routing, environment filtering,
sandbox selection, live execution events and session shutdown apply to the
runner. The adapter neither spawns a process with an independent unrestricted
launcher nor implements another model loop.

The tool is exposed only when shell execution and unified execution are enabled,
with exactly one local environment. Reviewer-only sessions, remote executors,
multiple environments and configurations that disable resumable execution do
not gain an MBTX tool. M3 validates Linux and macOS; Windows is not a validated
deployment target.

Runner JSONL is decoded across arbitrary pipe fragments. Request/job IDs,
sequence numbers and terminal ordering are checked. Model replies remain valid
JSON within 8,000 serialized bytes, with omitted output counted explicitly.
Codex's existing live command events still carry the raw runner transcript.
If Codex's 1 MiB transport buffer omits protocol bytes, the adapter fails and
terminates that job instead of attempting to reconstruct missing events.

The host sandbox is the isolation boundary. A force-stopped runner may not run
its own temporary-file cleanup. MBTX does not add persistence, container
deployment or guarantees beyond Codex's existing host confinement. M4 will
measure shell/MBTX outcomes with actual model tasks; M3 does not establish a
performance or model-quality improvement.

## Validation without a model account

```bash
cargo test --locked --manifest-path adapter/Cargo.toml
moon run scripts/m3_test.mbtx
```

The second command requires `cargo-nextest` and `just`. It uses Codex's
`core_test_support` mock Responses server, a compiled fake runner and the real
MoonBit runner. No model API key, GPU or model deployment is needed. Tests
exercise model-visible tool registration, shell coexistence, literal argv,
background controls, malformed output, script failures, host deadlines,
approval refusal, and real read-only sandbox enforcement.
Every integration test explicitly closes its Codex session. The MBTX test
profile uses two concurrent tests, disables retries, and treats child-process output handles still open
two seconds after test exit as a failure. Runner cancellation tests separately
assert that the OS process has exited.

CI also runs the M1/M2 Wasm/native suites, Rust format/Clippy checks, affected
upstream config/tool regression tests, builds the modified Codex executable and
uploads the Linux development binaries. The workflow can be started manually
from GitHub Actions. These are experimental development builds, not an official
OpenAI release.

The release tag's Cargo manifests carry `0.153.4`, while its checked-in lockfile
uses `0.0.0` for workspace packages. The integration patch records Cargo's
workspace-version normalization so subsequent builds can use `--locked`;
upstream third-party dependency selections are retained.
