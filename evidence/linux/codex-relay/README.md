# Linux Codex relay evidence

`moon run scripts/collect-linux-online.mbtx` creates one timestamped run
directory here. Each run contains the run-local Shell and Transparent config,
raw Codex JSONL/stderr per attempt, immutable attempt metadata, the event log,
and Markdown/JSON/CSV/HTML reports.

The API key is inherited from `OPENAI_API_KEY` and is never written to this
directory. Failed attempts remain alongside successful pairs. Upload this
directory after a run; macOS can inspect it after `git pull` without mixing it
with local launcher evidence.

The current tree retains the [eight-category baseline](20260913T152436165546119/review/README.md)
and the [expanded online run](20260914T072517-1789341917874/report.md).
The [report index](../../../docs/reports/README.md) distinguishes their protocols,
sample counts and conclusion limits. Failed attempts within retained runs
remain available. Removed preliminary runs remain in Git history.
