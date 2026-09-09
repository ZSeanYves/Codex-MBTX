# M4 evaluation

The [2026-09-09 delivery report](reports/m4-2026-09-09.md) records the earlier
explicit-MBTX cohort. It is useful for compatibility and script-authoring
behavior, but it is not evidence for transparent process replacement. The
current M4 plan is a fresh shell/transparent comparison; its report is kept
separate so the two questions cannot be conflated.

The first full transparent comparison is recorded in
[m4-transparent-2026-09-09](reports/m4-transparent-2026-09-09.md). It is a
complete 24-row observation set, but the workflow failed its evidence gate on
relay overload and therefore does not support a performance or reliability
claim.

The expanded backend-only run uses the same Codex build and relay, but selects
six host-process tasks that do not ask the model to create, edit, or run a
`.mbtx` file. It is the cleanest test of transparent process replacement. The
`full` mode contains all twelve tasks (six process tasks and six deliberate
MoonBit-script tasks), two backends, and two repetitions: 48 rollouts. The
`backend` mode contains only the six process tasks: 24 rollouts.

The completed backend-only observation is recorded in
[m4-backend-2026-09-09](reports/m4-backend-2026-09-09.md). Its 24 rows are
complete, but relay overload and stream disconnects prevent a performance or
reliability claim.

CI pins MoonBit to `0.10.11+6ff76a5f9`, the version used for M3 verification.
Using the rolling `latest` changed dependency warnings during M4 development
and contaminated stderr assertions. The installer honors
`MOONBIT_INSTALL_VERSION`; local Codex configuration is never modified.

M4 is a bounded, reproducible pilot for the M3 adapter. It runs a fresh
workspace and home directory through the shell baseline and the transparent
backend, records one structured row per run, and grades the result against an
answer key that is kept inside the evaluator. The model's final message is not
treated as proof of success.

Prompt revision 3 supplies the same `eval/MOONBIT.md` reference inline to both
backends, in addition to placing it in the workspace. Both groups receive the
same model-visible tools and routing instructions; transparent mode is selected
only in the local Codex configuration. Each workspace is
an empty Git repository and both backends have ripgrep installed. Schema
version 4 encodes the workload class and observed file contents as a JSON string
or null; the evidence
CLI rejects other shapes instead of classifying invalid evidence as a model
failure. Earlier diagnostic scores affected by the array-wrapping bug are
identified in the report and are excluded from the formal comparison.

The process runner accepts warning JSON records that MoonBit may emit before
its artifact record, preserving them as stderr diagnostics while selecting the
artifact from the later JSON line. This keeps warning-producing scripts
executable and prevents a runner protocol error from being attributed to MBTX.

The script workload covers structured JSON and JSONL processing, paths containing
spaces and shell metacharacters, literal argv, a small MoonBit repair, and a
background job. The process workload repeats the host-side equivalents with
Python helpers and adds stdin streaming, bounded large output, and nonzero-exit
recovery. Process tasks intentionally do not invoke MoonBit. Tasks are run twice
in `full` mode; `backend` runs only the process workload. Backend order alternates
by task and repetition. Each run has a 300 second wall limit. The observed-token
budget is 4,000,000 for `full` and 2,000,000 for `backend` or `smoke`. A run is classified as `none` only
when the output is correct, inputs are preserved, required usage is present,
the model completed, and the requested backend was actually used. Other
categories include `incorrect_output`, `input_modified`, `backend_violation`,
`intervention_required`, `timeout`, `agent_error`, `provider_error`,
`missing_usage`, and `audit_error`.

The `.mbtx` fixtures are deliberate script-workload cases. When a model runs
`moon run capture.mbtx`, repairs `broken.mbtx`, or starts `worker.mbtx`, the
MoonBit compiler time belongs to that requested task in both cohorts. It is not
evidence that transparent MBTX compiled a hidden script. Only the transparent
launcher itself is evaluated as the candidate backend; it receives the final
resolved shell argv and does not call the MBTX script protocol.

This separation also fixes the interpretation of model familiarity. A model
must know enough MoonBit syntax and APIs only in the script workload because the
task explicitly asks it to author or repair MoonBit. The process workload still
uses the normal `exec_command` and `write_stdin` tools, but its helpers are
already-written host programs; any difference there is attributable to command
construction, approval/sandbox integration, launcher behavior, process I/O, or
the relay, rather than MoonBit syntax.

The evaluator uses the pinned Codex Responses API proxy from the upstream
source. The workflow passes the repository secret to the proxy over stdin; the
agent runs as the unprivileged `mbtx-eval` user and cannot read that stdin or
the private dump directory. Raw request/response dumps, prompts, and model
text remain in a temporary private directory. The committed JSON and Markdown
reports contain only task/backend/repetition, timing, sanitized tool counts,
provider-reported token fields, model names, classifications, and bounded
failure diagnostics, and the observed `result.json` file from the public
fixture workspace (null when absent). MBTX stderr and shell output are retained
only for failed calls, with at most four distinct excerpts of 2,048 characters
per backend per rollout. Successful tool output and model reasoning are not
published.

The installed toolchain is read-only. Its separate dependency cache is owned by
the agent account and is explicitly writable in the Codex sandbox because Moon
requires a cache lock even when every dependency is already downloaded. This
cache is shared between runs; each task's workspace and Codex home are fresh.
Before using the API, CI executes an MBTX fixture as the isolated account
through `codex sandbox` on Linux with the same workspace and cache policy.

## Local checks

The evaluator package and its parser tests are part of the ordinary MoonBit
suite. These checks run without credentials:

```text
moon test --target wasm
moon test --target native
moon run --build-only --deny-warn scripts/m4_eval.mbtx
moon run scripts/m4_verify.mbtx
```

The live command `moon run scripts/m4_eval.mbtx smoke` requires the isolated
Linux CI tools and the repository secret; it does not use local Codex auth.
The `.mbtx` scripts drive processes; `cmd/m4-evidence` is the local, testable
JSON evidence transformer. Script mode resolves registry modules, so the
unpublished evaluator package is accessed through this command.

## Remote run

Actions exposes **M4 Evaluation** as a manual workflow. `full` runs 48 agent
rollouts, `backend` runs 24 process-only rollouts, and `smoke` runs one task
with both backends and is useful for relay/proxy diagnostics. Set
the model and reasoning inputs to the exact values being compared, for example
`gpt-5.6-terra` and `xhigh`. The workflow is not a PR gate and does not claim a
general performance improvement from this small pilot. A default backend should
only be changed after a larger task set, repeated runs, cost data, and review of
the raw private evidence. Until then transparent MBTX remains an explicit
opt-in.

`probe` makes one bounded Responses request without starting an agent, reports
HTTP and transport diagnostics, and is excluded from the evaluation score.
`binary_run` may reference a successful M3 workflow in this repository. Before
reusing its artifact, the builder compares all Codex patches and MBTX runtime
sources with that run's commit; any difference rejects reuse. The built-in
`codex responses-api-proxy` command provides the isolated credential proxy.
Omitting `binary_run` builds from source. Reports include both the evaluator
commit and binary provenance.

If the observed-token guard stops a batch, `start_rollout` can explicitly
continue from an absolute zero-based index (for example, `42` runs the last six
slots of `full`). The original task/backend/repetition order is preserved; smoke
always starts at zero and runs both backends. Reports distinguish the original
`planned_rollouts` from the segment's `expected_rollouts` and `start_rollout`.
Each invocation has its own guard, so continuing authorizes additional requests;
aggregate usage across all segments and retain every earlier failure. A green
continuation validates only that segment and does not change the previous
workflow's result. Keep the same model, effort, prompt, suite and runner revision
when combining segments, and record their separate workflow provenance.

Example dispatch (the referenced binary source must still be available):

```text
gh workflow run m4-evaluation.yml --ref codex/m4-evaluation -f mode=backend -f model=gpt-5.6-terra -f reasoning_effort=xhigh
```

The comparison is intentionally same-prompt: transparent MBTX is enabled only
in the candidate configuration, while both groups expose the normal
`exec_command` and `write_stdin` tools. Calling an explicit `mbtx` function or
another execution backend is a backend violation, even if file output is
correct. `usage_complete=false`
explicitly marks incomplete accounting. Token totals are observed lower bounds
for those runs. Reported model names and tokens come from the relay; no model
identity or relay price is inferred from them. This pilot is not an adversarial
grader or a measurement of interactive human intervention: all runs use
approval `never`, with intervention requests counted separately.

The pinned Terra catalog forces `code_mode_only` even when feature flags are
false. M4 uses the supported `model_catalog_json` override with exactly one
catalog change: `tool_mode=direct`, identical for both backends. This pilot
therefore measures direct tool execution, not the JavaScript Code Mode host.
The M3 two-binary archive does not include that host. The gateway also showed
long first-byte waits and transient overload; before the formal baseline, the
run deadline was set to 300 seconds, with two request retries and one stream
retry. All attempts remain in the usage audit.
