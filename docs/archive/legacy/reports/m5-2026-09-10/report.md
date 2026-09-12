# M5 Codex-MBTX Decision Report

Experiment: m5-pilot-20260910. Schema: 5.
Implementation: 3f6bbad4f8f2bf3c496ea5438457562b1bc6916b. Platform: Darwin arm64. Model: unspecified.

## Execution Summary

B0-shell, B1-transparent and B2-bare-proxy are kept separate. Provider/transport failures and unknown observations remain infrastructure/evidence states.

## Relay and Validity

Saved validity: {"relay_health_valid":false,"relay_batch_valid":false,"planned_time_windows_valid":false,"block_health_valid":false,"observed_batch_valid":false,"runner_configured":false,"runner_errors":0,"evidence_complete":false,"backend_observable":false,"intention_to_treat_included":true,"relay_clean_conditional_available":false,"runtime_artifact_present":true,"runtime_artifact_valid":true,"runtime_artifact_complete":false,"replay_artifact_present":true,"replay_artifact_valid":true,"replay_artifact_complete":true,"deterministic_platforms":"Darwin arm64","deterministic_cross_platform_valid":false,"deterministic_manifest_valid":false,"artifacts_rebuildable":true,"analysis_pair_count":0,"confidence_intervals_computed":false,"intention_to_treat_retained":true,"relay_clean_conditional":false,"token_metrics_available":false}.
Planned runs: 0 blocks; recorded runs: 0.

## Intention-to-treat

Planned arms: 80; recorded arms: 0. Shell success rate: 0 bp; Transparent success rate: 0 bp; difference: 0 bp. Provider/transport, timeout and missing-usage runs remain in this denominator.

## Relay-clean Paired Analysis

Paired blocks: 0.
Win/tie/loss: shell 0, transparent 0, ties 0.
Attempts: total 0, retries 0, recovered 0, provider 0, transport 0. Retry token cost: input 0, cached 0, output 0; retry elapsed 0 ms.
Total tokens: shell input/cached/output 0/0/0; transparent input/cached/output 0/0/0. Tool calls: shell 0, transparent 0.
Success difference (basis points): 0; 95% bootstrap interval: unavailable (no relay-clean paired samples).
p95 difference (basis points): 10000; 95% bootstrap interval: unavailable (no relay-clean paired samples).

## Decision

{"status":"INCONCLUSIVE","reasons":["relay batch health invalid","block time-window coverage is invalid","one or more blocks have an invalid relay health snapshot","no planned blocks","no clean observed runs","pilot or script cohort cannot produce a default-backend decision","formal process cohort is incomplete","deterministic runtime/replay sample size is below the formal gate","Linux and macOS deterministic evidence is not both present","deterministic platform manifest is missing or invalid","fewer than preregistered paired samples","evidence is partial or backend observation is unknown"]}

A missing health gate, insufficient paired samples or unknown backend observation is INCONCLUSIVE, not a backend failure. Cost is not inferred without independent price data.

## Reproduction

Use report.json, analysis.json, pairs.csv, plan.json, relay-health.json, the saved hashes and the exact runner command.
