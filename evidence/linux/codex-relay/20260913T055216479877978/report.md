# MBTX execution report

## Evidence summary

```json
{
  "backend_failures": 0,
  "complete_pairs": 0,
  "external_failures": 5,
  "intention_to_treat": {
    "failed_or_unknown_arms": 5,
    "observed_arms": 5,
    "planned_arms": 64,
    "successful_arms": 0
  },
  "not_observed_arms": 59,
  "observed_arms": 5,
  "observed_pairs": 3,
  "partial": true,
  "planned_arms": 64,
  "planned_pairs": 32,
  "status_counts": {
    "relay_error": 5
  },
  "valid_comparable_pairs": 0
}
```

## Pair comparison

| pair | task | shell | transparent | comparable | observed timing | first difference |
|---|---|---|---|---|---|---|
| `pair-00-cwd_environment` | `cwd_environment` | `relay_error` | `unknown` | `false` | `insufficient_evidence` | `missing_arm` |
| `pair-00-literal_argv` | `literal_argv` | `relay_error` | `relay_error` | `false` | `insufficient_evidence` | `elapsed_ms` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `relay_error` | `relay_error` | `false` | `insufficient_evidence` | `elapsed_ms` |

## Event evidence

| pair | task | backend | phase | event | status | failure | elapsed ms | command | evidence |
|---|---|---|---|---|---|---|---:|---|---|
| `pair-00-literal_argv` | `literal_argv` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `codex` | `exit` | `relay_error` | `relay_or_transport` | `602` | `unknown` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `codex` | `reap` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `io` | `drain` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `relay` | `error` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `python3 -c 'import sys; print(repr(sys.argv[1:]))' 'alpha beta' 'quote"double' 'dollar$' '$(touch SHOULD_NOT_EXIST)'` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `codex` | `exit` | `relay_error` | `relay_or_transport` | `452` | `unknown` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `codex` | `reap` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `io` | `drain` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `relay` | `error` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `exit` | `relay_error` | `relay_or_transport` | `426` | `unknown` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `codex` | `reap` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `io` | `drain` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `relay` | `error` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '第一行\n\n最后一行\n'   python3 -c 'import sys; print(sys.stdin.read(), end="")'` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `exit` | `relay_error` | `relay_or_transport` | `527` | `unknown` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `codex` | `reap` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `io` | `drain` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `relay` | `error` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `codex` | `start` | `unknown` | `unknown` | `unknown` | `printf '%s %s\n' "$PWD" "$MBTX_EVAL_TOKEN"` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `codex` | `exit` | `relay_error` | `relay_or_transport` | `402` | `unknown` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `codex` | `reap` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `io` | `drain` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `relay` | `error` | `relay_error` | `relay_or_transport` | `unknown` | `unknown` | `attempts/pair-00-cwd_environment/shell` |
