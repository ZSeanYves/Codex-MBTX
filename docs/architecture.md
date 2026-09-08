# Codex-MBTX Architecture

## Goal

Preserve Codex's model orchestration, agent loop, sessions, automation,
approval flow, and event handling while adding MBTX as a script execution
backend.

The first integration must keep the existing shell execution path available.
This makes the two execution paths comparable and gives the experiment a
reliable fallback.

## Execution boundary

```text
Model
  -> Codex agent loop
  -> mbtx tool
  -> Rust MBTX adapter
  -> MBTX runner
  -> JSONL execution events
  -> Codex tool result and context
```

The MBTX runner owns script execution and job lifecycle. Codex remains the
authority for model interaction, workspace selection, approvals, and the host
sandbox. The runner must not become an alternate model client or agent loop.

## Packages

- `runtime/`: validated execution requests, results, and event types.
- `process/`: compiles to private artifacts and manages streaming, cancellable
  `moonrun` processes through `moonbitlang/async`.
- `protocol/`: JSONL requests and execution events.
- `jobs/`: bounded job registry, lifecycle state, cancellation, and output history.
- `session/`: bounded JSONL framing, foreground/background scheduling, responsive
  control dispatch, and a single output writer.
- `cmd/mbtx/`: connects the session to stdin and stdout.
- `examples/`: small runnable `.mbtx` scripts.
- `scripts/`: `.mbtx` automation, including the CLI smoke test.
- `policy/` (planned): workspace, environment, executable, and argument restrictions.
- `eval/`: fixed comparative tasks, output grading and provider/tool evidence audit.
- `cmd/m4-evidence/`: local JSON interface to the evaluator for `.mbtx` automation.
- `codex/`: upstream revision, reproducible integration patch, and Rust adapter
  source overlay. The prepared Codex source lives in `_build/codex-upstream`.
- `adapter/`: standalone Rust contract tests and the fake-runner test binary;
  its library target compiles the same protocol source used inside Codex.

Dependencies flow from `cmd/mbtx` to `session`, then to `protocol` and `jobs`.
Jobs use `process`; shared contracts live in `runtime`. None of these packages
depend on the Codex model client.

## Execution protocol

The protocol uses one JSON object per line and a caller-provided correlation ID. Requests
contain `op: "run"`, exactly one of `source` or `script_path`, optional literal
`args`, and optional `cwd`. M2 adds `background` and `timeout_ms` plus `job_output`
and `job_stop` control requests. Each admitted run receives a session-local job ID.
The request does not select a different executable or backend.

Each job compiles with `moon run --build-only --output-json --target wasm` in its
own temporary build directory, then runs the artifact through a directly managed
`moonrun` child. The default deadline is 30 seconds, configurable up to 10 minutes.
Stdout and stderr stream in bounded chunks with per-job sequence numbers.
Readers and child processes finish cleanup before the terminal event is emitted.
Nonzero toolchain exits preserve their code and diagnostics. Compilation/runtime
error categories are not inferred from diagnostic strings.

The session limits active jobs, queued requests, output history, and protocol
frames. Foreground runs serialize later runs while controls remain responsive;
background runs release the run queue. EOF drains accepted work, and a transport
failure cancels the session's structured task group. See [the protocol](protocol.md)
for state transitions, retention, output cursors, and failure behavior.

Host sandboxing remains the Codex adapter's responsibility. Direct VM cancellation
does not confine arbitrary descendants; the standalone runner inherits the host
environment and has no workspace isolation.

## Milestones

### M0: repository baseline (complete)

- Normalize the MoonBit module and Git remote names.
- Keep the repository buildable with `moon check` and `moon test`.
- Record the execution boundary and protocol decisions in this document.

### M1: standalone MBTX runner (implemented)

- Run a foreground script.
- Pass arguments and working directory explicitly.
- Capture stdout, stderr, and exit code.
- Add deterministic tests for success, failure, malformed input, and argument
  preservation.

### M2: job lifecycle (implemented)

- Add timeouts, cancellation, background jobs, and job IDs.
- Emit streaming JSONL events.
- Test the complete lifecycle without a model API.

Acceptance is covered by the process, jobs, protocol, and session test suites,
plus `scripts/jobs_smoke.mbtx` against the actual CLI on Wasm and native backends.

### M3: Codex adapter (implemented)

- Add a replaceable MBTX execution backend in Codex.
- Keep the existing shell backend and route the new `mbtx` tool through the
  adapter.
- Test the tool call with a mock model and a fake runner.

The opt-in `mbtx_command` configuration exposes a new `mbtx` function. The Rust
adapter passes the complete request as literal argv to Codex's existing unified
executor, retaining its approval, sandbox, streaming and process-lifetime
handling. Shell tools remain available. Each run uses the runner's
`--request JSON` entry point; adapter background jobs use Codex process ownership
and opaque session-scoped IDs. See [the adapter guide](codex-adapter.md) for the
configuration boundary, polling semantics, reproducible build and validation.

### M4: evaluation

- Run the same task set through shell and MBTX execution.
- Record success, latency, output correctness, token use, intervention, and
  failure category.
- Use the baseline to decide whether MBTX should become the default backend.

The [M4 evaluation guide](m4-evaluation.md) defines the fixed task suite,
instruction-routed shell/MBTX comparison, deterministic grading, sanitized
evidence and manual live workflow. MBTX remains opt-in until measurements
justify changing the default.

## Validation

CI runs MoonBit checks and tests for Wasm and native runners on Linux, the M1
compatibility smoke script, and the M2 CLI lifecycle script. Generated interfaces
and formatting are checked for drift. Rust contract and mock-model integration
tests cover the Codex adapter with fake and real runners. Live model evaluations will be manually triggered
and use repository secrets; they must not be required for a normal pull request.
