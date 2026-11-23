# CL-1B C22：新任务上完整付费自修改的真实生命周期收益（原生141世界）

**状态：COMPLETE / original Rust native + fully independent per-paid-CIL replay SUCCESS；45/45新目标中主动执行可执行图PatchNext自修改的同源码严格对照**全部经济失败**，未增加任何真实正确阶段。B3/B4 NOT_MET。**

## 来源与可复核性

- [先于任何C22原生运行提交的141世界冻结机器清单](../studies/CL1B-C22-PAID-PATCHNEXT-SELF-EDIT-RETURN-frozen-141-worlds-v1.json)与[完整预注册协议](../studies/CL1B-C22-PAID-PATCHNEXT-SELF-EDIT-RETURN-preregistration.md)，原来源是此前冻结的[POLICY-P4F6 180节点CIL](../studies/CL1B-POLICY-P4F6-frozen-actor-paid-clean-success-refill-542-world-v1.json)，原Git blob `7ad357f0c97f074484ce9428e44ca066b1ebba3c`（见机器清单`frozen_source_git_blob_sha`）。
- [原始141个Rust世界测试](../../cl0/crates/world/tests/cl1b_c22_paid_self_edit_heldout_roi.rs)；[完全独立Python普通收费CIL重放，按原始事件检查首次真实PatchNext效果](../audits/cl1b_c22_paid_self_edit_new_heldout_independent_replay.py)。
- [原生、独立复放及45对严格同源首效果证明：Actions #45156358171 SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/45156358171)，被测代码SHA `3fec55f901b418b81ed06f08c234a78724024977`，141条原始JSONL完备，普通CIL逐指令`first_divergence=null`。
- [永久原件归档并从远端Git重新读取二次SHA256验证：Actions #45156526493 SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/45156526493)，[Git独立证据分支run/SHA钉死的原件目录](https://github.com/iFrankStudio/cioc/tree/cl1b-ci-evidence/run-records/3fec55f901b418b81ed06f08c234a78724024977-45156358171-1)。原始ZIP大小**2,148,381字节**，SHA256 `ef44677809546195960772e7d7dadc093d0a0cf28d2d1adacd35995f93d8a150`；包含141条全程原生轨迹、结果、原生Rust日志、独立复放结果和受测SHA记录，**不是只保存概要报告**。

## 为什么C22不同于C19–C21

C19–C21证明了真实有费源码自读、凭证标记的单物理效果因果差异与若干负经济结果，但其严格单PC效果干预是**研究者在宿主做的物理压制**，没有证明主体自己花钱搜索程序后获得可盈利代码。C22回到之前原生`POLICY-P4F6`的具实际付费反馈策略：相同PID1必须自己执行CIL读取环境真反馈，在`PC46`上付费`PatchConst`修改可执行常量并以`PatchNext`真正修改现用源图的控制边；有费读写修改预算/连胜证据，持续在线修改策略。正因此，**编辑行为确实来自主体执行程序，不能被误认为宿主免费改代码**。

但所有规则、候选范围、连胜阈值、搜索语法是**研究者先写死的有限编码**，没有新基因产生新算法语法，不能凭“自主执行修改”叫做B3/B4。

C22在新提交前冻结45个未出现在原P4F6历史180任务集合（168条不同完整序列）的24阶段目标，涵盖`late_period_shift`、`high_entropy_blocks`、`burst_recovery`、`nonstationary_triplet`、`irregular_phase`五个族，各9例；全部JSON目标及种子`0xC22F6A11`一次冻结，**未根据原生实际成本挑例**。旧P4F6源码完全复用，没有重设本次基础规则；每世界单PID1，6000个私有有费CIL，16数据键、180程序节点上限、总存储300。每个目标运行严格三独立真实反馈臂，45×3=135；再以首/中/末固定目标运行6个倒置/全0假反馈控制，合计**141个原生Rust世界**。

## 三臂的科学比较及其精确边界

**F6_on**：原已冻结180节点程序完全有费执行、真实生效`PatchConst`与全部`PatchNext`可执行代码修改。

**F6_off**：**初始180节点源码完全相同**，仍有费执行同样的每条`PatchConst`和`PatchNext`CIL，且真实`PatchConst`保持生效；但作为研究者反事实，内核抑制**所有`PatchNext`物理修改效果**。这是一组操作效果的**成束消融**，不是某一PC的单因素消融，也不是主体自愿关掉编辑器。两臂相同任务、同一环境与完整预算，要求前首次真正控制边改变之前的原生付费/反馈轨迹完全相同。45/45配对均存在首次真`PatchNext`物理改变，独立检查该瞬间只改变一个可执行边且此前同源全原生记录完全相同。

**F5_on**：继承旧P4F6基线的**不同源码153节点**，与F6只有经济比较意义，不可用F5差额当成F6的单PC因果归因。

## 预注册新任务全量结果（不筛例）

| 新45个24阶段真任务，同一原生预算 | 真实完成阶段 | 45条任务全生命周期付费CIL |
| --- | ---: | ---: |
| **F6_on** — 自己付费生成/执行有限候选并修改可执行边 | 1080/1080 | **83,335** |
| **F6_off** — 相同180源码但物理PatchNext效果全部压制 | 1080/1080 | **56,766** |
| **F5_on** — 不同153源码早期强经济基线 | 1080/1080 | **73,586** |

同源码`F6_off - F6_on = -26,569`条CIL，代表**F6真的自修改，反而贵26569条，约比抑制臂多46.8%**；真实任务正确率未提高。严格45对的完整生命周期结果：**经济获胜0/45、平局0/45、经济亏损45/45**；且**45/45配对发生真实物理PatchNext改变**，排除了“机制全程不触发”的解释。F6_on比异源码F5_on贵9749条CIL（但这不同源码比较不是因果消融），F6_off比F5还少16820条；**不能**因F6_off花得最少，就断言真实个体能直接无代价地选择全局抑制PatchNext。

原生45个F6_on世界一共实际产生**346次`PatchConst`自身代码修改**，并实际执行**300次具有物理效果的`PatchNext`可执行边编辑**；所有CIL步骤、服务、真实任务答案、反馈、有费读取、原生执行和最终源图由独立Python逐事件准确对账（`first_divergence=null`）。其余6个假反馈原生世界也纳入同一独立完整重演；不能由此概括无限敌对环境安全性。全部141原生世界计费总数以已归档原始`cl1b_c22_original_native_141_worlds.json`的`counts.total_original_paid_instructions`为准，**不要只把135真反馈世界费用之和冒充141世界成本**。

本实验**不证明**“修改源代码不可能有收益”，仅在这45条新但人工生成的任务、既有P4F6有限规则、固定预算和整个PatchNext效果成束反事实下得出负收益证据。此前P4F6其它冻结任务中有过有限有费改进，不能抹除；但新分布的强外推负证据要求后续先证明收益检测方法可付费回收，而非继续堆积检查节点。

## 可落实的后续方向：付费收益预估门而非新宿主oracle

既有F6包含有费`Read37`、清洁成功连续观察`Read39`与编辑预算`Read38`，但没有对**修改带来的整期费用回报**提出有费可证伪估计再决定保留，导致固定`PatchNext`策略在所有45个新任务付费亏损。下一轮首先需要区分：成本从哪一项可执行边变化后增加、哪些重复服务/重试产生高负担，以及是否有在主体权限和预算内可得的早期信号能阻止负回报修改。**先开展原始全45世界配对的成本路径归因**；若后续引入新的成本门，必须真实CIL内计算、收费、有独立源码/假反馈对照与冻结未知目标，不能拿`F6_off`这个研究者物理开关当主体已学会策略，也不能借查询未来任务结果进行免费的全知选择。

**CL-1B仍ACTIVE。B3“自主形成长期盈利的累积演化策略”、B4“内生组织闭合”严格NOT_MET。**
