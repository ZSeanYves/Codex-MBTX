# Code verification record

This records implementation checks, not formal performance results. No real relay
requests were made during these checks. Formal Linux collection, the 1,000-pair
startup experiments, historical reproduction and scientific acceptance remain
pending the separately collected evidence.

## Scope

The implementation uses Codex revision `3d2ee51ca2d5db578f328aa75e20aa22c0197c9a`,
its reviewed lock file and the repository integration patch. Local compilation
used macOS arm64, Rust 1.98.1 and MoonBit compiler
`v0.10.12+1634b282e`. Codex and the Rust adapter used the dev profile with debug
information disabled; the launcher used native release. These are validation
builds, not release-profile performance samples.

Checks cover the 24 scenarios with real Codex, sandbox and child processes in
both direct tool mode and Code Mode. Fixed local Responses/SSE supplies the model
output. Additional checks exercise numeric/signal exits, cancellation and reaping,
policy denial, invalid configuration, 429/503/disconnection, collector kill,
timeout, immutable resume, frozen replay and report reconstruction. The proxy
test sends concurrent clients to a local server and verifies that upstream streams
never overlap and that Retry-After controls the next send.

The Linux regression also launches fixtures in separate bubblewrap PID namespaces,
resolves their host identities and verifies that repeated namespace-local PIDs
produce separate evidence. It never sends signals to a PID merely because that
number appeared in a sandbox receipt. Spawn-to-ready attribution requires a tool
call ID; missing identity or timing evidence remains unknown.

Linux verification exposed two shared sandbox behaviors: status normalization
and descendant containment. The raw comparison showed 143 for both numeric exit
and inner SIGTERM at Codex's outer wait, while the launcher recorded the inner
signal separately. This matches [bubblewrap's status propagation](https://github.com/containers/bubblewrap/blob/v0.9.0/bubblewrap.c#L429).
The pipe-tail case produced only `parent-done` in both the independent stream and
the Codex result, with no controlled process remaining. Platform-aware oracles
preserve these observations and report the corresponding conclusion limits;
observed emitted tail bytes still cannot be silently lost.

## Verification status

- MoonBit check, all ten native tests and generated-interface/format checks passed.
- All 19 Rust unit/parser/report checks and clean upstream patch application passed.
- All seven macOS real-process/real-Codex integration tests passed, including
  all 24 scenarios in both tool modes and a frozen replay of each mode.
- Linux [CI run 34780390653](https://github.com/ZSeanYves/Codex-MBTX/actions/runs/34780390653)
  passed on implementation commit `c5cb3a5774927da31b430769013032b2b4181978`.
  All eight real-process/real-Codex integration tests passed in 321.59 seconds,
  including all 24 scenarios in both tool modes and their immutable recovery and
  frozen replay checks. The aggregate required status passed. CI preserves
  bundle metadata and retains raw scenario evidence on integration failures;
  successful test scratch directories are removed.
- A small macOS launcher run produced Markdown, JSON, CSV, HTML and trace output.
  Its HTML was checked at desktop and narrow viewport sizes. It is not a formal
  startup sample or evidence for a 10% advantage.

## Build cost and reuse

A Linux CI build without a restored Codex build cache recorded 523,274 ms in its
bundle manifest. This is a build observation from a development configuration,
not a launcher latency. The final local launcher bundle
`7692478f43e3111a4ed576b5f2d44e87df3667ce0d16d8f1152b728940e32fd6`
recorded 10,086 ms for its incremental release build. Its immediately repeated
lookup took about 0.28 seconds and reported a verified cache hit without starting
a compiler. This lookup includes hash verification and automation startup.

The measurement loops call prebuilt binaries and reuse one fixture and one
Responses endpoint. CI records bundle hashes and retains offline failure evidence.
This removes repeated construction from measured arms. A same-host comparison
against the old complete offline workflow has not been collected, so this record
does not assign a numerical speedup to the workflow change.

## Remaining evidence

Run the [Linux collection procedure](linux-online.md) to collect the short online
validation, 192 target online pairs, startup minimal/full observations and the
larger offline replay. macOS evidence remains a separate platform artifact.
The historical startup claim still needs its exact original build and measurement
recipe; current measurements cannot substitute for that reproduction.

Model computation and internal relay waiting remain a combined external interval
unless the relay supplies trustworthy server-side timestamps. Neither these code
checks nor a successful small run establish universal lossless replacement or
justify changing the default Shell backend.
