# B56-F0 native executable graph manifest v1 — frozen BEFORE native source

Frozen 2026-11-17, prior to F0 native code. [Parent F0 preregistration](B56-F0-source-generation-feasibility-manifest-v1.md). **No results**; this is explicitly **N56-A paid assembly feasibility only**, not independent origin of a metapolicy.

## Founder inventory and fixed operations

Start with **exact 72-node B51c paid affine-search founder**, changing only original node50's continuation `next:30→72` so that *after* native candidate retention the PID1 reaches the assembly controller. Initial PID1 PC3. Node2 `Write` starts with `next:35` for the original trial program; paid retained node50 patches it to30. Initial B51c templates 0 Add,1 Sub and feedback-training logic 35–70 are **researcher authored**, not novel.

New initial assembly nodes (indexed 72–101 inclusive, 28 ordinary CIL instructions and 2 inert Halt placeholders):
```
72 Const r2=27 →73; 73 Read r3,[r2] →74; 74 Branch r3 nonzero75 else30
75 Const r2=35 →76; 76 CloneNode r3,[r2] →77; 77 Const r2=40 →78; 78 Write [r2]=r3 →79
79 Const r2=36 →80; 80 CloneNode r3,[r2] →81; 81 Const r2=41 →82; 82 Write [r2]=r3 →83
83 Const r2=40 →84; 84 Read r1,[r2] →85; 85 Const r2=41 →86; 86 Read r3,[r2] →87
87 PatchNext node=r1 target=r3 →88
88 Const r2=41 →89; 89 Read r1,[r2] →90; 90 Const r3=98 →91
91 PatchNext node=r1 target=r3 →92
92 Const r2=40 →93; 93 Read r3,[r2] →94; 94 Const r1=2 →95
95 PatchNext node=r1 target=r3 →30
98 Branch r3 nonzero30 else99
99 Const r2=25 →100; 100 Const r3=1 →101; 101 Write [r2]=r3 →30
```
**Index 72–101 contains 28 actual active nodes and 2 inert Halt gaps at 96–97.** Exact instruction source may use inert Halt padding. The **controller includes generic templates + a generic response branch**, but the founder has **NO complete reachable task-output→feedback-Read→branch monitoring chain**: node2, on retained source, still goes directly to30. Cloning generic `Const20` and `Read20` into new program indices and modifying both successors **via native-paid instructions** is what creates that chain. If the author-provided node98 branch and node99–101 marker themselves count as a preformed useful complete policy under an independent strict topology audit, classify F0 **negative** even if the paid clone event occurs; do not assert independent invention.

B51c original candidate cloning uses two native `CloneNode` steps allocated immediately after initial program. Expected initial graph length 102, original paid arithmetic clones **102,103**. Assembly uses subsequent two native clones **104,105** (new Const20 and Read20), and paid patches Const104.next→105, Read105.next→98, output node2.next→104. This expected index sequence is a **prospective prediction**, not an implementation excuse to let the host allocate/edit it. Independent replay must verify it.

### Frozen controls

| Arm | Founder | External regular cell27 | Feedback | Expected causal interpretation |
|---|---|---:|---|---|
| A_ACTOR_ASSEMBLE | original 102-node F0 founder | 1 | truthful bit20 | candidate N56-A positive if native source edits and actual feedback Reads |
| B_BLIND_ASSEMBLE | same original founder | 1 | constant success1 | actor assembly can happen without informative feedback; no discovery claim |
| C_EDITS_DISABLED | same original founder | 1 | truthful | native `with_native_code_edits_disabled()`; same finite credits; code mutation and possibly original task adaptation impaired, report confound |
| D_NO_ASSEMBLY | same original founder | 0 | truthful | PID1's conditional source route skips assembly, retains original task service |
| E_STATIC_EVERY | explicitly **different authored static baseline founder** | n/a | truthful | B51c founder + prewired complete EVERY feedback monitor and source-return edges, same caps/budget |
| F_STATIC_FOURTH | explicitly **different authored static baseline founder** | n/a | truthful | B51c founder + prewired complete FOURTH counter/control; distinct authorial program and graph size |

E/F must never be counted as source-identical controls for creation provenance; they demonstrate fixed-policy utility only. For the static arms, change retained candidate target constant at B51c node49 from 30 to static monitor node120 or110 respectively, with node50 continuing directly30, and append prebuilt bounded monitoring code, source-authored before process creation. They **cannot** be smuggled into A/B/C/D initial founder.

## World schedules and exact limits

Six arms × paired seeds **[101,211,307,401]** = **24 actual one-PID worlds**. Seed parity selects A `x−2→−x+3→x−2` or B `−x−3→x+2→−x−3`. Task initial trained from frozen probes **[5,-3]** by the unchanged B51c paid operator search. Three 24-service-output stages; genuine service-input `[11,-7,0]` phase rotated by `seed_index % 3`. Count all missing outputs and end-of-compute stops; no survivor exclusion. After the first72 service writes, maintain the returned-to-old third task for continued full-budget outputs.

**Every world starts with 4,000 native CIL credits, 16 data cells,180 graph nodes,300 storage units**; no external funding, host code mutation or task label. A/B/D/E/F start source at PC3 and can seek original task using B51c's paid grammar; C edits disabled still pays attempts and failures. Laboratory externally writes only inputs7, feedback20, pre-start arm selector27 where applicable; task mode only determines hidden evaluator bit20 and offline score, and no answer/solver/source patch is injected. Strict record of every actor Paid `Read`, `PatchNext`, `PatchConst`, `CloneNode`, output, source diff and external intervention required.

## Decision gates

F0 mechanical passage requires initial 102-node founder verified to have **no complete output-linked monitoring topology**, A original paid code clones+patches to make a previously impossible live output→Read20→Branch path and observes at least one correct task service output then reads the truthful failure bit after hidden change; C/D cannot create this path; full event replay exact. A blind B may also assemble; **this shows assembly was researcher-programmed, not intrinsically evidential discovery**. Success on N56-A does not imply useful superiority over E/F, N56-B, N56-C, B3/B4 or cyberlife.

If the initial 102/126-node cost, VM semantics, static baseline or 16-cell capacity cannot be met, keep failed native evidence; any change to this manifest **after first native execution** must create a new v2 file and make the scope/cost difference explicit.
