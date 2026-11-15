# CL-1B B08 — independent CIL path audit (not native VM evidence)

**Evidence:** independent lightweight instruction-level interpreter, replaying the frozen B08 24-op CIL code. **Not a Rust test, not SQLite event audit and not acceptance of CL-1B.** Source: [protocol](../studies/B08-indistinguishable-histories.md), [Rust test](../../cl0/crates/world/tests/cl1b_b08_indistinguishable_histories.rs). Performed after preregistration and test commit; held-out input cases are asserted by Rust but the interpreter manually inspected one input (23).

## Common future from different histories

Three worlds used the same 24-node CIL program except for the original PC8.next in their **past**:

- Lost history H_A: PC8 originally next9, native bootstrap checkpoints [9,9], then external code damage PC8.next10 and checkpoint overwrites [10,10].
- Born different H_B: PC8 originally next10, native bootstrap checkpoints [10,10], then externally repeated no-op writes to next10/checkpoints [10,10].
- Reference retained H_A: PC8 originally next9, native bootstrap checkpoints [9,9], then external PC8.next10 and no-op checkpoint writes preserving [9,9].

At the convergence point after bootstrap8, three host actions, and one further native PC8 read: **all three** have the same process code including PC8.next10, PC10 to execute next, registers `[8, 23, 7, 9]`, and 241 credits. Lost H_A and born-different H_B also share *all current readable cell values* (key7=23, keys12/13=10, no output at key9), while the reference-retained control instead has keys12/13=9. Their pre-convergence observer-only native `CellWritten` traces distinguish H_A from H_B but those traces are inaccessible to CIL.

After **80 extra simulated native instructions**, predictions from source-matched simplified interpretation:

| Arm | Final live code PC8.next | Output writes PC9 | Internal code restoration | Ending credits |
| --- | ---: | ---: | --- | ---: |
| H_A erased | 10 | 0 | 0 | 161 |
| H_B originally different | 10 | 0 | 0 | 161 |
| H_A reference retained | 9 | 6 | one patch 8:10→9 | 161 |

H_A and H_B have identical simulated future instruction PCs, writes, code, registers and costs. The only difference is inaccessible prior provenance. H_A_REFERENCE_RETAINED restores its code because reference value9 remains available to its **researcher-authored guard**.

**Interpretation:** with the current VM and absent additional distinguishable input, no deterministic CIL policy operating on a converged CIL-observable state can choose a different action based on information erased from that state. This is a conditional **information limit**, not proof that every damaged agent must fail: new external feedback, trials, independent physical records, environmental constraints, or uncorrupted older state may supply missing evidence.

**Boundaries:** This is a counterexample to the assumption that program rollback is inherently guaranteed, not a positive result for autonomous evolution. All source and interventions were researcher designed; there was no self-originated trust policy, no autonomous variant proposal, and no real native Rust execution confirmation. A differential Native CI is required to compare process states and future events, and to audit code/memory credit accounting across all five input cases.
