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

The isolated-home authentication path can be checked without a real API key or
provider by running the Rust integration test against a local mock relay:

```bash
MBTX_TEST_CODEX="$PWD/_build/codex-upstream/codex-rs/target/release/codex" \
  cargo test --locked --manifest-path adapter/Cargo.toml --test online_auth -- --ignored
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
