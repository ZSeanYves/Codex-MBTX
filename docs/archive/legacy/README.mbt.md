# Codex-MBTX

Codex-MBTX is an experimental integration that keeps the Codex agent loop,
sessions, automation, and approval flow while exploring MBTX as the script
execution backend.

The project is intentionally being built in stages:

1. Define and test a standalone MBTX execution protocol.
2. Add process output, timeout, cancellation, and background job handling.
3. Connect the protocol to Codex through a replaceable execution backend.
4. Compare transparent MBTX launching with the existing shell execution path.

M2 provides a standalone job runner with a JSONL interface. It accepts
inline MoonBit source or a `.mbtx` file, passes literal arguments and cwd, and
streams stdout and stderr, and reports exit code and elapsed time. Background
jobs support output polling, cancellation, and configurable deadlines. M3 adds
two opt-in modes to a pinned Codex build while retaining its shell backend,
execution approvals and sandbox. The explicit `mbtx` tool runs MoonBit scripts;
transparent mode keeps the normal `exec_command` interface and inserts MBTX
only at final process launch. See [the adapter guide](docs/codex-adapter.md).

M4 adds a fixed shell/transparent comparison suite with deterministic grading,
provider token accounting, backend-adherence checks, and a manual GitHub Actions
workflow. The suite separates six host-process tasks from six deliberate
MoonBit-script tasks; `backend` measures the former alone and `full` runs both.
See [the evaluation guide](docs/m4-evaluation.md) for remote runs using
`OPENROUTER_ICU_API_KEY`; local validation needs no model credentials or GPU.
The earlier [M4 delivery report](docs/reports/m4-2026-09-09.md) is an explicit-tool
cohort and is not evidence for transparent replacement. The first
[transparent comparison report](docs/reports/m4-transparent-2026-09-09.md) is a
complete 24-row observation set, but relay overload prevented a green evidence
gate, so it does not establish a performance or reliability advantage.
The process-only [backend report](docs/reports/m4-backend-2026-09-09.md) has
the expanded 24-row cohort and the same evidence boundary.
Transparent MBTX remains opt-in.

M5 replaces the exploratory online comparison with a versioned evidence
pipeline. It keeps B0 shell, B1 transparent, and B2 bare-proxy separate,
records relay health and three-state observability, and reports both
intention-to-treat and relay-clean paired views. The process cohort is limited
to host execution semantics; MoonBit script capability is reported separately.
Run the credential-free acceptance path with:

```bash
moon run scripts/m5_verify.mbtx
```

已保存的 relay probe 可以在离线环境重建健康门禁：

```bash
moon run scripts/m5_probe.mbtx < probes.json
```

The protocol and default-backend gates are frozen in
[`docs/m5-evaluation.md`](docs/m5-evaluation.md). The current
[M5 decision report](docs/reports/m5-decision-2026-09-12.md) remains
`INCONCLUSIVE`: the controlled pilot, Linux/macOS deterministic suite, replay,
and formal W1 are valid, but three W2 collection attempts were invalidated by
relay/provider failures, so W3 could not legally start. The evidence supports
keeping Transparent MBTX opt-in; it does not support making it the default.
`M5_START_BLOCK` is
reserved for resumptions; a continuation must be merged at a complete block
boundary before analysis. Raw runner stdout/stderr/exit evidence is saved per
block with a unique retry suffix, so a failed continuation cannot overwrite an
earlier artifact.
The controlled workflow derives formal continuations as three independent
7/7/6 repetition windows (56/56/48 paired blocks) and requires explicit pilot,
deterministic, and previous-window run IDs. Exact dispatch commands and frozen
toolchain versions are part of the preregistered protocol.
The final redacted evidence index is stored under
[`docs/reports/m5-2026-09-12/`](docs/reports/m5-2026-09-12/README.md); the
earlier implementation-only snapshot remains unchanged under
[`docs/reports/m5-2026-09-10/`](docs/reports/m5-2026-09-10/README.md).
Formal online runs also require `scripts/m5_platforms.mbtx` to validate the
saved Linux and macOS runtime/replay artifacts; a declared platform list alone
cannot satisfy the cross-platform gate.

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
pull requests. It records the current MoonBit toolchain in the log. M5 was
validated with `moon 0.1.20260904`, `moonc v0.10.12+1634b282e`, and
`moonbitlang/async@0.21.3`.

Agent-authored automation scripts use the `.mbtx` extension and are run with
MoonBit's script mode. See [`docs/architecture.md`](docs/architecture.md) for
the execution boundary and milestone plan.

## Repository

- Remote: <https://github.com/ZSeanYves/Codex-MBTX>
- License: Apache-2.0
