# MBTX execution report

## Evidence summary

```json
{
  "backend_failures": 0,
  "complete_pairs": 0,
  "external_failures": 0,
  "intention_to_treat": {
    "failed_or_unknown_arms": 64,
    "observed_arms": 64,
    "planned_arms": 64,
    "successful_arms": 0
  },
  "not_observed_arms": 0,
  "observed_arms": 64,
  "observed_pairs": 32,
  "partial": true,
  "planned_arms": 64,
  "planned_pairs": 32,
  "status_counts": {
    "unknown": 64
  },
  "valid_comparable_pairs": 0
}
```

## Pair comparison

| pair | task | shell | transparent | comparable | observed timing | first difference |
|---|---|---|---|---|---|---|
| `pair-00-background_cleanup` | `background_cleanup` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-00-cwd_environment` | `cwd_environment` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `none_observed` |
| `pair-00-large_output` | `large_output` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-00-literal_argv` | `literal_argv` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `none_observed` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-00-signal_exit` | `signal_exit` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-01-background_cleanup` | `background_cleanup` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-01-cwd_environment` | `cwd_environment` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-01-large_output` | `large_output` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-01-literal_argv` | `literal_argv` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-01-signal_exit` | `signal_exit` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-02-background_cleanup` | `background_cleanup` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-02-cwd_environment` | `cwd_environment` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-02-large_output` | `large_output` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-02-literal_argv` | `literal_argv` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-02-signal_exit` | `signal_exit` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `none_observed` |
| `pair-03-background_cleanup` | `background_cleanup` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-03-cwd_environment` | `cwd_environment` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-03-large_output` | `large_output` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-03-literal_argv` | `literal_argv` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `none_observed` |
| `pair-03-signal_exit` | `signal_exit` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `unknown` | `unknown` | `false` | `insufficient_evidence` | `elapsed_ms` |

## Event evidence

| pair | task | backend | phase | event | status | failure | elapsed ms | command | evidence |
|---|---|---|---|---|---|---|---:|---|---|
| `pair-00-literal_argv` | `literal_argv` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `452` | `unknown` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `602` | `unknown` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `451` | `unknown` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `551` | `unknown` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `451` | `unknown` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `451` | `unknown` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `451` | `unknown` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-00-large_output/shell` |
| `pair-00-large_output` | `large_output` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `427` | `unknown` | `attempts/pair-00-large_output/shell` |
| `pair-00-large_output` | `large_output` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-large_output/shell` |
| `pair-00-large_output` | `large_output` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-large_output/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `551` | `unknown` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `452` | `unknown` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `552` | `unknown` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `452` | `unknown` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-01-literal_argv` | `literal_argv` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-01-literal_argv/transparent` |
| `pair-01-literal_argv` | `literal_argv` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-01-literal_argv/transparent` |
| `pair-01-literal_argv` | `literal_argv` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-literal_argv/transparent` |
| `pair-01-literal_argv` | `literal_argv` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-literal_argv/transparent` |
| `pair-01-literal_argv` | `literal_argv` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-01-literal_argv/shell` |
| `pair-01-literal_argv` | `literal_argv` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `451` | `unknown` | `attempts/pair-01-literal_argv/shell` |
| `pair-01-literal_argv` | `literal_argv` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-literal_argv/shell` |
| `pair-01-literal_argv` | `literal_argv` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-literal_argv/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-01-stdin_utf8/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `476` | `unknown` | `attempts/pair-01-stdin_utf8/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-stdin_utf8/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-stdin_utf8/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-01-stdin_utf8/transparent` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-01-stdin_utf8/transparent` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-stdin_utf8/transparent` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-stdin_utf8/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-01-cwd_environment/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `501` | `unknown` | `attempts/pair-01-cwd_environment/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-cwd_environment/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-cwd_environment/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-01-cwd_environment/shell` |
| `pair-01-cwd_environment` | `cwd_environment` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-01-cwd_environment/shell` |
| `pair-01-cwd_environment` | `cwd_environment` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-cwd_environment/shell` |
| `pair-01-cwd_environment` | `cwd_environment` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-cwd_environment/shell` |
| `pair-01-large_output` | `large_output` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-01-large_output/shell` |
| `pair-01-large_output` | `large_output` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `427` | `unknown` | `attempts/pair-01-large_output/shell` |
| `pair-01-large_output` | `large_output` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-large_output/shell` |
| `pair-01-large_output` | `large_output` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-large_output/shell` |
| `pair-01-large_output` | `large_output` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-01-large_output/transparent` |
| `pair-01-large_output` | `large_output` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-01-large_output/transparent` |
| `pair-01-large_output` | `large_output` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-large_output/transparent` |
| `pair-01-large_output` | `large_output` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-large_output/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-01-nonzero_exit/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `476` | `unknown` | `attempts/pair-01-nonzero_exit/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-nonzero_exit/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-nonzero_exit/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-01-nonzero_exit/shell` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-01-nonzero_exit/shell` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-nonzero_exit/shell` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-nonzero_exit/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-01-background_cleanup/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `427` | `unknown` | `attempts/pair-01-background_cleanup/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-background_cleanup/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-background_cleanup/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-01-background_cleanup/transparent` |
| `pair-01-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `477` | `unknown` | `attempts/pair-01-background_cleanup/transparent` |
| `pair-01-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-background_cleanup/transparent` |
| `pair-01-background_cleanup` | `background_cleanup` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-background_cleanup/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-01-signal_exit/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `451` | `unknown` | `attempts/pair-01-signal_exit/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-signal_exit/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-signal_exit/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-01-signal_exit/shell` |
| `pair-01-signal_exit` | `signal_exit` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-01-signal_exit/shell` |
| `pair-01-signal_exit` | `signal_exit` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-signal_exit/shell` |
| `pair-01-signal_exit` | `signal_exit` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-signal_exit/shell` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-01-repeated_recovery/shell` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `427` | `unknown` | `attempts/pair-01-repeated_recovery/shell` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-repeated_recovery/shell` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-repeated_recovery/shell` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-01-repeated_recovery/transparent` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `477` | `unknown` | `attempts/pair-01-repeated_recovery/transparent` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-repeated_recovery/transparent` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-01-repeated_recovery/transparent` |
| `pair-02-literal_argv` | `literal_argv` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-02-literal_argv/shell` |
| `pair-02-literal_argv` | `literal_argv` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `452` | `unknown` | `attempts/pair-02-literal_argv/shell` |
| `pair-02-literal_argv` | `literal_argv` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-literal_argv/shell` |
| `pair-02-literal_argv` | `literal_argv` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-literal_argv/shell` |
| `pair-02-literal_argv` | `literal_argv` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-02-literal_argv/transparent` |
| `pair-02-literal_argv` | `literal_argv` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `476` | `unknown` | `attempts/pair-02-literal_argv/transparent` |
| `pair-02-literal_argv` | `literal_argv` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-literal_argv/transparent` |
| `pair-02-literal_argv` | `literal_argv` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-literal_argv/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-02-stdin_utf8/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `401` | `unknown` | `attempts/pair-02-stdin_utf8/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-stdin_utf8/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-stdin_utf8/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-02-stdin_utf8/shell` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `401` | `unknown` | `attempts/pair-02-stdin_utf8/shell` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-stdin_utf8/shell` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-stdin_utf8/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-02-cwd_environment/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `451` | `unknown` | `attempts/pair-02-cwd_environment/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-cwd_environment/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-cwd_environment/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-02-cwd_environment/transparent` |
| `pair-02-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `526` | `unknown` | `attempts/pair-02-cwd_environment/transparent` |
| `pair-02-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-cwd_environment/transparent` |
| `pair-02-cwd_environment` | `cwd_environment` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-cwd_environment/transparent` |
| `pair-02-large_output` | `large_output` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-02-large_output/transparent` |
| `pair-02-large_output` | `large_output` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-02-large_output/transparent` |
| `pair-02-large_output` | `large_output` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-large_output/transparent` |
| `pair-02-large_output` | `large_output` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-large_output/transparent` |
| `pair-02-large_output` | `large_output` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-02-large_output/shell` |
| `pair-02-large_output` | `large_output` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `477` | `unknown` | `attempts/pair-02-large_output/shell` |
| `pair-02-large_output` | `large_output` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-large_output/shell` |
| `pair-02-large_output` | `large_output` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-large_output/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-02-nonzero_exit/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `551` | `unknown` | `attempts/pair-02-nonzero_exit/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-nonzero_exit/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-nonzero_exit/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-02-nonzero_exit/transparent` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `601` | `unknown` | `attempts/pair-02-nonzero_exit/transparent` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-nonzero_exit/transparent` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-nonzero_exit/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-02-background_cleanup/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-02-background_cleanup/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-background_cleanup/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-background_cleanup/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-02-background_cleanup/shell` |
| `pair-02-background_cleanup` | `background_cleanup` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `452` | `unknown` | `attempts/pair-02-background_cleanup/shell` |
| `pair-02-background_cleanup` | `background_cleanup` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-background_cleanup/shell` |
| `pair-02-background_cleanup` | `background_cleanup` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-background_cleanup/shell` |
| `pair-02-signal_exit` | `signal_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-02-signal_exit/shell` |
| `pair-02-signal_exit` | `signal_exit` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-02-signal_exit/shell` |
| `pair-02-signal_exit` | `signal_exit` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-signal_exit/shell` |
| `pair-02-signal_exit` | `signal_exit` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-signal_exit/shell` |
| `pair-02-signal_exit` | `signal_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-02-signal_exit/transparent` |
| `pair-02-signal_exit` | `signal_exit` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `451` | `unknown` | `attempts/pair-02-signal_exit/transparent` |
| `pair-02-signal_exit` | `signal_exit` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-signal_exit/transparent` |
| `pair-02-signal_exit` | `signal_exit` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-signal_exit/transparent` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-02-repeated_recovery/transparent` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `551` | `unknown` | `attempts/pair-02-repeated_recovery/transparent` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-repeated_recovery/transparent` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-repeated_recovery/transparent` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-02-repeated_recovery/shell` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `577` | `unknown` | `attempts/pair-02-repeated_recovery/shell` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-repeated_recovery/shell` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-02-repeated_recovery/shell` |
| `pair-03-literal_argv` | `literal_argv` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-03-literal_argv/transparent` |
| `pair-03-literal_argv` | `literal_argv` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `476` | `unknown` | `attempts/pair-03-literal_argv/transparent` |
| `pair-03-literal_argv` | `literal_argv` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-literal_argv/transparent` |
| `pair-03-literal_argv` | `literal_argv` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-literal_argv/transparent` |
| `pair-03-literal_argv` | `literal_argv` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-03-literal_argv/shell` |
| `pair-03-literal_argv` | `literal_argv` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-03-literal_argv/shell` |
| `pair-03-literal_argv` | `literal_argv` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-literal_argv/shell` |
| `pair-03-literal_argv` | `literal_argv` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-literal_argv/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-03-stdin_utf8/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-03-stdin_utf8/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-stdin_utf8/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-stdin_utf8/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-03-stdin_utf8/transparent` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `551` | `unknown` | `attempts/pair-03-stdin_utf8/transparent` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-stdin_utf8/transparent` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-stdin_utf8/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-03-cwd_environment/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `452` | `unknown` | `attempts/pair-03-cwd_environment/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-cwd_environment/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-cwd_environment/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-03-cwd_environment/shell` |
| `pair-03-cwd_environment` | `cwd_environment` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `451` | `unknown` | `attempts/pair-03-cwd_environment/shell` |
| `pair-03-cwd_environment` | `cwd_environment` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-cwd_environment/shell` |
| `pair-03-cwd_environment` | `cwd_environment` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-cwd_environment/shell` |
| `pair-03-large_output` | `large_output` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-03-large_output/shell` |
| `pair-03-large_output` | `large_output` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `501` | `unknown` | `attempts/pair-03-large_output/shell` |
| `pair-03-large_output` | `large_output` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-large_output/shell` |
| `pair-03-large_output` | `large_output` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-large_output/shell` |
| `pair-03-large_output` | `large_output` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-03-large_output/transparent` |
| `pair-03-large_output` | `large_output` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `476` | `unknown` | `attempts/pair-03-large_output/transparent` |
| `pair-03-large_output` | `large_output` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-large_output/transparent` |
| `pair-03-large_output` | `large_output` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-large_output/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-03-nonzero_exit/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `476` | `unknown` | `attempts/pair-03-nonzero_exit/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-nonzero_exit/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-nonzero_exit/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-03-nonzero_exit/shell` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-03-nonzero_exit/shell` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-nonzero_exit/shell` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-nonzero_exit/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-03-background_cleanup/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `451` | `unknown` | `attempts/pair-03-background_cleanup/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-background_cleanup/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-background_cleanup/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-03-background_cleanup/transparent` |
| `pair-03-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-03-background_cleanup/transparent` |
| `pair-03-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-background_cleanup/transparent` |
| `pair-03-background_cleanup` | `background_cleanup` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-background_cleanup/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-03-signal_exit/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `576` | `unknown` | `attempts/pair-03-signal_exit/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-signal_exit/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-signal_exit/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-03-signal_exit/shell` |
| `pair-03-signal_exit` | `signal_exit` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-03-signal_exit/shell` |
| `pair-03-signal_exit` | `signal_exit` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-signal_exit/shell` |
| `pair-03-signal_exit` | `signal_exit` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-signal_exit/shell` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-03-repeated_recovery/shell` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-03-repeated_recovery/shell` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-repeated_recovery/shell` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `shell` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-repeated_recovery/shell` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-03-repeated_recovery/transparent` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `426` | `unknown` | `attempts/pair-03-repeated_recovery/transparent` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-repeated_recovery/transparent` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-03-repeated_recovery/transparent` |
