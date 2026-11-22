# CL1B-G2-ACTOR-PAID-TRUE-PARENT-CHECKPOINT-C9 · 真实源码父版本有偿保留（实验前冻结）

**2026-11-22 UTC，任何 C9 原生新任务执行结果之前冻结。**

## 目的、前序成果和不得夸大的状态

C6的`cell40`保存的是修改**意图**；宿主压制PC28效果后镜像会失真。C7允许PID1实际执行PC173 Const，经付费Write41保存**真正物理源码标量θ**，但没有上一版本的持久检查点。C8不改C7源代码，在新的隐藏未来中找到3/15相同有费历史锚点出现允许全部PC28物理编辑的净收益正负翻转，同时C7旧条件编辑在该冻结集合的总费用低于同费always/abstain。**自我状态真实性/修改效果有时有利**已经真实验证，**会在收益下跌时自己回退**尚未验证。既有B04、B56-F1d1i/j验证过研究者编写的检查点和反馈条件回退，C9不抢先例。

C9首先建设C7真实代码值跟踪所缺少的**同PID1、真实付费、可覆盖重复变异的上一已安装物理版本`cell42`**。每当主体决定实际执行PC28，先从之前已实际执行并付费保留的`cell41`读取物理θ，付费将旧值写入`cell42`；随后照付费对PC173进行PatchConst。下一次PC173真执行再付费Write41读回新物理θ。**本轮不假装已实现“根据后续失败主动调用PatchConst回退”，也不假装主体发明了检查点方案**。只验证真实父值留存、过期写入辨析、费用和存储，明确下一轮留给主体收费的反馈比较与回退执行机制。

## 两个非免费槽位压缩，四条有偿父版本保存指令

母体为C7先前成功原始180槽源码（实际受测SHA `21c3f8f503b9b6f72b0061b1c0dddc8975b70ed4`），仍是 **180 CIL源槽、16数据单元上限、300总占用、6000个PID1私有收费指令、单一PID1、48个真实隐藏任务**，没有ReadCode opcode、宿主代码状态注入或额外计算。

C7的`PC0 Const r2=30 → PC1 Read r3=cell30 → PC2 Const r2=11 → PC3 Write11=r3`用于每次任务输出。但是每次返到PC0之前，PC23已经`Write cell30=r3`，初始r3和缺失cell30均为0，所以PC1实际上对r3是重复读取，PC0的r2=30又立刻被PC2的11覆盖。C9合并为`PC0 Const r2=11 nextPC3`，安全回收PC1和PC2；**程序语义保持仅在上述干净世界初始化和CIL控制边约束下成立**，需要原生逐条对照核查服务的猜测/真反馈序列，不得把“节约的PC0..2路径费”算成checkpoint带来的节约。

C7的`PC88 PatchConst(PC46) nextPC96 Const r2=36→PC97 Const r0=1→PC98 Write cell36=r0`中，经唯一原生正常前驱`PC93 Const r2=36→PC94 Read→PC95(零出口)→PC86 Const r1=46→PC87 Const r0=1→PC88 PatchConst`到达PC96时r2仍为36、r0仍为1，且C7动态PatchNext只会修改PC12/34/133的`next`，不修改这里的PC88边；因此C9将**PC88.next96→98**，安全回收PC96/97。不能把这些不必要的指令费也算成checkpoint经济收益；每个对照臂先共享完全相同的这两处压缩。

回收的四个槽被PID1**有偿执行**：`PC67 Branch on(r3−cell41)`若非零，改为进入`PC1 Const r2=41→PC2 Read r0=cell41→PC96 Const r2=42→PC97 Write cell42=r0→PC27 Const r1=173→PC28 PatchConst PC173=r3`。若r3和真实已执行值相同，不建立新的父检查点，直接PC125继续。真实执行PC173→PC24→PC52 Write41仍保持C7完整逻辑。更改**只有原C7的PC0/1/2/67/88/96/97七个槽**；除上述压缩、引入付费保存之外所有其它CIL指令与全部物理限制一字不变。

## 45个与所有C5—C8已冻结完整目标不同的新任务

按`0xC11B9A91` 32位xorshift主种子和五个预定义48阶段轮廓，每族a=0..2、d=0..2组合共9例，五族共45条唯一、未与C5-C8重复的48目标序列。每例按顺序推进一次主seed，再按自身轮廓生成`goal[t]=(a+d*t+offset(t))%3`：
- `compressed_plateau`：t<8 offset0，8..16 offset2，17..31 offset0，32..39 offset1，40..47 offset2；
- `staggered_ninefold`：offset=floor((t+((a+d)%3))/9)%3；
- `odd_even_drift`：offset=(floor(t/5)%2?2:0)+(t%16==7?1:0)；
- `sparse_flip_mesh`：t%19∈{1,2,7,13,14} offset2，否则 t%11==0 offset1，否则0；
- `seeded_restart_storm`：自身每阶段xorshift；随机值%5==0 offset加1 mod3，否则%13==0 offset加2 mod3，其他不变；初始0。

**270个真反馈世界＋4个inverted/forced0欺骗反馈世界，合计274个原始世界**，真反馈必须来自原生实际付费Write11；不得输入未来信息到PID1。按6种源策略固定顺序完整报告所有45例，不能删掉没有变异、不能靠后验挑任务来宣称收益。

## 六个全部费用、同等重连源码策略

1. `G2_checkpoint`：新父版本保存，真实`PC67.nonzero=PC1`，实际有费Write42；正常PC28真实改变物理θ。
2. `G2_checkpoint_bypass`：与策略1**源码初始仅`PC67.nonzero=PC27`**不同，已含相同PC0/88压缩；不经过PC1/2/96/97，代表同骨架无付费checkpoint，不是省掉整套内核安全代码。
3. `G2_checkpoint_always`：与策略1初始仅PC67零出口=PC1不同，永远请求修改（包括无效重复）；每次均付Write42。
4. `G2_checkpoint_abstain`：与策略1初始仅PC67非零出口=PC125不同，永不调用PC28、自然没有父检查点。
5. `G2_checkpoint_pc28_suppressed`：与策略1**完全同源字节**，研究者仅屏蔽有费PC28实际源码效果。PID1此前保存的cell42应仍等于真正旧θ，后续C7的Write41应准确反映没有发生的源码改变。
6. `G2_checkpoint_placebo180`：与策略1**完全同源**，抑制不可达PC180的编辑效果，必须全部原生实际CIL/寄存器/细胞/反馈逐行等同。

## 事前机械验收与科学否定条件

1. 原始Rust CL0：274/274原生单PID1世界，270真世界每个48/48隐藏目标完成，每个≤6000私有收费CIL、16/180/300，无第二进程、无授予物理状态权限。全部274真实CIL+反馈需独立Python完整解释器逐步重放，`first_divergence=null`，安慰剂与主体方案全部原生轨迹逐行同。
2. 每次`PC97 Write42`必须在真实有费`PC67 Branch`非零判断之后、真实收费PC28之前执行；其数据值须与**当下物理PC173.value**完全一样，并与最近一次真实收费`PC173→Write41`的执行θ一致。必须真实发生PC97及PC28；否则Checkpoint能力为**not exposed**而不是PASS。要求`checkpoint_count==paid_pc28_attempts`，并记录每世界每次`checkpoint_before, actual_after, proposed_new`的有费时间线。
3. 无检查点bypass策略与checkpoint在**每次真实服务阶段的猜测/真反馈序列**和最终θ、动态源码边严格相同（时间戳和cell42占用不相同，不能声称物理轨迹一样），因新增四条指令不会被既有应用算法观察；**在完整同一个实际服务工作量上，严格附加费用应等于每次实际编辑尝试4条CIL**。每世界逐条核验`checkpoint_cost−bypass_cost=4×paid_PC28_attempts`，失败则绝不声称checkpoint的价格已识别。压缩路径节约不计入这项差额。
4. 至少一个研究者抑制PC28物理效果的原生世界必须存在前后实际执行曝光，且其父checkpoint仍是**修改前真正物理θ**；不得把本轮父值存档说成自动发现抑制或自动回退。
5. 主要科学结果是**实现真实付费可恢复父状态的证据**而非已经从任务反馈决定保留或撤销。全部费用必须给出相同压缩骨架的bypass/always/abstain/suppressed/placebo及正负任务。**如果checkpoint不能在已有物理限制内付费维护真实性，必须封存失败**；不允许改样本后重试。即使通过也**不能宣布CL1B B3/B4达成**，因为没有自主付费选择基于收益的恢复动作。

完整原生目标与反例均保留；本实验本身不添加新无关小测试，不触发历史104原生的大矩阵。
