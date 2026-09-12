# Linux Codex relay evidence

`moon run scripts/collect-linux-online.mbtx` creates one timestamped run
directory here. Each run contains the run-local Shell and Transparent config,
raw Codex JSONL/stderr per attempt, immutable attempt metadata, the event log,
and Markdown/JSON/CSV/HTML reports.

The API key is inherited from `OPENAI_API_KEY` and is never written to this
directory. Failed attempts remain alongside successful pairs. Upload this
directory after a run; macOS can inspect it after `git pull` without mixing it
with local launcher evidence.
