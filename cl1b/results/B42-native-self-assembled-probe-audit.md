# CL-1B B42 — native verification: a new service node assembled from a generic Write, but single-probe overfitting persists

**FULL NATIVE VERIFICATION PASS** — [GitHub Actions run 43837857058](https://github.com/iFrankStudio/cioc/actions/runs/43837857058), exact tested source SHA `114795cfd581884b0157b8a6cab6a022098ac054`. [Immutable source/run-pinned summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/114795cfd581884b0157b8a6cab6a022098ac054-43837857058-1/summary.txt) · [actual B42 Rust event trace](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/114795cfd581884b0157b8a6cab6a022098ac054-43837857058-1/cl1b_b42_self_assembled_probe.log) · [executed independent source-parsed interpreter output](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/114795cfd581884b0157b8a6cab6a022098ac054-43837857058-1/b42_self_assembled_probe_scan.json).

**Complete CI scope:** **42/42 Rust CL-1B integration targets PASS**, **29/29 independent source-parsed model programs PASS**, B31/B34 private opcode compatibility PASS, `failed_target_count=0`, `failed_observer_count=0`, archived `native_job_result=success`. One nonfatal dead-code warning for debug-only B42 observation fields appeared; this run is **not** a separate Rust-format/clippy certification. Prior B00–B41 Rust targets passed again in the same source-pinned run.

## Genuine native construction, not an external candidate injection

The only externally loaded source is PID1's 38-op CIL founder. The environment sets the task input cell7, **not two candidate program addresses as in B41**. The actor reads the actual input, pays to save it at cell30, calculates a diagnostic value using an existing arithmetic instruction, then itself writes that value to cell7. It runs its original faulty constant-zero output node10 and reads the actual task output at cell11. On a discrepancy, **native `CloneNode` clones its already-existing generic diagnostic Write node7 to a new code node38 (storage charged)**; paid `PatchNext` rewires node38's successor and changes the executable service router at node9. It executes the new service candidate, reads back actual output, then changes the selected Write's successor to node34 for continued task service. The original input cell7 is restored from the actor's own checkpoint before observer-side scoring of genuine external task work.

For both plus and minus probe policies, **6 of 8 native worlds allocate node38**, and source edges identify the paid clone/route/retention sequence. All eight worlds in each continuing probe arm restore cell7. This is a new source-node construction mechanism within a causally continuous single process, **not merely selecting one of two externally provided executable addresses**.

The source template, instructions for cloning/relinking, challenge rule and task comparator were all **researcher-authored**. This is *endogenous execution of authored construction instructions*, not endogenous **invention** of algorithms.

## Fully matched 48-world native evidence

[Protocol frozen before native trial](../studies/B42-self-assembled-probe-boundary.md) · [source Rust test](../../cl0/crates/world/tests/cl1b_b42_self_assembled_probe.rs) · [independent CIL model](../analysis/b42_self_assembled_probe_scan.mjs) · [model-only prediction logged before CI](B42-model-only-self-assembled-probe.md).

Six arms × training input `[-1, 0, 1, 17]` × unseen input `[-9, 23]` = **48 native worlds**, 128 initial local compute per world, two periods of at most64 native attempts, 0 shared-pool credits. For each output, trace the actual `CellWritten(key11,value)` by PID1. During the diagnostic interval after the actor has written cell7 but *before* the actor restores it, those outputs are counted as **diagnostic** and **excluded** from genuine external task-service denominators.

| Arm / 8 worlds | Training true/wrong external-task writes | Holdout true/wrong | Actual total compute used | New executable nodes | Halts |
|---|---:|---:|---:|---:|---:|
| **ProbePlus (x+1)** | **30 / 16** | **72 / 26** | 1,024 | 6 | 0 |
| **ProbeMinus (x-1)** | **30 / 16** | **72 / 26** | 1,024 | 6 | 0 |
| **NoProbe (x+0)** | **46 / 0** | **72 / 26** | 1,024 | 6 | 0 |
| IgnoreFeedback | 16 / 48 | 0 / 104 | 1,024 | 0 | 0 |
| PatchDisabled | 0 / 4 | 0 / 0 | **312** | 8 | **8** |
| **StaticCorrect (4 op)** | **128 / 0** | **128 / 0** | 1,024 | 0 | 0 |

The actual **48 native source-state/event traces** and independently executed 48-world source-parsed CIL model agree on these totals. Under the PatchDisabled arm, `CloneNode` still allocates new code (8 clones), but paid PatchNext attempts are suppressed; all eight processes halt, with unspent energy. **Do not interpret 0/0 holdout writes as a validated successful service or equal actual compute consumption.**

### Negative finding: one diagnostic merely moves an equivalence class

With `NoProbe`, externally supplied training input0 admits an indistinguishable constant-zero faulty program; under any nonzero holdout it is wrong. With the actor-generated `x+1` probe, training0 is now diagnostic1 and produces a genuine discrepancy, so paid clone/relink yields correct future writes. **But training -1 produces diagnostic0** and causes the same false acceptance. The x-1 rule merely moves that failure to training1. The three source policies tie on aggregate unseen performance at **72/26**, and the four-opcode known-correct source produces **128/0**.

This is not a sample artifact that should be averaged away: it is a causal indistinguishability counterexample. A single fixed probe cannot certify general correctness just because it moved outside one earlier false-positive input. The source code remains mutable, so the program that evaluates its own trial can also be damaged. The task cell itself is writable, which means a malicious or misguided actor could manipulate its own measured conditions; provenance alone does not make the oracle trusted. External input may change after a Read before a service Write, so even a correct retained program can issue a stale output.

## Next scientific barrier

**Native mechanism construction accepted (limited engineering claim). B3 cumulative endogenous adaptation: NOT MET. B4 endogenous organization: NOT MET. Cyberlife: NOT ESTABLISHED.** Do not launch a series of increasingly elaborate authored fixed probe recipes and call it autonomous discovery.

The next informative experiment should remove at least one preinstalled search/validation assumption: *who generates meaningful functional contexts, who determines the error criterion, and why does code exploration improve future heldout ecology relative to the simplest source after all paid overhead?* Consider causal intervention on the evaluator and the data channel (including internally forged task conditions), with full compute/storage cost and irreversible failed worlds retained.
