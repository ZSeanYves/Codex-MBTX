# Linux online smoke review

Reviewed on 2026-09-13. Source data: commit
`a533d04`, collected with implementation
`424f4a5ac3920fa26898df7dc11fa903bed8bc57` on Linux x86_64.
The original attempt directories, events, summary, and reports are unchanged.
This directory contains a revised interpretation, not new online samples.

The smoke reached the API and exercised both execution backends. No command
oracle failure was observed. The collector and report had classification and
counting defects that need the accompanying fixes before full collection.

| Measure | Corrected result |
|---|---:|
| Planned / observed arms | 16 / 16 |
| Commands passing the recorded exit, marker, and side-effect oracle | 13 |
| Pairs with both command oracles passing | 5 / 8 |
| Complete successful Codex outcomes | 9 |
| Pairs with both Codex outcomes complete | 1 / 8 |
| External errors | 6 |
| Collector deadline, underlying cause unknown | 1 |
| Observed command oracle failures | 0 |

Four commands passed but their Codex turns subsequently failed with
`request timed out`: Shell literal argv, Shell stdin/UTF-8, Shell nonzero exit,
and Transparent SIGTERM. The original summary incorrectly called these complete
successes. The old report additionally counted Codex and child exit events as
separate arms, producing 29 observed arms and 26 successes for 16 actual arms.

| Task | Shell command / Codex outcome | Transparent command / Codex outcome |
|---|---|---|
| literal_argv | pass / API request timeout | pass / complete |
| stdin_utf8 | pass / API request timeout | pass / complete |
| cwd_environment | pass / complete | pass / complete |
| large_output | pass / complete | unobserved / overloaded stream disconnected |
| nonzero_exit | pass, exit 7 / API request timeout | pass, exit 7 / complete |
| background_cleanup | pass / complete | unobserved / API request timeout |
| signal_exit | pass, exit 143 / complete | pass, exit 143 / API request timeout |
| repeated_recovery | pass / complete | unobserved / collector deadline at 300 seconds |

The five `request timed out` failures are attributed to the external request
path; these logs do not distinguish the relay from the upstream provider.
The overload response is also external. The final deadline has only
`thread.started` and `turn.started`, with no observed command event or terminal
provider response. It must remain a timeout of unknown underlying cause.
No 401, 429, missing Code Mode host, or launcher build error appears in this run.

## Fixes Before Full Collection

- Require Codex exit 0 and `turn.completed` for complete success, while retaining
  `command_outcome` separately when a later request fails.
- Classify `request timed out` as `relay_error/relay_request_timeout`; retain
  collector deadlines as `timeout/codex_timeout` without inventing an HTTP code.
- Count each arm once in both the pair comparison and intention-to-treat totals.
- Rebuild reports from saved JSONL, stderr, and metadata with the shared live
  classifier. Preserve original classifications in `recorded_status`.
- Remove the outer script's one-hour collection deadline. Keep per-arm deadlines
  and atomically checkpoint summaries after every completed arm.

These fixes allow full collection to proceed with the existing infrastructure
stop rules. A full run plans 32 pairs; it may still be partial if either arm
fails. It does not guarantee 32 complete pairs or repair external availability.

## Scope of the Evidence

This is useful smoke evidence for the Linux launch path and failure reporting.
It does not establish a performance advantage or universal lossless replacement.
Codex elapsed time includes model and relay delays; it cannot isolate launcher
or child runtime. Default Shell and Direct Shell are not separate online
cohorts in this run. macOS launcher evidence is independent.

The online oracle checks expected exit, an output marker, and a forbidden file.
The background case waits for a short-lived child; the recovery case prints a
marker. They do not establish process-group cancellation, orphan cleanup,
SIGKILL handling, repeated recovery, or leak freedom. Those require dedicated
deterministic evidence. Command strings also remain visible for inspection;
for example, the model rewrote a printf newline escape in one cwd arm, despite
the exact-command prompt. The observed cwd output is correct, but a prompt alone
does not enforce byte-identical tool inputs.

## Artifacts

- [Corrected Markdown report](report.md)
- [Corrected JSON report](report.json)
- [Corrected CSV report](report.csv)
- [Corrected offline HTML report](report.html)
- [Source evidence SHA-256 manifest](source.sha256)
- [Analysis source SHA-256 manifest](analysis.sha256)
- [Original event log](../events.jsonl)
- [Original summary](../summary.json)
- [Original provenance](../provenance.txt)

Rebuild a read-only report from the repository root:

```bash
cargo run --locked --manifest-path adapter/Cargo.toml --bin mbtx-eval -- \
  report evidence/linux/codex-relay/20260913T141847943929204 --format md
```

The regression suite includes this immutable Linux smoke, request timeouts after
successful commands, expected nonzero/signal exits, and missing terminal evidence.
Real Codex and MBTX tests use a deterministic local Responses server for success,
authentication failure, and a stream failure after command execution. They do
not contact the production relay.

Local validation on macOS passed: 17 Rust unit/replay checks, all three real
Codex/local-relay integration tests (167.50 seconds total), MoonBit native
check/test, interface/format checks, and compilation of the Linux entry script.
The Linux production relay was not rerun during this review. All 134 files in
the local macOS launcher run retained their pre-pull SHA-256 values.
