# Pinned Codex dependencies

`Cargo.lock` is derived from the revision in `upstream.json`:
`3d2ee51ca2d5db578f328aa75e20aa22c0197c9a` (`rust-v0.153.4`).

The release manifests use workspace version `0.153.4`, while the upstream lock
still lists `0.0.0`. Running `cargo update --workspace` against that upstream
lock changes exactly 149 workspace version fields. External package versions,
sources and checksums remain pinned. The observer adds an explicit dependency
on the already locked `httpdate` package for `Retry-After` dates. All Rama
packages remain at `0.3.0-alpha.4`.

`moon run scripts/codex.mbtx prepare` installs this lock and backs up any
different local lock. `capture` excludes Cargo.lock from the integration patch.
The collector only uses `--locked` builds. Dependency updates require a reviewed
change to this file; regenerating the entire graph during collection is not
part of the workflow.

## Transparent integration

`integration.patch` changes existing upstream files; `overlay/` contains new
observation modules. Preparation applies both. It recognizes previously applied
repository patches, reverses only a verified patch, and applies the new version
without clearing Cargo targets. Conflicting local source edits cause an explicit
error. The previous patch is restored if the new patch cannot be applied.

`mbtx_backend = "transparent"` requires an absolute `mbtx_command` launcher.
The original resolved command still passes policy, approval, environment and
sandbox planning before the launcher is added. Remote environments, unsupported
backend values and malformed launcher commands return explicit errors.
An absent setting retains default Shell. Actual Direct/ZshFork use is traced.

`MBTX_TRACE_DIR` enables the same observer for both arms. It records actual tool
arguments, policy decisions, spawn/wait boundaries, independent streams, EOF,
FD counts and tool-result preparation. `MBTX_LAUNCH_TRACE` enables internal
launcher events; minimal startup runs disable it. Timing includes spawn and
uses the OS monotonic clock. The observer never owns a second OS wait.

The patch also fixes shared Codex behavior, so these changes apply to both arms:

- Unix PTY waits preserve numeric exits separately from signal termination.
- Event streaming snapshots existing buffered output before subscribing, under
  the producer lock, so early output is not lost or duplicated in the transcript.
- Post-exit stream drain allows up to 500 ms for descendant tail output. EOF
  returns immediately; expiration is observable and does not prove full drain.
- Quoted literal program names retain their identity for execution policy.
  Variable expansion and command substitution remain outside literal parsing.

These are shared corrections, not evidence of an MBTX performance advantage.
The original CLI can omit a command item when it heuristically classifies a
permission-denied exit as sandbox denial. The evaluator retains that original
CLI output and also records the real tool response, including exit 126.

`responses-api-proxy` holds a single request gate until the upstream response
has finished forwarding. Starts are paced, automatic Codex retries are disabled,
and 429 updates the gate deadline from `Retry-After` or a 30-second fallback.
The upstream key is passed over stdin and is absent from published credentials.

Run `moon run scripts/verify-offline.mbtx full` for the prebuilt-bundle tests.
They include 24 scenarios in both tool modes, denial/configuration boundaries,
HTTP faults, collector kill/resume, evidence modification, and detached children.
