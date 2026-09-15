# Programmable MBTX Step Research

**Status:** Planning handoff. The Codex fork exists; implementation and new
collection have not started. This document is held in the existing research
repository while the fork is being cloned. Move its authoritative copy into
the fork when implementation starts, and replace this copy with a link.

## Repository responsibilities

| Repository | Responsibility |
|---|---|
| [Codex-MBTX](https://github.com/ZSeanYves/Codex-MBTX) | Retain the runnable transparent-launcher evaluation, local OTel collector, OS trace, shutdown observations, reports and Linux/macOS collection procedures. Use it for future launcher optimization measurements. |
| [codex-mbtx-runtime](https://github.com/ZSeanYves/codex-mbtx-runtime) | Develop the programmable MBTX tool directly in the Codex fork and study agent steps against a Shell tool control. |

The existing research repository remains usable on `main`. A second historical
branch is unnecessary because the repositories now provide the separation.
Commit `3527b02a4bc35df8017ac8d1f3b296d9fcdc1d2c` records the existing local OTel
and shutdown-trace implementation and remains reachable through `main` history.
Old evidence and published reports retain their original hashes and limitations.

The new fork was verified as a fork of `openai/codex` on 2026-09-15. Its remote
`main` was `31ffe2bc9adccfe5fd3d29208250f796a13aa7a0` at that check. This is an
observed revision, not a build validation or a permanent upstream pin. After
clone completes, record and validate the actual checkout before selecting the
implementation baseline. The existing research harness uses a different pinned
Codex revision; its patch must not be assumed to apply to the new fork.

## Research question and scope

Under the same task goal, model, permissions and correctness oracle, does a
programmable MoonBit/MBTX interface reduce the logical provider rounds needed
to complete work compared with a Shell execution tool?

Use exactly two experiment arms: `shell_tool` and `mbtx_program`. The fork does
not add a transparent-launcher compatibility cohort, port the old 24-scenario
launcher benchmark, or adopt the historical `mbtx_backend = "transparent"`
configuration. The existing repository owns those measurements independently.

The primary results are task success and steps to success. Tool calls, errors,
recovery, process operations and tokens explain the observed trajectory. Timing
can support diagnosis but is not the primary measure of agent efficiency.
Fewer visible steps do not establish fewer CPU instructions, less hidden model
reasoning, lower cost or faster completion.

## First implementation and ownership

Keep upstream Codex's workspace and build conventions. Add the program tool at
the actual tool registration and dispatch boundary found in the new checkout:

```text
Codex model loop -> tool dispatch -> approval and sandbox policy
  Shell arm: submitted script -> execute -> structured tool result
  MBTX arm: submitted MoonBit program -> build -> run -> structured tool result
Codex model loop receives the result and chooses the next action
```

Codex owns tool registration, configuration, approval, sandboxing and session
lifecycle. Supported MoonBit tooling owns compilation and program execution.
The MoonBit evaluation package owns task definitions, oracles, step-count rules
and statistics. Rust adapters own OS/HTTP calls, evidence capture and OTLP
transport. Automation remains in thin `.mbtx` entries.

Define the smallest useful program interface first: inline source, literal
arguments, cwd, output limits, a build deadline and a run deadline. Expose build
failure separately from runtime failure. Add saved-file execution and background
sessions only when the corresponding tasks and lifecycle tests require them.
Select the execution target after proving that both arms have the required task
capabilities under equivalent policy; do not assume the historical native
launcher and an OpenSeek-style program runner are interchangeable.

Compilation, execution and child commands must all respect Codex policy. Errors
must identify their stage. Cancellation must finish the owned lifecycle and
stream handling before the terminal result, with residual state retained as
evidence. A successful fallback cleanup does not erase a tool failure.

Ship all required source in the fork with one documented build entry. Cargo
builds Codex; MoonBit builds the components that require it. Record toolchain,
lock, platform, profile and artifact hashes. Do not fetch and patch a hidden
upstream checkout during installation or collection. Keep the Shell tool as the
product default while the program tool is explicitly enabled for experiments.

## Reuse the existing observability work

Reuse reviewed components rather than copying the whole old harness:

| Existing component | Use in the fork |
|---|---|
| Local OTLP collector and Codex JSONL capture | Adapt to the fork's native telemetry and event interfaces. |
| OS monotonic events and shutdown observations | Port only missing boundaries still relevant in the selected upstream revision. |
| Serial Responses proxy and external-failure classification | Retain request pacing and evidence for relay failures. |
| Immutable attempts, hashes and report reconstruction | Preserve provenance while introducing an explicit step schema. |
| MoonBit analysis and existing parser tests | Reuse sound rules; replace launcher-specific task and pair-selection logic. |
| Transparent launcher and its benchmark matrix | Keep runnable in Codex-MBTX; port individual runtime helpers only if the new tool needs them. |

Shared upstream fixes must affect both arms and remain distinguishable from
MBTX changes. Freeze source revisions when borrowing code. The fork must build
and run without a checkout of the old repository or downloading its datasets.

## Step and evidence contract

An `agent_step` is one logical provider round in the agent loop. Count a
completed step when its response is accepted; separately record rounds started,
failed or interrupted. Several tool calls in one response remain one step.
Reading persisted checkpoint events does not create steps. A new execution
against a fixed Responses replay does have real loop iterations.

Audit the fork's native rollout/JSONL and OTel fields first. Reuse a logical-round
identifier when available; otherwise add a small explicit boundary in the model
loop. A user turn, a log row, an SSE chunk or an OTel span is not automatically
an agent step. Avoid a second logging system that guesses step boundaries.

| Field | Meaning |
|---|---|
| `agent_steps`, `agent_steps_started` | Accepted rounds and entered rounds, with completeness recorded. |
| `model_requests`, `transport_retries` | Actual provider requests and retries within a logical round. |
| `tool_calls`, `tool_executions`, `polls` | Model requests, actual dispatches and status/output polling. |
| `tool_errors`, `repair_steps` | Observed failures and recovery rounds with an explicit or reviewed causal label. |
| `process_spawns` | Observed process starts, accompanied by coverage limitations. |
| `success`, `status`, `failure_class` | Independent task oracle and terminal classification. |

For example, a response with three tool calls followed by a final response is
two completed steps and three tool calls. A request that fails with 429 before
any response is a failed task with a started round, not a zero-step success.
Ambiguous repair attribution is `inferred` or `unknown`; an unobserved count or
duration is `null`, not zero. Do not sum the counters into an artificial total.

Join raw events with experiment, pair, attempt, task, execution-interface,
thread/turn, step, request and call IDs. Add event sequence, parent ID,
monotonic timestamp, clock domain, status and observed process identity where
available. Auxiliary model requests, compaction and retries carry their own
purpose and remain visible in request/token totals.

Write attempts atomically and seal them with hashes. Resume verifies completed
attempts and allocates new evidence for retries; it never rewrites an earlier
outcome. Reports must rebuild from retained raw evidence without current API
credentials or an active observability service.

## Fair comparison and collection

Use goal prompts and independent acceptance oracles. Allow Shell to submit a
complete script and use the same installed capabilities. Freeze model/reasoning
settings, common instructions, permissions, context/tool budgets, fixtures,
tool descriptions and output caps. Record the tool-schema difference as the
intended treatment. Each arm has its own mutable workspace and session state.

Expose only the assigned execution tool in each arm. Keep shared file tools
identical, and record their use. Disable optional delegation and Code Mode for
the initial experiment; study them later only as separate declared conditions.
Shell subprocesses intentionally invoked inside a MoonBit program are observed
operations, not a reason to claim that the program eliminated Shell execution.

The initial task pool is 24 goal-oriented tasks: three each for repository
inspection, structured data, file transformation, command orchestration,
diagnostics, repair, output handling and session recovery. Specify the actual
fixtures and oracles in the fork. These tasks do not inherit the launcher suite's
claim of scenario coverage.

Keep the earlier 12-pair pilot and 96-attempted-pair first dataset as planning
defaults, not evidence of adequate statistical power. Use the pilot to settle
task difficulty, budgets and request cost. Before formal collection, freeze the
task list, sample size, round allocation, AB/BA order seed, step/tool/token
limits, timeouts and analysis rules. Do not change them to favor observed MBTX
outcomes. Additional collection must be a separately declared tranche.

Report intention-to-treat success by step budget, overall success differences,
and paired step differences for pairs where both arms succeeded. Label that
last analysis as conditional. Retain failures, censored attempts and external
errors in their original denominators. Use paired, task-aware uncertainty
intervals with a fixed analysis seed; avoid unsupported per-task tail claims.
Equivalent policy is a prerequisite for an interface-only claim.

Keep the proxy's concurrency at one for the complete response stream, with
15 seconds between request starts by default. Probes and auxiliary requests
use the same gate. Disable automatic request/stream retries for collection;
preserve 429 results and honor `Retry-After`, with 30 seconds as fallback, for
later requests. Other applications using the account remain outside this gate.

Estimate total time from measured request counts, not pair counts alone. For
96 pairs and three to six requests per arm, 15-second pacing alone is about
2.4 to 4.8 hours. Slow streams, tool execution and cooldown may extend this.
Use the pilot to choose a realistic fixed budget, rather than adding concurrency.

Retain the permissive infrastructure gates: one successful probe in three;
pause after five consecutive final infrastructure failures, at least eight
among the latest 16 arms, or loss of a core collector component. Always produce
a partial report. Expected tool cancellation is not an infrastructure failure.

## OTel and SigNoz

Use native Codex telemetry plus the missing tool/step boundaries. SigNoz is the
inspection UI; immutable JSONL, OTLP and manifests remain the evidence source.
Keep trace/span identities and explicit links across asynchronous lifetimes.
Derived timelines retain their source-event mapping and uncertainty labels.

Show each attempt's steps, requests, tool dispatch, program build/cache status,
execution, output/result publication and shutdown. Provide paired trace links,
step-to-success distributions, the first observed divergence, external-failure
filters and raw-evidence links. Prefer native SigNoz views; keep offline HTML
as a readable summary rather than developing another trace explorer.

Both arms use the same observation profile. Timestamp before writing evidence;
record flush costs and missing events. Use small fixed replays to validate counts
and traces; their prescribed trajectories are not formal step-efficiency data.
Diagnostic collection and export are separate from formal timing. OS-monotonic
and OTLP epoch clocks require explicit alignment before timeline arithmetic.
Publish observer-calibration limits; equal instrumentation does not establish
that measurement is free of perturbation. Model compute and relay-internal
waiting stay a combined external interval without trustworthy server evidence.

## Implementation order and acceptance

1. **Inspect the completed clone.** Read its instructions and current tool,
   model-loop, sandbox, telemetry and build interfaces. Record the baseline and
   verify an unmodified build. Move this plan into the fork with a link back.
2. **Implement one program tool path.** Inline source builds and runs under
   Codex policy; structured success, build failure, runtime failure and
   cancellation work from a fresh clone.
3. **Establish exact counters.** Hand-checked fixed transcripts cover multiple
   tools in a response, failed requests, retries, interruption, resumed logs and
   error recovery. Complete evidence reproduces the expected counts.
4. **Adapt observation and tasks.** Reuse the collector/proxy/evidence pieces,
   define independent task oracles, and verify SigNoz import and standalone
   report reconstruction, including partial and malformed evidence.
5. **Deliver the validated code.** Fast PR checks and one reusable offline build
   cover program policy/lifecycle, counting and report behavior. Use one
   aggregate required status; doc changes do not rebuild Codex. Cache keys
   include source, toolchain, locks, platform, target, profile and fixture hash.
6. **Collect and analyze separately.** The user runs the Linux pilot and formal
   collection after the code is ready. Only returned evidence supports research
   acceptance and results. macOS results remain a separate platform dataset.

The initial code delivery contains the tool, tests, fixed replay, task/oracle
definitions, collection configuration and report/SigNoz export guidance. It
does not claim a step reduction before real task data is available. Fixed
fixtures and host components build once; compilation of newly submitted
MoonBit source is legitimate tool work and is recorded separately from cache hits.

## Later measurements in Codex-MBTX

The current [Linux procedure](../linux-online.md),
[evaluation protocol](../evaluation.md) and [architecture](../architecture.md)
remain valid for this repository's existing launcher research. Keep the local
OTel collector, shutdown trace and report support so future optimizations can
be inspected and measured here.

After a launcher optimization, compare the baseline and candidate on the same
Linux host with the same Codex revision, workload, toolchain/profile and
observation conditions. Use a small diagnostic replay to locate the mechanism,
then the existing minimal-observation procedure for performance measurements.
Create new versioned evidence; do not overwrite old runs or compare across an
uncontrolled upstream change. A programmable-interface change in the new fork
requires that fork's own tests and data; this launcher harness does not measure
its agent-step benefit.
