# CL-1B B51b — native CIL composes two executable arithmetic candidates

**Engineering feasibility of paid actor-local candidate composition: VERIFIED for this restricted, investigator-authored construction.** B51c feedback selection and B51d unseen task-family transfer remain **NOT IMPLEMENTED**. This is not B3/B4 acceptance, spontaneous algorithm discovery or cyberlife.

## Exact-source verified evidence

- **Tested source SHA:** `4b02d7d5dc9ea837f4cc5f13901931800b3fe70d`.
- [GitHub Actions full native run 44006194657](https://github.com/iFrankStudio/cioc/actions/runs/44006194657): **scope=full, 50/50 named native Rust integration targets PASS, 37/37 independent observer programs PASS**, plus B31/B34 private library ABI checks. Both failure counters zero; the separate evidence-publish job succeeded.
- [Original full native summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/4b02d7d5dc9ea837f4cc5f13901931800b3fe70d-44006194657-1/summary.txt) · [original exact-SHA run conclusion](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/4b02d7d5dc9ea837f4cc5f13901931800b3fe70d-44006194657-1/result.txt).
- [Actual Rust trace JSONL](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/4b02d7d5dc9ea837f4cc5f13901931800b3fe70d-44006194657-1/b51b_native_steps.jsonl) · [source-parsed independent JS per-step replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/4b02d7d5dc9ea837f4cc5f13901931800b3fe70d-44006194657-1/b51b_native_candidate_replay.json) · [native integration log](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/4b02d7d5dc9ea837f4cc5f13901931800b3fe70d-44006194657-1/cl1b_b51b_native_candidate_composition.log).
- [Rust code](../../cl0/crates/world/tests/cl1b_b51b_native_candidate_composition.rs) · [independent interpreter](../analysis/b51b_native_candidate_replay.mjs) · [pre-implementation B51 protocol](../studies/B51-native-candidate-composition-cross-task-preregistration.md).

## What the native actor really does

A **single PID1** starts at source node3 of a **26-node researcher-authored CIL controller**. Source nodes0 and1 are **disconnected arithmetic templates** (`Add` and `Sub`); source node2 is an ordinary reusable `Write`. There is **no connected live Read → arithmetic candidate → Write chain** in the initial graph. There is also no `LIFE`, `OPTIMIZE` or dedicated assembly opcode.

Through its **own paid CIL** actions, the actor:

1. Creates new node26 by executing `CloneNode` on disconnected `Add`, then pays `PatchNext` for candidate26→output bridge and the actual input-read→candidate26 edge. Reads cell7, calculates `x+2` and performs a native `Write` to cell11.
2. Creates new node27 by executing `CloneNode` on disconnected `Sub`, pays patches for candidate27→output bridge, output→halt and second input-read→candidate27, then reads the same input and performs a native `Write` for `2-x`.
3. Halts with both newly assembled executable candidate nodes stored; the two original disconnected templates remain unchanged. The construction controller, two operator types, sequence, offset2 and task IO addresses are **all researcher-authored**; the actor did not invent the synthesis policy.

## Exact native numbers

Each independent world has **64 initial local CIL instruction credits**, 16 data cells maximum, 80 code nodes maximum, 192 strict shared storage units. It spends **28** native instructions (including `Halt`), grows from 26 to28 executable nodes, pays **five actual `ExecutableEdgePatched` changes**, and peaks at **31 occupied storage units**. No resource replenishment, laboratory code patching, credit injection or secondary process.

| External input x | Native output #1 (Add candidate) | Native output #2 (Sub candidate) | Paid native instructions | New CIL nodes | Native edge changes |
|---|---:|---:|---:|---:|---:|
| 5 | 7 | -3 | 28 | 2 | 5 |
| -3 | -1 | 5 | 28 | 2 | 5 |
| 0 | 2 | 2 | 28 | 2 | 5 |
| **Total** | | | **84** | **6** | **15** |

A separate JavaScript CIL interpreter parses the *real Rust founder source* and reproduces **84/84 actual paid Rust CIL events with all source edits, dynamic clones, registers, PC, storage, credits, observed cells and final rows**. Its reported **first divergence is null**; 3 native worlds fully matched. Note that at x=0 the two different programs return the *same value*; success here is demonstrated by their **structurally different native executable nodes and distinct later behavior on nonzero inputs**, not mere diversity of a single output.

**Negative control:** with opt-in native `PatchNext` suppression, the same authored founder spends the full **64** credits but generates **zero** output-cell11 `Write` events and **zero** actual edge patches. This confirms that the patch-based control-flow construction matters, though it does not create a matched equal-trajectory, equal-output control for B51c's selection utility.

## B51a and preservation of real failures

An earlier [B51a native interface gate](../../cl0/crates/world/tests/cl1b_b50_feedback_vs_forced_clone.rs) verifies that two hidden task modes produce indistinguishable actor state **before** laboratory success/failure feedback, then an ordinary charged `Read` observes exactly one external-origin feedback bit. It is a **researcher-authored feedback-interface calibration**, not genuine candidate selection or a proof against every possible side channel. It executed as part of the B50 native target in the full 50-target source revision above; there is not a separate B51a observer.

Historical failures must remain visible rather than disappearing into the successful checkpoint:

- [B51a scoped run43999161435](https://github.com/iFrankStudio/cioc/actions/runs/43999161435): native B51a/B50 target passed, but B49 observer rejected the missing native trace in a changed-only run.
- [B51b first full run44005958456](https://github.com/iFrankStudio/cioc/actions/runs/44005958456): GitHub workflow Bash syntax error due escaped literal `\\n` in new comments, native tests did **not** execute.
- [B51b second full run44006098058](https://github.com/iFrankStudio/cioc/actions/runs/44006098058): escaped literal `\\n` remained in the fixture-producer conditional, causing another pre-native Bash syntax failure.
- [Final corrected full run44006194657](https://github.com/iFrankStudio/cioc/actions/runs/44006194657): all 50 native and 37 observer programs passed; B49/B50/B51b dynamic evidence produced in the same complete job. The workflow now selects all three native fixture producers in **scoped** jobs so their always-run observers remain fail closed.

## Scientific boundary and next gate

**H51 engineering candidate-composition prerequisite: demonstrated narrowly** by two graph-distinct source-level arithmetic constructions executed by one paid actor. This is not yet H51's full training/selection/holdout test. **B51b selection utility H51b is NOT MET/NOT TESTED; B51c cross-task transfer H51c is NOT MET/NOT TESTED.** Even a future successful selection study would require proof against unconditional construction and a compact per-task static baseline, with no hidden labels leaked. The offset and synthesis ordering here remain script-authored.

**Next:** Build the preregistered B51c trial/feedback/retention mechanism on top of this two-candidate generator, with at least two different nonzero probes per candidate, hidden affine-family modes and equal realized compute/provenance controls. Reject any apparent advantage that comes from a preauthored winner, a mutable task input as its own reference, stale feedback, or a suppressed-arm early halt. Then B51d prospectively tests previously withheld modes without human source redesign.

**CL-1B B3 cumulative endogenous adaptation: NOT MET. B4 endogenous organization: NOT MET. Cyberlife: NOT ESTABLISHED.**
