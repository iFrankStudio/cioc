# CL-1B B56-F1d1d — paid generic graph instruction encoding breaks the literal-Branch source-template closure (preflight v1)

**FROZEN BEFORE native code implementation and evidence. Engineering mechanism only, NOT same-PID life, autonomous algorithm discovery or B3/B4.** It follows B56-F1d1c's source-proven strict finding: original F1c has13 Branch(test3) templates, B53 requires Branch(test1), and its original CloneNode/PatchConst/PatchNext grammar cannot create the exact B53 branch.

## Research question and unaltered substrate

Does the **already existing generic CIL GraphNew + GraphEncode** facility allow an actor to pay ordinary instructions to encode and execute a real Branch(test1) node **without any Branch opcode in the original actor source**, under the unchanged strict16 data cells/180 code nodes/300 total occupied units? Does the same operation change the process identity?

The kernel already supports five-word opcode descriptors, with GraphEncode reading five ordinary data cells and appending a newly decoded Op into a graph owned by the actor, and GraphLaunch transferring the graph into **a newly created PID**, not mutating the original actor's executable graph. No kernel instruction changes or privileged host code insertion are allowed.

## Frozen paid native experiment

- Three deterministic isolated strict-capacity worlds, each with exactly one original parent PID1 and **1,000 initial charged credits**; world cap `World::with_strict_capacity(16,180,300)`. No additional compute pool, no actor-privileged interventions, no free host graph changes.
- Two constructive arms `BRANCH_ZERO` and `BRANCH_ONE`: original parent source includes **no Op::Branch at any position** and contains only ordinary paid `GraphNew`, `Const`, `Write`, `GraphEncode`, `GraphLaunch`, `Halt` instructions; its linear assembler writes **five descriptor words to cells40..44** for each of seven graph nodes, then executes paid GraphEncode. The researcher authors the descriptor values in ordinary parent Const operands, explicitly NOT an endogenous objective/policy.
- The seven descriptor-decoded child opcodes are:
  0. `Const(dst1,0 or1,next1)`
  1. **`Branch(test1,nonzero2,zero3)`**, encoded solely from descriptor `[5,1,2,3,0]`
  2. `Const(dst3,1,next4)`
  3. `Const(dst3,0,next4)`
  4. `Const(dst2,11,next5)`
  5. `Write(address2,value3,next6)`
  6. `Halt`
- After paying for all seven GraphEncode operations, parent pays a native GraphLaunch, transferring its own credits (not minting child credits). Parent must retain PID1; **only new PID2 executes the generated Branch and writes cell11**. ZERO writes0, ONE writes1. This is NOT a continuous same-PID individual variation result.
- The causal negative arm `NO_ENCODE` retains the original source authoring structure and the same budget, but the parent's first GraphEncode instruction is replaced by an ordinary Halt before load. It must not create a graph node, launch PID2, or create a cell11 outcome. Clearly flag that this is a researcher-authored, non-source-identical inhibition, not the full B56 monitor-edit-only control.
- Both constructive cases must use at least 35 real CIL descriptor writes, seven real GraphEncode events, one GraphNew event, one GraphLaunch event, native graph provenance and finite credit/storage accounting, plus genuine PID2 Branch/test1 execution. No result may rely solely on source inspection. Fail closed if events or origins mismatch.
- Archive original native JSONL including full parent source, all paid instruction events and resulting process/cell state; separately maintain an independent JS source-derived graph+event audit. The independent observer must not count mere human-authored descriptor bytes as autonomous innovation. If it does not reexecute every CIL event, state its scope precisely.

## Scientific acceptance

Accept narrowly **the generic data→new Op grammar mechanical bridge** if native verification shows real PID1-paid GraphEncode produced a Branch(test1) absent from its initial source, that PID2 executed the branch and the two outcomes are correct, strict caps are obeyed, and negative no-encode cannot launch. This does not falsify B56-F1d1c: that closure was for the unchanged F1c founder **without GraphEncode**.

Do not accept same-PID continuous evolution, algorithm-origin novelty, feedback-informed selection, usefulness, re-adaptation, semantic truth authentication, full B56-F1d1, strict N56-A/B/C, B3/B4, or cyberlife. **The process-ID discontinuity is an essential negative finding for CL-1B** and must be highlighted, not silently renamed into self-maintenance.

**Next design decision:** investigate whether any existing paid primitive permits graph-object code to replace/continue within the *same* PID, or whether graph identity would require a separately declared organism continuity criterion. Never claim a fresh PID2 is the original same-PID CL-1B individual without an explicit, falsifiable lineage/identity test.
