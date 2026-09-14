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

The native launcher returns ordinary exit codes and re-raises negative signal
results after unblocking that signal. Numeric 143 and an actual SIGTERM therefore
remain distinct in the parent's Unix wait status. The launcher observes incoming
INT/TERM/HUP, forwards PID-scoped signals to its child, and escalates to SIGKILL
after 500 ms if that child has not exited. Its existing wait owns child reaping.

Codex owns the process group and stream readers. For transparent launches it
sets the internal `MBTX_CANCEL_SCOPE=caller-process-group` marker, which MBTX
consumes before spawning the child. This avoids sending a group-delivered signal
to the child twice. The collector's controlled TERM test sends group SIGTERM,
then SIGKILL after 500 ms when the recorded PID identity still matches. Native
Ctrl-C is tested separately through the real unified_exec session.

The terminal result must follow wait and IO drain. A wait-return timestamp is
the parent's observation, not a kernel exit timestamp. Residual fixture processes
are recorded before fallback cleanup; detached or unobserved descendants are not
assumed reaped. Cleanup never targets an entire UID.

Linux bubblewrap gives each invocation a PID namespace. A fixture records its
namespace and kernel process start time; the collector resolves these to a host
PID before signalling. Missing identity evidence stays unknown. Receipt filenames
also include the observed start timestamp, so namespace PID reuse cannot overwrite
a previous invocation. Tool call IDs connect spawn to fixture ready across the
sandbox boundary; namespace-local PIDs are not used as host timing joins.

Rust observes OS and Codex boundaries; the report renderer consumes immutable
JSONL events. Missing values remain null or unknown. Relay, provider, harness,
timeout, censored, and backend failures remain distinct.

The shared MoonBit `evaluation` package owns scenarios, input equality, behavior
oracles, classification and paired bootstrap statistics. A single prebuilt
JSONL worker serves an entire collection or report. Rust owns process execution,
the fixed Responses server, HTTP transport observation, evidence sealing and
rendering. Measurement loops do not invoke compilers or start new proxies.

Both arms enable the same Codex and fixture observation. Actual Direct/ZshFork
execution is recorded after fallback selection. The common Unix PTY status fix
preserves real signals in both arms, so its benefit cannot be attributed to MBTX.
Separate minimal/full startup observations quantify trace perturbation.

For fixed-replay diagnosis, an optional loopback OTLP collector and Codex
JSONL receipt observer are enabled only by the `diagnostic` observation
profile. Their raw payloads are retained and indexed in the report, but their
export and parsing work is excluded from formal latency statistics. The
formal `minimal` profile keeps the same shared OS-monotonic event layer for
both arms and does not start the collector.
