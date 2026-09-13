# Linux Online Comparison: Evidence and Decision

Source run: `20260913T152436165546119`, published in `6da3b57`.
Implementation: `c3479b685a120905e065ea3057cd56d97c959aae`.
Platform: Linux x86_64, kernel `6.14.0-28-generic`.
Model configuration: `gpt-5.6-terra`, `xhigh`, OpenrouterICU Responses relay.

## Supported Conclusion

Under this configuration, transparent MBTX completed all 32 online task
attempts, as did the Shell control. All eight task classes have four complete
pairs. The collected exit codes, fixed output contents, cwd/environment outputs,
and large-output character counts satisfy the tested contracts. This provides
real evidence of basic execution compatibility and operational feasibility.

The experiment does not establish universal lossless replacement, a speed
advantage, or improved reliability over Shell. One pair has a merged-output
ordering difference. Its cause remains unknown, and the current collector
does not measure the lifecycle and timing segments needed for full attribution.

## Raw Evidence Audit

The independent [audit](audit.mbtx) reads all 64 saved Codex JSONL streams and
attempt metadata, with no model requests or new task executions. It verifies:

- Exactly one completed command and one completed turn per arm, no error event,
  and Codex exit code 0 in every attempt.
- All 48 ordinary command exits are 0; the eight nonzero-exit commands return 7;
  the eight SIGTERM commands return 143.
- Full fixed output strings for the small tasks, the exact per-arm workspace
  and environment marker, and 65,536 `x` plus 4,096 `e` characters in each
  large-output result. This is stronger than the collector's marker check.
- Every saved home matches its backend's base configuration plus the same
  trusted-project record added to the configuration. No arm-specific model or
  reasoning-setting change was found.
- The report rebuilt from raw evidence equals the report uploaded from Linux.
  Source files are retained unchanged and listed in [source.sha256](source.sha256).

| Measure | Shell | Transparent MBTX |
|---|---:|---:|
| Attempted arms | 32 | 32 |
| Complete Codex outcomes | 32 | 32 |
| Command oracle passes | 32 | 32 |
| Observed relay/provider/harness errors | 0 | 0 |
| Observed timeouts | 0 | 0 |
| Complete pairs | 32 total | 32 total |

AB/BA ordering is balanced: 16 pairs start with Shell and 16 with MBTX.
The observer's collection window was 1,641.798 seconds (27.36 minutes), including
the inter-arm pauses and excluding the earlier build step. There are no failed
arms to omit within this run, so its intention-to-treat and complete-pair counts
agree. Earlier failed runs remain separate evidence of external availability;
this successful window does not erase them or establish long-term reliability.

## Observed Differences

In `pair-01-large_output`, the saved command and exit code are identical, but
the merged output differs from byte 0:

| Backend | First block | Second block |
|---|---|---|
| Shell | 65,536 `x` from the stdout-writing expression | 4,096 `e` from the stderr-writing expression |
| MBTX | 4,096 `e` from the stderr-writing expression | 65,536 `x` from the stdout-writing expression |

Both results retain every expected character. The evidence records a merged
string, not individually timestamped reads from the two streams. It therefore
cannot identify whether the order changed in process scheduling, Codex stream
collection, or the launcher. This is an observed difference with unknown
attribution, not a demonstrated MBTX corruption bug. Calling all observable
outputs byte-identical would be incorrect.

The other merged outputs match between arms after normalizing the deliberately
different workspace paths. In `pair-00-cwd_environment`, the model changed a
literal printf newline escape into an actual newline in one command. The
result is correct, but this pair does not have byte-identical command text.
The other 31 pairs have identical displayed commands. The original prompt
does not itself enforce identical tool arguments.

Raw sources: [Shell large output](../attempts/pair-01-large_output/shell/codex.stdout.jsonl),
[MBTX large output](../attempts/pair-01-large_output/transparent/codex.stdout.jsonl),
[Shell cwd command](../attempts/pair-00-cwd_environment/shell/codex.stdout.jsonl),
[MBTX cwd command](../attempts/pair-00-cwd_environment/transparent/codex.stdout.jsonl).

## Timing and Usage

| Measure | Shell | Transparent MBTX |
|---|---:|---:|
| Mean Codex elapsed time | 19.674 s | 19.817 s |
| Median Codex elapsed time | 18.815 s | 19.057 s |
| Total reported input tokens | 922,069 | 927,634 |
| Total reported cached input tokens | 762,880 | 755,712 |
| Total reported output tokens | 6,459 | 6,498 |
| Total reported reasoning output tokens | 3,655 | 3,816 |

MBTX's mean total time is 142.844 ms (0.726%) higher. MBTX is faster in 17 pairs
and slower in 15; the median paired difference is -375.5 ms. These descriptive
statistics do not show a consistent direction and are not an equivalence test.
Both arms use variable model responses and provider caching, and no launcher,
child, or relay duration is measured separately. A total-time difference cannot
be assigned to launcher startup or execution. There is no demonstrated token
saving; reported usage also depends on the model's variable responses and tool
output handling. Token totals are not a monetary cost estimate.

Each row below has only four pairs. Signs describe this sample, not established
task-specific advantages or disadvantages. Delta is MBTX minus Shell.

| Task | Shell mean, s | MBTX mean, s | Mean delta, s | MBTX faster pairs |
|---|---:|---:|---:|---:|
| literal_argv | 18.849 | 19.483 | +0.634 | 1/4 |
| stdin_utf8 | 20.400 | 18.698 | -1.703 | 3/4 |
| cwd_environment | 19.658 | 17.506 | -2.152 | 4/4 |
| large_output | 19.758 | 22.732 | +2.974 | 1/4 |
| nonzero_exit | 22.275 | 22.291 | +0.017 | 1/4 |
| background_cleanup | 18.725 | 20.127 | +1.403 | 2/4 |
| signal_exit | 18.823 | 18.801 | -0.022 | 2/4 |
| repeated_recovery | 18.903 | 18.895 | -0.009 | 3/4 |

The earlier tracked Linux local launcher run
`20260913T055354901398588` also shows additional observed cost: MBTX is slower
in 32/32 pairs, with a median paired difference of +0.634 ms. That run uses the
debug launcher and a different workload. Its observer interval includes process
execution and evidence-file writes; it is not a pure startup measurement and
must not be pooled with this release-built online run. macOS local evidence
remains a separate platform dataset and is not part of these Linux conclusions.

## Coverage Still Missing

- `background_cleanup` only waits for a short-lived child. It does not exercise
  cancellation with descendants, orphan cleanup, escalation, or zombie checks.
- `repeated_recovery` executes one print statement. It does not test repeated
  session recovery or resource leaks.
- `signal_exit` checks self-SIGTERM and exit 143. It does not test SIGKILL or
  cancellation through Codex and MBTX. No timeout occurs in this full run.
- The collector emits Codex exit/reap/drain and a child-exit record after
  collection. These timestamps do not measure each underlying lifecycle event.
  There is no observed launcher-start record; `launcher.txt` describes the
  configured path. Approval-denial and sandbox-enforcement boundaries are not
  independently tested by these successful commands.
- Commands are displayed as `/bin/bash -lc ...`. The pinned source makes
  Transparent use Direct and only enables ZshFork for a Zsh session. Direct is
  consistent with this Bash evidence, but execution mode is not recorded as a
  runtime event, and no separate default-ZshFork versus Direct cohort was run.
- There is one Linux host and one online window, with four pairs per fixed task.
  No online macOS replication, predefined equivalence margin, or rare-failure
  bound has been established. The fixture/build cost is not timed separately.

## Decision

Retain MBTX as an explicit experimental backend. The evidence supports a
working baseline implementation with basic compatibility on this Linux setup.
It provides no demonstrated performance or reliability reason to switch the
default from Shell.

The next useful work is an offline reproduction of the merged-output ordering
difference with symmetric stream/lifecycle observations, then real process-group
cancellation, SIGKILL, recovery and leak cases. Complete those measurements and
record the actual execution mode before spending on more copies of this same
online task set. No additional production API requests were made in this review.

## Reproduce the Review

From the repository root, build only the small report CLI and run the audit:

```bash
cargo build --locked --manifest-path adapter/Cargo.toml --bin mbtx-eval
moon run evidence/linux/codex-relay/20260913T152436165546119/review/audit.mbtx
```

The audit rewrites only its derived `audit.json` and `source.sha256`. It never
changes the original attempts, event log, summary, or report. Numeric per-pair
results are in [audit.json](audit.json). The original four report formats remain
available: [Markdown](../report.md), [JSON](../report.json),
[CSV](../report.csv), [offline HTML](../report.html).
