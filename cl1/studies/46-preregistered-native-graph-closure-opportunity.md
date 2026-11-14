# CL-1 Study46 — preregistered random native graph-closure opportunity ecology

**FROZEN before runner, auditor, seeds or results.** This is a new stochastic CIL grammar study, motivated by Study45's *post-hoc* observation that 49/60, 41/58 and 56/74 native-launched child code graphs retained out-of-bounds executable edges in Studies41/42/44. The goal is to measure opportunity for native construction of valid code topology, **not** to insert an engineered reproduction/repair program.

## World and independent seeds

- All **128 independent seeds 3,400,000..3,400,127**, no exclusions, reuse the same three-actor nine-op initial graph scaffold and exact 27 common prefork charged native attempts as Study44; initial 56 credits/process, 47 per process postfork, researcher-endowed world compute pool48. **No kernel or CIL instruction semantics changes**; per-process code max72 nodes and strict total storage cap320 for the larger random continuation; cell limit32.
- Native code continuation indices **9..44 inclusive** (36 random nodes), `Halt` at node45. Fixed 64-bit wrapping LCG `state=state*6364136223846793005+1442695040888963407`, consumed **once for choice `random(state)%24` per node**, plus **one extra random draw only for choice8** to choose a general code source index uniformly `random(state)%45` (0..44). This is NOT a "successful child" optimizer: every original pre-Halt node is equally eligible, including unsafe and useless ones.
- Exactly the Study44 20-choice opcode table for choices0..19, with the Study44 choice8 `Const(dst=2,value=random(state)%45,next)`. **Four added generic graph primitive weights**:
  - choice20: `GraphCopyNode {graph:3,source:2,dst:2,next}`
  - choice21: `GraphSetNext {graph:3,node:2,target:0,next}`
  - choice22: same `GraphSetNext {graph:3,node:2,target:0,next}`
  - choice23: `GraphLaunch {graph:3,credits:1,dst:2,next}`.
- Original Study44 choices0..19 unchanged otherwise (including its own `GraphCopyNode`, `GraphSetNext`, `GraphLaunch`, `PatchNext`, `PatchConst`, `CloneNode`, `GraphEncode`, `GraphReadTag`, `Write` etc.). The program generator does **not** order copying, edge repair or launching, preselect a valid edge, insert extra Write instructions, reward successful children, inspect previous positive seeds, or place fixed life/repair functionality. All coding opportunities remain stochastic, resource charged, and may trap.

## Registered 4 matched environment conditions (512 runs, 12,288 epochs)

Exactly Study44's `ambient-general`, `ambient-only`, `closed-general`, `zero-credit-ambient-general` arms and action scheduling: 24 externally observed epochs, up to12 charged native attempts/epoch; external ambient +8 world-pool credits at epochs1..23 when enabled; neutral external starter (if no process runnable, choose lowest-ID *unstopped zero-credit* process and transfer one existing pool credit to it) at most once per epoch, without inspecting its CIL code or success. All 128×4 arms fork the exact same initial serialized prefork world per seed. No host-assisted child construction, repair, preselected graph closure or targeted code lesion.

## Frozen primary and secondary diagnostics

**Primary:** number of `ambient-general` independent worlds with at least one *successful* `CellWritten` **by a native-launched child** with strict earlier `GraphLaunched` parent/child/source provenance, independent actor origin, valid currently identifiable child executable `Write` at event PC, and exact actual cell key/value. Report worlds/128, Wilson 95% interval, first source-pinned witness or an explicit zero. This measures opportunity for a *function-bearing child*, but does **not** demonstrate hereditary causal function or endogenous maintenance.

**Secondary:**
- Worlds with any native child and successful non-`GraphCreated` child instruction; child opcode distribution (including traps), child native Write with exact earlier same key/value parent Write, across ≥2 independent epochs and ≥2 W3 epochs with no same-epoch host actor starter.
- Number of closed native child executable graphs, first invalid edge, strict absolute CIL PC and GraphSetNext provenance. Report both structural graph closure and actually executed successful opcodes, without equating them.
- Original-actor native code modification by strict Study41 epoch-only event attribution, postmutation original-actor output, child launch/continue, W1/W2/W3 activity, storage caps and graph copy/launch resource costs.
- Falsifiable controls: all 128 independent seed worlds, same four-armed initial snapshot, exact source-pinned Rust laboratory full epoch replay, fresh SQLite file, all 12,288 SHA256 event/snapshot checks; independent Python LCG generator must account for **choice8's extra RNG draw** and 24-way choices; strictly reconstruct **External** ambient/neutral starter and one-credit native attempt conservation. A successful `GraphCopyNode` touching a graph object is never counted as an original process code edit or repaired organism.
- A single child-write synthetic positive and forged child negative calibration; no hundreds of trivial tests.

## Interpretation

Any positive child output may follow a mechanically repaired code graph but can still be accidental, sterile, externally energized or host-started. CL-1 life acceptance requires independently replicated *internally necessary* multi-component function, survival across genuine damage, native functional restoration and unassisted sustained maintenance, not GraphLaunch/CellWritten alone. A null requires preserving its correct denominator and protocol, not tuning the grammar after inspection.

**CL-1 OPEN.**
