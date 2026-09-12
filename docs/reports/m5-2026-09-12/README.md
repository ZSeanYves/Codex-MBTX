# M5 2026-09-12 脱敏证据索引

本目录保存最终收口时可提交到仓库的 M5 证据。主结论见
[`../m5-decision-2026-09-12.md`](../m5-decision-2026-09-12.md)。结论为
`INCONCLUSIVE`，部署建议为 `KEEP_OPT_IN`。

## 证据边界

- `report.json`、`analysis.json`、`pairs.csv`、`plan.json` 和
  `relay-health.json` 是正式 W1 run `34640581498` 的原始脱敏 artifact，代表
  当前最后一个通过全部 relay gate 的完整 formal 前缀，共 56 个 paired
  block；它不是 160-block 完整 formal 报告。
- `pilot-*` 是有效 pilot run `34632860192`，只用于验证任务和评测链，不参与
  默认后端结论。
- `w2-attempt-1-*`、`w2-attempt-2-*`、`w2-attempt-3-*` 保存三次有证据的
  W2 中转失败。第二次 probe 通过但 batch 失效，因此同时保留完整
  `w2-attempt-2-report.json` 和 analysis；这些数据不能与 W1 拼成有效 formal
  结果。
- `run-index.json` 保存所有选用、拒绝和未运行项；另有一次人工取消且无
  artifact 的 run，也明确列出而不作因果归类。
- `runtime-*-summary.json` 与压缩的 `runtime-*.json.gz` 是 Linux x86_64、
  Darwin arm64 各 21600 条 B0/B2 确定性样本；`.gz` 使用 `gzip -n`，可稳定
  校验摘要。
- `replay-*.json` 是两平台各 30 条 trajectory、60 个 B0/B1 run 的完整回放；
  `replay-*-summary.json` 是便于审阅的重建摘要。
- `fault*.json` 是本地 mock relay 故障注入结果，不进入在线性能统计。
- 原始逐 block runner envelope 体积较大，继续由对应 GitHub Actions run
  artifact 保存；本目录保留能重建聚合的 report、pairs、health 和关键失效
  report，不复制任何凭据。

## 关键文件

| 文件 | 用途 |
| --- | --- |
| `run-index.json` | run、重试、纳入/排除原因和 W3 未运行原因 |
| `report.json` | 最后有效 formal 前缀的 schema-5 报告 |
| `analysis.json` | W1 的 ITT 与 relay-clean 配对统计、CI 和 token |
| `pairs.csv` | W1 的 56 个可重建配对 |
| `task-results-window-1.json` | W1 八个任务的逐 backend 成功数 |
| `probe-health-window-1.json` | W1 10-request relay gate |
| `w2-attempt-*-probe-health.json` | 三次 W2 relay/provider 失效证据 |
| `w2-attempt-2-report.json` | probe 健康但 batch 大面积失败的完整证据 |
| `deterministic-platforms.json` | Linux/Darwin artifact 身份和工具链绑定 |
| `runtime-*-summary.json` | B0/B2 语义、生命周期和 paired overhead 摘要 |
| `replay-*.json` | B0/B1 固定轨迹回放原始证据 |
| `faults.json` | 502、429、断流、malformed、timeout/cancel 等注入结果 |
| `SHA256SUMS` | 本目录全部证据文件的 SHA-256（不包含自身） |

## 复建和核对

```bash
jq '.decision, .validity' docs/reports/m5-2026-09-12/report.json
jq '.' docs/reports/m5-2026-09-12/run-index.json
jq '.success_difference_ci, .p95_difference_ci' \
  docs/reports/m5-2026-09-12/analysis.json
jq '.pair_summaries' docs/reports/m5-2026-09-12/runtime-linux-summary.json
gzip -dc docs/reports/m5-2026-09-12/runtime-linux.json.gz | \
  jq '.samples | length'
(cd docs/reports/m5-2026-09-12 && sha256sum -c SHA256SUMS)
```

完整评测入口仍为：

```bash
moon run scripts/m5_verify.mbtx
moon run scripts/m5_runtime.mbtx
moon run scripts/m5_replay.mbtx
moon run scripts/m5_faults.mbtx
moon run scripts/m5_analyze.mbtx
```
