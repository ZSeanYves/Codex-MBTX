# M5 离线验收证据

本目录是 2026-09-10 的脱敏离线快照，不是正式在线实验数据。

- `report.json` 和 `analysis.json` 的结论必须保持 `INCONCLUSIVE`；
- `pairs.csv` 只有表头，因为没有设置 `M5_RUNNER`，在线配对样本为 0；
- `report.json` 同时保留 `block_health_valid=false`，证明没有有效 relay snapshot 时不会把计划 block 当成可分析样本；
- 离线报告把 Codex、MBTX 和 runner 的 binary hash 明确记为 `unavailable`，只记录实际构建的 evaluator hash，避免把证据 CLI 冒充被测在线二进制；
- 本地快照没有 `deterministic-platforms.json`，所以 `deterministic_manifest_valid=false`；`M5_DETERMINISTIC_PLATFORMS` 声明不会被当作跨平台证据；
- 正式 manifest validator 还会检查 Linux/Darwin artifact 的 formal cold/warm 覆盖、生命周期字段和 replay mismatch；当前 smoke artifact 会被拒绝；
- `runtime.json` 使用 `M5_COLD_RUNS=1`、`M5_WARM_RUNS=1` 的 smoke 样本；`replay.json` 使用协议默认的 3 次重复（30 条 trajectory），两者记录 implementation SHA、完整 MoonBit toolchain 和平台；Transparent 每条轨迹经过 launcher 并保留 start/exit trace；二者只验证 schema、语义和生命周期路径，不用于性能结论；
- `relay-health.json` 是未配置 relay 时的失效门禁；`probe-health.json` 是离线 9/10 probe 聚合 fixture；
- `faults.json` 覆盖预注册的本地故障分类；`fault-probe-health.json` 和 `fault-relay-health.json` 是故障注入专用输出，不覆盖 canonical probe/relay artifact；`fault-runner-crash.json` 证明 runner 崩溃时证据仍保留；`plan.json` 保存固定 seed 的完整 pilot 计划；`report.md` 是同一份机器报告的文本渲染。

正式 Linux pilot/formal 运行应保存到新的日期或 experiment-id 目录，不能覆盖本快照。中文解释和当前决策见 [M5 实现验收报告](../m5-decision-2026-09-10.md)。
