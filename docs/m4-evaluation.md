# M4 evaluation

M4 is a bounded, reproducible pilot for the M3 adapter. It runs the same six
tasks with a fresh workspace and home directory through the shell baseline and
the MBTX tool, records one structured row per run, and grades the result against
an answer key that is kept inside the evaluator. The model's final message is
not treated as proof of success.

The task set covers structured JSON and JSONL processing, paths containing
spaces and shell metacharacters, literal argv, a small MoonBit repair, and a
background job. Tasks are run twice in `full` mode. Backend order alternates by
task and repetition. Each run has a 180 second wall limit and the complete
pilot has a 2,000,000 observed-token budget. A run is classified as `none` only
when the output is correct, inputs are preserved, required usage is present,
the model completed, and the requested backend was actually used. Other
categories include `incorrect_output`, `input_modified`, `backend_violation`,
`intervention_required`, `timeout`, `agent_error`, `provider_error`,
`missing_usage`, and `audit_error`.

The evaluator uses the pinned Codex Responses API proxy from the upstream
source. The workflow passes the repository secret to the proxy over stdin; the
agent runs as the unprivileged `mbtx-eval` user and cannot read that stdin or
the private dump directory. Raw request/response dumps, prompts, and model
text remain in a temporary private directory. The committed JSON and Markdown
reports contain only task/backend/repetition, timing, sanitized tool counts,
provider-reported token fields, model names, and classifications.

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

## Remote run

Actions exposes **M4 Evaluation** as a manual workflow. `full` runs 24 agent
rollouts, each potentially making multiple API requests; `smoke` runs one task
with both backends and is useful for relay/proxy diagnostics. Set
the model and reasoning inputs to the exact values being compared, for example
`gpt-5.6-terra` and `xhigh`. The workflow is not a PR gate and does not claim a
general performance improvement from this small pilot. A default backend should
only be changed after a larger task set, repeated runs, cost data, and review of
the raw private evidence. Until then MBTX remains an explicit opt-in.
