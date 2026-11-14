# CL-1 Study72 — prospective live within-world sensor tracking under randomly composed CIL feedback

**Scientific status: CI-VERIFIED prospective within-world native information feedback. NOT self-maintenance, endogenous code construction, damage repair, heredity or digital life.**

## Frozen population, source revision and independent audit

- Preregistered **before code execution and before selecting seed outcomes** in `cl1/studies/72-preregistered-within-world-changing-sensor-feedback.md` (commit `4bde866034`). The original 256 raw integer seeds **4,800,000..4,800,255** were each subjected to one-time SplitMix64 whitening followed by the frozen Study71 CIL LCG.
- Exactly **12 source/control arms per seed**, topologies `broad` vs `short` random backward Branch target, input `dynamic` vs `static-low` vs `static-high`, and `Read` enabled vs ablated to `Const(r0,0)`. All other sampled native opcodes, source ports and initial code are paired and unchanged. The 24 random CIL continuation nodes PC3..26 are **not** a hand-authored working Read→Write sequence; there is no engineered cyclic PC27 tail.
- In `dynamic`, external `SetCell(key7,value8)` occurs at the **start of epoch12**, after initializing key7=3. Static-low remains3 and static-high begins8; no runtime actor credits, code repair or compute-pool interventions occur. All process credits (**300 initial**) remain a finite researcher-endowed initial supply; no native compute harvesting is tested.
- Independently audited exact source revision **`e4dd0737ed107fd6489ba842083dc5106d952c06`** with [GitHub Actions43579435147](https://github.com/iFrankStudio/cioc/actions/runs/43579435147) **SUCCESS**, archive `cl1-study72-changing-sensor-causal-feedback` artifact ID `14108025379`: unmodified ~1.2GB original SQLite and complete independent source/causality JSON. **256 original independent CIL genotypes, 3,072 matched native runs, 73,728 SHA256-verified exact Rust-replayed epochs**, exactly 256 unique broad genotype encodings/256 raw seed labels.

## Frozen primary: true PRE/POST dynamic tracking in the *same running world*

PRE = epochs8..11, POST = epochs12..15. A seed/topology counts only when a fresh internally attributed native `CellRead(key7)` precedes an internally attributed `CellWritten(key9)` from the **same native writer PC** in each window, source-tainted register0 is not clobbered by Const, dynamic PRE matches fixed-low PRE, dynamic POST matches fixed-high POST, and matched static-high−static-low output at that writer PC is exactly +5 in **both** windows. The Read-ablated control executes zero Read and its native CIL output sequence is sensor-independent. A merely changed host cell, later charged instruction or unchanged cached register never counts.

| Prospective dynamic PRE→POST source response | Broad randomized backward edges | Short adjacent-only edges |
|---|---:|---:|
| **Qualified original worlds/256** | **78/256 (30.47%)** | **0/256** |
| Wilson95 | **[0.251536,0.363614]** | [0,0.014784] |
| Paired discordance | broad-only **78** | short-only **0** |
| Both / neither | both0 | neither178 |

Registered directional criterion broad-only>short-only passed. The purely conditional two-sided paired sign test is **6.617444900424222e-24**; do not treat deterministic pseudorandom raw seed labels as proven IID sampling or as directly estimating life probability. The result demonstrates a **source/feedback-topology contrast**, not spontaneous selection, adaptation or maintained organism.

## Original native first-witness chronology

Original seed **4,800,000** broad topology, original actor1 and writer PC6:

- PRE epoch8/tick99: `dynamic` native `Read(key7,value3)` then native `CellWritten(key9,value3)`; fixed-low also writes3, fixed-high writes8.
- POST epoch12/tick149: after a uniquely `External` `SetCell(key7,value8)` at the beginning of that epoch, `dynamic` executes a **new successful native Read** and writes8 at the same PC6. Fixed-low writes3 and fixed-high writes8 at corresponding native instructions (their absolute ticks are148 because they lack the extra External intervention event).
- Fixed-high and fixed-low native outputs differ +5 in both windows. The dynamic native output changes3→8, directly responding to the researcher-introduced sensor change. No manually emitted output or host actor-credit intervention was counted as native.
- Another true native witness seed4,800,004 produced PRE4 vs high9 and POST dynamic9 at writer PC4; the response is not restricted to direct pass-through: random native ALU composition can alter the input before output.

## Integrity and earlier CI failure

The initial CI run [43579049799](https://github.com/iFrankStudio/cioc/actions/runs/43579049799) completed **all 3,072 original native runs and exact native replay**, but its independent auditor incorrectly compared *absolute global tick numbers* for the Read-ablated dynamic vs static traces. The unique legitimate external sensor intervention increments the global tick counter by1 after epoch12, even when the native Read is ablated; this does not change the native PC, source-independent value or charged instruction order. The subsequent fix committed at `e4dd0737ed` normalized precisely that **known one External-event offset** and changed **no** registered primary endpoint, CIL genome, seed set or native source-intervention semantics. The new full [CI43579435147](https://github.com/iFrankStudio/cioc/actions/runs/43579435147) reran **the full original cohort**, confirmed exact native byte/seed grammar, actor/source provenance, all 73,728 snapshot hashes, matched Read-ablation invariance and sensor-switch chronological order, and passed.

## Scientific interpretation and next gate

This is the first **non-hand-scripted stochastic CIL cohort** in this branch to demonstrate real **within-world source switching and causally appropriate native output** across two separate observation windows. It supports the post-Study62 review: wider generic native control-flow paths, a genuinely usable data interface and ALU/Read/Write opportunities can create repeated finite feedback function without embedding a complete operation-order blueprint. It does **not** demonstrate that the random executable genome was natively self-built, self-repaired after damage, acquired compute autonomously or maintained an organization across generations. The source change and original 300 compute credits are explicitly external; `GraphLaunch`, native mutators, self-maintained resource harvesting and evolutionary selection are not in this isolated Study72 ecology.

**Next requirement:** move from externally supplied fixed executable feedback topology to **native executable modification while function is running**, with a predeclared counterfactual repair/creation criterion and event-level evidence. A separate generic resource-access physics comparison is also required because Study66/67 showed random DrawCompute+ambient pool frequently enters fatal trap or zero-credit ignition deadlock. Do not equate any existing initial static/random function with self-maintenance, or silently drop nonqualified seeds.

**CL-1 remains OPEN.**
