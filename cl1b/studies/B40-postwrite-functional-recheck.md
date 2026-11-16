# CL-1B B40 — paid post-output input recheck reduces B39 temporal false feedback

**Prospective registration 2026-11-16, before native B40 test or independent-model outcome inspection.** Single continuing individual only. B39 showed real output feedback can yield a false decision when shared input changes **after** the actor first reads it but **before** it writes/verifies output. B40 tests a paid, native-CIL-only defense: read the **latest** shared input **after** the output, before computing the discrepancy. This is not atomicity, a new VM opcode, a task-quality oracle or autonomous policy invention.

## Source and exact mutation

Use exact imported B38 12-node READBACK CIL founder (one externally loaded PID1, original deliberately faulty Write0 at PC4, real output `Read` at PC5 and `PatchNext` repair PC3.next4→11) and compact STATIC_CORRECT founder. A new **15-node RECHECK** authored program is constructed from that same B38 founder by redirecting **PC5 output Read.next from6 to12** and appending:
- PC12 `Const(r2=7,next13)` — task source address,
- PC13 `Read(r0,address=r2,next14)` — **actual latest input value** (overwriting the old expected r0),
- PC14 `Const(r2=11,next6)` — restore output address,
then the same B38 PC6 `Sub(expected−output)`, PC7 `Branch`, PC10 native source `PatchNext`, PC11 repaired Write. Every extra instruction costs exactly one local compute credit.

**Important:** RECHECK still has a time-of-check/time-of-use race *after* its extra input read. This study only addresses the specifically frozen B39 injection time, with one exogenous change between the **first** task input read and the **first** output write. Never claim universal consistency.

## Frozen worlds and controls

Exactly **3 sources × 2 initial→current changes (0→17,17→0) × 2 arrival timings =12 deterministic native worlds**. All load one PID1 with96 local compute credits, no neutral resource; host sets shared cell7 to initial input then sets to new input either:
- **BEFORE_READ**: before first actor instruction;
- **AFTER_READ**: immediately after first genuine native `CellRead(key7,value=old)`.

No host code edits, rewards, targeted compute or second actor. After the update the environment does not change again. For each READBACK and RECHECK source, stop at **actual first PC7 Branch after one output Write**; for compact STATIC_CORRECT stop after its first actual Write. Compare source cell7 (current) with real output cell11, branch outcome PC1 or8, and real native read provenance.

Then complete **exactly96 charged native steps**. Track all actual `CellWritten(key11,value)` events after injection, classify correct/wrong against the current task input, native `ExecutableEdgePatched` events, final executable PC3.next, real input reads and compute conservation. Do not infer success from actor's own marker or readback branch alone.

## Registered first-check expectations

- For **0→17 AFTER_READ**, original READBACK returns false success (PC1) because old r0=0 matches output0. RECHECK reads the now-current source17 again after writing output0, calculates a true mismatch and chooses PC8 repair.
- For **17→0 AFTER_READ**, original READBACK falsely decides repair PC8 because old r0=17 differs from output0 even though current input is0. RECHECK rereads current0 and returns PC1 without making a gratuitous code edit.
- For both **BEFORE_READ** controls, READBACK and RECHECK verdicts agree with the current input, but RECHECK pays **three additional instructions per verification loop**, reducing useful work per compute unit.
- STATIC_CORRECT has no feedback/patch path; when its first input was sampled before the task change it may write one stale value, then recover on the next cycle, and is expected to remain more efficient than the larger self-evaluating programs. It must not be omitted.

## Acceptance and scientific boundary

If the extra native read corrects B39's false first judgements while preserving real source-edit causality, this is a **locally stronger task-evidence freshness architecture**. But the goal, source address, error comparison, fix candidate and corrective code are authored by the researcher; RECHECK is not a learned mutation strategy. The input can still change after the second read. More CIL source instructions and a correct verdict do not automatically imply higher ecological productivity, especially against compact STATIC_CORRECT. No special host fitness or automatic survival mechanism is allowed. CL-1B B3/B4 and open-ended cyberlife remain NOT MET.
