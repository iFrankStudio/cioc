# CL-1B C24B：对原生付费服务计数自修改门的完整新任务压力复验

**PRE_NATIVE_FROZEN_NEW_DISJOINT_STRESS_45_GOALS · 先于任何C24B原生运行已冻结全部45条24阶段目标、186世界、源图与判据。** 这是**复制现有C24代码**的独立新目标试验，不允许依据C24结果调阈值或加新修补。

C24原生F8第一个新冻结45任务组把180节点旧F6主体拓展为195节点：自己每次服务付费`Read40/Write40`累计尝试；编辑前付费Read40，`attempts==2`或`3`才允许原`PC82 PatchNext`，其余跳过，所有动作在主体CIL中发生，宿主不代写。这在此前一批新任务上曾**相比不同源码旧F6节省5615 CIL (6.51%)**而全部真阶段完成，但严格同初始F8源码“所有PatchNext物理效果压制”的研究者对照还比它少付8925 CIL。因此**不能**认为已自主学会可盈利编辑。此次完全不动原F8源码、固定阈值、资源或对照，只增新任务验证这个有限结果是否能外推。

**严格源图冻结：** 本次机器清单内`exact_frozen_F8_source_195_nodes`与[原C24版本](CL1B-C24-PAID-SERVICE-COUNT-EDIT-GATE-frozen-186-worlds-v1.json)逐节点完全相等，原Git blob `fcb1a20f5641b291dc6d702ca81f8fc684e44129`，包含完整PC3.next=184、PC81.next=180、15个新CIL节点。所有F8对照的真实预算6000收费CIL、17数据键、195源码槽、320总存储不变，旧F6BASE与F5源码也不变。

**45条独立新目标：** 全部5类压力族 `reversal_stress`、`dense_noisy`、`early_shocks`、`late_discontinuity`、`sparse_jumps`，每类9项。公开无符号xorshift32初始种子`0xC24B9F31`，按fi/a/d循环，每例取`base=rand()%3,period=2+rand()%7,break1=5+rand()%8,break2=14+rand()%7`，每24阶段各一次`q=rand()`，目标数学式：

- reversal_stress: `(base+floor(t/period)+(t>=break1?2:0)+(t>=break2?d+1:0)+(q%11==0?1:0))%3`
- dense_noisy: `(base+q%3+(t%6==0?a+1:0)+(t>=break1?d+1:0))%3`
- early_shocks: `(base+floor(t/(2+a))+(break1<=t<break2?d+1:0)+(q%5==0?2:0))%3`
- late_discontinuity: `(base+floor(t/period)+(t>=break2?d+1:0)+(q%7==0?1+a:0))%3`
- sparse_jumps: `(base+(t%period==0?d+1:0)+floor(t/(4+a))+(q%9==0?2:0))%3`

**冻结的JSON保存全部45条实际24数列**，与此前P4F6/C22/C23/C24四份冻结历史任务的完整目标序列逐一无交集，45新项自身也无重复；不以名字替代内容校验。每任务四独立真反馈臂：**F8_on**（完整自身付费记账并原生修改）、**F8_off**（相同初始195源码、仍全部收费，只研究者抑制全部PatchNext物理效果）、**原F6BASE**（不同180源码旧基线）、**原F5**（不同153源码旧基线）。额外六个首/中/末固定倒置和强制零假反馈负控，总**186个新原生世界**。

依旧核对所有`PC3真实付费服务`与`PC185 Read40`和`PC189 Write40`，PID每次累计真实有费服务次数，观察`PC181`与PC190/194的付费分支，从原始逐CIL轨迹和独立Python重新计算；严格F8同源码on/off配对首次真实PatchNext物理效果前的所有原始事件完全一致。**不能宣称事后选取较有利样本、免费算历史ROI、研究者帮改源码是主体发明。** 即使新的F8仍比旧F6省费，也是**不同源码策略总经济比较**；B3/B4继续NOT_MET。
