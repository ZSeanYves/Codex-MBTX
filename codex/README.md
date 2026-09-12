# Pinned Codex dependencies

`Cargo.lock` is derived from the revision in `upstream.json`:
`3d2ee51ca2d5db578f328aa75e20aa22c0197c9a` (`rust-v0.153.4`).

The release manifests use workspace version `0.153.4`, while the upstream lock
still lists `0.0.0`. Running `cargo update --workspace` against that upstream
lock changes exactly 149 workspace version fields. External package versions,
sources, checksums, and dependency lists are unchanged. All Rama packages remain
at `0.3.0-alpha.4`.

`moon run scripts/codex.mbtx prepare` installs this lock and backs up any
different local lock. `capture` excludes Cargo.lock from the integration patch.
The collector only uses `--locked` builds. Dependency updates require a reviewed
change to this file; regenerating the entire graph during collection is not
part of the workflow.
