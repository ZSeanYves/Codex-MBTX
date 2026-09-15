# Direct Codex Fork, Agent-Step Measurement, and SigNoz Observability Plan

**Status:** Proposed. This document defines the next implementation cycle. It
does not change the runtime, the evaluator, or the current report conclusions.

**Baseline preserved:** the pre-migration `main` state is preserved at
[`archive/historical-research-version`](https://github.com/ZSeanYves/Codex-MBTX/tree/archive/historical-research-version),
commit `3527b02a4bc35df8017ac8d1f3b296d9fcdc1d2c`. The branch was pushed before
this plan was written. The baseline contains the transparent launcher, the current
evaluation harness, the pinned Codex revision, and the existing evidence
layout. It is treated as a frozen reference for migration and regression review;
this is a project convention, not a claim that branch protection was configured.
This cycle creates the historical branch and this document only. Fork creation,
source migration, dashboard deployment, and new collection are future work.

The initial migration baseline remains upstream
`3d2ee51ca2d5db578f328aa75e20aa22c0197c9a` (`rust-v0.153.4`), as recorded in
[`codex/upstream.json`](../../codex/upstream.json). Updating upstream and changing
the execution interface are separate changes so regressions can be attributed.

## 1. Decision to make explicit

The new fork has one research question:

> Under the same task goal, model, permissions and correctness oracle, can a
> programmable MoonBit/MBTX execution interface complete work in fewer logical
> provider rounds than a shell-oriented execution interface?

The transparent Shell-versus-MBTX launcher experiment is archived historical
research. It remains available on the archived branch and is not a new cohort, a
control arm, or a result to be recomputed by this plan. The new work measures
agent-visible execution structure: steps, tool calls, process operations,
retries and successful completion. It does not claim a launcher latency benefit
or silently combine the old compatibility results with the new step results.

The proposed implementation will use a direct fork of upstream Codex as the
distribution and runtime repository. The MBTX program interface will live in
Codex source and normal build targets, rather than being reconstructed by
applying a local patch and overlay at collection time. The existing Codex-MBTX
repository remains the evaluation and historical evidence reference until the
fork has passed its migration gates.

## 2. Repository and distribution architecture

### 2.1 Canonical repository

Create a GitHub fork of `openai/codex` under `ZSeanYves`. The
fork is the user-facing product repository and must be cloneable and buildable
without this repository, a patch file, or a generated overlay. Add the upstream
repository as a read-only `upstream` remote and retain the fork as `origin`.

Use `codex-mbtx-runtime` as the provisional new repository name, subject to
availability at implementation time. Keep the existing `Codex-MBTX` repository
and its branches intact as the evidence and migration reference. Adding an
`upstream` remote or copying upstream files into this repository does not itself
establish a GitHub fork relationship. Record the parent/source repository
metadata and verify it after creation. A later public-name change is optional
and is outside this migration's default path.

The fork starts from the pinned upstream commit on an integration branch. It
retains upstream ancestry and receives reviewed, ordinary source commits; do not
replace the existing repository's `main` with unrelated history or rewrite the
historical branch. The fork's default branch becomes the integrated product only
after the clean-clone and program-tool validation gates pass.

### 2.2 Source layout in the fork

Keep upstream Codex's normal Rust workspace. The new product surface is a
programmable MBTX tool; the archived transparent launcher is not duplicated as a
new experiment. Reuse low-level process, sandbox and stream primitives only
where the program tool needs them, with its own capability and lifecycle tests.
The proposed layout is:

```text
codex-rs/
  ... upstream crates ...
  core/               # tool registration, configuration, policy and dispatch
mbtx/
  program/            # programmable MoonBit tool and build/run contract
  runtime/            # process and stream primitives used by the program tool
  cmd/                # program tooling and evaluation-worker entry points
  evaluation/         # MoonBit scenarios, oracles, counters, statistics
  adapter/            # Rust OS/HTTP collector and report/export adapter
scripts/mbtx/          # thin .mbtx build, install, replay, and report commands
docs/mbtx/             # user and developer documentation
```

The final layout is subject to the upstream workspace conventions. The key
property is that one fresh clone contains all required source. Cargo builds
Codex; MoonBit builds MBTX. One documented `.mbtx` build entry orchestrates these
normal build targets once and emits a manifest with the resulting artifact hashes. It must
not imply that Cargo alone compiles MoonBit. Remove `integration.patch`,
`overlay/`, and preparation scripts that synthesize a private upstream checkout
from the product path.
They may remain in the historical branch only as evidence of the old workflow.

Ship the evaluator as an optional directory in the fork by default, so code and
protocol versions are reproducible together. Large historical evidence remains
in the existing repository and is referenced by commit/path/hash. Running Codex
must not require building the evaluator or downloading historical runs.

Source builds require the recorded Rust and MoonBit toolchains. Release bundles
for supported Linux/macOS targets contain Codex and its required program-tool
components. Document the MoonBit compiler/runtime requirement and the program
build-cache contract explicitly. Compiling submitted programs is part of the
tool's execution work, not a product installation step hidden in measurement.

### 2.3 Migration inventory

| Current path | Treatment in the fork |
|---|---|
| `launcher/`, `cmd/mbtx/` | Historical transparent-launch implementation; extract reusable primitives only when the program tool requires them, with no new Shell-versus-launcher cohort |
| `codex/integration.patch` | Port only the Codex tool/configuration changes required for the programmable interface into actual upstream source files |
| `codex/overlay/` | Remove from the product path; no generated upstream checkout |
| `evaluation/`, `cmd/evaluation-model/` | Reuse schemas and statistics where sound; replace compatibility scenarios with step tasks and capability oracles |
| `adapter/` | Reuse OS, proxy, evidence, replay and OTLP collection; remove active compatibility-arm decisions |
| `scripts/` | Keep thin functional `.mbtx` entries; replace checkout preparation with direct build targets |
| `codex/Cargo.lock` | Reconcile the existing dependency fixes into the fork's normal lock file once |
| `evidence/`, `docs/reports/` | Preserve originals; link frozen evidence without relabeling it as new-interface results |

Do not revive retired standalone runners or jobs/session/protocol APIs. The new
program tool is a separately specified Codex integration that reuses supported
MoonBit tooling and Codex lifecycle handling. The archive preserves the original
transparent-launch comparison and its conclusion boundaries.

### 2.4 Upstream synchronization

Every fork release records:

- upstream repository URL and commit;
- fork commit containing the MBTX changes;
- Rust and MoonBit toolchain versions;
- dependency lock hashes and target platform;
- generated artifacts and their hashes.

Upstream synchronization is performed by an explicit update branch and reviewed
merge. A changed upstream tool or OTel interface must fail an integration
check until the integration is consciously migrated. No collection command may
silently fetch a different upstream revision or regenerate a lock file.

## 3. Runtime boundaries and public behavior

The experiment compares two model-visible tools within the same Codex fork:

```text
Codex agent loop -> tool dispatch -> approval / policy / sandbox
  -> Shell tool: submitted script -> execution -> result
  -> MBTX program tool: submitted MoonBit program -> build -> run -> result
Codex agent loop consumes the result and chooses the next step
```

Use explicit experiment arm names `shell_tool` and `mbtx_program`. The historical
`mbtx_backend = "transparent"` setting is not the programmable tool selector and
will not be migrated as a new research feature. Define the new tool's opt-in
configuration and schema in the fork; leave upstream Shell behavior as the
default until a separate product decision is made.

Before collecting step results, validate the program tool's actual capabilities:
source/filename handling, arguments, cwd/environment, build and runtime errors,
output limits, cancellation, foreground/background lifecycle, and recovery.
Policy and approval must cover both compilation and execution, including any
child commands. Compilers and fixtures are pinned, workspace state is isolated,
and tool results expose the evidence required by the task oracle.

These are implementation tests for the new tool and the shared Codex boundary,
not a revived transparent-launcher compatibility experiment. Reuse relevant
process cleanup and stream tests only where the program tool depends on them.
Shared Codex fixes apply to both experiment arms and cannot be reported as an
MBTX-specific step improvement.

## 4. Agent-step metric

### 4.1 Definition

Adopt the OpenSeek meaning of `agent_step`: **one logical provider round in the
agent loop**, from the request for a model response through the response that
ends that round. A response may contain zero, one, or many tool calls. The next
logical model request starts the next step. Reading already persisted checkpoint
events does not create steps; a new run against a fixed Responses replay has
real loop iterations and counts them normally.

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
| `agent_steps` | Completed logical provider rounds, excluding persisted replay copies |
| `agent_steps_started` | Logical rounds entered, including interrupted or failed rounds |
| `model_requests` | Requests sent to the provider, including a retry field |
| `tool_calls` | Model-requested calls, deduplicated by call ID |
| `tool_executions` | Actual tool invocations at observed dispatch boundaries |
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
step ID plus `experiment_id`, `pair_id`, `attempt_id`, `task_id`,
`execution_interface`, and `parent_id`.

First audit existing upstream IDs and reuse a logical-round ID if one has the
required lifecycle. Add `agent_step_id` only where the current native interface
does not expose that boundary. In the pinned code, CLI `turn.started` and
`turn.completed` describe a user turn, not each provider round. One turn may
contain several steps; neither event is a step counter by itself.

The native `codex exec --json` stream remains the durable primary source for
thread, turn, item, command, and terminal events. Native Codex OTel events remain
the diagnostic source for API requests, stream events, tool decisions, and tool
results. The evaluator joins these sources by run and call IDs; it does not count
OTel spans or JSONL rows as steps.

Missing IDs, incomplete streams, or a request whose step boundary cannot be
proved are `unknown`, not zero. Retry attempts retain their own request sequence
under the same logical step. Internal model reasoning that the provider does not
expose is not estimated from tokens or log-line counts.

### 4.4 Counting and attribution rules

- Allocate a step ID before the logical request; count `agent_steps` when a
  complete response is accepted. Retain `agent_steps_started` and the terminal
  status for failed or interrupted rounds. Report known partial counts and
  completeness separately rather than imputing a complete task total.
- One response with three tool calls followed by a final response is two
  completed steps and three tool calls. HTTP chunks, commentary items, and
  output polls do not each become a step.
- A failed transport attempt followed by a successful retry within one round is
  one completed step, two model requests, and one transport retry. A terminal
  429 before any response is zero completed steps, one started round, and a
  failed task; it is never scored as efficient completion.
- An explicit tool execution is different from a function call inside a program
  or an OS spawn. Nested executions are counted only when an actual invocation
  boundary is observed; arbitrary MoonBit statements are not tool executions.
- `process_spawns` is an observed count with a coverage field. In-process file
  operations and unobserved descendants cannot be inferred from it.
- An extra round following an error is observable. Calling it a `repair_step`
  requires an explicit recovery cause or a reviewed attribution rule. Ambiguous
  recovery stays `inferred` or `unknown`; keyword matching is insufficient.
- Compaction, review, and auxiliary model requests carry a request purpose and
  are counted separately. Freeze their configuration and include them in request
  and token totals. Internal provider reasoning and CPU instruction counts are
  outside this metric.

These rules measure visible agent interaction structure. Fewer steps alone do
not establish less model computation, fewer CPU instructions, lower monetary
cost, or faster execution.

## 5. Programmable MBTX step experiment

### 5.1 Execution interfaces

Compare:

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

Fixed Responses replay validates step counting, tool handling and observability
with known transcripts. Each interface has the tool arguments appropriate to
its schema. These validation runs are not a separate compatibility cohort or
evidence of reduced agent steps: that claim requires open-ended task execution.

### 5.2 Task and control matrix

Define 24 goal-oriented tasks, three per family, for step measurement. The old
24-scenario launcher suite stays in the historical branch; the new suite uses
capability oracles designed for the program tool:

| Task family | Three proposed cases | Acceptance evidence |
|---|---|---|
| Repository inspection | Locate a declaration; join configuration references; summarize dependency relationships | Exact paths and structured facts checked against a frozen fixture |
| Structured data | Filter JSON; aggregate JSONL; join two datasets | Canonical structured result and expected error handling |
| File transformation | Update one file; apply a rule across files; make a repeated transformation idempotent | Content hashes and allowed-change manifest |
| Command orchestration | Run dependent commands; branch on exit status; collect several independent command results | Receipts, exit codes, and expected artifacts |
| Diagnostics | Locate a failing test; identify a configuration mismatch; explain a controlled error | Fault identity and bounded diagnostic output |
| Repair | Fix a small syntax error; correct a data conversion; repair a failing test | External acceptance tests and edit-scope oracle |
| Output handling | Summarize large streams; preserve Unicode records; extract data from mixed diagnostic output | Exact records, completeness and truncation checks |
| Session recovery | Poll a background task; cancel and continue; recover after a failed execution | Session trajectory, final result and controlled-process cleanup |

Use neutral goal prompts and equally detailed interface documentation. Freeze
the common system instructions, model identifier, reasoning settings, context
budget, output caps, compiler versions and dependencies. Store both tool schemas;
their difference is the intended treatment and must not be hidden as an input
equality check. Shell may use installed utilities and complete scripts. Both
arms have equivalent task capabilities, file permissions and acceptance tests.
If equivalent permissions cannot be implemented, report that scenario as an
interface-plus-policy comparison, not a launcher effect.

Disable optional delegation and Code Mode for the initial programmable cohort.
If Code Mode is studied later, enable the same mode in both arms and report it
as a separate stratum. Record successful compilation, compile errors and cache
hits in the program arm; one program containing many operations remains one
model tool call, while its internal operations remain observable separately.

### 5.3 Sample design and analysis

The proposed pilot is six representative tasks with two pairs each (12 pairs).
Its purpose is to check task difficulty, counting completeness and request cost;
it is stored separately from formal results. A proposed first formal dataset is
24 tasks with four pairs each (96 attempted pairs), split into two balanced
rounds. This replaces reuse of the old 192-pair latency schedule for the new
question; it does not alter that schedule or its historical results.

Before collecting formal data, freeze task IDs, a seed for task order, AB/BA
allocation, sample count, a 12-step limit per arm, tool/context budgets and the
timeout in a versioned manifest. If the pilot shows the budget is unsuitable,
revise the protocol before formal collection and record the decision without
selecting tasks for favorable MBTX outcomes. Ninety-six pairs is an initial
coverage budget, not a claim of statistical power for every task.

For this goal-oriented experiment, collect a fixed number of attempts rather
than filling a quota of successful pairs. Retain all failures and use an
intention-to-treat curve of oracle-verified success by step budget. Report the
success-rate difference alongside paired step differences among pairs where
both arms succeeded. The latter is a conditional analysis and cannot conceal
one arm's larger failure rate. Relay failures and censored attempts receive
their own breakdown; they are neither zero-step successes nor backend faults.

Use a fixed seed for 95% bootstrap intervals, preserving pair identity and task
clustering. Show per-task counts, aggregate absolute step differences and
ratios only where denominators are valid. Report medians and distributions,
not unsupported per-task p99 claims. Additional formal samples require a new
predeclared tranche; do not stop early when a favorable interval appears.

### 5.4 Request budget and runtime

Reuse the current proxy's run-wide gate: one complete upstream stream at a time,
with a default 15-second minimum between request starts (at most four RPM from
this run). Probes and auxiliary requests must use the same gate. Keep Codex's
automatic request/stream retries disabled in the collection profile; retain
429 as a failure and apply `Retry-After`, or a 30-second fallback, to subsequent
requests. Record pacing and cooldown separately. Another application using the
same account is outside this gate, so this bounds collection load rather than
guaranteeing that the relay never returns 429.

Estimate pacing from requests, not pairs: approximately
`2 * pairs * mean_requests_per_arm * interval`. For 96 pairs at 15 seconds,
three to six requests per arm imply about 2.4 to 4.8 hours of pacing; 12 requests
per arm imply about 9.6 hours. Tool work, slow streams and cooldown can extend
this. Use the pilot to replace these planning examples with an observed request
budget before a full run. Persist and resume between pairs without rewriting
completed attempts.

Preserve the current permissive infrastructure gates: at least one successful
probe out of three; pause after five consecutive final infrastructure failures,
at least eight infrastructure failures in the latest 16 arms, or loss of a core
collector component. Produce a partial report with all completed evidence.

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
  │   │   ├─ program build / compiler (when applicable)
  │   │   ├─ script or program execution / child processes
  │   │   └─ wait / reap / stream drain
  │   └─ tool result
  └─ terminal result
```

Relay and provider events are linked by request ID and represented as external
segments. If server-side timing is unavailable, model computation and relay
queueing remain one external interval and are labeled `unknown` internally.
Trace attributes include execution interface, task, pair, round, step, call ID, phase,
status, failure class, platform, execution mode, and evidence path/hash.

The step span can enclose the response and its resulting tool batch; its duration
is not itself the step counter. Preserve native trace/span IDs and use links
when asynchronous lifetimes overlap rather than inventing a nested sequence.
If a combined attempt trace is reconstructed offline, mark it `derived`, retain
the original IDs and the source-event mapping, and label missing boundaries.

OTLP timestamps use Unix epoch time, while the primary OS events use a monotonic
clock. Capture a documented clock anchor and uncertainty for display conversion.
Only aligned times may share a quantitative timeline; wall-clock adjustments,
missing anchors, or cross-host uncertainty must remain visible. All formal
duration arithmetic continues to use the original comparable monotonic domain.

### 6.2 Ingestion and measurement isolation

The formal minimal profile must not start SigNoz, a collector, or a second
observer in the measured process path. It writes the common OS-monotonic event
layer and native JSONL needed for counters, under the same profile for both arms.
Diagnostic runs export to a loopback OTel collector or saved OTLP
files, and the files are imported into SigNoz after the run.

The Shell tool and MBTX program tool use the same observation profile. A trace-on versus trace-off
calibration quantifies observer perturbation; trace export time and database
ingestion time are never included in tool build or execution intervals.

Capture event timestamps before buffering/writing them. Record buffer overflow,
missing events and flush time; do not silently drop rows. Equal instrumentation
does not prove zero perturbation, so publish the calibration and its limits
instead of describing measured time as disturbance-free.

Self-hosted SigNoz may run on Linux and be viewed from macOS over a controlled
network. Its OTLP endpoints are normally 4317 (gRPC) and 4318 (HTTP). The
dashboard setup is operational tooling and must not become a dependency of
offline replay or evidence reconstruction.

### 6.3 Required views

The first dashboard set should provide:

1. pair selector with Shell-tool/MBTX-program side-by-side step timelines;
2. step histogram and step-to-success distribution;
3. first observed step or tool-path divergence, grouped by model decision,
   tool build, program execution, Codex, relay, or unknown;
4. API/retry/429/stream-error filters;
5. process tree, signal, wait/reap, and IO-drain details;
6. links from each span to the immutable attempt evidence and its hash.

Use SigNoz's existing filters, trace details and dashboards first. Validate the
chosen version's support for pair navigation; if it cannot embed two waterfalls
in one view, provide paired trace links or two trace panes. Do not start another
custom trace UI to reproduce that capability. Keep offline HTML as a small
readable report summary with raw-evidence and trace links.

SigNoz visualizations are explanatory. The report generator remains the authority
for pair selection, confidence intervals, ITT treatment, and conclusion labels.

## 7. Implementation phases and gates

### Phase A — freeze and fork scaffold

- Verify the archive branch points to the pre-plan baseline hash recorded above.
- Create the upstream Codex fork and document remotes, ownership, and license.
- Import the pinned upstream commit into a normal fork branch.
- Build upstream Codex from a fresh clone before adding MBTX.
- Include the optional evaluation directory and references to frozen evidence.

**Gate:** a fresh upstream fork clone builds with ordinary documented commands;
no patch or overlay is needed for the upstream baseline.

### Phase B — direct programmable MBTX integration

- Define the MoonBit program tool contract, source/filename policy, arguments,
  target, sandbox, build cache, foreground/background behavior, and output cap.
- Register the tool and its opt-in configuration directly in Codex source.
- Make compilation, execution and background lifecycle separately observable.
- Implement approval, sandbox, error propagation, output and cleanup tests for
  the program tool and its actual runtime dependencies.
- Add a one-command build and install path suitable for an external contributor.

**Gate:** the program tool and upstream Shell control work from a fresh clone on
supported Linux/macOS targets. The tool's capability and policy tests pass, and
installation requires no patch or generated upstream checkout.

### Phase C — explicit step events

- Reuse native loop IDs and add the missing logical `agent_step_id` boundary.
- Add parent IDs and retry sequence fields to JSONL and OTel records.
- Update the evaluator to derive step and related-counter vectors.
- Add fixtures for one response with many tools, retries inside one step,
  checkpoint replay, tool failure followed by repair, and cancelled turns.

**Gate:** fixed replay counts match hand-checked transcripts and no duplicate or
missing step IDs occur in complete evidence.

### Phase D — task and control implementation

- Implement the 24 goal-oriented tasks and their independent acceptance oracles.
- Configure the `shell_tool` and `mbtx_program` arms with equivalent capabilities,
  permissions and budgets; allow complete Shell scripts.
- Validate known transcripts for each interface using fixed Responses replay.
- Record build/cache state, task artifacts and actual tool arguments.
- Keep historical launcher scenarios and reports out of the new collection path.

**Gate:** both interfaces can satisfy each oracle under the declared policy.
Replay proves the counters and evidence pipeline work; the step experiment
allows the model to choose its trajectory under neutral goal prompts.

### Phase E — OTel and SigNoz export

- Normalize Codex, program build, execution, relay and evaluator records into the trace
  shape above.
- Validate trace IDs, parent relationships, null handling, and evidence links.
- Add a local diagnostic export command and a documented self-hosted SigNoz
  profile. Keep the default formal profile exporter-free.
- Confirm that importing a derived trace never changes the original evidence.

**Gate:** validation transcripts for both tools can be inspected in SigNoz, and
their raw evidence independently regenerates the same counts without SigNoz.

### Phase F — step-focused evaluation

- Run a small offline replay matrix first.
- Run limited online validation only after the offline counters and oracles are
  stable.
- Keep requests serial, use the existing start interval and `Retry-After`
  handling, and retain all relay/provider failures.
- Expand after checking shared task/model/fixture/permission controls. Measure
  different trajectories under the same goal and declared tool treatment;
  fixed validation transcripts are not included in the formal step dataset.

**Gate:** publish a step report with ITT, successful comparable attempts,
external-failure counts, and explicit unknowns. Do not generalize a step result
beyond the tested tasks, interfaces, model, platform and policy.

### Phase G — distribution and documentation

- Replace patch-era instructions with fork clone, build, configuration, and
  upgrade instructions.
- Link the archived repository branch and explain its historical role.
- Document the programmable step experiment as the sole active research scope;
  link the historical branch for the transparent-launcher research.
- Publish English Markdown, JSON, CSV, offline HTML, and SigNoz import guidance.
- Include a short contributor guide for synchronizing with `upstream`.

### CI and build acceptance

Preserve one aggregate required status and separate three layers:

- Fast PR checks: MoonBit/Rust checks, program-tool contracts, JSONL/OTLP counter
  fixtures, report parsing, schema and formatting checks; no relay.
- Offline integration: build the fork, program tooling and immutable fixture once;
  run real Codex transcript validation and program-tool policy, output and
  lifecycle regressions without rerunning the historical launcher suite.
  Exercise missing/corrupt events, collector interruption, 429/5xx/stream faults,
  immutable attempts, HTML escaping and null preservation.
- Release validation: build supported Linux/macOS bundles, verify checksums and
  fresh-clone/install/configuration instructions, and check upstream ancestry.
  Live relay collection remains an explicit local operation.

Cache keys include source, lock files, toolchains, platform, target, profile and
fixture hash. Documentation-only changes do not rebuild Codex. A second build
with identical inputs must reuse the bundle. Fixtures and host binaries are
never compiled inside measurement loops. Compiling a newly model-authored
MoonBit program is legitimate tool work in the programmable cohort and must be
recorded; cold compilation and a verified cache hit are separate conditions.

## 8. Acceptance criteria

The migration is complete only when all of the following are true:

- a clean clone of the direct Codex fork builds and runs without applying a
  project-local patch or overlay;
- upstream synchronization and dependency revisions are reproducible;
- upstream Shell behavior remains the default and the program tool is opt-in;
- program-tool capability, policy and lifecycle tests pass on supported platforms;
- step IDs, tool IDs, process IDs, retries, and terminal states are traceable;
- replay validation is identified separately from formal step measurements;
- the active suite contains only the Shell-tool versus MBTX-program comparison;
- SigNoz shows both arms through the same OTel observation layer;
- formal timing excludes exporter, report, and database work;
- raw evidence alone regenerates every reported count and conclusion;
- relay/provider errors are never classified as backend failures;
- lower steps are reported with oracle correctness, ITT failures, conditional
  sample selection and explicit scope;
- no conclusion is generalized beyond the tested model, platform, toolchain,
  execution interface, and sample design.

## 9. Non-goals and unresolved decisions

This cycle will not change Codex's default Shell tool, claim universal shell
replacement, infer hidden model reasoning steps, or treat SigNoz as a source of
truth. It will not merge SeekMoon into the project; SeekMoon remains a client
for OpenSeek conversations, while SigNoz is the observability view for this
evaluation.

The implementation defaults are the provisional fork name in Section 2 and an
optional evaluator inside that fork. Validate name availability when creating
the fork. Final source paths and SigNoz version are implementation decisions;
record them in the build/export manifest. The runtime target and permission
mapping of the programmable interface must pass the capability-parity gate
before its online comparison begins. This document introduces no new results.

## 10. Reference points

- [GitHub fork model](https://docs.github.com/en/pull-requests/reference/forks):
  repository relationship and collaboration semantics.
- [Pinned OpenSeek loop](https://github.com/moonbitlang/openseek/blob/d818b71b1760b3f9f78cf662c662af2e716edbb1/agent/turn_loop.mbt):
  provider-round step semantics; the new metric uses this as its reference.
- [Pinned OpenSeek MBTX tool](https://github.com/moonbitlang/openseek/blob/d818b71b1760b3f9f78cf662c662af2e716edbb1/agent_tool/mbtx/README.mbt.md):
  programmable execution and explicit build/run phases. Its implementation and
  policy are reference material for the new program tool.
- [Pinned Codex JSONL events](https://github.com/openai/codex/blob/3d2ee51ca2d5db578f328aa75e20aa22c0197c9a/codex-rs/exec/src/exec_events.rs)
  and [native telemetry](https://github.com/openai/codex/blob/3d2ee51ca2d5db578f328aa75e20aa22c0197c9a/codex-rs/otel/src/events/session_telemetry.rs):
  the upstream surfaces to extend only where necessary.
- [SigNoz Trace Explorer](https://signoz.io/docs/userguide/traces/) and
  [trace details](https://signoz.io/docs/userguide/span-details/): native analysis
  views to validate before designing extra presentation code.
- [Historical architecture](https://github.com/ZSeanYves/Codex-MBTX/blob/3527b02a4bc35df8017ac8d1f3b296d9fcdc1d2c/docs/architecture.md),
  [historical protocol](https://github.com/ZSeanYves/Codex-MBTX/blob/3527b02a4bc35df8017ac8d1f3b296d9fcdc1d2c/docs/evaluation.md)
  and [verification record](https://github.com/ZSeanYves/Codex-MBTX/blob/3527b02a4bc35df8017ac8d1f3b296d9fcdc1d2c/docs/code-validation.md):
  frozen background evidence, not an additional experiment in this plan.
