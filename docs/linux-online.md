# Linux online collection

This workflow builds the pinned Codex checkout, the MBTX launcher, and the
Rust collector once, then runs sequential Shell/Transparent pairs against the
configured OpenAI-compatible Responses relay.

From a clean checkout on Linux:

```bash
git pull --ff-only origin main
moon run scripts/install-linux.mbtx
. "$HOME/.cargo/env"

export OPENAI_API_KEY='sk-...'
export MBTX_RELAY_BASE_URL='https://tokenadvent.com/v1'
export MBTX_MODEL='gpt-5.6-terra'
moon run scripts/collect-linux-online.mbtx
```

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

The pinned upstream workspace currently carries stale internal package version
markers in its checked-in lock. The collector detects this before the first
build and refreshes that lock once; all subsequent builds use `--locked` and
the resulting lock hash is recorded in `provenance.txt`.

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
