# Evaluation Reports

Reports describe measured behavior, performance and evidence integrity for
specific configurations. Implementation checks are documented separately in the
[code verification record](../code-validation.md).

## Retained Studies

| Study | Evidence | Main result | Interpretation |
| --- | --- | --- | --- |
| [Expanded Linux evaluation, 2026-09-14](linux-evidence-review-2026-09-14/README.md) | 24 scenarios; 240 offline pairs; 209 online attempted pairs; 6,000 startup pairs | Offline 240/240; online strict 183/192; approximately +0.49 ms isolated startup cost | Tested compatibility supported; no established end-to-end speed advantage |
| [Linux baseline, 2026-09-13](../../evidence/linux/codex-relay/20260913T152436165546119/review/README.md) | 8 task categories; 32 complete online pairs | Shell 19.674 s, MBTX 19.817 s mean end-to-end time | Basic compatibility; timing lacks launcher-stage attribution |

These studies use different task definitions and observation facilities.
Their counts and timings must not be pooled. The baseline report describes its
original protocol and limitations; its proposed extensions are assessed by the
later report, not retroactively treated as baseline coverage.

The expanded report provides [JSON statistics](linux-evidence-review-2026-09-14/statistics.json),
[CSV statistics](linux-evidence-review-2026-09-14/statistics.csv),
an [integrity audit](linux-evidence-review-2026-09-14/audit.json) and a
[PTY recovery ledger](linux-evidence-review-2026-09-14/pty-recovery.json).
Original run directories retain generated Markdown, JSON, CSV, HTML and, where
available, trace artifacts. Some large files are gzip-compressed.

## Evidence Inventory

| Dataset | Path | Scope |
| --- | --- | --- |
| Expanded online | [20260914T072517-1789341917874](../../evidence/linux/codex-relay/20260914T072517-1789341917874/) | Linux, Code Mode, observed Direct Shell; partial coverage |
| Online baseline | [20260913T152436165546119](../../evidence/linux/codex-relay/20260913T152436165546119/) | Linux, earlier eight-category protocol |
| Fixed replay | [20260914T045159-1789332720090](../../evidence/linux/codex-replay/20260914T045159-1789332720090/) | Linux, 24 scenarios, fixed model responses |
| Startup attribution | [20260914T044744-1789332464819](../../evidence/linux/launcher/20260914T044744-1789332464819/) | Linux, release launcher, minimal/full observation |
| Earlier launcher comparison | [20260913T055354901398588](../../evidence/linux/launcher/20260913T055354901398588/) | Debug launcher; different timing boundary; not pooled with startup attribution |

No macOS measurement dataset is published in the current tree. The
[macOS evidence directory](../../evidence/macos/launcher/README.md) remains a
separate collection destination.

## Retention and Interpretation

The current tree retains the two online studies above. Preliminary online runs
and superseded experimental documents were removed from the working tree;
their previous contents remain available at
[data revision 370432d](https://github.com/ZSeanYves/Codex-MBTX/tree/370432d3289f0c5be790788f136f47db5cabbc9f).
This retention change does not remove failed attempts within either retained
run. The retained windows do not estimate the relay's overall historical
availability.

All report narrative and developer documentation use English. Original Unicode
test inputs, captured commands and provider output remain unchanged. These
strings are experimental data, not report prose.

The expanded study's PTY artifacts require the exact-byte recovery documented
in its integrity section before a fresh checkout can reproduce its strict
counts. The statistical findings are conditional on that verified recovery.
The default backend remains Shell.
