# Linux Execution Backend Evaluation

Report date: 2026-09-14. Data revision:
`370432d3289f0c5be790788f136f47db5cabbc9f`.
This analysis uses previously collected evidence. It introduces no new task
executions or external API requests and does not modify the source artifacts.

## Abstract

This study evaluates whether transparent MBTX preserves the tested Codex
execution contracts and whether it changes process startup or end-to-end
performance. On the tested Linux configuration, all 240 paired fixed-response
replays across 24 scenarios passed their predefined behavioral oracles. Online
collection produced 183 strictly comparable pairs from 209 attempts, below the
192-pair target. No arm was classified as a backend failure.

In the isolated startup experiment, three workloads each contributed 1,000
minimal-observation pairs. Adding the MBTX launcher increased mean startup
latency by approximately 0.49 ms. Online mean end-to-end time was 2.76% lower
with MBTX, but the 95% paired bootstrap interval ranged from 12.46% lower to
8.14% higher. The data do not establish an end-to-end speed advantage.

The results support compatibility within the tested contracts and identify a
measurable startup cost in the current implementation. They do not establish
universal lossless substitution, superior reliability, or a reason to change the
default Shell backend. A common approximately 10-second interval after the final
response remains an unresolved source of Codex latency.

## Experimental Design

| Experiment | Source artifact | Formal samples |
| --- | --- | --- |
| Launcher startup | [20260914T044744-1789332464819](../../../evidence/linux/launcher/20260914T044744-1789332464819/) | 6,000 pairs: 3 workloads, 1,000 pairs each in minimal and full observation |
| Real Codex, fixed Responses/SSE | [20260914T045159-1789332720090](../../../evidence/linux/codex-replay/20260914T045159-1789332720090/) | 240 pairs: 10 per scenario |
| Real Codex, external relay | [20260914T072517-1789341917874](../../../evidence/linux/codex-relay/20260914T072517-1789341917874/) | 209 attempted pairs; 183 strictly comparable; target 192 |

All three experiments ran on Linux x86_64. Codex and the launcher used release
builds. Both Codex experiments used Code Mode; the observed default Shell path
was Direct, so the same evidence provides the Direct Shell control. These
measurements do not cover ZshFork or direct tool mode. macOS results are not
pooled with Linux.

Each pair used the same immutable fixture and task inputs, with isolated mutable
workspaces and homes. Backend order alternated AB/BA. Warmup samples were recorded
separately, and every startup arm created a new process. Measurement loops used
prebuilt binaries.

All 48 recorded source hashes in each of the startup and Codex build inputs
matched the evaluated checkout. The data revision did not change the experiment
implementation. The three manifests record identical binary hashes for:

- Fixture: `773f769a6356cd8b06590e2f0925c7ded43cdef3440d08b6e467cd3f882139f0`
- Launcher: `233c8a313f89c42d1a797f6e503d109823b7ae6a9e19243a81ba6b8c9dab036c`

### Statistical Method

Differences are MBTX minus Shell; ratios are MBTX divided by Shell. Confidence
intervals use the shared MoonBit implementation: 2,000 paired bootstrap
resamples, xorshift32 seed 20260913, and percentile 95% intervals. Ratios are
ratios of means, not means of individual ratios.

Intention-to-treat (ITT) includes all formal attempted pairs. The strict subset
requires successful arms, matching task trajectories and complete evidence.
Exclusion depends on these conditions, not duration. Per-scenario and per-stage
results are available in [statistics.json](statistics.json) and
[statistics.csv](statistics.csv). The original generated reports retain
round-specific results.

Intervals describe sampling uncertainty under this experiment's conditions.
They do not capture systematic measurement error, temporal dependence, changes
in hardware, or changes in provider behavior. Failure to detect a difference is
not an equivalence test. Exploratory comparisons across many scenarios do not
establish a general benefit, and eight online pairs per scenario cannot support
reliable p99 estimates.

## Evidence Integrity

The published checkout contained 74 PTY stream files whose bytes did not match
their existing attempt seals: 40 offline and 34 online. Each received file used
LF line endings. Restoring CRLF produced an exact match to the SHA-256 already
recorded at collection time for every affected file.

Recovery was performed only in independent analysis copies. A candidate was
accepted only after matching the pre-existing seal; no expected task output was
used to substitute for missing evidence. All original top-level seal entries and
attempt seals then verified. Reconstructed summaries, statistics, pair records
and arm results matched the uploaded reports.

| Integrity check | Result |
| --- | --- |
| Startup attempt seals, including warmup | 12,120 verified without recovery |
| Startup top-level sealed files | 33,341 verified |
| Offline attempt seals | 480 verified after 40 PTY recoveries |
| Online attempt seals, including 3 probes | 421 verified after 34 PTY recoveries |
| Reconstructed report equality | All three experiments matched |

Rebuilding directly from the received, unrecovered checkout reduces strict pair
counts to 220 offline and 167 online because seal mismatches are rejected.
The original files have not been repaired in this documentation revision.
The exact-byte recovery supports the reported analysis, but a fresh checkout
still requires that recovery before reproducing it.

The newline transformation is observed; the specific publishing configuration
that introduced it is unknown. Binary evidence publication needs an explicit
preservation rule and a documented repair. Per-file received, recovered and
original hashes are recorded in [pty-recovery.json](pty-recovery.json);
verification outcomes are in [audit.json](audit.json).

## Startup Results

The primary metric is parent spawn begin to fixture ready, measured with the
same OS monotonic clock and minimal observation. For native workloads, the
Shell label denotes the existing direct launch path without an additional shell
process. For the shell workload, both arms execute the same `/bin/sh` command.
The transparent arm inserts MBTX before the otherwise identical argv.

| Workload | Pairs | Shell mean, ms | MBTX mean, ms | Difference, ms [95% CI] | Ratio [95% CI] |
| --- | ---: | ---: | ---: | --- | --- |
| Native no-output | 1,000 | 0.433 | 0.929 | +0.496 [0.493, 0.499] | 2.145 [2.135, 2.156] |
| Native small-output | 1,000 | 0.432 | 0.921 | +0.490 [0.486, 0.493] | 2.134 [2.123, 2.146] |
| Shell launching native no-output | 1,000 | 0.741 | 1.229 | +0.487 [0.482, 0.492] | 1.657 [1.649, 1.665] |

Separate AB and BA analyses, each with 500 pairs per workload, show the same
direction. With identical fixtures and no Codex or network activity, the
approximately 0.49 ms increment is attributable to the added MBTX launch path
under this host and build configuration. These data do not separately identify
runtime initialization, system-call cost and scheduling cost.

Full observation increased MBTX means by approximately 45-47 us relative to
minimal observation; Shell means increased by approximately 1-2 us. The added
paired penalty was approximately 44-46 us. These are comparisons of separate
sample groups, not direct measurements of the cost of an individual trace call.
Formal startup conclusions use minimal observation.

The current protocol does not reproduce the proposed 10% startup advantage.
Reproduction of an earlier result requires its exact revision, toolchain,
profile, execution mode and measurement definition. Current startup results
cannot substitute for that historical experiment, and an earlier end-to-end
point estimate cannot establish a launcher startup benefit.

## Compatibility and Coverage

All 480 offline arms passed the predefined behavioral oracles. Online formal
outcomes, excluding probes, were:

| Backend | Success | Model trajectory deviation (`unknown`) | Relay error | Backend failure |
| --- | ---: | ---: | ---: | ---: |
| Shell | 190 | 15 | 4 | 0 |
| Transparent MBTX | 189 | 17 | 3 | 0 |

All 32 unknown outcomes have recorded model trajectory deviations. They remain
in ITT and cannot be counted as either demonstrated MBTX failures or successful
backend checks. The seven relay errors also remain in ITT.

| Scenario | Online attempted pairs | Online strict pairs / target | Offline strict pairs |
| --- | ---: | ---: | ---: |
| argv_empty_unicode | 8 | 8/8 | 10 |
| argv_metacharacters | 9 | 8/8 | 10 |
| argv_long | 12 | 0/8 | 10 |
| stdin_eof | 9 | 8/8 | 10 |
| stdin_pty_utf8 | 8 | 8/8 | 10 |
| stdin_backpressure | 10 | 8/8 | 10 |
| cwd_symlink | 9 | 8/8 | 10 |
| environment_empty_unset | 8 | 8/8 | 10 |
| environment_path_login | 8 | 8/8 | 10 |
| output_split_large | 9 | 8/8 | 10 |
| output_interleaved | 8 | 8/8 | 10 |
| output_truncation_tail | 8 | 8/8 | 10 |
| exit_nonzero | 8 | 8/8 | 10 |
| exec_missing_permission | 9 | 8/8 | 10 |
| exit_code_vs_signal | 8 | 8/8 | 10 |
| session_poll | 8 | 8/8 | 10 |
| descendant_cancel | 8 | 8/8 | 10 |
| descendant_pipe_tail | 8 | 8/8 | 10 |
| cancel_ctrl_c | 9 | 8/8 | 10 |
| cancel_sigterm | 8 | 8/8 | 10 |
| cancel_sigkill | 9 | 8/8 | 10 |
| session_sequence | 11 | 7/8 | 10 |
| session_failure_recovery | 8 | 8/8 | 10 |
| session_cancel_resources | 9 | 8/8 | 10 |
| Total | 209 | 183/192 | 240 |

All 24 long-argument arms deviated from the specified trajectory. Five
deviations occurred in sequential execution, two in stdin backpressure and one
in repeated cancellation. The long-argument case passed fixed replay, but its
online coverage remains incomplete. The run correctly retains `partial: true`.

### Output and Lifecycle Boundaries

Two offline and two online pairs in `output_truncation_tail` have different
merged output. In all four pairs, stdout contains 262,153 bytes and stderr
contains 4,096 bytes, with identical per-stream hashes between backends and a
passing final-output oracle. This supports preservation of independent stream
bytes, not byte-identical cross-stream merging. The relative contribution of
child scheduling and Codex stream reads remains unresolved. Differences in
cwd/environment output reflect intentionally isolated workspace and HOME paths;
the oracles validate each arm against its own environment.

All 480 offline and 418 online formal arms record `processes_clean=true` and an
empty `unidentified_live` list. This observation covers controlled processes at
the recorded checkpoints. Linux PID-namespace teardown may terminate a
descendant before its delayed output is written. The sandbox's outer wait may
also normalize real signals to numeric exit codes. These results do not prove
every inner signal identity, cleanup of arbitrary detached descendants, or the
absence of long-term resource leaks.

## End-to-End Performance

| Cohort | Pairs | Shell mean, s | MBTX mean, s | Difference, s [95% CI] | Ratio [95% CI] |
| --- | ---: | ---: | ---: | --- | --- |
| Offline ITT = strict | 240 | 10.686 | 10.728 | +0.042 [-0.109, +0.211] | 1.004 [0.990, 1.020] |
| Online ITT | 209 | 37.838 | 37.515 | -0.323 [-4.322, +3.985] | 0.991 [0.891, 1.110] |
| Online strict | 183 | 35.562 | 34.581 | -0.981 [-4.637, +2.723] | 0.972 [0.875, 1.081] |

None of these intervals establishes an end-to-end advantage. The strict online
aggregate also excludes the long-argument scenario and is not an equally
weighted estimate of the complete planned 24-scenario population.

### Stage Attribution

Codex `startup_to_ready_ns` sums observed tool startup intervals within an arm.
It is not a single-process latency and is not interchangeable with the isolated
startup metric.

| Strict-cohort stage | Measured pairs | Shell mean, ms | MBTX mean, ms | Difference, ms [95% CI] |
| --- | ---: | ---: | ---: | --- |
| Offline cumulative tool calls | 240 | 840.349 | 841.890 | +1.541 [-0.689, +3.779] |
| Offline cumulative spawn to ready | 230 | 41.574 | 43.154 | +1.580 [+1.181, +1.993] |
| Online cumulative tool calls | 183 | 902.315 | 902.797 | +0.481 [-5.691, +6.553] |
| Online cumulative spawn to ready | 175 | 43.283 | 44.297 | +1.014 [+0.555, +1.505] |
| Online cumulative external response | 183 | 15,449.073 | 14,758.305 | -690.768 [-4,517.519, +3,200.963] |
| Online evaluation rate waiting | 175 | 9,727.249 | 9,533.842 | -193.407 [-680.177, +279.631] |

Missing/non-executable program cases have no fixture-ready event. Missing
measurements are not replaced with zero. The current statistics implementation
admits a stage only when both arms have positive values; zero and missing values
therefore produce different sample counts across stages. In particular, the
rate-wait row is conditional and is not an all-arm estimate. Offline replay
events use a different phase from the external proxy; its external-response
metric remains null.

The approximately one-second online point difference accompanies changes in
external response and rate waiting, while tool-path differences are measured in
milliseconds. It cannot be attributed to MBTX. Model computation and relay
internal waiting remain inseparable without server-side evidence. Stages can
overlap and have different sample sets; their means are not an additive
accounting of total latency.

### Post-Response Waiting

Raw OS monotonic timestamps identify a repeated interval near ten seconds:

| Measurement | Arms in 9.9-10.1 s | Shell median, s | MBTX median, s |
| --- | ---: | ---: | ---: |
| Online final `external/stream_eof` to `codex/wait_return` | 388/418 | 10.008739 | 10.008922 |
| Offline final `replay/response_send` to `codex/wait_return` | 460/480 | 10.008155 | 10.008267 |

The replay event is a server-side send timestamp, not client consumption
completion. Offline mean `unexplained_ns` is 9.846 s for Shell and 9.887 s for
MBTX, approximately 92% of mean end-to-end time. Post-response waiting is
consistent with this large residual, but the intervals are not identical.

The same pattern under fixed offline responses cannot be explained solely by
model or relay variability. The collector does not yet provide sufficient
boundaries for turn completion, unsubscribe, internal task draining and
shutdown to identify the responsible component.

The pinned [Codex exec implementation](https://github.com/openai/codex/blob/3d2ee51ca2d5db578f328aa75e20aa22c0197c9a/codex-rs/exec/src/lib.rs)
requests unsubscribe after turn completion and awaits client shutdown. Its
[in-process app server](https://github.com/openai/codex/blob/3d2ee51ca2d5db578f328aa75e20aa22c0197c9a/codex-rs/app-server/src/in_process.rs)
includes task cleanup, timeout waits and analytics flushing. These are
instrumentation candidates, not established causes. The duration alone does
not prove that two five-second timeouts were reached.

### Build, Artifact Writes and Collection Duration

Bundle metadata records 2,676 ms for the startup bundle build and 168,835 ms for
the Codex bundle build. These costs are outside measured arm startup. They are
not demonstrated uncached cold builds and do not establish a numerical speedup
over an earlier complete workflow.

Mean recorded artifact-write time is approximately 40-46 ms offline and 56-64 ms
online. It occurs after the measured `codex_total` boundary and does not explain
the ten-second interval. This metric does not cover every disk operation;
in-execution trace writes can still perturb timing.

Total collection elapsed time was 86 min 8 s offline and 4 h 25 min 52 s online,
including failed attempts and rate waiting.

## Request Pacing and External Failures

Request-level evidence, including probes, records:

- 839 outgoing requests; maximum concurrency of 1 from send to request finish,
  with no requests remaining active.
- A minimum start interval of 15.000144207 s, consistent with the configured
  15-second interval and approximately four sustained request starts per minute.
- 834 HTTP 200 responses, five HTTP 502 responses and no HTTP 429 responses.
- 831 `response.completed` events and 839 stream EOF events. HTTP 200 and EOF
  alone do not establish protocol-complete success.

The gate worked in this collection window. It does not guarantee that future
relay behavior or other clients sharing the account cannot cause 429. The
seven formal arm relay errors remain in ITT; request-level error counts and
arm-level failure counts describe different units.

## Limitations and Remaining Validation

1. Public PTY artifacts require exact-byte recovery before direct report
   reconstruction. Publication rules and repair provenance remain to be fixed.
2. The common post-response waiting interval needs finer offline instrumentation.
   Any Codex correction must be applied to both arms; its benefit cannot be
   assigned to MBTX.
3. Online long-argument and sequential-execution quotas are incomplete. Revised
   task delivery must preserve original model arguments and retain prior failures;
   new protocol versions and samples must remain separately identifiable.
4. Cross-stream merge attribution and treatment of legitimate zero-valued
   stages need further work.
5. macOS, direct tool mode, historical reconstruction and independent hosts or
   collection windows are not covered by this formal dataset. CPU model, kernel,
   background load and power-state metadata are missing from these run records,
   limiting hardware-level reproducibility.

## Conclusion

Under the tested Linux configuration, transparent MBTX preserved the predefined
execution contracts across 24 fixed-replay scenarios and 183 strictly comparable
online pairs. Isolated measurements identify an approximately 0.49 ms startup
cost in the current launcher path. End-to-end performance intervals do not
establish an advantage. The evidence supports a functioning optional backend
within the tested scope, while retaining Shell as the default.
