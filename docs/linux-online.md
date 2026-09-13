# Linux online collection

This workflow builds the pinned Codex CLI and Code Mode host, the MBTX launcher, and the
Rust collector once, then runs sequential Shell/Transparent pairs against the
configured OpenAI-compatible Responses relay.

The repository uses the standard stable Rust toolchain. The installer and CI
track `stable` and record the actual Cargo version in `provenance.txt`.
Codex dependencies are frozen in `codex/Cargo.lock`.

On a clean Linux host, install the MoonBit CLI once, then make it available in
the current shell:

```bash
curl -fsSL https://cli.moonbitlang.com/install/unix.sh | bash
export PATH="$HOME/.moon/bin:$PATH"
```

From a clean checkout on Linux:

```bash
git pull --ff-only origin main
moon update
moon run scripts/install-linux.mbtx
. "$HOME/.cargo/env"

export OPENAI_API_KEY='sk-...'
export MBTX_RELAY_BASE_URL='https://tokenadvent.com/v1'
export MBTX_MODEL='gpt-5.6-terra'
moon run scripts/collect-linux-online.mbtx
```

`moon update` refreshes the package registry index after installing MoonBit.
Standalone `.mbtx` scripts resolve their own dependencies, independently of
the root `moon.mod`. Their imports explicitly select `moonbitlang/async@0.21.3`,
matching the launcher module, so an older registry default cannot select a
release without the required `shell` package. CI builds every script without
running installation or online collection.

For the JSON credential form, store it outside the checkout with mode `600`
and export it without copying the key into the repository:

```bash
chmod 600 /private/path/credentials.json
export OPENAI_API_KEY="$(jq -r '.OPENAI_API_KEY' /private/path/credentials.json)"
```

The default run target is `evidence/linux/codex-relay/<timestamp>`. Set
`MBTX_PAIRS=1` for a short smoke, or leave the default `4` for 32 planned
pairs (64 arms). The collector keeps external relay/provider errors and stops
only at the registered infrastructure thresholds. It writes `events.jsonl`,
immutable per-attempt evidence, `online-summary.md`, `summary.json`,
`provenance.txt`, and Markdown/JSON/CSV/HTML reports.

Arms are executed by one sequential loop, with a 6-second pause before the next
arm. `MBTX_MIN_INTERVAL_MS` controls this pause. It limits arm starts, not API
requests: Codex normally requests a tool call and then requests a final response,
and may make more requests within an arm. Sequential collection cannot prevent
an account-wide RPM limit or requests from other clients. Use `0` only with a
local mock relay.

Each Codex arm has a 300-second deadline. The outer collection script has no
one-hour total deadline; all 64 arms can finish. `summary.json` and
`online-summary.md` are atomically checkpointed after every arm. A stopped run
can be reported from its raw evidence without contacting the relay:

```bash
cargo run --locked --manifest-path adapter/Cargo.toml --bin mbtx-eval -- \
  report evidence/linux/codex-relay/<timestamp> --format md
```

The collector binds `model_providers.OpenrouterICU.env_key` to `OPENAI_API_KEY`
in every isolated Codex home. A saved interactive Codex login is not required.
Both arms use `features.unified_exec = true`, `features.code_mode_host = true`,
and `features.plugins = false`; plugin startup sync is unrelated to the launcher
comparison and would add an uncontrolled GitHub dependency to every arm.
The pinned `gpt-5.6-terra` metadata selects `code_mode_only`; the model invokes
`exec_command` through Code Mode. Both Codex binaries are produced by one Cargo
build, and their hashes are retained in provenance.

Before that Cargo build, `scripts/prepare-codex-v8.mbtx` resolves the `v8`
version from the pinned lockfile, downloads the matching Codex-published
sandbox archive and Rust binding, and verifies both against the release
checksum manifest. The verified pair is cached under `_build/codex-v8/` and
reused on later runs; it is never mixed across targets or crate versions.

Every arm prints its result status, failure class, and observed HTTP error status
(`unknown` when absent). A `401` is a provider authentication failure even when
the message includes a relay URL. `partial` means the planned comparison did not
complete; writing a report successfully is not a successful experiment.
Keep a failed run intact and start a new timestamped run after fixing its cause.

`status` describes the complete Codex outcome. Success requires both a successful
command oracle and `turn.completed` with Codex exit code 0. `command_outcome`
records the command oracle separately: an expected command exit of 7 or 143 can
pass, while a subsequent API request timeout makes the Codex outcome
`relay_error/relay_request_timeout`. A collector deadline remains
`timeout/codex_timeout`, with its cause unknown unless additional evidence exists.
Missing usage remains `null`. Multiple completed commands are a harness protocol
failure, rather than selecting the last command as a successful sample.

Reports count each arm once, even though it has both Codex and child exit events.
`command_oracle_pairs` preserves useful command results from incomplete turns;
`valid_comparable_pairs` requires complete successful Codex outcomes on both arms.
The oracle currently checks the expected exit, output marker, and forbidden side
effect. It is not a proof of byte-exact output, full process cleanup, or recovery.
Codex elapsed time includes model/relay time and cannot identify launcher cost.

The report command reclassifies online results from the saved Codex JSONL,
stderr, and attempt metadata with the same classifier as live collection.
It does not modify the run or consult API credentials. `recorded_status` retains
the original collector classification, and `classification_source` identifies
the revised interpretation. Original reports from older commits remain historical.
See the [Linux smoke review](../evidence/linux/codex-relay/20260913T141847943929204/review/README.md).

If the relay returns a new error, run the read-only diagnostic bundle on Linux:

```bash
moon run scripts/diagnose-linux-online.mbtx diagnostics/linux-online \
  evidence/linux/codex-relay/<failed-run>
```

It records versions, artifact hashes, V8 checksum status, and matching error
lines from the failed run without sending a model request. To make one optional
authenticated `GET /models` probe, set `MBTX_DIAGNOSTIC_PROBE=1`; the response
status and body are recorded, never the key itself.

The isolated-home authentication path can be checked without a real API key or
provider by running the Rust integration test against a local mock relay:

```bash
MBTX_TEST_CODEX="$PWD/_build/codex-upstream/codex-rs/target/release/codex" \
MBTX_TEST_LAUNCHER="$PWD/_build/native/release/build/cmd/mbtx/mbtx.exe" \
  cargo test --locked --manifest-path adapter/Cargo.toml --test online_auth -- --ignored --test-threads=1
```

The pinned upstream lock has stale workspace version markers. The repository's
`codex/Cargo.lock` corrects those markers while preserving all upstream external
dependencies, including the matching Rama `0.3.0-alpha.4` packages. Preparation
installs this lock atomically, backing up a differing existing lock alongside
it as `Cargo.lock.before-mbtx-<git-blob-hash>`. Repeated preparation leaves an
identical lock untouched. This also repairs locks created by older collectors.

The collector never runs `cargo generate-lockfile` or updates dependencies.
Builds use `--locked` and stream their output to the terminal. Each run retains
a copy of `Cargo.lock`; the successful build's lock hash is also recorded in
`provenance.txt`. Download failures stop preparation or the build without
changing the pinned dependency graph.

The collector creates separate `CODEX_HOME` and workspace directories for
every arm. It uses `approval_policy = "never"` for unattended collection and
`[sandbox_workspace_write] network_access = true` so the relay-enabled run does
not turn child network policy into a hidden variable. The local templates are
in `config/`; the run-local copies are retained under the run directory.

After reviewing the raw attempt stderr and reports, publish the run as data:

```bash
git add evidence/linux/codex-relay/<timestamp>
git commit -m "data: add Linux Codex relay collection"
git push origin main
```

On macOS, `git pull --ff-only origin main` makes the Linux run available for
inspection. macOS local launcher evidence remains under
`evidence/macos/launcher/` and is never mixed into the Linux online report.
