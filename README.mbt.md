# Codex-MBTX

Codex-MBTX is an experimental integration that keeps the Codex agent loop,
sessions, automation, and approval flow while exploring MBTX as the script
execution backend.

The project is intentionally being built in stages:

1. Define and test a standalone MBTX execution protocol.
2. Add process output, timeout, cancellation, and background job handling.
3. Connect the protocol to Codex through a replaceable execution backend.
4. Compare MBTX execution with the existing shell execution path.

The current repository contains the project baseline and architecture notes;
the executor and Codex adapter will be added in later milestones.

## Development

This is a MoonBit module. Install the MoonBit toolchain, then run:

```bash
moon check
moon test
```

Agent-authored automation scripts use the `.mbtx` extension and are run with
MoonBit's script mode. See [`docs/architecture.md`](docs/architecture.md) for
the execution boundary and milestone plan.

## Repository

- Remote: <https://github.com/ZSeanYves/Codex-MBTX>
- License: Apache-2.0
