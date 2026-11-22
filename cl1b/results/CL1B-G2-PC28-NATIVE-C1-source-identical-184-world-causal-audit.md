# CL1B-G2-PC28-NATIVE-C1 · 原生同源 PC28 参数修改因果审计：新环境未见净收益

**2026-11-22 UTC；严格结果：原生因果对照 VERIFIED，个体自主优化未得到证据。**

## 来源与真正的验证级别

- [完整 GitHub Actions #44983484895 — SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44983484895)，唯一被验证的实验源码提交 SHA `9d8bef2712c4c5750dd416ddedc59990219010c2`；**scope=full，104/104 Rust 原生集成目标、63/63 独立观察器全部通过，failed_target_count=0，failed_observer_count=0，证据归档发布 SUCCESS**。
- [独立源码解释器逐指令核对](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/9d8bef2712c4c5750dd416ddedc59990219010c2-44983484895-1/cl1b_g2_pc28_c1_independent_native_replay.json)：184/184 个原始单 PID1 Rust 世界、**306,432/306,432 条真实私有付费 CIL 指令精确一致，first_divergence=null**；完整验证包括 CIL PC、4个寄存器、Const源码的实际修改/不修改、执行边、单元存储、额度及真实反馈。
- [完整原生世界结果](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/9d8bef2712c4c5750dd416ddedc59990219010c2-44983484895-1/cl1b_g2_pc28_c1_native_184_worlds.json) · [测试全目标清单](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/9d8bef2712c4c5750dd416ddedc59990219010c2-44983484895-1/summary.txt) · [精确 SHA 与发布成功记录](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/9d8bef2712c4c5750dd416ddedc59990219010c2-44983484895-1/result.txt)。
- [先行预注册](../studies/CL1B-G2-PC28-NATIVE-C1-preregistration.md)、[完整45新任务+184环境与G2原始源码冻结](../studies/CL1B-G2-PC28-NATIVE-C1-frozen-original-native-184-worlds-v1.json)、[原生Rust对照和失败关闭测试](../../cl0/crates/world/tests/cl1b_g2_pc28_native_c1.rs)、[完全独立的Python原CIL解释器](../analysis/cl1b_g2_pc28_c1_independent_native_replay.py)。

原先的 G2 已证明 PID1 真实付费执行 PC28 PatchConst 并让 PC173 决策参数影响后续分支，**但从没有隔离“PC28改变未来源码的效果”自身对于全生命周期费用的因果作用**。后来的 [+156 条源码级探索](CL1B-G2-PC28-exploratory-counterfactual-statistical-evidence-audit.md) 使用的是旧的、已见过的45组样本，且没有被原生 VM 执行，更不能被冒充成新原生证据。

## 唯一改动的“物理”：研究者局部抑制修改效果

完全同一个已经冻源的原始 **G2_changed_allow 180节点 CIL**，在 45 组先冻结、未在 R1/G1/G2 出现过的24任务阶段中运行四臂：

1. `G2_on`：默认原生物理，PID1 自己执行付费 PC28 PatchConst、可以修改以后真正会执行的 PC173 Const。
2. `G2_pc28_suppressed`：**初始源码同 G2_on 每个字节一致**；研究者只在原生 PID1=1、原始 PC=28、代码目标=173 匹配时，保留完整 PC28 的收费、边/寄存器/代码目标合法性检查、下一PC，但剥离对 PC173 Const 的写入效果；所有其它源码修改以及普通 CIL Write 仍完全生效。这个开关**并非演员可调用指令，不能赋予个体先知能力**。
3. `G2_placebo180`：使用原本不存在的 PC180 配置相同抑制律作安慰剂。原始 180源码槽下它不可执行；**全部45对原生 CIL 和反馈 JSONL 逐事件逐字相等**。
4. `G2_never`：同实验环境里独立加载原已冻结的“始终不恢复额度”程序，**不同源码**，只能作强固定策略成本基线，不得冒充同源干预结果。

原内核默认行为、CIL指令集、任务服务、反馈、额度、储存限制均不变。独立测试另强制验证：当研究者错误指定其他真实 opcode 或错误目标代码节点，原生 VM 必须消耗当次指令并失败关闭（trap），绝不静默屏蔽无关执行。

每个世界 **16 数据单元、180 源码节点、300 总占用、6000 私有付费 CIL**，没有第二个 PID 或给个体赠送资源。所有 **180 真反馈世界完成24/24真阶段、合计4320真实阶段**；其余4个为不能算真完成的欺骗反馈控制。成本差均在真正任务完成后计算，不允许靠提前停机“省费用”。

## 五族新环境：同源因果差的符号反转

| 预冻结环境族（各9对） | G2允许修改 | G2仅抑制PC28效果 | PC180安慰剂 | G2始终不恢复（不同源） | 同源因果差 off−on |
|---|---:|---:|---:|---:|---:|
| slow_two_regime_changes | 14,177 | 14,177 | 14,177 | 13,155 | 0 |
| fast_reverting_pulses | 16,578 | **16,447** | 16,578 | 15,752 | **−131** |
| late_reverse_drift | 15,084 | 15,084 | 15,084 | 14,946 | 0 |
| reset_with_sparse_impulses | 16,488 | 16,488 | 16,488 | 16,383 | 0 |
| low_frequency_terminal_perturbations | 13,934 | 13,934 | 13,934 | 13,934 | 0 |
| **45组原生真反馈总成本** | **76,261** | **76,130** | **76,261** | **74,170** | **−131** |

**预先约定的 Δ=cost(off)−cost(on)：负数表示允许自修改多消费CIL。** 所以本轮在45个真正预冻结新世界里，**允许PC28改参数产生了额外131条真实原生CIL费用**，没有观察到净收益。逐例胜负为 **0个正Δ、1个负Δ、44个0Δ**。这不是一个新模型能稳定泛化的证据，也不是其总体收益为负的无分布假设数学定理。

**物理干预确实发生**：原生 PC28 的抑制事件累计 **28次**、覆盖 **26/45** 个配对世界；**19/45** 世界从未执行可抑制的PC28，因此两臂原始轨迹完全一致。即使在26个真实暴露的世界中，**25个净成本同价，只有1个受损**。原生逻辑验证 first intervention 以前的逐事件CIL历史等价、PC180 placebo 的每一条原始 CIL/反馈轨迹严格等价；独立解释器又核实了完整184世界、306432条真实收费指令，first_divergence=null。

## 一个因果分叉的真正原生局部证人

唯一非零的 [`c1_fast_reverting_pulses_a0_d1` 原始 on JSONL](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/9d8bef2712c4c5750dd416ddedc59990219010c2-44983484895-1/cl1b_g2_pc28_c1_original_c1_fast_reverting_pulses_a0_d1_G2_on.jsonl) / [同源 off JSONL](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/9d8bef2712c4c5750dd416ddedc59990219010c2-44983484895-1/cl1b_g2_pc28_c1_original_c1_fast_reverting_pulses_a0_d1_G2_pc28_suppressed.jsonl)：

- 前**735条原生CIL/反馈 JSONL事件完全相同**，首次轨迹差异恰在付费步**720的PC28**：on把PC173参数theta从0改为1，off仍收取PC28并记载“suppressed_pc28”，theta保持0。
- 到付费步**1326**，两臂的原生 PC173 真正执行；on读取之前保存的 **theta=1**，off只读到 **theta=0**。
- 付费步**1328**，原生 PC175 两臂此刻 `r3=0`：on由于差值`r3−1=-1`，选择PC176 **恢复学习额度**；off因`r3−0=0`选择PC179 **跳过恢复**。这不是模拟的分支预测，而是实际已归档的原生PC/寄存器/代码数据差。
- 结果 on 整个真实24任务付费 **1,870** 条，off **1,739** 条；on 有 **2次真实额度恢复 / 3次原生PC122在线修改 / 41次任务探测**，off **1次恢复 / 2次PC122修改 / 39次任务探测**；两者都实际完成24个目标。**自修改源参数确实通过改变未来控制流导致付费路径差异，却在这一真实任务上多付131条。**

这里“因果”严格限于**单一原生源码效果干预**下观察到的成对轨迹成本差；并不等于评估了原本研究者之外的另一个自主发明者，也不证明其他分布会重演这条分叉。

## 欺骗反馈不被作为成功训练

另外4个控制（inverted/forced0 × on/off），全部实际产生原生轨迹且计入306432条完整CIL，未混入上表45对真完成样本：

- inverted 两臂分别付费**1740**条，在**42次**真实任务探测中仅**18次**真匹配目标，但接受了24次伪成功反馈、原生 cell21 仍声明成功，不能计入真实24阶段成功。
- forced0 两臂分别付费**65**条，三次探测仅1次真匹配、反馈全0，原生失效安全停机，cell21没有成功声明（未被创建）。
  
当前G2只能读取反馈值，不能认证反馈来源；即使有原生因果作用，也不足以证明具备可靠的自我改进。

## 与旧探索对照、方法边界与下一决定

之前的**45旧世界、源码级模拟**得到 off−on=+156（4正/2负/39平），新**45未来冻结世界、原生VM**得到 off−on=−131（0正/1负/44平）。二者**来源、任务集与证据等级不同**，因此不可当成严格的“同分布前瞻性复现检验”，但它们足以拒绝仅凭旧+156声称参数更新普遍有益。此次新测试完全没有按旧正例筛选环境，也没有后验调参数。

最新已经验证的事实如下：**单个PID1能付费改变源参数 → 稍后在同一生命周期重新执行 → 改变下一次原生分支，且完整净成本能通过同源抑制严密分离**。但是在固定24阶段任务下，多数改变没有机会在生命周期中带来有意义的收益，且该人工写入的上次猜测阈值并没有验证过的未来风险预测性。甚至不用修改未来参数的不同源紧凑/固定基线仍低于所有改写策略，这也是不可回避的资源竞争因素。

**停止继续反复手动调猜测值规则的必要性更强**：下一步应区分“可信的任务证据/错误反馈”和“未来剩余任务足以摊销自修改”的机制，再探索能被 PID1 自己付费更新的收益预测；须另行冻结更长且可持续的真实任务世界、相同真实完成约束、强固定基线与原生源级因果对照。不可偷偷延长原始6,000计算额度或给个体外置决策器。不能把研究者事前选择的阈值称为个体的算法发明。

**状态：C1原生同源因果证据 VERIFIED；G2参数修改在新任务上的总费用优势 FALSIFIED；CL1B OPEN，B3/B4 NOT MET。**
