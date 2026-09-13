# Codex relay online collection

- Planned pairs: 8
- Recorded arms: 16
- Successful arms: 13
- Complete pairs: 5
- Partial: true

| Pair | Task | Backend | Status | Failure | Command exit | Evidence |
|---|---|---|---|---|---:|---|
| `pair-00-literal_argv` | `literal_argv` | `shell` | `success` | `` | `0` | `attempts/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `success` | `` | `0` | `attempts/pair-00-literal_argv/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `success` | `` | `0` | `attempts/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `success` | `` | `0` | `attempts/pair-00-stdin_utf8/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `success` | `` | `0` | `attempts/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `success` | `` | `0` | `attempts/pair-00-cwd_environment/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `relay_error` | `relay_or_transport` | `` | `attempts/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `shell` | `success` | `` | `0` | `attempts/pair-00-large_output/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `success` | `` | `7` | `attempts/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `success` | `` | `7` | `attempts/pair-00-nonzero_exit/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `unknown` | `missing_command_event` | `` | `attempts/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `success` | `` | `0` | `attempts/pair-00-background_cleanup/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `success` | `` | `143` | `attempts/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `success` | `` | `143` | `attempts/pair-00-signal_exit/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `transparent` | `timeout` | `codex_timeout` | `` | `attempts/pair-00-repeated_recovery/transparent` |
| `pair-00-repeated_recovery` | `repeated_recovery` | `shell` | `success` | `` | `0` | `attempts/pair-00-repeated_recovery/shell` |
