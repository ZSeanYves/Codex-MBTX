# MBTX execution comparison

```json
{
  "complete_jsonl": true,
  "failed_arms": 0,
  "formal_pairs_per_stratum": 1000,
  "intention_to_treat": {
    "attempted_pairs": 6000
  },
  "limits": [
    "Minimal observation is primary. Full trace is a separate perturbation calibration.",
    "All arms create new processes. Warmup is not process reuse."
  ],
  "partial": false,
  "purpose": "formal",
  "strict_comparable_pairs": 6000,
  "target_pairs": 6000
}
```

| Stratum | Metric | Pairs | MBTX/Shell | 95% interval |
|---|---|---:|---:|---|
| linux / native-noop / full | spawn_begin_to_fixture_ready | 1000 | 2.2452238054770466 | [2.232807003448524,2.257585940473215] |
| linux / native-noop / minimal | spawn_begin_to_fixture_ready | 1000 | 2.145255557376085 | [2.1345411529697067,2.1562670842540386] |
| linux / native-small / full | spawn_begin_to_fixture_ready | 1000 | 2.238789817513398 | [2.2265278127371593,2.2508642110634067] |
| linux / native-small / minimal | spawn_begin_to_fixture_ready | 1000 | 2.134269538152395 | [2.122918342065945,2.1460196825963327] |
| linux / shell-noop / full | spawn_begin_to_fixture_ready | 1000 | 1.7157340980219924 | [1.7069887197139888,1.7244365175142184] |
| linux / shell-noop / minimal | spawn_begin_to_fixture_ready | 1000 | 1.6574069787301011 | [1.6492185586125698,1.6654299053101216] |
