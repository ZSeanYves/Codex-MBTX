# M5：Codex-MBTX 替换 Shell 的可信评测协议

状态：实现于 2026-09-10，正式在线样本尚未开始。本文件是 M5 的预注册协议；正式运行前不得静默修改阈值、任务、样本或分类规则。

## 研究问题

主问题是：在 Codex 中，Transparent MBTX 是否值得替换现有 Shell 后端，并且是否足以成为默认后端？

结论对象只包括 Transparent MBTX 替换 Shell 的能力。显式 mbtx 工具和 MoonBit 脚本生成能力属于独立的 script-capability cohort，不混入主结论。

运行臂固定为：

| 编号 | 运行臂 | 因果含义 |
| --- | --- | --- |
| B0 | shell | Codex 现有 Shell 路径 |
| B1 | transparent | 当前真实 Transparent 配置；adapter 在最终启动阶段使用 Direct shell 路径 |
| B2 | bare-proxy | 脱离 Codex 和模型，直接测量 mbtx exec -- wrapper |

B0/B1 的主比较回答后端问题，B2 用来分离 wrapper 自身开销。B2 不被当作 Codex 端到端结果。

## 平台和冻结变量

- Linux 是在线 Codex 和默认替换结论的主平台。
- macOS 只用于本地运行时、语义和回放对照。
- 本协议不对 Windows 作结论。
- 正式 CI 固定 MoonBit compiler `0.10.12+1634b282e`、当前最新依赖 `moonbitlang/async@0.21.3` 和仓库 `rust-toolchain.toml`；不得在 W1--W3 之间升级。Linux 在线 runner 固定为 `ubuntu-24.04`；macOS 确定性 runner 使用 async 官方矩阵已验证的 `macos-latest`，但只在 pilot 采集一次并冻结，实际 OS、架构和工具链以该 run 的 artifact 与 Actions 元数据为准。三个窗口必须使用同一 implementation SHA、runner/evaluator hash、model、reasoning effort、prompt/fixture hash 和 tool catalog。
- 每个 block 固定 task、model、reasoning effort、tool catalog、prompt hash、fixture hash、seed、workspace 初始化方式和 timeout。
- block 内两个 backend 串行执行，顺序由带 seed 的 planner 决定；同一 block 的两次运行共享任务定义但使用独立 workspace/home。formal 的 20 次重复连续划分为 7/7/6 三个窗口，每个窗口由独立 CI workflow run 收集，W2/W3 只能接续前一窗口的完整前缀 artifact。
- block artifact 必须保存 `relay_health`、`time_window` 和该 block 的 `runs`；报告顶层另保存完整 health、`mode`、`planned_blocks` 和 `start_block`，使单个脱敏 block 可以重建判断。
- continuation 只能从完整 block 边界开始，不能重排、覆盖或按失败类型筛选旧样本。
- retry 保留为独立 attempt；不把 retry 后恢复的请求改写成从未失败。
- 300 秒 timeout 只表示 timeout，不作为真实运行时间。
- 冷启动样本不复用跨 block 的工作区、进程或响应缓存；热启动只在同一 backend 的预注册批量阶段复用进程外基础设施，且记录 cache policy/hit/miss。两臂不共享会影响执行结果的缓存。
- pilot 只发现协议和基础设施问题，不提前停止并宣布默认替换；formal 只有完成预注册 block 数或触发 relay validity gate 才停止，触发 gate 时保留已收集 artifact 并标为 INCONCLUSIVE。

## 任务和样本

process cohort 固定 8 个任务：

1. literal argv；
2. stdin/EOF/UTF-8；
3. 特殊路径和文件快照；
4. background/poll；
5. 大 stdout/stderr；
6. 非零退出恢复；
7. cwd/environment；
8. cancellation/timeout。

script-capability cohort 保留现有 6 个 MoonBit 任务，独立报告。

回放轨迹固定为 10 个模板（默认 3 次重复，即 30 条 trajectory；可提高重复次数），覆盖 argv、stdin、cwd、输出顺序/大输出、非零退出、background/poll、cancel、timeout 和空 stdin。回放的 Transparent 臂实际经过评测 launcher -> `mbtx exec --` 最终启动链，并保存 `launcher_start`/`launcher_exit`；Codex 请求编排和 mock Responses/SSE 事件由现有 Rust 集成测试覆盖。回放不是在线性能样本，也不冒充真实模型随机轨迹。

正式在线样本为每个 process 任务 20--30 个成对 block，至少分布在三个独立时间窗口；pilot 为每个任务 5 个成对 block，只用于发现协议或基础设施缺陷，不宣布默认替换。script cohort 每任务 10--20 对样本。

本地确定性 runtime 默认执行 argv、stdin/EOF/UTF-8、cwd/env、大输出、非零退出以及 background/poll、stop、timeout、cancel 共 9 个场景，每个场景 200 次冷启动和 1000 次热启动；冷启动和热启动都保留独立 child launch，热启动只复用已构建的 MBTX binary 与 harness，不伪装成持久 daemon，因而报告会把这一测量边界写明。可通过 M5_COLD_RUNS、M5_WARM_RUNS 和 M5_SCENARIOS 做短 smoke。child 输出内部开始/结束标记，外部 harness 以单调时钟测量 wrapper 和回收开销；主动取消场景允许没有 child end marker，但必须有 start marker、reap 顺序和无 late output。

## Oracle 和可观测性

每个 fixture 的机器可读 contract 包含 `fixture_nonce`、`receipt_path`、`expected_stdout`、`expected_stderr`、`expected_exit_code` 和排序后的 `expected_manifest`；运行时还必须生成 helper receipt 与独立 process trace。`workspace_diff` 至少包含 `entries`、`permissions_unchanged` 和 `symlinks_unchanged`，entries 中保存路径、类型、权限和脱敏 digest/target，不能只报一个成功布尔值。`host_cancel_timeout` 使用 `expected_exit_code=-1` 表示主动取消，不把平台相关 signal 编码混入语义结果。重试任务的每次中间退出仍放在 `attempts` 中，不能被终态字段覆盖。通过条件必须同时满足：

- 任务目标或 result.json 正确；
- helper 确实执行；
- 输入文件、权限和符号链接未被改变；
- 未出现未授权文件；
- 子进程已清理；
- 实际 argv/cwd/stdin 与任务要求一致。

runner 对完整 manifest 中的每个条目保留类型、权限和脱敏 digest；预期输出文件必须是普通文件，输入文件的权限/符号链接状态也必须保持不变。任何 manifest、权限或类型不一致都会进入 workspace/lifecycle 失败，而不会被成功率分析忽略。

observability 有三态：complete、partial、unobserved。backend_observation 有 compliant、violation、unknown。unknown 永远不能转换成 violation；当 approval 证据也缺失时，failure category 为 `backend_observation_unknown`，而不是 backend violation。M5 的可选时间字段在 JSON 中缺失时省略该字段；这是当前 MoonBit FromJson 对可选原生数值字段的稳定 wire 形式。

## Relay 健康门禁

正式 batch 前执行 10 次 probe：

- 至少 9 次成功；
- 不得出现连续两次 provider/transport error；
- first-byte p95 不超过 15 秒；
- probe 记录的 arm/window 必须完整且合法；每个正式窗口分别执行自己的 10 次 probe，单次 probe artifact 只标记当前真实窗口。
- 正式样本的 block `time_window` 必须完整覆盖 1、2、3；每个 block 的窗口编号、batch id、seed 和 health artifact 必须可追溯，analyzer 会拒绝缺失或不足三个窗口的 formal batch。

Probe gate 与 batch gate 分开：probe 允许 10 次中 9 次成功；正式 block 收集完成后，才按全部 block attempt 计算失败率，超过 5% 或两臂差异超过 2 个百分点即使 batch 失效。

batch 中 provider/transport error 超过 5%、usage 完整率不足以计算 token 指标、顺序/seed 变化或 artifact 不完整时，batch 只能标记 INCONCLUSIVE。provider error 不得填成 backend 失败，也不得静默删除。

## 统计视图

所有 block 同时进入 intention-to-treat（ITT）视图：provider error、timeout、missing usage 都保留在计划分母中；`analysis.json` 固定输出计划 arm 数、实际记录 arm 数、Shell/Transparent 各自成功数和按计划分母计算的成功率差异。

backend observation 为 `unknown` 的 run 仍保留在 ITT 分母和单独计数中，但不能进入成功分子；它也不能被改写成 violation。

relay-clean conditional 只使用 provider/transport 完整、observability 为 complete 的成对样本，报告：

- 配对成功率；
- p50/p95/p99 端到端延迟；
- token 和工具调用；
- win/tie/loss；
- 确定性 bootstrap 95% 单侧决策界（固定 seed；成功率使用第 5 百分位下界，p95 延迟使用第 95 百分位上界，同时保存另一侧端点供审计）；
- `analysis.json` 固定记录 CI 方法和 2000 次重采样次数，便于从脱敏 pairs.csv 重建；
- bootstrap/permutation 的重建输入和版本。

relay-clean 但任务结果错误的在线 run 记为 `success=false`，保留在成对成功率和置信区间中；单次或两臂共同的模型任务错误本身不是“确定性后端回归”，不得直接升级为 `NO_GO`。`NO_GO` 的确定性语义依据来自 runtime/replay，安全兼容依据来自可观测的 approval/backend violation。

不使用简单总平均值，不把 missing usage 当作零成本。没有独立账单或可信价格数据时，只报告 token，禁止推导费用。

## 默认替换门槛

只有全部满足才是 GO_DEFAULT：

1. Linux/macOS 确定性语义零回归；
2. approval、sandbox、argv、退出码和生命周期无安全/兼容回归；
3. relay-clean 成功率差异的 95% 单侧下界不低于 -2 个百分点；
4. relay-clean p95 端到端延迟的 95% 上界不增加超过 5%；
5. 没有单个关键 process 任务确定性回归；
6. 证据完整率和 backend 可观测率达到预注册要求；
7. 保存的脱敏 artifact 能重建机器报告。

分类：

- NO_GO：确定性、安全或兼容性失败；
- OPT_IN_ONLY：语义可行但没有足够实际收益；
- INCONCLUSIVE：relay 健康、样本量、可观测性或置信区间不足；
- GO_DEFAULT：全部门槛通过。

## 实现和复跑

核心接口：

- eval/m5.mbt：版本化 schema、planner、oracle、分类、relay gate、汇总和决策；
- cmd/m5-evidence/：suite、fixture/prompt、plan、health、classify、attempts、pair、reliability、decision CLI；
- scripts/m5_runtime.mbtx：本地 B0/B2 确定性 runtime；
- scripts/m5_replay.mbtx：固定轨迹 B0/B1 回放；
- scripts/m5_eval.mbtx：健康门禁和在线 runner contract；
- scripts/m5_probe.mbtx：从脱敏 probe attempts 重建 relay-health.json，不发起网络请求；
- scripts/m5_platforms.mbtx：校验实际 Linux/macOS runtime 与 replay artifact 的 platform 字段并生成跨平台 manifest；
- scripts/m5_analyze.mbtx：成对分析、bootstrap 区间、CSV 和报告更新；
- scripts/m5_faults.mbtx：故障注入；
- scripts/m5_verify.mbtx：无密钥离线验收。

离线验收命令：

    moon run scripts/m5_verify.mbtx

受控入口：

    moon run scripts/m5_eval.mbtx pilot
    moon run scripts/m5_eval.mbtx formal
    moon run scripts/m5_eval.mbtx script
    moon run scripts/m5_probe.mbtx < probes.json
    moon run scripts/m5_analyze.mbtx

在线 runner 必须接受 --block BLOCK_ID，从 stdin 读取一个 M5 block JSON，并返回：

    {"runs": [/* shell run */, /* transparent run */]}

每个 run 必须包含 `attempts`、工具计数、input/cached/output token、`monotonic_start_ms`/`monotonic_end_ms` 起止字段、兼容性的 `started_ms`/`ended_ms`、实际 `stdout`/`stderr`/`exit_code`、`workspace_manifest`、`workspace_diff`、`receipt`、`process_trace`、三态 observability、backend observation 和 failure category。起止字段是 run-local 的单调时钟坐标；没有发生的可选事件时间点只能按统一 wire 规则省略，不能用 `0` 冒充观测值。runner 不能用缺失字段代替 `unknown`，也不能把 provider/transport error 改写成 backend failure。

每个 relay attempt 记录脱敏 `request_group`、组内 `retry_index`、HTTP 状态、first-byte、completed、断流、provider error、token usage 和最终是否恢复。相同 request body 的尝试属于同一组，新的模型工具轮次从 retry index 0 重新开始；固定 Codex proxy dump 记录首字节和完成 wall timestamp，runner 将其转换成 run-local 时间，不能把正常的后续工具轮次计作 retry。

runner 不得删除或覆盖已有 block artifact。未设置 M5_RUNNER 或 relay health 不达标时，入口仍保存 plan、health 和 INCONCLUSIVE report，不发起在线样本。

## Artifact

每次运行保存：

- report.json（schema 5）；
- pairs.csv；
- analysis.json；
- relay-health.json；
- probe-health.json（当前窗口）及 `probe-attempts-window-N.json`、`probe-health-window-N.json`（由保存的 probe attempts 重建）；
- 每个 block 的 `runner-<block>.json` 原始 stdout/stderr/exit code；重复或崩溃尝试使用 `-retry-N`，不覆盖旧 artifact；
- plan.json；
- runtime/replay/fault 原始汇总；replay 每条 Transparent 记录必须保留 launcher trace，Shell 记录必须证明没有 launcher；
- deterministic-platforms.json：只接受由实际 artifact platform 字段证明的 Linux 与 Darwin 配对，并校验二者的 implementation SHA 与完整 MoonBit toolchain 文本等于在线 runner 的 checkout；环境变量声明不能替代它；
- runtime artifact 必须同时保存 child start/end、execution 和 wrapper overhead 的 p50/p95/p99；replay artifact 必须保存 argv、approval 原命令、sandbox/cwd、事件顺序、workspace diff 和子进程清理状态；
- implementation SHA、binary SHA-256、toolchain、platform、model、reasoning effort、prompt/fixture hash；
- `binary_sha256` 在在线运行时指向被测 Codex binary；同时记录 `codex_binary_sha256`、`mbtx_binary_sha256`、`runner_binary_sha256` 和 `evaluator_binary_sha256`。没有 Codex binary 的离线证据仅以 evaluator hash 填充兼容字段，并把三个在线 binary 字段明确记为 unavailable；
- 固定 seed 和复跑命令。

历史 M4 报告保持原样，不与 M5 schema 互相覆盖。任何未通过 relay validity gate 的 batch 只能生成 INCONCLUSIVE，不能产生默认替换结论。

## 受控 CI 调度

在线顺序固定如下，所有命令使用同一个 branch/ref。pilot 同时生成正式规模的 Linux/macOS deterministic artifact；formal 三个窗口通过 run id 复用这组证据，避免机器差异进入 W1--W3 比较。

```bash
gh workflow run m5-evaluation.yml --ref <REF> \
  -f mode=pilot -f enable_online=true \
  -f model=gpt-5.6-terra -f reasoning_effort=xhigh -f time_window=0

gh workflow run m5-evaluation.yml --ref <REF> \
  -f mode=formal -f enable_online=true \
  -f model=gpt-5.6-terra -f reasoning_effort=xhigh -f time_window=1 \
  -f deterministic_run_id=<PILOT_RUN_ID>

gh workflow run m5-evaluation.yml --ref <REF> \
  -f mode=formal -f enable_online=true \
  -f model=gpt-5.6-terra -f reasoning_effort=xhigh -f time_window=2 \
  -f deterministic_run_id=<PILOT_RUN_ID> \
  -f previous_run_id=<WINDOW_1_RUN_ID>

gh workflow run m5-evaluation.yml --ref <REF> \
  -f mode=formal -f enable_online=true \
  -f model=gpt-5.6-terra -f reasoning_effort=xhigh -f time_window=3 \
  -f deterministic_run_id=<PILOT_RUN_ID> \
  -f previous_run_id=<WINDOW_2_RUN_ID>
```

W1 固定收集 block 0--55，W2 收集 56--111，W3 收集 112--159，对应 7/7/6 次完整 task repetition。workflow 会先验证引用 run 与当前 checkout 的 head SHA；evaluator 再验证前缀报告的 seed、二进制、工具链和全部冻结变量。任何窗口 probe 未通过时不增加 block；runner 首次返回不完整 block 时立即停止，只保留此前完整 block 前缀，结果为 INCONCLUSIVE，禁止跳过失败 block 或用后续样本填洞。
