# Codex-MBTX

MBTX is a transparent process launcher for Codex: `mbtx exec -- COMMAND...`.
Codex still owns command resolution, approval and sandbox policy. MBTX launches
the resolved child, forwards cancellation, waits for its exit, and preserves its
observable exit status. It is neither a shell language parser nor a script runtime.
The default Codex backend remains Shell.

## Build and Check

Install the latest MoonBit CLI and stable Rust, then:

```bash
moon update
moon check --target native --deny-warn
moon test --target native
cargo test --locked --manifest-path adapter/Cargo.toml
moon run scripts/build-evaluation.mbtx launcher
```

The build command prints a verified bundle path. A matching bundle is reused
without starting a compiler. Immutable fixture, launcher, evaluator and, for the
`codex` bundle, Codex, Code Mode host and Responses proxy are built once. Source,
toolchain, dependency locks, platform and profile contribute to the cache key.

## Collect Evidence

```bash
# macOS local launcher comparison, no API:
moon run scripts/collect-macos.mbtx

# Linux local launcher comparison, no API:
moon run scripts/collect-linux.mbtx

# Real Codex with fixed local Responses, no API credentials:
moon run scripts/collect.mbtx codex-replay

# Linux only, after configuring OPENAI_API_KEY:
moon run scripts/collect-linux-online.mbtx
```

See [Linux setup and collection](docs/linux-online.md) for installation,
credentials, short validation, resume, rate limits and uploading evidence.
Online requests share one proxy: concurrency one, default 15 seconds between
request starts, with `Retry-After` cooldown after 429. Requests from other
applications using the same account are outside this gate.

The [evaluation protocol](docs/evaluation.md) defines 24 scenarios, 192 target
online pairs in two rounds, at most 288 attempted pairs, fixed offline replay,
and a separate 1,000-pair startup experiment for each workload and observation
mode. Failed attempts remain in intention-to-treat results. Short validation is
stored separately from formal samples.

```bash
<BUNDLE>/mbtx-eval log <RUN> --follow
<BUNDLE>/mbtx-eval report <RUN> --format all
<BUNDLE>/mbtx-eval replay <RUN> --output <NEW-RUN>
```

Reports include Markdown, JSON, CSV, offline HTML and standard trace JSON. They
separate external request time, rate waiting, Codex processing, launcher startup,
child execution and artifact writes. Missing measurements remain unknown. The
HTML links each pair to raw evidence and shows both timelines and first observed
differences. A startup advantage is a hypothesis until the paired measurements
and uncertainty interval support it; previous end-to-end results do not establish
a launcher startup benefit.

## Layout

| Path | Purpose |
|---|---|
| `launcher/`, `cmd/mbtx/` | Native launcher and command entry |
| `codex/` | Pinned upstream revision, dependency lock, integration patch and overlays |
| `evaluation/` | Shared MoonBit scenarios, oracles, classification and statistics |
| `cmd/evaluation-model/` | Prebuilt JSONL analysis worker |
| `adapter/` | Real OS, Codex and HTTP collection, CLI and report rendering |
| `scripts/` | Thin MoonBit automation and bundle preparation |
| `evidence/{macos,linux}/launcher/` | Platform-specific startup evidence |
| `evidence/{macos,linux}/codex-replay/` | Fixed-response Codex evidence |
| `evidence/linux/codex-relay/` | Linux real relay evidence |
| `docs/archive/` | Unmodified historical experiments and migration index |

The [architecture](docs/architecture.md) and [Codex integration](codex/README.md)
describe the ownership and configuration boundaries. Linux and macOS artifacts
are independent; results do not establish universal lossless replacement or
justify changing the default backend without reviewing the collected evidence.
The [code verification record](docs/code-validation.md) separates implementation
checks from the formal measurements that remain to be collected.
