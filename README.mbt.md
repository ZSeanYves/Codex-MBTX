# Codex-MBTX

Codex-MBTX is an experimental integration that keeps the Codex agent loop,
sessions, automation, and approval flow while exploring MBTX as the script
execution backend.

The project is intentionally being built in stages:

1. Define and test a standalone MBTX execution protocol.
2. Add process output, timeout, cancellation, and background job handling.
3. Connect the protocol to Codex through a replaceable execution backend.
4. Compare MBTX execution with the existing shell execution path.

M2 provides a standalone job runner with a JSONL interface. It accepts
inline MoonBit source or a `.mbtx` file, passes literal arguments and cwd, and
streams stdout and stderr, and reports exit code and elapsed time. Background
jobs support output polling, cancellation, and configurable deadlines. M3 adds an
opt-in MBTX tool to a pinned Codex build while retaining its shell backend,
execution approvals and sandbox. See [the adapter guide](docs/codex-adapter.md).

M4 adds a fixed shell/MBTX comparison suite with deterministic grading,
provider token accounting, backend-adherence checks, and a manual GitHub Actions
workflow. See [the evaluation guide](docs/m4-evaluation.md) for remote runs using
`OPENROUTER_ICU_API_KEY`; local validation needs no model credentials or GPU.
The [M4 delivery report](docs/reports/m4-2026-09-09.md) records all 24 pilot slots
across two segments: MBTX completed 12/12, shell 9/12, with three relay 502 failures.
Engineering checks pass; the live gates remain failed. MBTX remains opt-in.

Build the modified Codex and runner with:

```bash
moon run scripts/m3_build.mbtx
```

Run the Codex integration tests without a model account or GPU:

```bash
moon run scripts/m3_test.mbtx
```

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

Run the interactive job lifecycle demonstration with:

```bash
moon run scripts/jobs_smoke.mbtx
```

Foreground runs remain sequential; background runs allow subsequent runs to
start while controls remain responsive in both modes. The deadline defaults to
30 seconds and is configurable per request. Output is limited to 1 MiB and 4096
chunks per job. The runner compiles each job in a private directory and directly
manages its Wasm VM. It inherits the host environment and does not provide
workspace isolation, detached jobs, or interactive stdin. See
[`docs/protocol.md`](docs/protocol.md) for the exact contract and failure behavior.

## Development

This is a MoonBit module. Install the MoonBit toolchain, then run:

```bash
moon check
moon test
moon test --target native
moon run scripts/smoke.mbtx
moon run scripts/jobs_smoke.mbtx
moon run scripts/jobs_smoke.mbtx native
moon info
moon fmt
```

Tests include real script execution, live output, direct VM cancellation,
timeouts, UTF-8 chunk boundaries, output/history bounds, queue saturation,
transport failure cleanup, and EOF draining. CLI scripts exercise both M1
compatibility and M2 interactive controls. Tests require no model account,
API key, or GPU.

The `CI` GitHub Actions workflow runs Wasm and native jobs on Linux for pushes and
pull requests. It pins MoonBit to `0.10.11+6ff76a5f9` and records its
version in the log. Local development was validated with `moon 0.1.20260827`
and `moonc v0.10.11+6ff76a5f9`; `moonbitlang/async` is declared at `0.21.2`.

Agent-authored automation scripts use the `.mbtx` extension and are run with
MoonBit's script mode. See [`docs/architecture.md`](docs/architecture.md) for
the execution boundary and milestone plan.

## Repository

- Remote: <https://github.com/ZSeanYves/Codex-MBTX>
- License: Apache-2.0
