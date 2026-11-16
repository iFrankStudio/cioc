# CL-1B B40 — paid post-write source reread fixes registered temporal false verdicts, at substantial compute cost

**Fully native-verified result:** [GitHub Actions run 43828672838](https://github.com/iFrankStudio/cioc/actions/runs/43828672838), tested SHA `91dd65ef49edd67f5f5acf5772c32b782485c18b`. [Source-pinned full CI evidence summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/91dd65ef49edd67f5f5acf5772c32b782485c18b-43828672838-1/summary.txt): **40/40 Rust native CL-1B integration targets PASS, 27/27 independent Rust-source-parsed CIL models PASS**, zero native/observer failures, B31/B34 private opcode ABI checks PASS and evidence publication PASS. [Prospective frozen B40 protocol](../studies/B40-postwrite-functional-recheck.md), [native source-reusing Rust 12-world assay](../../cl0/crates/world/tests/cl1b_b40_postwrite_recheck.rs), [independent CIL source-checked model](../analysis/b40_postwrite_recheck_scan.mjs).

## What actually improved

B38 built a single continuing CIL process that reads an input task value at cell7, writes an output at cell11, reads its own output and conditionally patches its own executable Write edge. B39 exposed a timing failure: external task input can change **after the first input Read but before the output Read and comparison**. The comparison then checks the real output against a *stale input register*, giving either false success or false failure.

B40 leaves the original B38 12-opcode READBACK and compact 4-opcode STATIC_CORRECT programs unchanged. A third, investigator-authored **15-opcode RECHECK** extends B38's own executable source using only existing general CIL primitives, redirecting PC5 output `Read.next` to three extra paid instructions:
- PC12: `Const r2=7` (the task input address);
- PC13: `Read r0, [r2]` (obtain **latest** actual input);
- PC14: `Const r2=11` (restore output address);
then PC6 subtracts the output from the refreshed input and PC7 branches to the same charged PC10 `PatchNext` repair logic when genuinely inconsistent.

No new VM instruction, host-side fix, free recheck, bonus compute, second actor, self-certified success marker or privileged fitness score is introduced.

## Twelve pre-registered world outcomes

One externally loaded PID1, **96 local compute credits**, zero world pool; exact96 native attempts in each world. Two changing task inputs (`0→17`, `17→0`), two relative arrival times (**BEFORE_READ**, **AFTER_READ** first true `CellRead(key7)`), and three CIL source arms (**READBACK**, **RECHECK**, **STATIC_CORRECT**) form the complete **12-world** set. All branches, source edits and writes use actual native instruction events; the independent model's **exact** correct/wrong write forecasts were frozen into native assertions before the verified CI run.

| Task input / arrival | Original READBACK correct / wrong writes | RECHECK correct / wrong writes | Compact STATIC_CORRECT correct / wrong writes |
| --- | --- | --- | --- |
| `0→17` BEFORE_READ | **12 / 1** | **8 / 1** | **24 / 0** |
| `0→17` AFTER_READ | **11 / 2** | **8 / 1** | **23 / 1** |
| `17→0` BEFORE_READ | **14 / 0** | **10 / 0** | **24 / 0** |
| `17→0` AFTER_READ | **13 / 0** | **10 / 0** | **23 / 1** |

**First genuine feedback verdict is improved by the added paid input Read:**
- `0→17 AFTER_READ`: original B38 READBACK has old r0=0 and actual output0, **incorrectly accepts a wrong current output**, looping back to PC1. RECHECK actually rereads new input17 after its output0, computes real mismatch, and enters native repair PC8. Its executable PC3.next becomes11 through a true paid internal `PatchNext`, producing subsequent correct outputs.
- `17→0 AFTER_READ`: original B38 compares old17 with currently correct output0 and **unnecessarily attempts repair**. RECHECK rereads new current input0 and correctly accepts output0, preserving original PC3.next=4 with **no source change**.
- Both BEFORE_READ histories produce correct first verdicts from both feedback programs. RECHECK has no accuracy advantage in those cases but **always spends three extra charged CIL instructions** per verification cycle, decreasing useful write throughput.

**The compact STATIC_CORRECT program remains stronger** under every environment: 23–24 correct writes for the same96 compute attempts, versus RECHECK's8–10. The simpler source also sometimes writes one stale value under the AFTER_READ timing, as any non-atomic read/write sequence can, but resumes correct copying in its next cycle without source repair.

## Causal and scientific boundaries

This is a verified engineering improvement in **task-evidence temporal freshness**—not a generic proof of serializability, atomicity or stable input/output transactions. An environment can still change cell7 immediately **after** the PC13 re-read, and no CIL instruction can infer an unobserved future input change. The modified comparator and the extra sensor read were entirely authored by the investigator; the functional objective, cell locations, candidate fix path and edge patch were all specified in advance. There is still no autonomous selection among unknown candidate source changes, no self-generated reliable goal, no internally evolved evaluator and no superior adaptive policy compared with compact immutable code.

The B38–B40 trilogy is a useful causal staircase:
1. **B38:** true *task-function* readback can cause genuine same-process executable repair;
2. **B39:** temporal input changes can make truthful readback yield false positive and false negative repair decisions;
3. **B40:** a paid second input read locally resolves those registered false decisions but imposes real opportunity costs and remains subject to future concurrent changes.

**CL-1B B3/B4 autonomous adaptation and open-ended cyberlife remain NOT MET.** The next primary path is exploring how to make **functional evidence trustworthy enough to compare genuinely different unanticipated code alternatives**, with carefully measured energy costs and immutable baselines; multiplying hand-authored rereads alone is not an evolutionary breakthrough.
