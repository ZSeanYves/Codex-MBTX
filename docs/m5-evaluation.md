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
- 每个 block 固定 task、model、reasoning effort、tool catalog、prompt hash、fixture hash、seed、workspace 初始化方式和 timeout。process cohort 的模型可见 tool catalog 固定为排序后的 `apply_patch,exec_command,view_image,write_stdin`；与任务无关且在 Default 模式不可用的 `request_user_input`、未启用的 update-plan、web、MCP、code-mode 和 collaboration 工具在 runner 配置中显式关闭。runner 必须从每次真实 Responses 请求的 `tools` 数组重建排序后的 `observed_tool_catalog`，并证明同一 run 内每次请求一致；任何非基础设施 run 的实测目录缺失、不一致或与顶层冻结值不同都会停止当前 block，并使批次保持 INCONCLUSIVE。
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

本地确定性 runtime 默认执行 argv、stdin/EOF/UTF-8、cwd/env、大输出、非零退出以及 background/poll、stop、timeout、cancel 共 9 个场景，每个场景 200 次冷启动和 1000 次热启动；同一 scenario/phase/index 的 Shell 与 bare proxy 构成相邻配对，按 index 奇偶执行 AB/BA 交替顺序，并在每条样本保存 `pair_order` 和 `pair_position`，避免整臂串行时机器负载漂移压过几毫秒 wrapper 差异。artifact 同时保存每个 scenario/phase 的 `bare-proxy - shell` 配对延迟 p50/p95/p99、AB/BA 子组中位数和 win/tie/loss，validator 会从原始样本核对配对顺序与摘要规模。冷启动和热启动都保留独立 child launch，热启动只复用已构建的 MBTX binary 与 harness，不伪装成持久 daemon，因而报告会把这一测量边界写明。可通过 M5_COLD_RUNS、M5_WARM_RUNS 和 M5_SCENARIOS 做短 smoke。child 输出内部开始/结束标记，外部 harness 以单调时钟测量 wrapper 和回收开销；主动取消场景使用 2 秒目标进程，在 receipt 后 80ms 发起取消，允许没有 child end marker，但必须有 start marker、reap 顺序和无 late output。由于 Codex 的 Unix Direct 路径为 launcher 建立独立 session/process group，runtime 的 lifecycle 样本也先以 `setsid + exec` 建立独立组，再向整个组发送 `SIGTERM`、等待预注册的 50ms、最后发送 `SIGKILL`；每条样本必须记录 `cancellation_scope=process-group` 和 `cancellation_grace_ms=50`。这避免把只对 wrapper PID 发信号的人工父子竞态误判为真实 Codex 后端回归。

## Oracle 和可观测性

每个 fixture 的机器可读 contract 包含 `fixture_nonce` 模板、`receipt_path`、`expected_stdout`、`expected_stderr`、`expected_exit_code` 和排序后的 `expected_manifest`；在线 runner 在每个 block 开始时生成新的运行时 nonce，同一 block 的 Shell 与 Transparent 两臂共享该 nonce，不同 block 不得复用，且每条 run 和 helper receipt 必须保存相同值。evaluator 与 analyzer 会拒绝空 nonce、成对或跨 block nonce 不一致，以及任何声称 `receipt_valid=true` 却无法与 run 的 task/nonce 对应的证据。运行时还必须生成 helper receipt 与独立 process trace。`workspace_diff` 至少包含 `entries`、`permissions_unchanged` 和 `symlinks_unchanged`，entries 中保存路径、类型、权限和脱敏 digest/target，不能只报一个成功布尔值。`host_cancel_timeout` 使用 `expected_exit_code=-1` 表示主动取消，不把平台相关 signal 编码混入语义结果。重试任务的每次中间退出仍放在 `attempts` 中，不能被终态字段覆盖。通过条件必须同时满足：

- 任务目标或 result.json 正确；
- helper 确实执行；
- 输入文件、权限和符号链接未被改变；
- 未出现未授权文件；
- 子进程已清理；
- 实际 argv/cwd/stdin 与任务要求一致。

runner 对完整 manifest 中的每个条目保留类型、权限和脱敏 digest；预期输出文件必须是普通文件，输入文件的权限/符号链接状态也必须保持不变。任何 manifest、权限或类型不一致都会进入 workspace/lifecycle 失败，而不会被成功率分析忽略。

process cohort 的目标 helper 由 fixture 中唯一的 `.py` 或 `.mbtx` 输入文件确定。stdout、stderr、最终退出码和 receipt 只能取自 process trace 中 helper 名称与该目标完全匹配的记录；模型用于检查结果的 `ls`、`python3 -c` 等辅助命令仍保留在 trace 中，但不得覆盖目标 receipt 或污染任务流 oracle。trace 中引用的每个流文件还必须验证为当前 agent 证据目录的直接子文件，并符合受控文件名前后缀。script-capability cohort 没有预置目标 helper，继续作为独立能力实验，不把它的流选择规则用于 Transparent 默认替换结论。

`host_background` 与 `host_cancel_timeout` 固定使用 Codex unified-exec session，而不是 shell `&`、nohup、重定向、wrapper shell 或 PID kill。模型必须用精确目标命令和 250ms yield 获得 session ID；background 至少以空 `write_stdin` 轮询同一 session，cancel 使用 120 秒目标进程，先空轮询同一 session，再以唯一一次单字节 Ctrl-C 中断同一 session。runner 从私有 relay request history 的成对 function-call/function-call-output 结构重建脱敏 `session_lifecycle`，保存 target session 数、poll/interrupt/其他写入数和 same-session 判定；`codex exec --json` 的合并 command event 不能单独充当 write_stdin 证据。cancel 只有在该调用链、唯一目标 argv/cwd、一个允许因中断而缺少 exit 的目标 trace、完成的 Codex turn、result 缺失和预清理零残留同时成立时，才可由 runner 生成 stopped receipt。Transparent 同样只允许与该目标中断对应的一个未闭合 launcher trace；其他未闭合命令或 launcher 一律失败。

在线 runner 必须在 Codex 进程退出后先等待固定 100 ms 观测宽限，再查询专用 `mbtx-eval` 用户是否仍有任何进程；只有观测成功且没有残留时 `child_processes_clean` 才能为 true。为隔离下一运行臂，runner 随后可以杀掉残留，但必须分别记录 `residual_before_cleanup`、`harness_cleanup_succeeded` 和宽限时间；harness 兜底清理成功不能覆盖或修复 backend lifecycle failure。

observability 有三态：complete、partial、unobserved。backend_observation 有 compliant、violation、unknown。unknown 永远不能转换成 violation；当 approval 证据也缺失时，failure category 为 `backend_observation_unknown`，而不是 backend violation。M5 的可选时间字段在 JSON 中缺失时省略该字段；这是当前 MoonBit FromJson 对可选原生数值字段的稳定 wire 形式。

## Relay 健康门禁

正式 batch 前执行 10 次 probe：

- 至少 9 次成功；
- 不得出现连续两次 provider/transport error；
- first-byte p95 不超过 15 秒；10-request 小样本使用 nearest-rank 定义，只剩 9 个可观测 first-byte 时 p95 等于其中最大值，不静默丢弃最慢观测；
- probe 记录的 arm/window 必须完整且合法；每个正式窗口分别执行自己的 10 次 probe，单次 probe artifact 只标记当前真实窗口。
- 正式样本的 block `time_window` 必须完整覆盖 1、2、3；每个 block 的窗口编号、batch id、seed 和 health artifact 必须可追溯，analyzer 会拒绝缺失或不足三个窗口的 formal batch。

Probe gate 与 batch gate 分开：probe 允许 10 次中 9 次成功；正式 block 收集完成后，才按全部 block attempt 计算失败率，超过 5% 或两臂差异超过 2 个百分点即使 batch 失效。

batch 中 provider/transport error 超过 5%、usage 完整率不足以计算 token 指标、顺序/seed 变化或 artifact 不完整时，batch 只能标记 INCONCLUSIVE。provider error 不得填成 backend 失败，也不得静默删除。

## 统计视图

所有 block 同时进入 intention-to-treat（ITT）视图：provider error、timeout、missing usage 都保留在计划分母中；`analysis.json` 固定输出计划 arm 数、实际记录 arm 数、Shell/Transparent 各自成功数和按计划分母计算的成功率差异。

backend observation 为 `unknown` 的 run 仍保留在 ITT 分母和单独计数中，但不能进入成功分子；它也不能被改写成 violation。

relay-clean conditional 只使用 provider/transport 完整、observability 为 complete 的成对样本，报告：

- 配对成功率；
- p50/p95/p99 端到端延迟；所有 M5 runtime、probe 与在线分析统一使用 nearest-rank 定义，缺失样本不能通过较低秩静默压低尾延迟；
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

每个 run 必须包含 `attempts`、工具计数、`observed_tool_catalog`、`tool_catalog_consistent`、`session_lifecycle`、input/cached/output token、`monotonic_start_ms`/`monotonic_end_ms` 起止字段、兼容性的 `started_ms`/`ended_ms`、实际 `stdout`/`stderr`/`exit_code`、`workspace_manifest`、`workspace_diff`、`receipt`、`process_trace`、三态 observability、backend observation 和 failure category。起止字段是 run-local 的单调时钟坐标；端到端时延优先取 `started_ms` 到已观测的 `turn_completed_ms`，缺少完成事件时只能回退到 `monotonic_start_ms` 到 `monotonic_end_ms`。`process_launch_ms` 与 `process_exit_ms` 则由独立 trace 中目标 helper 的 wall timestamp 转换为 run-local 坐标，分别表示最早目标调用开始和最晚目标调用结束；它们只描述目标进程区间，不能作为 agent 端到端时延的回退。Codex 子进程自身的边界只以 `codex_launch_ms`/`codex_exit_ms` 保存在私有 meta。没有发生的可选事件时间点只能按统一 wire 规则省略，不能用 `0` 冒充观测值。runner 不能用缺失字段代替 `unknown`，也不能把 provider/transport error 改写成 backend failure。

每个 relay attempt 记录脱敏 `request_group`、组内 `retry_index`、HTTP 状态、first-byte、completed、断流、provider error、token usage 和最终是否恢复。相同 request body 的尝试属于同一组，新的模型工具轮次从 retry index 0 重新开始；固定 Codex proxy dump 记录首字节和完成 wall timestamp，runner 将其转换成 run-local 时间，不能把正常的后续工具轮次计作 retry。

runner 不得删除或覆盖已有 block artifact。未设置 M5_RUNNER 或 relay health 不达标时，入口仍保存 plan、health 和 INCONCLUSIVE report，不发起在线样本。runner 在下一个完整 block 边界前退出、返回非零或给出无法解析的 envelope 时，evaluator 必须先保存 `runner_errors` 和 INCONCLUSIVE report，再以非零状态结束，使 CI 明确失败；该不完整 block 不得进入样本，也不得由后续 block 填洞。

Pilot 协议修正记录：Actions run `34513877160` 在 implementation `cf1ca4d` 上只完成确定性阶段，未发起 relay probe 或在线 block。该 run 的 Darwin lifecycle 样本有 3/21600 次在固定 80 ms 等待结束时 child 尚未写出 start marker（Shell 2 次、bare proxy 1 次），跨平台 manifest 按零回归门槛拒绝了产物。后续版本改为 child 先写 start marker 和唯一 receipt，harness 观测 receipt 后才开始 stop/timeout/cancel 计时；离线验证注入 200 ms 启动延迟以防固定 sleep 回归。该无效 run 仅作为 fixture 缺陷诊断证据，不得并入 pilot 或 formal 统计。

第二次诊断 run `34519793691` 在 implementation `474e463` 上完成了 Linux/Darwin 零回归确定性证据，但 runtime 仍按整臂顺序先执行全部 Shell、再执行全部 bare proxy；Darwin 简短任务的时延方向反转表明时段负载漂移可能压过 wrapper 差异。该 run 在 relay probe 前取消，未产生在线 block；后续版本把 runtime 改为相邻配对且 AB/BA 交替，并由 artifact validator 验证实际顺序。该 run 只证明 receipt 修复与语义零回归，不进入最终性能或在线结论。

第三次诊断 run `34526824314` 在 implementation `ba2eaf1` 上完成了最终规模的 Linux/Darwin 相邻配对确定性证据，但 live probe 在写任何 attempt 前因重复创建已存在的 `_build/m5-results` 目录而退出。该 run 的在线 artifact 明确记录 0 probe、0 block、0 run；后续版本把结果目录准备改为幂等操作，并在离线验证中连续两次覆盖预先存在目录的路径。该 run 不进入 pilot 或 formal 在线统计，双平台确定性 artifact 也不作为后续冻结证据复用，避免跨 implementation SHA 混用。

第四次诊断 run `34533540258` 在 implementation `40f6e2d` 的 Linux 确定性阶段通过，但 Darwin 的 `bare-proxy/stop` 热启动样本出现 1/21600 次 late output。审计显示原 harness 仅向 wrapper PID 发 `SIGTERM`，50ms 后杀掉 wrapper，恰好与真实 Codex 对独立进程组发信号的语义不同；Codex Direct executor 在 Unix 上建立 session/process group 并向整个组发 `SIGTERM`。后续版本把 B0/B2 lifecycle harness 对齐为 `setsid + exec`、group `SIGTERM`、固定 50ms、group `SIGKILL`，同时将 scope/grace 写进并强制校验 artifact。该 run 仍不进入任何正式统计；这项更改不放宽 50ms 门槛，而是消除了不等价的单 PID 注入方式。

第五次诊断 run `34546904733` 在 implementation `7b055f4` 上完成了 Linux/Darwin 零回归确定性 artifact，并启动了真实 relay pilot。10-request probe 实际收到断流、502 和超慢首字节，健康门禁按预注册规则拒绝了这批在线样本；同时发现 collector 将 nullable primitive 直接放入对象时产生的 Option wire 形态无法被 probe decoder 稳定解析，导致健康摘要未能落盘。该问题属于评测器证据边界，不是 backend 结果；后续版本在 collector 使用显式 `number|null` 编码，在 parser 端兼容历史单元素 Option 数组并对缺失字段生成失效但可审计的 health artifact。该 run 的在线 block 仍为 0，不进入正式统计。

第六组诊断 run 使用 implementation `8e89536`。Pilot run `34556256403` 生成了 Linux/Darwin 各 21600 条 runtime 样本和各 30 条 replay trajectory，确定性语义、trace 与子进程清理均为零回归；但 probe 只有 3/10 完成，包含 2 次 HTTP 502、5 次 transport disconnect，最长连续失败 7 次且 first-byte p95 为 53221 ms，故在线 block 为 0。Formal W1 首次尝试 `34560405930` 的 probe 只有 5/10 完成，包含 1 次 HTTP 502、4 次 disconnect，最长连续失败 3 次且 p95 为 30243 ms，同样没有在线 block。两次均由预注册 relay gate 正确拒绝，只作为 relay 稳定性证据，不进入 pilot 或 formal 对比统计。

Formal W1 第二次尝试 `34562806033` 的 probe 达到 9/10、最长连续失败 1 次；旧 percentile 实现报告 first-byte p95 为 3419 ms，并据此允许进入首个 block。两臂实际完成目标并生成正确结果、receipt、manifest 和 process trace，但 Linux 用户探测命令 `id -u` 的 stdout 泄漏到 runner envelope，使严格 JSON 解析失败；同时旧 oracle 把后续辅助校验命令的输出并入目标 helper 流，并以命令文本必须包含 workspace、`python3` 或 `moon` 的任意规则误拒绝合法 `ls -l`。最终 artifact 保存 `runner_errors=1`、0 个完整 block，却错误返回绿色 CI。该 run 暴露的是 runner/oracle 缺陷，不是 backend 失败，不进入 formal 统计。后续版本隔离用户探测输出，按 fixture helper 绑定流、退出码和 receipt，把 approval 检查限定为“不向模型可见命令注入可信 launcher”，并让任何 runner error 在报告落盘后使 CI 失败；按现行 nearest-rank 规则，该 run 的 15019 ms 最慢观测也会使 probe gate 直接失败。

第七次诊断 run `34564607726` 在 implementation `5d42840` 的双平台 deterministic 采集阶段被主动取消，尚未执行 relay probe 或在线 block。静态复核发现 probe 的旧 p95 使用 `(n-1)*p` 下取整：当一次 probe 无 first-byte、只剩 9 个值时会选择第 8 大秩，run `34562806033` 因而把一个 15019 ms 观测排除并报告 3419 ms。该方法在 9--10 个小样本上不够保守；后续版本固定为 nearest-rank，并增加 9 个可观测值中最慢值为 15001 ms 时健康门禁必须失败的回归测试。该取消 run 没有完成 artifact，不进入任何性能、pilot 或 formal 统计。

第八次诊断 run `34565719119` 在 implementation `eb12f85` 的双平台 deterministic 采集阶段被主动取消，尚未执行 relay probe 或在线 block。runner 的旧 teardown 在检查前先对专用评测用户执行 `pkill -KILL`，所以 `child_processes_clean=true` 只能证明 harness 最终清理成功，不能排除 backend 曾遗留子进程。后续版本改为固定宽限后先观测该 UID 的所有进程，再单独记录并执行兜底清理；任何预清理残留都保持 lifecycle failure。该取消 run 没有完成 artifact，不进入任何性能、pilot 或 formal 统计。

第九次诊断 run `34567805251` 在 implementation `f194ab5` 的双平台 deterministic 采集阶段被主动取消，尚未执行 relay probe 或在线 block。最终只读协议审计发现 suite 中的 `fixture_nonce` 仍是任务版本常量，重复 block 无法用 nonce 证明 receipt 属于本次运行。后续版本在 runner 中为每个 block 生成新的运行时 nonce，让成对两臂使用相同值，并要求 run 与有效 receipt 的 task/nonce 一致。该取消 run 的 deterministic artifact 不复用，也不进入任何性能、pilot 或 formal 统计。

同一次协议冻结审计还发现旧 `argv_cwd_valid` 会因为正确的 literal argv 包含 `SHOULD_NOT_EXIST` 字样而拒绝 `host_literal_argv`，同时没有逐任务验证目标 helper 的参数序列。正式采集前将该规则改为按 helper basename 筛选目标调用，并严格验证每个 process fixture 的 argv；`host_exit_recovery` 必须依次出现无参数调用和 `--confirm` 调用。未授权文件是否出现继续由完整 workspace manifest 独立判断，不能用参数字串替代文件系统证据。

第十次诊断 run `34574556966` 在 implementation `4eb7b78` 上完成 Linux/Darwin 零回归确定性证据，在线 probe 10/10 完成、provider/transport error 为 0、first-byte p95 为 7104ms。在线阶段保存了 5 个完整 block 和第 6 个 block 的部分 runner evidence，但 Codex 达到 300 秒 agent timeout 后，runner 无界等待仍被后代进程持有的 stdout/stderr pipe reader，最终 evaluator 自身触发 `TimeoutError` 且没有生成 report。后续 implementation `694334a` 对超时后的 pipe reader join 设置 1 秒上限，并要求 evaluator 捕获 runner timeout、保存 `runner_errors` 与 INCONCLUSIVE report 后失败退出。该 run 没有完整 pilot 报告，所有在线 block 均视为诊断数据，不进入 pilot 或 formal 统计。

第十一次诊断 run `34587555743` 在 implementation `694334a` 上完成 Linux/Darwin deterministic job，但跨平台 validator 在 relay probe 前拒绝 Darwin runtime：21600 条样本中，`stop/shell/cold` 有 1 条在高负载下等待独立 signal helper 调度时让 0.5 秒目标自然结束，记录为 exit 0、late output 和 cleanup failure。Linux 与 Darwin 的其余 runtime/replay 样本均零回归；该 run 没有 probe 或在线 block。后续版本保持 receipt 后 80ms 取消和 50ms TERM grace 不变，只把被测目标寿命扩展到 2 秒，并以 stop/timeout/cancel 成对压力样本验证，避免把 runner 调度延迟误判为 backend 生命周期回归。该 run 不进入任何性能、pilot 或 formal 统计。

第十二次诊断 run `34594654731` 在 implementation `0374ceb` 上完成 Linux/Darwin deterministic job，且 10-request relay probe 步骤通过预注册 gate。正式 block 执行期间，对第十次 run 保存的逐臂 evidence 做独立复核后发现：旧 background/cancel prompt 允许模型用 shell 后台语法代替 unified-exec session，而 `codex exec --json` 会把真实 `write_stdin` 合并进原 command event，旧 runner 因而无法可靠区分“未调用”与“调用但不可见”；cancel shim 被进程组中断后，runner 又因 workspace 已归专用用户所有而无法原子写入 synthesized receipt。当前 run 因已知测量缺陷主动中止，不生成可用 pilot 或 formal 样本；已完成的双平台确定性结果只作为诊断记录，不跨 implementation SHA 复用。后续版本改为从私有 request history 重建工具/session 事实、冻结精确 lifecycle prompt、在无残留后恢复 runner ownership，并使用多源交叉条件生成取消 receipt。

## Artifact

每次运行保存：

- report.json（schema 5）；
- pairs.csv；
- analysis.json；
- relay-health.json；
- probe-health.json（当前窗口）及 `probe-attempts-window-N.json`、`probe-health-window-N.json`（由保存的 probe attempts 重建）；
- 每个 block 的 `runner-<block>.json` 原始 stdout/stderr/exit code；重复或崩溃尝试使用 `-retry-N`，不覆盖旧 artifact；
- 每条 run 的脱敏 `session_lifecycle`，包括 request history 完整性、目标 session 数、poll/interrupt/其他写入计数与 same-session 结论；
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
