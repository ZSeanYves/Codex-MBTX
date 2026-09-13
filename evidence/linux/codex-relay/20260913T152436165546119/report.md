# MBTX execution report

## Evidence summary

```json
{
  "backend_failures": 0,
  "command_oracle_failures": 0,
  "command_oracle_pairs": 32,
  "command_oracle_successful_arms": 64,
  "complete_pairs": 32,
  "external_failures": 0,
  "intention_to_treat": {
    "failed_or_unknown_arms": 0,
    "observed_arms": 64,
    "planned_arms": 64,
    "successful_arms": 64
  },
  "not_observed_arms": 0,
  "observed_arms": 64,
  "observed_pairs": 32,
  "partial": false,
  "planned_arms": 64,
  "planned_pairs": 32,
  "status_counts": {
    "success": 64
  },
  "valid_comparable_pairs": 32
}
```

## Pair comparison

| pair | task | shell | transparent | complete Codex pair | command oracle pair | observed timing | first difference |
|---|---|---|---|---|---|---|---|
| `pair-00-background_cleanup` | `background_cleanup` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-00-cwd_environment` | `cwd_environment` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-00-large_output` | `large_output` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-00-literal_argv` | `literal_argv` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-00-signal_exit` | `signal_exit` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-01-background_cleanup` | `background_cleanup` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-01-cwd_environment` | `cwd_environment` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-01-large_output` | `large_output` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-01-literal_argv` | `literal_argv` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-01-signal_exit` | `signal_exit` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-02-background_cleanup` | `background_cleanup` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-02-cwd_environment` | `cwd_environment` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-02-large_output` | `large_output` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-02-literal_argv` | `literal_argv` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-02-signal_exit` | `signal_exit` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-03-background_cleanup` | `background_cleanup` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-03-cwd_environment` | `cwd_environment` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-03-large_output` | `large_output` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-03-literal_argv` | `literal_argv` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |
| `pair-03-signal_exit` | `signal_exit` | `success` | `success` | `true` | `true` | `shell_faster_observed` | `elapsed_ms` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `success` | `success` | `true` | `true` | `transparent_faster_observed` | `elapsed_ms` |

## Event evidence

| pair | task | backend | phase | event | status | failure | elapsed ms | command | evidence |
|---|---|---|---|---|---|---|---:|---|---|
| `pair-00-literal_argv` | `literal_argv` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `codex` | `exit` | `success` | `unknown` | `17812` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `codex` | `exit` | `success` | `unknown` | `18350` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `exit` | `success` | `unknown` | `18674` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `exit` | `success` | `unknown` | `20441` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `codex` | `exit` | `success` | `unknown` | `20046` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `exit` | `success` | `unknown` | `17220` | `/bin/bash -lc "printf '%s %s
' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '%s %s
' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `codex` | `exit` | `success` | `unknown` | `20286` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-00-large_output/shell` |
| `pair-00-large_output` | `large_output` | `shell` | `codex` | `exit` | `success` | `unknown` | `17933` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-00-large_output/shell` |
| `pair-00-large_output` | `large_output` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-large_output/shell` |
| `pair-00-large_output` | `large_output` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-large_output/shell` |
| `pair-00-large_output` | `large_output` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-00-large_output/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `exit` | `success` | `unknown` | `18787` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `exit` | `success` | `unknown` | `22236` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `exit` | `success` | `unknown` | `16860` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `codex` | `exit` | `success` | `unknown` | `18779` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `codex` | `exit` | `success` | `unknown` | `18123` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `codex` | `exit` | `success` | `unknown` | `18552` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `exit` | `success` | `unknown` | `16979` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `exit` | `success` | `unknown` | `17389` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-00-repeated_recovery/shell` |
| `pair-01-literal_argv` | `literal_argv` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-01-literal_argv/transparent` |
| `pair-01-literal_argv` | `literal_argv` | `transparent` | `codex` | `exit` | `success` | `unknown` | `19021` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-01-literal_argv/transparent` |
| `pair-01-literal_argv` | `literal_argv` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-literal_argv/transparent` |
| `pair-01-literal_argv` | `literal_argv` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-literal_argv/transparent` |
| `pair-01-literal_argv` | `literal_argv` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-01-literal_argv/transparent` |
| `pair-01-literal_argv` | `literal_argv` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-01-literal_argv/shell` |
| `pair-01-literal_argv` | `literal_argv` | `shell` | `codex` | `exit` | `success` | `unknown` | `18219` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-01-literal_argv/shell` |
| `pair-01-literal_argv` | `literal_argv` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-literal_argv/shell` |
| `pair-01-literal_argv` | `literal_argv` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-literal_argv/shell` |
| `pair-01-literal_argv` | `literal_argv` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-01-literal_argv/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-01-stdin_utf8/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `exit` | `success` | `unknown` | `18110` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-01-stdin_utf8/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-stdin_utf8/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-stdin_utf8/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-01-stdin_utf8/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-01-stdin_utf8/transparent` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `exit` | `success` | `unknown` | `19543` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-01-stdin_utf8/transparent` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-stdin_utf8/transparent` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-stdin_utf8/transparent` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-01-stdin_utf8/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-01-cwd_environment/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `exit` | `success` | `unknown` | `18036` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-01-cwd_environment/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-cwd_environment/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-cwd_environment/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-01-cwd_environment/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-01-cwd_environment/shell` |
| `pair-01-cwd_environment` | `cwd_environment` | `shell` | `codex` | `exit` | `success` | `unknown` | `18461` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-01-cwd_environment/shell` |
| `pair-01-cwd_environment` | `cwd_environment` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-cwd_environment/shell` |
| `pair-01-cwd_environment` | `cwd_environment` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-cwd_environment/shell` |
| `pair-01-cwd_environment` | `cwd_environment` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-01-cwd_environment/shell` |
| `pair-01-large_output` | `large_output` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-01-large_output/shell` |
| `pair-01-large_output` | `large_output` | `shell` | `codex` | `exit` | `success` | `unknown` | `21143` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-01-large_output/shell` |
| `pair-01-large_output` | `large_output` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-large_output/shell` |
| `pair-01-large_output` | `large_output` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-large_output/shell` |
| `pair-01-large_output` | `large_output` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-01-large_output/shell` |
| `pair-01-large_output` | `large_output` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-01-large_output/transparent` |
| `pair-01-large_output` | `large_output` | `transparent` | `codex` | `exit` | `success` | `unknown` | `28934` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-01-large_output/transparent` |
| `pair-01-large_output` | `large_output` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-large_output/transparent` |
| `pair-01-large_output` | `large_output` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-large_output/transparent` |
| `pair-01-large_output` | `large_output` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-01-large_output/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-01-nonzero_exit/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `exit` | `success` | `unknown` | `20431` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-01-nonzero_exit/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-nonzero_exit/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-nonzero_exit/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-01-nonzero_exit/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-01-nonzero_exit/shell` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `exit` | `success` | `unknown` | `20230` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-01-nonzero_exit/shell` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-nonzero_exit/shell` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-nonzero_exit/shell` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-01-nonzero_exit/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-01-background_cleanup/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `shell` | `codex` | `exit` | `success` | `unknown` | `20395` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-01-background_cleanup/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-background_cleanup/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-background_cleanup/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-01-background_cleanup/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-01-background_cleanup/transparent` |
| `pair-01-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `exit` | `success` | `unknown` | `19436` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-01-background_cleanup/transparent` |
| `pair-01-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-background_cleanup/transparent` |
| `pair-01-background_cleanup` | `background_cleanup` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-background_cleanup/transparent` |
| `pair-01-background_cleanup` | `background_cleanup` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-01-background_cleanup/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-01-signal_exit/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `transparent` | `codex` | `exit` | `success` | `unknown` | `18194` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-01-signal_exit/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-signal_exit/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-signal_exit/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-01-signal_exit/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-01-signal_exit/shell` |
| `pair-01-signal_exit` | `signal_exit` | `shell` | `codex` | `exit` | `success` | `unknown` | `18759` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-01-signal_exit/shell` |
| `pair-01-signal_exit` | `signal_exit` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-signal_exit/shell` |
| `pair-01-signal_exit` | `signal_exit` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-signal_exit/shell` |
| `pair-01-signal_exit` | `signal_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-01-signal_exit/shell` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-01-repeated_recovery/shell` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `exit` | `success` | `unknown` | `18909` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-01-repeated_recovery/shell` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-repeated_recovery/shell` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-repeated_recovery/shell` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-01-repeated_recovery/shell` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-01-repeated_recovery/transparent` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `exit` | `success` | `unknown` | `16691` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-01-repeated_recovery/transparent` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-repeated_recovery/transparent` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-01-repeated_recovery/transparent` |
| `pair-01-repeated_recovery` | `repeated_recovery` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-01-repeated_recovery/transparent` |
| `pair-02-literal_argv` | `literal_argv` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-02-literal_argv/shell` |
| `pair-02-literal_argv` | `literal_argv` | `shell` | `codex` | `exit` | `success` | `unknown` | `19312` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-02-literal_argv/shell` |
| `pair-02-literal_argv` | `literal_argv` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-literal_argv/shell` |
| `pair-02-literal_argv` | `literal_argv` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-literal_argv/shell` |
| `pair-02-literal_argv` | `literal_argv` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-02-literal_argv/shell` |
| `pair-02-literal_argv` | `literal_argv` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-02-literal_argv/transparent` |
| `pair-02-literal_argv` | `literal_argv` | `transparent` | `codex` | `exit` | `success` | `unknown` | `18354` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-02-literal_argv/transparent` |
| `pair-02-literal_argv` | `literal_argv` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-literal_argv/transparent` |
| `pair-02-literal_argv` | `literal_argv` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-literal_argv/transparent` |
| `pair-02-literal_argv` | `literal_argv` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-02-literal_argv/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-02-stdin_utf8/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `exit` | `success` | `unknown` | `18055` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-02-stdin_utf8/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-stdin_utf8/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-stdin_utf8/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-02-stdin_utf8/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-02-stdin_utf8/shell` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `exit` | `success` | `unknown` | `22156` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-02-stdin_utf8/shell` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-stdin_utf8/shell` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-stdin_utf8/shell` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-02-stdin_utf8/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-02-cwd_environment/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `shell` | `codex` | `exit` | `success` | `unknown` | `17403` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-02-cwd_environment/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-cwd_environment/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-cwd_environment/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-02-cwd_environment/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-02-cwd_environment/transparent` |
| `pair-02-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `exit` | `success` | `unknown` | `17062` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-02-cwd_environment/transparent` |
| `pair-02-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-cwd_environment/transparent` |
| `pair-02-cwd_environment` | `cwd_environment` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-cwd_environment/transparent` |
| `pair-02-cwd_environment` | `cwd_environment` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-02-cwd_environment/transparent` |
| `pair-02-large_output` | `large_output` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-02-large_output/transparent` |
| `pair-02-large_output` | `large_output` | `transparent` | `codex` | `exit` | `success` | `unknown` | `22025` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-02-large_output/transparent` |
| `pair-02-large_output` | `large_output` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-large_output/transparent` |
| `pair-02-large_output` | `large_output` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-large_output/transparent` |
| `pair-02-large_output` | `large_output` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-02-large_output/transparent` |
| `pair-02-large_output` | `large_output` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-02-large_output/shell` |
| `pair-02-large_output` | `large_output` | `shell` | `codex` | `exit` | `success` | `unknown` | `17089` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-02-large_output/shell` |
| `pair-02-large_output` | `large_output` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-large_output/shell` |
| `pair-02-large_output` | `large_output` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-large_output/shell` |
| `pair-02-large_output` | `large_output` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-02-large_output/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-02-nonzero_exit/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `exit` | `success` | `unknown` | `31238` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-02-nonzero_exit/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-nonzero_exit/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-nonzero_exit/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-02-nonzero_exit/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-02-nonzero_exit/transparent` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `exit` | `success` | `unknown` | `18190` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-02-nonzero_exit/transparent` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-nonzero_exit/transparent` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-nonzero_exit/transparent` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-02-nonzero_exit/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-02-background_cleanup/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `exit` | `success` | `unknown` | `23429` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-02-background_cleanup/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-background_cleanup/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-background_cleanup/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-02-background_cleanup/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-02-background_cleanup/shell` |
| `pair-02-background_cleanup` | `background_cleanup` | `shell` | `codex` | `exit` | `success` | `unknown` | `17202` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-02-background_cleanup/shell` |
| `pair-02-background_cleanup` | `background_cleanup` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-background_cleanup/shell` |
| `pair-02-background_cleanup` | `background_cleanup` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-background_cleanup/shell` |
| `pair-02-background_cleanup` | `background_cleanup` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-02-background_cleanup/shell` |
| `pair-02-signal_exit` | `signal_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-02-signal_exit/shell` |
| `pair-02-signal_exit` | `signal_exit` | `shell` | `codex` | `exit` | `success` | `unknown` | `20197` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-02-signal_exit/shell` |
| `pair-02-signal_exit` | `signal_exit` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-signal_exit/shell` |
| `pair-02-signal_exit` | `signal_exit` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-signal_exit/shell` |
| `pair-02-signal_exit` | `signal_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-02-signal_exit/shell` |
| `pair-02-signal_exit` | `signal_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-02-signal_exit/transparent` |
| `pair-02-signal_exit` | `signal_exit` | `transparent` | `codex` | `exit` | `success` | `unknown` | `19365` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-02-signal_exit/transparent` |
| `pair-02-signal_exit` | `signal_exit` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-signal_exit/transparent` |
| `pair-02-signal_exit` | `signal_exit` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-signal_exit/transparent` |
| `pair-02-signal_exit` | `signal_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-02-signal_exit/transparent` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-02-repeated_recovery/transparent` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `exit` | `success` | `unknown` | `22582` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-02-repeated_recovery/transparent` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-repeated_recovery/transparent` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-repeated_recovery/transparent` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-02-repeated_recovery/transparent` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-02-repeated_recovery/shell` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `exit` | `success` | `unknown` | `17799` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-02-repeated_recovery/shell` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-repeated_recovery/shell` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-02-repeated_recovery/shell` |
| `pair-02-repeated_recovery` | `repeated_recovery` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-02-repeated_recovery/shell` |
| `pair-03-literal_argv` | `literal_argv` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-03-literal_argv/transparent` |
| `pair-03-literal_argv` | `literal_argv` | `transparent` | `codex` | `exit` | `success` | `unknown` | `22207` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-03-literal_argv/transparent` |
| `pair-03-literal_argv` | `literal_argv` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-literal_argv/transparent` |
| `pair-03-literal_argv` | `literal_argv` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-literal_argv/transparent` |
| `pair-03-literal_argv` | `literal_argv` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-03-literal_argv/transparent` |
| `pair-03-literal_argv` | `literal_argv` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-03-literal_argv/shell` |
| `pair-03-literal_argv` | `literal_argv` | `shell` | `codex` | `exit` | `success` | `unknown` | `20053` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-03-literal_argv/shell` |
| `pair-03-literal_argv` | `literal_argv` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-literal_argv/shell` |
| `pair-03-literal_argv` | `literal_argv` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-literal_argv/shell` |
| `pair-03-literal_argv` | `literal_argv` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote\"double' 'dollar"'$'"' '"'$(touch SHOULD_NOT_EXIST)'"'"` | `attempts/pair-03-literal_argv/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-03-stdin_utf8/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `exit` | `success` | `unknown` | `20894` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-03-stdin_utf8/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-stdin_utf8/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-stdin_utf8/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-03-stdin_utf8/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-03-stdin_utf8/transparent` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `exit` | `success` | `unknown` | `18518` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-03-stdin_utf8/transparent` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-stdin_utf8/transparent` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-stdin_utf8/transparent` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '第一行\\n\\n最后一行\\n'   python3 -c 'import sys; print(sys.stdin.read(), end=\"\")'"` | `attempts/pair-03-stdin_utf8/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-03-cwd_environment/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `exit` | `success` | `unknown` | `17707` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-03-cwd_environment/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-cwd_environment/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-cwd_environment/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-03-cwd_environment/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_MARKER"` | `attempts/pair-03-cwd_environment/shell` |
| `pair-03-cwd_environment` | `cwd_environment` | `shell` | `codex` | `exit` | `success` | `unknown` | `22721` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-03-cwd_environment/shell` |
| `pair-03-cwd_environment` | `cwd_environment` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-cwd_environment/shell` |
| `pair-03-cwd_environment` | `cwd_environment` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-cwd_environment/shell` |
| `pair-03-cwd_environment` | `cwd_environment` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "printf '%s %s\\n' \""'$PWD" "$MBTX_EVAL_MARKER"'` | `attempts/pair-03-cwd_environment/shell` |
| `pair-03-large_output` | `large_output` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-03-large_output/shell` |
| `pair-03-large_output` | `large_output` | `shell` | `codex` | `exit` | `success` | `unknown` | `22868` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-03-large_output/shell` |
| `pair-03-large_output` | `large_output` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-large_output/shell` |
| `pair-03-large_output` | `large_output` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-large_output/shell` |
| `pair-03-large_output` | `large_output` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-03-large_output/shell` |
| `pair-03-large_output` | `large_output` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; sys.stdout.write("x"*65536); sys.stderr.write("e"*4096)'` | `attempts/pair-03-large_output/transparent` |
| `pair-03-large_output` | `large_output` | `transparent` | `codex` | `exit` | `success` | `unknown` | `19684` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-03-large_output/transparent` |
| `pair-03-large_output` | `large_output` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-large_output/transparent` |
| `pair-03-large_output` | `large_output` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-large_output/transparent` |
| `pair-03-large_output` | `large_output` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; sys.stdout.write(\"x\"*65536); sys.stderr.write(\"e\"*4096)'"` | `attempts/pair-03-large_output/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-03-nonzero_exit/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `exit` | `success` | `unknown` | `28308` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-03-nonzero_exit/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-nonzero_exit/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-nonzero_exit/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-03-nonzero_exit/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print("expected failure", file=sys.stderr); sys.exit(7)'` | `attempts/pair-03-nonzero_exit/shell` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `exit` | `success` | `unknown` | `18843` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-03-nonzero_exit/shell` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-nonzero_exit/shell` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-nonzero_exit/shell` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import sys; print(\"expected failure\", file=sys.stderr); sys.exit(7)'"` | `attempts/pair-03-nonzero_exit/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-03-background_cleanup/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `shell` | `codex` | `exit` | `success` | `unknown` | `18523` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-03-background_cleanup/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-background_cleanup/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-background_cleanup/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-03-background_cleanup/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import subprocess; p=subprocess.Popen(["sh","-c","sleep 0.1"]); print("child-started", flush=True); p.wait(); print("child-clean")'` | `attempts/pair-03-background_cleanup/transparent` |
| `pair-03-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `exit` | `success` | `unknown` | `20784` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-03-background_cleanup/transparent` |
| `pair-03-background_cleanup` | `background_cleanup` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-background_cleanup/transparent` |
| `pair-03-background_cleanup` | `background_cleanup` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-background_cleanup/transparent` |
| `pair-03-background_cleanup` | `background_cleanup` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import subprocess; p=subprocess.Popen([\"sh\",\"-c\",\"sleep 0.1\"]); print(\"child-started\", flush=True); p.wait(); print(\"child-clean\")'"` | `attempts/pair-03-background_cleanup/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-03-signal_exit/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `transparent` | `codex` | `exit` | `success` | `unknown` | `19092` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-03-signal_exit/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-signal_exit/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-signal_exit/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-03-signal_exit/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import os,signal; print("term-start", flush=True); os.kill(os.getpid(), signal.SIGTERM)'` | `attempts/pair-03-signal_exit/shell` |
| `pair-03-signal_exit` | `signal_exit` | `shell` | `codex` | `exit` | `success` | `unknown` | `18211` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-03-signal_exit/shell` |
| `pair-03-signal_exit` | `signal_exit` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-signal_exit/shell` |
| `pair-03-signal_exit` | `signal_exit` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-signal_exit/shell` |
| `pair-03-signal_exit` | `signal_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'import os,signal; print(\"term-start\", flush=True); os.kill(os.getpid(), signal.SIGTERM)'"` | `attempts/pair-03-signal_exit/shell` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-03-repeated_recovery/shell` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `exit` | `success` | `unknown` | `21516` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-03-repeated_recovery/shell` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `shell` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-repeated_recovery/shell` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `shell` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-repeated_recovery/shell` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-03-repeated_recovery/shell` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'print("recovery-ok")'` | `attempts/pair-03-repeated_recovery/transparent` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `exit` | `success` | `unknown` | `19327` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-03-repeated_recovery/transparent` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `transparent` | `codex` | `reap` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-repeated_recovery/transparent` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `transparent` | `io` | `drain` | `success` | `unknown` | `unknown` | `unknown` | `attempts/pair-03-repeated_recovery/transparent` |
| `pair-03-repeated_recovery` | `repeated_recovery` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `/bin/bash -lc "python3 -c 'print(\"recovery-ok\")'"` | `attempts/pair-03-repeated_recovery/transparent` |
