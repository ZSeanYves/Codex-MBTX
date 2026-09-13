# Transparent launcher evaluation

The protocol answers three separate questions: execution compatibility, local
performance attribution, and the hypothesis of a 10% startup advantage.
Formal collection and scientific acceptance happen after code verification.
Default Shell remains the product default.

## Scenarios and sampling

`evaluation/suite.mbt` is the shared source of task definitions, complete tool
arguments and oracles. There are 24 scenarios, three per category:

| Category | Scenarios |
|---|---|
| argv | Empty/Unicode; metacharacters/newlines; long/many arguments |
| stdin | Empty/EOF; PTY segmented UTF-8/EOF; large pipeline/backpressure |
| environment | Unicode/symlink cwd; empty versus unset; PATH/login/inheritance |
| output | Separate large streams; interleaving; truncation and final output |
| errors | Nonzero; missing/non-executable program; numeric versus signal exit |
| background | Session polling; nested descendants; inherited output pipes |
| cancellation | Ctrl-C; TERM/143; ignored TERM followed by KILL/137 |
| recovery | Sequential tools; recovery after failure; repeated cancellation |

Online collection targets 8 valid pairs per scenario, in two rounds of 4.
Each round permits 6 attempts per scenario: target 192 pairs, maximum 288.
Task order uses seed 20260913; adjacent pairs alternate AB/BA. Failed attempts
remain in intention-to-treat (ITT). Replacement is based on evidence completeness,
never on a duration threshold. Short validation runs are separate directories.

Offline replay defaults to 10 pairs per scenario. It runs the actual Codex
executable, sandbox and launcher against a local fixed Responses/SSE server.
No external API credentials are used. Direct tools and Code Mode are separate
artifacts. Code Mode runs all scenario steps sequentially in one exec cell.

Startup collection has three workloads (native no-output, native small-output,
and shell launching a native no-output program), each with 1000 pairs in minimal
observation and another 1000 in full trace. Ten warmup pairs per stratum are
recorded separately. Every arm creates a new process. The primary interval is
parent spawn begin to fixture entry, delivered through an inherited pipe using
the OS monotonic clock. Build and artifact writes are outside that interval.
Minimal/full observation order alternates per pair as well as backend order.

## Request budget

One fixed-version Responses proxy serves the entire online run. Its lock covers
the complete upstream response stream, including error responses. Concurrency
is 1; request starts are at least 15 seconds apart by default (at most 4 RPM).
`MBTX_MIN_INTERVAL_MS` can increase this, or decrease it to a minimum of 6000 ms.
Codex request and stream retries are disabled. A 429 is retained as a failed
attempt, and later requests respect `Retry-After` (seconds or HTTP date), with
30 seconds as the fallback. There is no immediate retry of the failed request.

The usual two requests per Code Mode arm imply about 3.2 hours of pacing for
192 pairs, before extra requests, slow responses, and replacement attempts.
Direct tool mode can require substantially more requests. Do not run another
online collector or other clients using the same account concurrently.

The initial probe requires 1 success in 3 attempts. Collection pauses after
5 consecutive final infrastructure failures, or at least 8 infrastructure
failures among the latest 16 arms, or loss of the collector. The current pair
is retained. Expected cancellation is not an infrastructure failure. Partial
reports remain available; relay failures never become launcher failures.

## Observation and attribution

Raw JSONL events record experiment/pair/attempt, request and call identifiers,
session and PID/process-group information where observed, phase, sequence,
monotonic time and clock domain. Missing values remain null. All streams are
retained independently, with read-order events and original merged Codex output.
Cross-stream differences are shown even if each individual stream is correct.

The observation boundaries include proxy gate/rate waits, request send,
headers/first byte, tool argument completion, Codex arguments and resolved argv,
approval, sandbox preparation, process spawn, launcher entry and child spawn,
fixture entry, actual parent wait return, stream EOF and result preparation.
Wait return is the parent's observation, not an invented kernel exit timestamp.
FD snapshots and controlled-process residuals are retained. Fallback cleanup
cannot turn a residual into a successful lifecycle result. Cleanup uses only
this attempt's observed process identities and groups, never an entire UID.
The resource scenario performs six interrupt/recovery cycles. FD counts are
observations, not proof that every descriptor was closed; descriptor identity,
long-duration drift and unobserved descendants remain outside that claim.
The offline `--boundaries` option adds `detached_descendant`, which intentionally
leaves the original process group. Control signals target only the specified
root fixture. Surviving detached descendants are a recorded lifecycle failure,
even when the collector's subsequent containment succeeds.

Codex and launcher use the same OS monotonic clock. Unaligned or missing times
are not subtracted. Both arms share the observation layer and immutable fixture.
The launcher removes its internal cancellation-scope control before starting
the child; Codex owns process-group cancellation and IO drain.

Report intervals can overlap. Their sum is not an additive accounting model;
unexplained time is the complement of an observed interval union. Model compute
and internal relay waiting cannot be separated without trustworthy server logs.
Offline replay supplies the external-response control. Minimal/full startup
traces expose measurement perturbation separately from startup performance.

The runtime records the path actually used, including a ZshFork fallback to
Direct. When default Shell is already Direct, the same evidence is the Direct
control. Otherwise run an independent offline `MBTX_DIRECT_SHELL=1` artifact.

## Evidence and reports

Each attempt is created once, then sealed with SHA-256 hashes. Resume verifies
sealed attempts and never rewrites incomplete ones; those remain censored.
Request/response evidence, exact arguments, fixture receipts, independent stream
bytes and original Codex output are retained. Reports rebuild assessments from
immutable facts; no current API environment is consulted. Changes in evidence
or missing seals exclude an attempt from the strict subset.

`mbtx-eval report RUN --format all` emits Markdown, JSON, CSV, offline HTML and
Chrome trace JSON. HTML has parallel timelines on a common duration scale,
scenario/category/platform/status filters, first differences, and raw evidence
links. Commands and responses are rendered as text. `log RUN --follow` shows
collection progress and estimated remaining work; estimates include pacing.
Missing causal parent links stay null. An incomplete proxy snapshot limits
external-stage attribution even if the command's behavior is complete.

Statistics use paired differences, absolute durations and ratios, with 2000
fixed-seed bootstrap resamples and 95% intervals. They are stratified by platform,
scenario, round, tool mode and actual Shell mode. Eight online pairs per scenario
do not support reliable p99 estimates. A 10% startup claim requires at least
1000 minimal-observation pairs and a ratio interval upper bound no greater than
0.90. A current-protocol result is not a reproduction of an old end-to-end run.

Historical raw reports under `docs/archive/` remain unchanged. The archived
reports examined so far have end-to-end durations, not startup-ready timestamps.
Their proposed 10% startup benefit therefore remains a hypothesis, not an
attributed historical finding. Linux and macOS evidence is never pooled.
`moon run scripts/audit-history.mbtx ARCHIVED.json NEW-AUDIT.json` preserves the
selected report's checksum and recorded metadata. Historical rebuild/retest is
pending identification of the exact recipe behind the claimed startup result;
the available end-to-end reports cannot supply missing startup boundaries.
