# Linux Setup and Collection

Use a local build of the pinned Codex fork, not the globally installed Codex.
The scripts build Codex, its Code Mode host, the Responses proxy, MBTX and the
fixture once, then use those binaries throughout collection.

## Install

On a standard Linux host:

```bash
git pull --ff-only origin main
curl -fsSL https://cli.moonbitlang.com/install/unix.sh | bash
export PATH="$HOME/.moon/bin:$PATH"
moon update
moon run scripts/install-linux.mbtx
. "$HOME/.cargo/env"
```

The installer uses the distribution's standard packages, latest MoonBit and
stable Rust. Host-specific network, package-manager or sandbox restrictions
must be resolved locally. Codex's Rust dependencies are frozen in
`codex/Cargo.lock`. The V8 preparation script verifies the matching published
archive and binding checksums and caches them.

## Credentials

Keep the real key outside the repository and shell history:

```bash
read -rsp 'Relay API key: ' OPENAI_API_KEY
echo
export OPENAI_API_KEY
export MBTX_RELAY_BASE_URL='https://tokenadvent.com/v1'
export MBTX_MODEL='gpt-5.6-terra'
```

Alternatively, read an existing private JSON credential file:

```bash
chmod 600 /private/path/credentials.json
export OPENAI_API_KEY="$(jq -er '.OPENAI_API_KEY' /private/path/credentials.json)"
```

No interactive login or global `~/.codex/config.toml` is required. Each arm
receives an isolated HOME and generated TOML configuration. Codex uses a dummy
local credential; only the shared proxy receives the real key through stdin.
The generated configuration selects Responses, disables automatic request and
stream retries, and retains the workspace sandbox and approval path.

## Run

Build once, then use the printed bundle path:

```bash
moon run scripts/build-evaluation.mbtx codex
export MBTX_BUNDLE="$PWD/_build/bundles/<printed-key>"

# Short validation is a separate artifact, not part of formal data:
MBTX_PAIRS=1 MBTX_TASKS=argv_empty_unicode,cancel_sigkill moon run scripts/collect-linux-online.mbtx

# Full online protocol: 24 scenarios, eight valid pairs each, at most 288 attempts:
moon run scripts/collect-linux-online.mbtx

# Linux startup attribution: 1,000 pairs per workload and observation mode, no API:
moon run scripts/collect-linux.mbtx

# Real Codex fixed-response replay: ten pairs per scenario, no API:
moon run scripts/collect.mbtx codex-replay
```

Without `MBTX_BUNDLE`, collection resolves and verifies the cached bundle
automatically. macOS launcher collection uses `moon run scripts/collect-macos.mbtx`.
Use `MBTX_TOOL_MODE=direct` for a separate direct-tool artifact. The default is
Code Mode, with sequential tools within one program. `MBTX_DIRECT_SHELL=1`
creates a separate Direct Shell control; inspect actual `shell_modes` first.
If the default is already Direct, the default artifact is also the control.

## Request Budget

Every real outgoing API request, including tool continuations, passes through
one gate. Concurrency is one for the entire response stream. The default
`MBTX_MIN_INTERVAL_MS=15000` permits at most about four request starts per
minute. The minimum accepted interval is 6,000 ms. A 429 is recorded as an
external failure; subsequent requests respect `Retry-After` (seconds or HTTP
date), or wait 30 seconds if it is absent. Failed arms are not silently retried.

At two requests per arm, 192 pairs require about 768 requests and at least
3.2 hours at the default interval. Model latency, extra calls, cooldown and
supplementary attempts extend this estimate. Rate waiting is reported separately.
Other clients using the same account are outside this gate; stop those clients
or raise `MBTX_MIN_INTERVAL_MS` to reserve account RPM. No local setting can
guarantee the relay will never return 429.

## Inspect and Resume

The script prints the output directory and follow command:

```bash
"$MBTX_BUNDLE/mbtx-eval" log <RUN> --follow
"$MBTX_BUNDLE/mbtx-eval" report <RUN> --format all
MBTX_RESUME_RUN=<RUN> moon run scripts/collect-linux-online.mbtx
```

To include the detached descendant boundary in an offline replay, set
`MBTX_BOUNDARIES=1`; this never changes the online task quota.

Resume requires the same binaries, model, protocol, environment PATH and rate
configuration. Completed attempts are verified and reused. Interrupted attempts
remain censored; supplementary attempts use new directories. Reports can be
rebuilt without credentials. Repeated report generation writes a new revision,
preserving existing reports. `replay <RUN> --output <NEW-RUN>` uses the frozen
tool trajectory and binary hashes without contacting the relay.

Collection stops at five consecutive infrastructure failures, or eight in the
last sixteen arms, and still produces a partial report. The initial probe needs
one success out of three. Expected cancellation does not count as infrastructure
failure. `moon run scripts/diagnose-linux-online.mbtx <RUN>` records local
configuration and rebuilds a report without making API requests.

## Publish Evidence

Linux online results are stored in `evidence/linux/codex-relay/`; Linux/macOS
launcher and replay artifacts remain in their platform-specific directories.
Review the generated artifact, then publish only the intended run:

```bash
git add evidence/linux/codex-relay/<RUN-NAME>
git add evidence/linux/launcher/<RUN-NAME>
git commit -m "evidence: record Linux execution comparison"
git pull --rebase origin main
git push origin main
```

On macOS, `git pull --ff-only origin main` makes the raw evidence and offline
HTML/Markdown reports available to reviewers. Never add private credentials or
mutable Codex homes. Preserve the bytes of sealed artifacts, including PTY
CRLF sequences; do not translate captured commands or outputs. The
[current evaluation report](reports/linux-evidence-review-2026-09-14/README.md#evidence-integrity)
documents a known newline-conversion issue in the retained PTY artifacts.
Publishing a report does not establish that a fresh checkout passes its seals.

The [report index](reports/README.md) identifies the retained datasets and their
limits. Removed preliminary runs and superseded documents remain available
through Git history. New measurements use the [current protocol](evaluation.md)
and retain every failed attempt within their run.
