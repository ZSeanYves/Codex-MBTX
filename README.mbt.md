# Codes-MBTX

Codes-MBTX evaluates `mbtx exec -- COMMAND...` as a transparent launcher for
Codex's existing `exec_command` backend. MBTX receives a resolved argv, starts
the child process with the requested streams and working directory, waits for
it, and returns the same observable exit result. It does not parse shell
syntax, compile a MoonBit script, or expose a second model tool.

## Launcher

```bash
moon run --build-only --output-json --target native cmd/mbtx
_build/native/debug/build/cmd/mbtx/mbtx.exe exec -- printf '%s\n' 'literal $(echo text)'
```

The only supported command form is `mbtx exec -- COMMAND [ARGUMENTS...]`.
Ordinary exit codes are preserved and signal exits are re-raised. Transparent
mode keeps the original command for Codex approval, policy, sandbox, and
telemetry, then adds `exec --` only at the final local process launch.

## Codex integration

Configure the trusted launcher from a user, system, managed, or runtime layer:

```toml
mbtx_backend = "transparent"
mbtx_command = ["/absolute/path/to/mbtx"]
```

Project-local configuration cannot select the launcher. Remote environments are
rejected explicitly. The default shell backend remains unchanged.

## Evidence collection

`adapter` contains `mbtx-observe` for immutable job descriptions and JSONL
lifecycle events, and `mbtx-eval` for log, Markdown, JSON, CSV, and self-contained
HTML output. Events retain pair, attempt, task, backend, monotonic clock, exit,
signal, output-size, status, and failure class fields.

The first collection target is four valid pairs for each of eight functional
classes: literal argv; stdin and UTF-8; cwd and environment; large output;
non-zero exit; background cleanup; timeout and signals; repeated execution and
recovery. Failed attempts remain in the report and collection uses at most 48
attempts. Relay and provider failures are reported separately from backend
failures. The report includes default Shell versus MBTX and Direct Shell versus
MBTX because transparent mode disables the Shell zsh-fork optimization.

## Development

```bash
moon check --target native
moon test --target native
moon info
moon fmt
cargo check --locked --manifest-path adapter/Cargo.toml
```

Historical reports and the removed script/job/session runtime are preserved
under `docs/archive/legacy/` with `docs/archive/SHA256SUMS`.
