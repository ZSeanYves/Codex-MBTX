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

## Proposed packages

- `runtime/`: typed execution requests, results, job states, and errors.
- `process/`: process startup, argument passing, output capture, timeout, and
  cancellation.
- `protocol/`: JSONL requests and execution events.
- `policy/`: workspace, environment, executable, and argument restrictions.
- `cmd/mbtx/`: the executable entry point once the runner is implemented.
- `examples/`: small runnable `.mbtx` scripts.
- `eval/`: deterministic fixtures and comparative evaluation tasks.
- `codex/`: a pinned Codex source tree, added after the standalone protocol is
  stable.

Dependencies should flow from the command entry point into protocol, policy,
process, and runtime packages. The MBTX packages should not depend on the
Codex model client.

## Initial protocol

The first protocol should use one JSON object per line. A run request contains
the script source or script path, literal arguments, workspace-relative working
directory, timeout, and foreground/background mode.

Execution events should include a job ID and use explicit event types such as
`started`, `stdout`, `stderr`, `completed`, `failed`, and `stopped`. Completion
must report the exit code and duration. Errors must be structured so Codex can
distinguish a rejected request, a compile error, a timeout, a cancellation, and
an execution failure.

The first implementation should not translate arbitrary shell strings into
MBTX. It should expose a separate `mbtx` tool with a clear script contract and
preserve literal argument boundaries.

## Milestones

### M0: repository baseline

- Normalize the MoonBit module and Git remote names.
- Keep the repository buildable with `moon check` and `moon test`.
- Record the execution boundary and protocol decisions in this document.

### M1: standalone MBTX runner

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
