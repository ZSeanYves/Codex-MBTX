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
- `process/`: invokes `moon run` without a shell and captures output.
- `protocol/`: JSONL requests and execution events.
- `cmd/mbtx/`: reads JSONL requests from stdin and writes events to stdout.
- `examples/`: small runnable `.mbtx` scripts.
- `scripts/`: `.mbtx` automation, including the CLI smoke test.
- `policy/` (planned): workspace, environment, executable, and argument restrictions.
- `eval/` (planned): comparative evaluation tasks.
- `codex/`: a pinned Codex source tree, added after the standalone protocol is
  stable.

Dependencies flow from the command entry point into protocol and process, which
both depend on runtime. The MBTX packages do not depend on the Codex model client.

## Initial protocol

M1 uses one JSON object per line and a caller-provided correlation ID. Requests
contain `op: "run"`, exactly one of `source` or `script_path`, optional literal
`args`, and optional `cwd`. The request does not select a different executable
or backend. The runner invokes `moon run --quiet --target wasm --` with separate
argv elements; inline source is written to the child's stdin.

The deadline is fixed at 30 seconds and aggregate captured output at 1 MiB.
`started` is emitted when a validated request is accepted; stdout and stderr
are buffered and emitted before `completed`. Nonzero toolchain exits preserve
their code and diagnostics. M1 does not infer compile/runtime error categories
from diagnostic strings. See [the protocol](protocol.md) for details.

M2 will add job IDs, configurable lifecycle controls, cancellation, and live
output. Host sandboxing must remain in the Codex adapter: the standalone runner
currently inherits the environment and has no workspace isolation.

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

### M2: job lifecycle

- Add timeouts, cancellation, background jobs, and job IDs.
- Emit streaming JSONL events.
- Test the complete lifecycle without a model API.

### M3: Codex adapter

- Add a replaceable MBTX execution backend in Codex.
- Keep the existing shell backend and route the new `mbtx` tool through the
  adapter.
- Test the tool call with a mock model and a fake runner.

### M4: evaluation

- Run the same task set through shell and MBTX execution.
- Record success, latency, output correctness, token use, intervention, and
  failure category.
- Use the baseline to decide whether MBTX should become the default backend.

## Validation

Pull requests should run MoonBit checks, MoonBit tests, Rust formatting and
targeted integration tests. Live model evaluations should be manually
triggered in CI and use repository secrets; they must not be required for a
normal pull request.
