# MBTX execution report

## Evidence summary

```json
{
  "backend_failures": 0,
  "complete_pairs": 5,
  "external_failures": 1,
  "intention_to_treat": {
    "failed_or_unknown_arms": 3,
    "observed_arms": 29,
    "planned_arms": 16,
    "successful_arms": 26
  },
  "not_observed_arms": 0,
  "observed_arms": 29,
  "observed_pairs": 8,
  "partial": true,
  "planned_arms": 16,
  "planned_pairs": 8,
  "status_counts": {
    "relay_error": 1,
    "success": 26,
    "timeout": 1,
    "unknown": 1
  },
  "valid_comparable_pairs": 5
}
```

## Pair comparison

| pair | task | shell | transparent | comparable | observed timing | first difference |
|---|---|---|---|---|---|---|
| `pair-00-background_cleanup` | `background_cleanup` | `success` | `unknown` | `false` | `insufficient_evidence` | `status` |
| `pair-00-cwd_environment` | `cwd_environment` | `success` | `success` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-00-large_output` | `large_output` | `success` | `relay_error` | `false` | `insufficient_evidence` | `status` |
| `pair-00-literal_argv` | `literal_argv` | `success` | `success` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `success` | `success` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `success` | `timeout` | `false` | `insufficient_evidence` | `status` |
| `pair-00-signal_exit` | `signal_exit` | `success` | `success` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `success` | `success` | `true` | `shell_faster_observed` | `elapsed_ms` |

## Event evidence

| pair | task | backend | phase | event | status | failure | elapsed ms | command | evidence |
|---|---|---|---|---|---|---|---:|---|---|
| `pair-00-literal_argv` | `literal_argv` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `codex` | `exit` | `success` | `unknown` | `135587` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `codex` | `exit` | `success` | `unknown` | `84819` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `exit` | `success` | `unknown` | `155731` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `exit` | `success` | `unknown` | `73752` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `codex` | `exit` | `success` | `unknown` | `155499` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `exit` | `success` | `unknown` | `41932` | `/bin/bash -lc "printf '%s %s
' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '%s %s
' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `codex` | `exit` | `relay_error` | `relay_or_transport` | `35908` | `unknown` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `codex` | `reap` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `io` | `drain` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `relay` | `error` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-00-large_output/shell` |
| `pair-00-large_output` | `large_output` | `shell` | `codex` | `exit` | `success` | `unknown` | `110102` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-00-large_output/shell` |
| `pair-00-large_output` | `large_output` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-large_output/shell` |
| `pair-00-large_output` | `large_output` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-large_output/shell` |
| `pair-00-large_output` | `large_output` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-00-large_output/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `exit` | `success` | `unknown` | `239381` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `exit` | `success` | `unknown` | `166529` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `exit` | `unknown` | `missing_command_event` | `30874` | `unknown` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `reap` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `io` | `drain` | `unknown` | `missing_command_event` | `unknown` | `unknown` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `codex` | `exit` | `success` | `unknown` | `93627` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `codex` | `exit` | `success` | `unknown` | `291604` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `codex` | `exit` | `success` | `unknown` | `92406` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `exit` | `timeout` | `codex_timeout` | `300519` | `unknown` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `reap` | `timeout` | `codex_timeout` | `unknown` | `unknown` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `io` | `drain` | `timeout` | `codex_timeout` | `unknown` | `unknown` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `exit` | `success` | `unknown` | `143706` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-00-repeated_recovery/shell` |
