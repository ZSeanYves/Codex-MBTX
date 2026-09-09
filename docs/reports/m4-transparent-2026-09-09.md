# M4 Transparent Backend Evaluation (2026-09-09)

This report records the first full shell/transparent comparison after the
transparent MBTX path was implemented. The workflow was
[34328381894](https://github.com/ZSeanYves/Codex-MBTX/actions/runs/34328381894)
at implementation commit `b193ab21588587fd2d0011776efa8b294db98c84`.
It used `gpt-5.6-terra` with `xhigh`, prompt revision 3, schema version 3,
the pinned MoonBit toolchain, and 24 rollouts (six tasks, two repetitions,
both backends). The job recorded all 24 rows but ended unsuccessfully because
the evidence gate saw provider errors.

## Result

| Backend | Passed | Correct output | Mean wall ms | Requests | Input tokens | Cached input | Output tokens | Usage responses | Failure categories |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| shell | 9/12 | 10/12 | 172,155 | 73 | 670,873 | 510,464 | 19,562 | 68/73 | none 9, timeout 2, provider 1 |
| transparent | 4/12 | 7/12 | 187,272 | 63 | 562,343 | 479,744 | 15,507 | 51/63 | none 4, timeout 2, provider 5, incorrect 1 |

The numbers do not establish a transparent-backend regression or an MBTX
advantage. Five of the transparent provider failures and one shell provider
failure reported `Our servers are currently overloaded. Please try again
later.` or an equivalent stream-disconnect diagnostic. They occur in the
latter half of the alternating schedule (rollouts 12--23), which is a relay
availability confound. Both backends also had two 300-second timeouts. The
transparent `literal_argv` first repetition produced `quote'$` instead of the
expected `quote'"$`; its second repetition passed, with no command failure or
backend violation. That pattern is consistent with model/task stochasticity,
not a deterministic launcher transformation.

In this comparison, no completed row in either cohort used the `mbtx` tool. Every completed
transparent row used the normal `exec_command` contract (and the background
case also used `write_stdin`). Rows that ended before a completed response are
marked non-compliant by the evidence schema because backend use cannot be
proven after an interrupted turn; this is not evidence that the model selected
a forbidden tool.

## What Each Cause Means

| Cause | Evidence in this run | Interpretation |
| --- | --- | --- |
| Task design | `json_totals`, `jsonl_recovery`, `path_inventory`, `literal_argv`, `repair_moon`, and `background_job` deliberately include `.mbtx` fixtures. | When a model runs `moon run capture.mbtx`, repairs `broken.mbtx`, or starts `worker.mbtx`, that compiler work belongs to the requested task. It is present in both cohorts and is not hidden compilation by transparent MBTX. |
| Model and relay | MoonBit fixture authoring can require syntax/API recovery. The live run also recorded overload diagnostics and paired timeouts in both cohorts. | Explicit MoonBit tasks still require MoonBit knowledge. Relay overload, response length, retries, and model stochasticity affect completion and latency independently of the launcher. |
| Codex adapter | Transparent configuration kept `exec_command`/`write_stdin`, passed the original command through approval and sandbox planning, then added the trusted launcher at final launch. Approval and executable fixture tests passed. | There is no adapter-only failure signal in this run. A future run should still instrument launch/exit events to measure any wrapper-specific overhead. |
| MBTX runtime | `mbtx exec --` forwards literal argv and inherited streams, waits for the child, and preserves its exit status. It does not enter the JSONL script protocol or call `moon run`. The local and remote fixture checks passed. | The transparent runtime is a process proxy, not a MoonBit script compiler. This run does not prove a speed or reliability gain; its large wall-time differences are dominated by model/relay behavior. |

## Why MoonBit Familiarity Still Appears

There are two different contracts. In the historical explicit MBTX mode, the
model calls an `mbtx` function and supplies MoonBit source or a `.mbtx` path.
It therefore has to know the MBTX request schema, MoonBit syntax, and APIs well
enough to generate a compilable script. The earlier explicit cohort recorded
script-exit diagnostics such as missing async imports, invalid Map iteration,
an unnecessary `mut`, and calls to unavailable APIs; those are script-authoring
failures.

The transparent mode tested here exposes the ordinary Codex
`exec_command`/`write_stdin` tools and keeps MBTX internal. The model does not
need to know that the final process is launched through MBTX. It can still need
MoonBit knowledge when the task itself explicitly asks it to edit or run a
MoonBit fixture. That requirement comes from the workload, not from the
transparent backend.

## What This Run Can and Cannot Claim

It validates the transparent contract and its integration with approval,
sandbox planning, streams, exit status, and the existing automation path. It
does not support claims that MBTX is faster, cheaper, more reliable, or better
at MoonBit generation. The workflow was not green, provider availability was
not balanced across the alternating order, and 24 rollouts are too few for a
statistical comparison.

The next measurement should use a stable relay and a separate backend-focused
suite: prebuilt helper programs or ordinary host commands, no `.mbtx` source
generation, randomized paired order, repeated runs, and counters that record
whether `moon` or the MBTX script protocol was invoked. The MoonBit repair and
script-authoring tasks should remain a separate capability cohort. Until those
measurements exist, transparent MBTX should remain opt-in.

The sanitized source artifact is available as
`_build/m4-remote-34328381894/report.json` and `report.md` in the local
workspace; the workflow keeps raw prompts and request/response dumps private.
