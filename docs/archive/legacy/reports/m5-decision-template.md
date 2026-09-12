# M5 Codex-MBTX 决策报告

这是 M5 正式在线样本的报告模板。没有通过 relay validity gate 的样本必须保持 INCONCLUSIVE。

## 1. 执行摘要

- 实验 ID：
- schema：
- mode / planned_blocks / start_block：
- implementation SHA：
- Codex / MBTX / runner binary SHA-256：
- evaluator binary SHA-256：
- pilot / W1 / W2 / W3 workflow run id：
- platform：
- deterministic runtime/replay platforms：
- deterministic-platforms.json（由实际 Linux/Darwin runtime 与 replay artifact 生成）：
- model / reasoning effort：
- 最终分类：GO_DEFAULT / OPT_IN_ONLY / INCONCLUSIVE / NO_GO

一句话结论必须明确区分：MBTX runtime 事实、模型随机性、relay 可用性、Codex adapter 行为、任务设计因素和未验证范围。

## 2. 因果边界

说明 B0-shell、B1-transparent、B2-bare-proxy 的比较关系；明确 script-capability 不参与 Transparent 替换 Shell 的主结论。

## 3. Relay 健康与无效样本

记录 probe 数、成功数、provider/transport error、first-byte p95、time windows、每臂基础设施失败率、retry 恢复率和无效 block。ITT 分母不得删除无效样本。

## 4. 运行时确定性

报告 Linux/macOS 的 argv、cwd、环境变量、stdin、stdout/stderr 顺序、大输出、退出码、signal、timeout/cancel、background/poll/stop、子进程清理，以及冷/热启动 p50/p95/p99。说明 child 内部标记如何隔离 wrapper overhead。

## 5. 回放语义

报告固定轨迹的 approval 原命令、解析 argv、sandbox/cwd、输出、退出码、文件快照、事件顺序和残留进程。列出任意 mismatch 的最小 artifact。

## 6. 在线成对结果

分别给出 ITT 与 relay-clean conditional：

| 指标 | Shell | Transparent | 差异 | 95% 区间 |
| --- | ---: | ---: | ---: | --- |
| 成功率 |  |  |  |  |
| p50 |  |  |  |  |
| p95 |  |  |  |  |
| p99 |  |  |  |  |
| input/cached/output token |  |  |  |  |

必须保留 task-level 结果、win/tie/loss、retry 次数和 retry 额外延迟。

## 7. Script capability

单独报告 MoonBit 脚本任务；不得把该 cohort 的 prompt、token 或脚本生成能力解释为 Transparent 替换 Shell 的收益。

## 8. 限制和未验证范围

至少说明平台覆盖、模型/relay 时间窗口、样本量、missing usage、未覆盖 Windows、成本数据是否存在，以及任何无法观测的 backend 行为。

## 9. 决策

逐项列出七个默认替换门槛的证据链接和是否通过。runtime/replay 跨 Linux/macOS、完整 formal cohort、relay-clean 区间或 artifact 任一缺少可信证据，不得填写 GO_DEFAULT。

## 10. 给 Codex 团队的建议

给出具体部署动作：默认、opt-in、继续实验或停止；同时列出需要修复的 adapter、relay、任务 oracle 和证据管线问题。

## 复现信息

- 固定 seed：
- plan.json：
- report.json：
- pairs.csv：
- relay-health.json：
- runtime/replay/fault 汇总：
- deterministic-platforms.json：
- toolchain：
- 复跑命令：
