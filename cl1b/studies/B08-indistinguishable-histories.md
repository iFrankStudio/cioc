# CL-1B / B08 — indistinguishable histories after loss of recovery information

**Prospectively registered before B08 Rust implementation or B08 outcome inspection: 2026-11-15.** This study tests an information-theoretic **limitation**, not the emergence of self-maintaining life. It uses the previously frozen human-authored 24-node B07 CIL code, with a deliberate one-edge initial variant. No new host `REPAIR`, `FITNESS` or rollback operation is introduced.

## Scientific question

When two different source histories collapse into exactly the same **CIL-accessible present state**, can any future deterministic native CIL execution choose a history-specific recovery action? The expected answer is **no**, absent new distinguishing information. Historical event logs are visible to the laboratory but **are not readable by CIL** in the current CL-0 world.

This matters because user-proposed digital rollback is contingent on intact, actionable historical information; two agreeing checkpoints might be jointly wrong, and once their distinctive evidence is erased, comparing more copies of the same value cannot reconstruct missing history.

## Frozen construction (five external input labels)

Inputs: `[-13,0,7,23,101]`. Make three worlds per input using **the same 24-node B07 source structure**, no second process, 250 initial credits, strict capacity64, no compute pool and deterministic VM:

- **H_A (lost history):** original founder node8.next=9. Actor executes bootstrap PC0–PC7 (exactly 8 attempts); CIL therefore creates cells12=9 and13=9. Experimenter then patches executable PC8.next to10 and corrupts both checkpoint cells to10. One more native attempt at PC8 overwrites the transient working register r1 with input, advancing to PC10 (service write PC9 skipped).
- **H_B (different original history):** founder is **identical except for original node8.next=10**. Actor bootstrap writes native checkpoints12=10 and13=10. Experimenter applies the same `PatchEdge` next10 no-op and the same two `SetCell` next10 no-ops. One PC8 native attempt advances to PC10. The two world histories had distinct originals and different internal checkpoint write events; current code and usable present state converge.
- **H_A_REFERENCE_RETAINED:** same bootstrap and code damage as H_A but **does not overwrite checkpoint cells** (researcher still submits two no-op `SetCell` actions with value9 so timing and number of external actions match). After PC8, this actor still holds the original value9 in its two CIL-created checkpoint cells. It is a positive control for preserved recovery information.

**Important methodological distinction:** `original edge` is an experimenter-defined *history label*. H_B was deliberately born with next10, which skips the output; the native world has no intrinsic concept of the investigator's desired service or of which history is normatively “correct.” Comparing H_A and H_B proves a lack of accessible historical information, **not** that their hidden life identities are distinct.

## Predeclared measurements

- Two distinct prefork source states: A original edge9 versus B original edge10, with different CIL-written checkpoints. Native provenance on both checkpoint write events (PC3 and PC5) must be verified; no host-provided checkpoint.
- After the two external memory writes, one code-edge intervention and one native PC8 attempt, both H_A and H_B must have **byte/structurally identical current Process** (same code, pc, registers, credits, stopped); identical shared values at keys7,9,12,13, same total compute inventory, same number of charged attempts. Their observer-only earlier `events` differ because checkpoint writes differed — this difference **must not** be discarded or mislabeled as CIL-visible.
- Run all three from that point for **80 additional CIL attempts**, no new host actions. H_A vs H_B must produce exactly equal *new native events* (same PC, event kinds, credits, tick, process), process states and accessible shared cells, including identical service failures. Determinism and the inability of CIL to read prior provenance imply this equality.
- The retained-reference H_A control should produce an actual **internally sourced** edge change `8:10→9`, then recover at least one service write at PC9. H_A/H_B with both copies overwritten should not recover the original9 edge, and should produce no PC9 service writes in the comparable post-convergence window.
- All 15 worlds are retained; no births, traps, resource minting, after-fork external restoration or observer-side selection. All credit costs counted, failures reported. **No claim of spontaneous adaptation.**

## Falsifiers and interpretation

If H_A and H_B have different current CIL-observable state, the construction fails. If they produce divergent future native transitions with equal present CIL-visible state and identical deterministic physics, either the observer omitted a relevant state variable or there is a substantive VM determinism problem. If retained-reference A does not restore its edge and recover service, the positive control fails.

A successful B08 would illustrate a **state-information boundary**: no deterministic policy, however sophisticated, can infer a historical difference that has been erased from everything it can observe. Restoring functionality under such information loss requires some additional distinguishing causal evidence, protected information, external feedback, new experiment, or recoverable environmental regularity — *not* a magic number of checkpoint copies.

This conclusion is conditional on the specified observation channel, not a universal metaphysical limit on digital individuals; an organism with a richer source of environmental feedback could infer the better current behavior without recovering the exact erased historical edge.
