# Direct Codex Fork, Agent-Step Measurement, and SigNoz Observability Plan

**Status:** Proposed. This document defines the next implementation cycle. It
does not change the runtime, the evaluator, or the current report conclusions.

**Baseline preserved:** the current `main` state is preserved at
`archive/pre-openseek-step-refactor-2026-09-15` and was pushed before this plan
was written. The baseline contains the transparent launcher, the current
evaluation harness, the pinned Codex revision, and the existing evidence
layout. It remains a read-only reference for migration and regression review.

## 1. Decision to make explicit

The project has two related but different research questions:

1. Can MBTX transparently replace the final local process-launch boundary used
   by Codex while preserving command, policy, approval, sandbox, stream, exit,
   and cancellation semantics?
2. Can a programmable MoonBit execution interface allow an agent to complete
   the same accepted task in fewer agent steps, tool calls, retries, or process
   operations than a shell-oriented interface?

The current transparent launcher answers the first question. It does not turn a
shell script into a MoonBit program and therefore cannot be expected to reduce
the model trajectory in a fixed replay. The second question requires a separate
programmable-execution cohort. Results from the two cohorts must never be
combined into a single speed or step claim.

The proposed implementation will use a direct fork of upstream Codex as the
distribution and runtime repository. MBTX integration will live in Codex source
and normal build targets, rather than being reconstructed by applying a local
patch and overlay at collection time. The existing Codex-MBTX repository will
remain the evaluation and historical evidence reference until the fork has
passed the migration gates.

## 2. Repository and distribution architecture

### 2.1 Canonical repository

Create a GitHub fork of `openai/codex` under the project owner's account. The
fork is the user-facing product repository and must be cloneable and buildable
without this repository, a patch file, or a generated overlay. Add the upstream
repository as a read-only `upstream` remote and retain the fork as `origin`.

The exact fork name and owner are an administrative decision. The migration must
record both URLs and the upstream commit used for each release. GitHub forks are
separate repositories with their own branches, permissions, actions, and issue
space while retaining an upstream relationship; this is the collaboration model
the mentor is asking for.

### 2.2 Source layout in the fork

The fork should keep upstream Codex's normal Rust workspace layout. MBTX runtime
code should be integrated through an ordinary, reviewable source path, for
example:

```text
codex-rs/
  ... upstream crates ...
  mbtx-launcher/       # native launcher crate or supported binary boundary
  mbtx-integration/    # Codex configuration and process-boundary integration
evaluation/            # optional offline evaluator and scenario definitions
docs/mbtx/             # user and developer documentation
```

The final layout is subject to the upstream workspace conventions. The key
property is that a fresh clone can build Codex and the transparent backend using
normal Cargo commands. `integration.patch`, `overlay/`, and preparation scripts
that synthesize a private upstream checkout are removed from the product path.
They may remain in the historical branch only as evidence of the old workflow.

The evaluation harness may stay as a separate companion repository if upstream
policy or release size makes that preferable. If it remains in the fork, it must
be an optional evaluation target and must not be required to run Codex.

### 2.3 Upstream synchronization

Every fork release records:

- upstream repository URL and commit;
- fork commit containing the MBTX changes;
- Rust and MoonBit toolchain versions;
- dependency lock hashes and target platform;
- generated artifacts and their hashes.

Upstream synchronization is performed by an explicit update branch and reviewed
merge. A changed upstream process or OTel interface must fail a compatibility
check until the integration is consciously migrated. No collection command may
silently fetch a different upstream revision or regenerate a lock file.

## 3. Runtime boundaries and public behavior

The transparent backend retains the following call chain:

```text
Codex unified_exec
  -> approval / policy / sandbox
  -> MBTX launcher with resolved argv
  -> child process
  -> wait / reap / stdout+stderr drain
  -> Codex result
```

The default remains the upstream shell path. `mbtx_backend = "transparent"` is
opt-in and requires an absolute launcher path. The original command remains the
input to approval and policy decisions; MBTX is inserted only after those
decisions. Invalid backend values, missing launchers, remote execution, and
malformed configuration continue to produce explicit errors.

The fork must preserve the existing semantic contract before any step-efficiency
work begins:

- literal argv, whitespace, Unicode, stdin, EOF, cwd, and environment;
- independent stdout/stderr bytes, observed ordering, truncation, and tail data;
- numeric nonzero exits versus signal termination;
- process-group cancellation, 500 ms TERM grace, KILL escalation, wait/reap,
  and final IO drain;
- approval and sandbox decisions made by Codex;
- per-attempt process identity and cleanup evidence;
- default Shell behavior when the MBTX setting is absent.

Shared Codex fixes must be applied to both backends and labeled as shared. A
shared fix cannot be reported as an MBTX benefit.

## 4. Agent-step metric

### 4.1 Definition

Adopt the OpenSeek meaning of `agent_step`: **one logical provider round in the
agent loop**, from the request for a model response through the response that
ends that round. A response may contain zero, one, or many tool calls. The next
model response starts the next step. A checkpoint replay is not a new step.

This definition is supported by OpenSeek's implementation, which describes one
provider round per step and counts persisted non-replay assistant responses. Its
session analyzer reports steps, tool results, tool errors, and runtime notices as
separate fields. The project should follow that vocabulary so a report can be
compared with OpenSeek discussions without inventing a local meaning.

### 4.2 Related counters

Step is the primary agent-work unit, but it must be accompanied by a vector of
non-interchangeable counters:

| Counter | Meaning |
|---|---|
| `agent_steps` | Logical provider rounds, excluding replay copies |
| `model_requests` | Requests sent to the provider, including a retry field |
| `tool_calls` | Model-requested calls, deduplicated by call ID |
| `tool_executions` | Actual execution instances, including nested child programs |
| `process_spawns` | OS process starts observed by the common observer |
| `tool_errors` | Tool results classified as errors |
| `repair_steps` | Later model steps whose purpose is recovery from a prior failure |
| `transport_retries` | HTTP or stream retries; never silently counted as new agent steps |
| `polls` | Output/status polls, reported separately from new executions |
| `success` | Task oracle result and terminal status |

The evaluator must not add these fields into a single arbitrary “total steps”
number. A lower count is useful only when the task is equally correct and the
reduction is attributable to the execution interface.

### 4.3 Instrumentation contract

The direct Codex fork will emit an explicit monotonically increasing
`agent_step_id` at the model-loop boundary. Every provider request, response,
tool call, tool result, process execution, retry, and repair marker carries the
step ID plus `experiment_id`, `pair_id`, `attempt_id`, `task_id`, `backend`, and
`parent_id`.

The native `codex exec --json` stream remains the durable primary source for
thread, turn, item, command, and terminal events. Native Codex OTel events remain
the diagnostic source for API requests, stream events, tool decisions, and tool
results. The evaluator joins these sources by run and call IDs; it does not count
OTel spans or JSONL rows as steps.

Missing IDs, incomplete streams, or a request whose step boundary cannot be
proved are `unknown`, not zero. Retry attempts retain their own request sequence
under the same logical step. Internal model reasoning that the provider does not
expose is not estimated from tokens or log-line counts.

## 5. Experimental cohorts

### 5.1 Transparent compatibility cohort

This is the existing Shell versus Transparent MBTX comparison. Both arms receive
the same resolved command, fixture, model response trajectory, policy, and
environment. AB/BA order remains balanced. The primary outcomes are semantic
compatibility, failure classification, process lifecycle correctness, and
launcher-local cost. Agent-step counts are a control check and should normally
match between arms.

Latency remains secondary and is split into fixture/build, preparation, Codex
local work, launcher, child, external request, rate waiting, and publication
intervals. Minimal observation is used for formal timing; diagnostic exporters
are used only in replay or calibrated runs.

### 5.2 Programmable execution cohort

This cohort addresses the mentor's step question. Compare:

- a shell-oriented execution interface that can submit a complete script;
- a MoonBit/MBTX program interface that can perform equivalent file, parsing,
  control-flow, and process operations.

The task prompt must describe the goal and acceptance oracle, not prescribe a
fixed sequence of tool calls. Both interfaces receive the same model, tool
budget, workspace, permissions, dependencies, and timeout. A Shell arm must be
allowed to submit a complete script, so an artificial requirement to split it
into calls cannot create a false MBTX advantage.

The primary outcomes are successful-task rate and `agent_steps` to successful
completion. Secondary outcomes are tool calls, process executions, failed
probes, repair steps, output reads, and resource leaks. Failed and censored
attempts remain in intention-to-treat results. A task that uses fewer steps only
because it stopped early is not an efficiency win.

Fixed replay remains necessary for causal backend diagnosis. It supplies equal
model trajectories and isolates execution effects; it cannot establish that one
interface causes an agent to choose a shorter trajectory in open-ended work.

## 6. SigNoz and OpenTelemetry design

SigNoz is the proposed analysis UI, not the evidence store and not the SeekMoon
client. Raw JSONL, OTLP payloads, manifests, and sealed attempt directories stay
in the repository's evidence archive. SigNoz receives a derived copy for search,
waterfalls, and cross-signal navigation.

### 6.1 Trace shape

Use one trace per attempt and the following nested spans where evidence exists:

```text
attempt
  ├─ agent_step N
  │   ├─ provider request / response
  │   ├─ tool call
  │   │   ├─ launcher
  │   │   ├─ child process
  │   │   └─ wait / reap / stream drain
  │   └─ tool result
  └─ terminal result
```

Relay and provider events are linked by request ID and represented as external
segments. If server-side timing is unavailable, model computation and relay
queueing remain one external interval and are labeled `unknown` internally.
Trace attributes include backend, task, pair, round, step, call ID, phase,
status, failure class, platform, execution mode, and evidence path/hash.

### 6.2 Ingestion and measurement isolation

The formal minimal profile must not start SigNoz, a collector, or a second
observer in the measured process path. It writes the common OS-monotonic event
layer only. Diagnostic runs export to a loopback OTel collector or saved OTLP
files, and the files are imported into SigNoz after the run.

Shell and MBTX use the same observation profile. A trace-on versus trace-off
calibration quantifies observer perturbation; trace export time and database
ingestion time are never included in launcher or child latency.

Self-hosted SigNoz may run on Linux and be viewed from macOS over a controlled
network. Its OTLP endpoints are normally 4317 (gRPC) and 4318 (HTTP). The
dashboard setup is operational tooling and must not become a dependency of
offline replay or evidence reconstruction.

### 6.3 Required views

The first dashboard set should provide:

1. pair selector with Shell/MBTX side-by-side waterfall;
2. step histogram and step-to-success distribution;
3. first observed divergence, grouped by launcher, child, Codex, relay, or
   unknown;
4. API/retry/429/stream-error filters;
5. process tree, signal, wait/reap, and IO-drain details;
6. links from each span to the immutable attempt evidence and its hash.

SigNoz visualizations are explanatory. The report generator remains the authority
for pair selection, confidence intervals, ITT treatment, and conclusion labels.

## 7. Implementation phases and gates

### Phase A — freeze and fork scaffold

- Verify the archive branch points to the current main commit and record its hash.
- Create the upstream Codex fork and document remotes, ownership, and license.
- Import the pinned upstream commit into a normal fork branch.
- Build upstream Codex from a fresh clone before adding MBTX.
- Decide whether evaluation code lives in the fork or a companion repository.

**Gate:** a fresh upstream fork clone builds with ordinary documented commands;
no patch or overlay is needed for the upstream baseline.

### Phase B — direct transparent integration

- Port the launcher and Codex configuration changes into normal upstream files.
- Remove patch-generation and private checkout code from the product path.
- Preserve the default Shell path and explicit transparent configuration.
- Port launcher contract, approval, sandbox, signal, stream, and cleanup tests.
- Add a one-command build and install path suitable for an external contributor.

**Gate:** upstream tests plus launcher contracts pass on Linux and macOS, and a
fresh user can configure the launcher without knowing the old repository layout.

### Phase C — explicit step events

- Add the logical `agent_step_id` at the Codex model-loop boundary.
- Add parent IDs and retry sequence fields to JSONL and OTel records.
- Update the evaluator to derive step and related-counter vectors.
- Add fixtures for one response with many tools, retries inside one step,
  checkpoint replay, tool failure followed by repair, and cancelled turns.

**Gate:** fixed replay counts match hand-checked transcripts and no duplicate or
missing step IDs occur in complete evidence.

### Phase D — programmable MBTX interface

- Define the MoonBit program tool contract, source/filename policy, arguments,
  target, sandbox, build cache, foreground/background behavior, and output cap.
- Make compilation, execution, and adopted background jobs separate observable
  phases.
- Add shell-script controls that are allowed to submit an equivalent complete
  script.
- Keep this cohort separate from transparent launcher configuration and reports.

**Gate:** equal-oracle fixed tasks pass for both interfaces; compilation cost is
reported separately from pure execution; no model prompt silently prescribes a
trajectory in the open-ended cohort.

### Phase E — OTel and SigNoz export

- Normalize Codex, launcher, child, relay, and evaluator records into the trace
  shape above.
- Validate trace IDs, parent relationships, null handling, and evidence links.
- Add a local diagnostic export command and a documented self-hosted SigNoz
  profile. Keep the default formal profile exporter-free.
- Confirm that importing a derived trace never changes the original evidence.

**Gate:** a fixed replay can be inspected in SigNoz with both backends, and its
raw evidence can independently regenerate the same summary without SigNoz.

### Phase F — step-focused evaluation

- Run a small offline replay matrix first.
- Run limited online validation only after the offline counters and oracles are
  stable.
- Keep requests serial, use the existing start interval and `Retry-After`
  handling, and retain all relay/provider failures.
- Expand only after confirming that no arm uses a different model trajectory,
  prompt, fixture, or permission.

**Gate:** publish a step report with ITT, successful comparable attempts,
external-failure counts, and explicit unknowns. Do not change the default backend
based on this phase alone.

### Phase G — distribution and documentation

- Replace patch-era instructions with fork clone, build, configuration, and
  upgrade instructions.
- Link the archived repository branch and explain its historical role.
- Document transparent and programmable cohorts as separate features.
- Publish English Markdown, JSON, CSV, offline HTML, and SigNoz import guidance.
- Include a short contributor guide for synchronizing with `upstream`.

## 8. Acceptance criteria

The migration is complete only when all of the following are true:

- a clean clone of the direct Codex fork builds and runs without applying a
  project-local patch or overlay;
- upstream synchronization and dependency revisions are reproducible;
- default Shell behavior is unchanged and transparent mode remains opt-in;
- semantic launcher tests pass on the supported Linux and macOS targets;
- step IDs, tool IDs, process IDs, retries, and terminal states are traceable;
- fixed replay and open-ended programmable cohorts are reported separately;
- SigNoz shows both arms through the same OTel observation layer;
- formal timing excludes exporter, report, and database work;
- raw evidence alone regenerates every reported count and conclusion;
- relay/provider errors are never classified as backend failures;
- lower steps are reported only with equal correctness and explicit scope;
- no conclusion is generalized beyond the tested model, platform, toolchain,
  execution interface, and sample design.

## 9. Non-goals and unresolved decisions

This cycle will not make MBTX the default backend, claim universal shell
replacement, infer hidden model reasoning steps, or treat SigNoz as a source of
truth. It will not merge SeekMoon into the project; SeekMoon remains a client
for OpenSeek conversations, while SigNoz is the observability view for this
evaluation.

Before implementation starts, the project owner must choose the fork repository
name, decide whether the evaluator is shipped inside the fork, and confirm that
“programmable MBTX interface” is the intended mentor-facing comparison. Those
choices affect repository topology but do not change the archived baseline or
the step definition proposed here.
