# Codex-MBTX Architecture

## Goal

Preserve Codex's model orchestration, agent loop, sessions, automation,
approval flow, and event handling while adding MBTX as an execution backend.

The existing shell execution path remains available. The experiment has two
separate MBTX modes: an explicit `mbtx` tool for MoonBit-script workflows, and
the primary transparent mode, which keeps Codex's existing `exec_command`
contract and changes only the post-approval process launcher. Keeping these
modes separate prevents a model's MoonBit knowledge from being confused with
the effect of replacing the host executor.

## Execution boundary

```text
Model
  -> Codex agent loop
  -> exec_command (unchanged model-visible contract)
  -> policy, approval, and sandbox planning
  -> MBTX `exec --` launcher (transparent mode)
  -> original resolved argv
  -> Codex tool result and context

Explicit compatibility path:

Model -> mbtx tool -> Rust adapter -> MBTX JSONL runner -> tool result
```

The MBTX runner owns MoonBit script compilation and script-job lifecycle.
Codex remains the authority for model interaction, workspace selection,
approvals, and the host sandbox. In transparent mode MBTX does not receive
MoonBit source: it validates the trusted launcher, attaches the caller's
standard streams, and forwards the already-resolved literal argv to the child.
The runner must not become an alternate model client or agent loop.

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
- Keep the existing shell backend and route both explicit and transparent
  modes through the unified executor.
- Test the tool call with a mock model and a fake runner.

The opt-in `mbtx_command` configuration selects the trusted MBTX executable.
With `mbtx_backend = "transparent"`, the Rust adapter leaves the model's
command, policy input, approval display, events, streams, and exit status
unchanged, then prefixes the final launch argv with `mbtx exec --`. No
generated MoonBit source or MBTX compilation occurs on this path. Without the
backend selector, `mbtx_command` exposes the explicit `mbtx` function; that
path accepts MoonBit source or `.mbtx` files and therefore requires the caller
to know the MBTX protocol and MoonBit syntax. See [the adapter guide](codex-adapter.md)
for the configuration boundary and validation.

### M4: evaluation

- Run the same task set through shell and transparent execution with identical
  model-visible prompts and tools.
- Record success, latency, output correctness, token use, intervention, and
  failure category.
- Keep explicit-MBTX results as a separate compatibility cohort; do not use
  them as evidence for transparent replacement.
- Use the direct comparison to decide whether transparent MBTX should become
  a default backend.

The [M4 evaluation guide](m4-evaluation.md) defines the fixed task suite,
same-prompt shell/transparent comparison, deterministic grading, sanitized
evidence and manual live workflow. Transparent MBTX remains opt-in until
measurements justify changing the default.

## Validation

CI runs MoonBit checks and tests for Wasm and native runners on Linux, the M1
compatibility smoke script, and the M2 CLI lifecycle script. Generated interfaces
and formatting are checked for drift. Rust contract and mock-model integration
tests cover the Codex adapter with fake and real runners. Live model evaluations will be manually triggered
and use repository secrets; they must not be required for a normal pull request.
