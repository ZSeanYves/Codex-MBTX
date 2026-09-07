# Codex-MBTX

Codex-MBTX is an experimental integration that keeps the Codex agent loop,
sessions, automation, and approval flow while exploring MBTX as the script
execution backend.

The project is intentionally being built in stages:

1. Define and test a standalone MBTX execution protocol.
2. Add process output, timeout, cancellation, and background job handling.
3. Connect the protocol to Codex through a replaceable execution backend.
4. Compare MBTX execution with the existing shell execution path.

M1 provides a standalone foreground runner with a JSONL interface. It accepts
inline MoonBit source or a `.mbtx` file, passes literal arguments and cwd, and
returns captured stdout, stderr, exit code, and elapsed time. Codex integration
is planned for M3.

## Run the runner

From the repository root, with `moon` available on `PATH`:

```bash
moon run --quiet cmd/mbtx < examples/requests.jsonl
```

The input file contains two independent requests. Output is one JSON event per
line. To run the example directly:

```bash
moon run examples/hello.mbtx MBTX
```

The runner processes requests sequentially. M1 buffers output until the child
exits and applies a fixed 30-second deadline and a 1 MiB combined output cap.
It uses the installed MoonBit toolchain and inherits the host environment; it
does not yet provide workspace isolation or interactive/background jobs. See
[`docs/protocol.md`](docs/protocol.md) for the exact contract and failure behavior.

## Development

This is a MoonBit module. Install the MoonBit toolchain, then run:

```bash
moon check
moon test
moon run scripts/smoke.mbtx
moon info
moon fmt
```

Unit tests include real `moon run` executions. The smoke script checks the
JSONL CLI, recovery after malformed input and compile failure, and a final
record without a newline. Tests require no model account, API key, or GPU.

The `CI` GitHub Actions workflow runs these checks on Linux for pushes and
pull requests. It installs the current stable MoonBit toolchain and records its
version in the log. Local development was validated with `moon 0.1.20260827`
and `moonc v0.10.11+6ff76a5f9`; `moonbitlang/async` is declared at `0.21.2`.

Agent-authored automation scripts use the `.mbtx` extension and are run with
MoonBit's script mode. See [`docs/architecture.md`](docs/architecture.md) for
the execution boundary and milestone plan.

## Repository

- Remote: <https://github.com/ZSeanYves/Codex-MBTX>
- License: Apache-2.0
