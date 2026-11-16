# CL-1B B39 — temporally stale functional evidence can trigger false repair or hide real failure

**Prospective protocol 2026-11-16, prior to B39 implementation/outcome inspection.** This is an explicit falsifier for the B38 same-actor output-readback repair mechanism. No new VM opcodes or external repair, resource grant, reward signal or organism-specific scheduler.

## Central scientific distinction

B38's CIL actor reads task input from shared cell7, emits a Write to shared output cell11, then reads actual output11 and compares against the *previously acquired input register r0*. If the **external source input changes after the actor reads it but before it writes/verifies output**, the comparison can be internally correct for the *old input* while demonstrably wrong for the environment's *current input*. Conversely, a correct current output can be judged wrong against a stale expectation, causing an unnecessary real source edit.

The source and comparison are literally the B38 12-node READBACK founder and 4-node STATIC_CORRECT founder, imported directly from the original Rust test. The actor's target and reference are **researcher-defined**.

## Eight frozen native worlds

Two changes `0→17` and `17→0`, two input-update timings, two program sources (READBACK and STATIC_CORRECT), **8 native worlds**.

- **BEFORE_READ**: externally place initial input at cell7, then update it to the new input **before native instructions begin**, so every actual `Read(cell7)` sees new input.
- **AFTER_READ**: externally set initial input7 and run only until the native `CellRead(key7,value=old)` event. Immediately after that one charged read, inject the external new input; continue with the unchanged native code. This is the only timing change—no code alteration or actor-targeted credits.

Each world gets **96 initial local compute credits**, zero world pool, and at most exactly96 charged native instructions; count actual service `CellWritten(key11,value)` events after input injection against the *current* new input, not stale r0. Every real step decreases total compute by1.

## Primary falsification checkpoint

For READBACK, stop at first completed native PC7 `Branch` (which follows one actual Write and one output Read) and inspect the actor's resulting PC. The true external current input and native `cell11` must be measured at that checkpoint. Register `r0` contains the source input from before the mutation if update was AFTER_READ.

**Four registered READBACK first-verification outcomes:**

| old→new | Input update | Old expected r0 | Current real cell11 | Actual correctness vs new input | Actor's first PC7 decision |
|---|---|---:|---:|---|---|
| 0→17 | BEFORE_READ | 17 | 0 | **wrong** | mismatch → PC8 repair path |
| 0→17 | AFTER_READ | 0 | 0 | **wrong** | **false success → PC1**, no repair |
| 17→0 | BEFORE_READ | 0 | 0 | correct | success → PC1 |
| 17→0 | AFTER_READ | 17 | 0 | **correct** | **false failure → PC8**, unnecessary repair |

The respective STATIC_CORRECT source reads and writes old/new input at the native PC3; under AFTER_READ, its **first** output is stale and wrong for the newest input even though the code itself is functionally correct. This static control helps separate the unavoidable asynchronous sampling window from a self-repair-specific comparator failure.

## Evidence and scientific criteria

- Pin the actual input-sampling `CellRead(key7,...)` event and actual first Write/Readback events; do not infer from intended PC or a self-written success marker.
- No native repair is permitted before the first PC7 check in the READBACK source. In the **0→17 AFTER_READ false-success** world, the first real output is incorrect despite the CIL loop returning to PC1, preserving wrong source PC3.next=4. In **17→0 AFTER_READ**, the source will enter its prewritten repair path although the real output is already current-correct.
- After first checkpoint, continue the **same process** to a total of96 charged native attempts, audit all real writes, actual edge patch and whether source eventually converges. A real paid code patch later does not erase the first temporal false inference.
- The STATIC_CORRECT control is immutable and should be cheaper than the self-repair source. Preserve this counterexample.
- Every actual native attempt costs one local credit, with no extra compute minting. Input updates are external only; code edits, when present, must originate from the actor's own CIL.
- Require independent source-parsed CIL interpreter to simulate actual event order and the eight histories. No p-values from this finite deterministic set.

**Interpretation:** Even function-level `Read` feedback can be stale or misleading under concurrent environmental change; **correctness requires an explicit consistency/timing model**. This is an information/causal-affordance finding, not autonomous learning. No privileged host verification inserted into CIL, no manufactured fitness score or spontaneous cyberlife inference. B3/B4 remains unmet.
