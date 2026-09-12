# Launcher architecture

The product boundary is deliberately small:

```text
Codex unified_exec -> approval / policy / sandbox -> mbtx exec -- resolved argv
  -> child process -> wait / reap / drain IO -> Codex result
```

MBTX is a process launcher. It is not a shell parser and it is not a MoonBit
script runtime. Codex keeps the original command for approval and policy
decisions; the trusted launcher is inserted only after those decisions and only
for local transparent execution.

The launcher returns ordinary exit codes and re-raises negative signal results.
The host remains responsible for process-group scope and cancellation policy:
request cancellation, select scope, signal, wait, reap, drain both output
streams, and publish the terminal event.

Rust observes OS and Codex boundaries; the report renderer consumes immutable
JSONL events. Missing values remain null or unknown. Relay, provider, harness,
timeout, censored, and backend failures remain distinct.
