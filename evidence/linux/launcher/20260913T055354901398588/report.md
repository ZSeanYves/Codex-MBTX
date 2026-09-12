# MBTX execution report

## Evidence summary

```json
{
  "backend_failures": 0,
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

| pair | task | shell | transparent | comparable | observed timing | first difference |
|---|---|---|---|---|---|---|
| `pair-00-background_cleanup` | `background_cleanup` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-00-cwd_environment` | `cwd_environment` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-00-large_output` | `large_output` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-00-literal_argv` | `literal_argv` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-00-recovery` | `recovery` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-00-signal_exit` | `signal_exit` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-01-background_cleanup` | `background_cleanup` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-01-cwd_environment` | `cwd_environment` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-01-large_output` | `large_output` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-01-literal_argv` | `literal_argv` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-01-recovery` | `recovery` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-01-signal_exit` | `signal_exit` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-02-background_cleanup` | `background_cleanup` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-02-cwd_environment` | `cwd_environment` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-02-large_output` | `large_output` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-02-literal_argv` | `literal_argv` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-02-recovery` | `recovery` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-02-signal_exit` | `signal_exit` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-03-background_cleanup` | `background_cleanup` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-03-cwd_environment` | `cwd_environment` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-03-large_output` | `large_output` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-03-literal_argv` | `literal_argv` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-03-recovery` | `recovery` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-03-signal_exit` | `signal_exit` | `success` | `success` | `true` | `unknown` | `none_observed` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `success` | `success` | `true` | `unknown` | `none_observed` |

## Event evidence

| pair | task | backend | phase | event | status | failure | elapsed ms | command | evidence |
|---|---|---|---|---|---|---|---:|---|---|
| `pair-00-literal_argv` | `literal_argv` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-literal_argv` | `literal_argv` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-literal_argv/shell` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-literal_argv` | `literal_argv` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-literal_argv/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-stdin_utf8/transparent` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-stdin_utf8/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-cwd_environment` | `cwd_environment` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-cwd_environment/shell` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-cwd_environment` | `cwd_environment` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-cwd_environment/transparent` |
| `pair-00-large_output` | `large_output` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-large_output` | `large_output` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-large_output/transparent` |
| `pair-00-large_output` | `large_output` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-large_output` | `large_output` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-large_output/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-nonzero_exit/shell` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-nonzero_exit/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-background_cleanup` | `background_cleanup` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-background_cleanup/transparent` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-background_cleanup` | `background_cleanup` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-background_cleanup/shell` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-signal_exit` | `signal_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-signal_exit/shell` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-signal_exit` | `signal_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-signal_exit/transparent` |
| `pair-00-recovery` | `recovery` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-recovery` | `recovery` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-recovery/transparent` |
| `pair-00-recovery` | `recovery` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-00-recovery` | `recovery` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-00-recovery/shell` |
| `pair-01-literal_argv` | `literal_argv` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-literal_argv` | `literal_argv` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-literal_argv/transparent` |
| `pair-01-literal_argv` | `literal_argv` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-literal_argv` | `literal_argv` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-literal_argv/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-stdin_utf8/shell` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-stdin_utf8/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-cwd_environment` | `cwd_environment` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-cwd_environment/transparent` |
| `pair-01-cwd_environment` | `cwd_environment` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-cwd_environment` | `cwd_environment` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-cwd_environment/shell` |
| `pair-01-large_output` | `large_output` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-large_output` | `large_output` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-large_output/shell` |
| `pair-01-large_output` | `large_output` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-large_output` | `large_output` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-large_output/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-nonzero_exit/transparent` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-nonzero_exit/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-background_cleanup` | `background_cleanup` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-background_cleanup/shell` |
| `pair-01-background_cleanup` | `background_cleanup` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-background_cleanup` | `background_cleanup` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-background_cleanup/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-signal_exit` | `signal_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-signal_exit/transparent` |
| `pair-01-signal_exit` | `signal_exit` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-signal_exit` | `signal_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-signal_exit/shell` |
| `pair-01-recovery` | `recovery` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-recovery` | `recovery` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-recovery/shell` |
| `pair-01-recovery` | `recovery` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-01-recovery` | `recovery` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-01-recovery/transparent` |
| `pair-02-literal_argv` | `literal_argv` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-literal_argv` | `literal_argv` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-literal_argv/shell` |
| `pair-02-literal_argv` | `literal_argv` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-literal_argv` | `literal_argv` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-literal_argv/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-stdin_utf8/transparent` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-stdin_utf8/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-cwd_environment` | `cwd_environment` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-cwd_environment/shell` |
| `pair-02-cwd_environment` | `cwd_environment` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-cwd_environment` | `cwd_environment` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-cwd_environment/transparent` |
| `pair-02-large_output` | `large_output` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-large_output` | `large_output` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-large_output/transparent` |
| `pair-02-large_output` | `large_output` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-large_output` | `large_output` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-large_output/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-nonzero_exit/shell` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-nonzero_exit/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-background_cleanup` | `background_cleanup` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-background_cleanup/transparent` |
| `pair-02-background_cleanup` | `background_cleanup` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-background_cleanup` | `background_cleanup` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-background_cleanup/shell` |
| `pair-02-signal_exit` | `signal_exit` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-signal_exit` | `signal_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-signal_exit/shell` |
| `pair-02-signal_exit` | `signal_exit` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-signal_exit` | `signal_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-signal_exit/transparent` |
| `pair-02-recovery` | `recovery` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-recovery` | `recovery` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-recovery/transparent` |
| `pair-02-recovery` | `recovery` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-02-recovery` | `recovery` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-02-recovery/shell` |
| `pair-03-literal_argv` | `literal_argv` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-literal_argv` | `literal_argv` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-literal_argv/transparent` |
| `pair-03-literal_argv` | `literal_argv` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-literal_argv` | `literal_argv` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-literal_argv/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-stdin_utf8/shell` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-stdin_utf8` | `stdin_utf8` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-stdin_utf8/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-cwd_environment` | `cwd_environment` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-cwd_environment/transparent` |
| `pair-03-cwd_environment` | `cwd_environment` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-cwd_environment` | `cwd_environment` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-cwd_environment/shell` |
| `pair-03-large_output` | `large_output` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-large_output` | `large_output` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-large_output/shell` |
| `pair-03-large_output` | `large_output` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-large_output` | `large_output` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-large_output/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-nonzero_exit/transparent` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-nonzero_exit` | `nonzero_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-nonzero_exit/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-background_cleanup` | `background_cleanup` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-background_cleanup/shell` |
| `pair-03-background_cleanup` | `background_cleanup` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-background_cleanup` | `background_cleanup` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-background_cleanup/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-signal_exit` | `signal_exit` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-signal_exit/transparent` |
| `pair-03-signal_exit` | `signal_exit` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-signal_exit` | `signal_exit` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-signal_exit/shell` |
| `pair-03-recovery` | `recovery` | `shell` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-recovery` | `recovery` | `shell` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-recovery/shell` |
| `pair-03-recovery` | `recovery` | `transparent` | `child` | `start` | `unknown` | `unknown` | `unknown` | `unknown` | `unknown` |
| `pair-03-recovery` | `recovery` | `transparent` | `child` | `exit` | `success` | `unknown` | `unknown` | `unknown` | `/home/zseanyves/Moonbit/Codex-MBTX/evidence/linux/launcher/20260913T055354901398588/pair-03-recovery/transparent` |
