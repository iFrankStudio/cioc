# CL-1B / B23 — organizational construction cost versus a simpler solo compute policy

**Prospectively registered 2026-11-15, before authoring the B23 native differential or computing its outcomes.** B23 is a **critical negative control** on B22. The success of native graph assembly and interprocess relay must not be mistaken for higher adaptive fitness if an existing simple solo strategy services the same energy environment more efficiently.

## Fixed experimental substrate and comparison

All arms start with **exactly one externally loaded process, 29 local compute credits**, zero pool, strict capacity (16 cells, 64 nodes per process, 128 total stored units), and `with_pool_paid_draw()` plus `with_recoverable_draw_shortage()`. No host `SetCredits` or second process load is permitted.

Two neutral world-pool schedules, each supplying exactly **8** external compute credits across two packet windows:

- **Previously designed/development:** `[3,5]` from B22.
- **New prospectively specified control:** `[4,4]`.

Run the deterministic scheduler until blocked before the first packet and after each packet. Every native charged attempt decrements combined local+world pool inventory1, every external pool packet increases that inventory by its quantity. Count genuine **native** `CellWritten(key9,value1)` from process1, not status markers or the child's output.

Programs compared, all researcher-authored:

- **ASSEMBLED_RELAY:** the exact 35-node B22 single founder, which natively constructs, edits and launches a five-node helper with genuine `TransferCompute`, at a 25-instruction and 2-transferred-credit formation cost, then leaves the worker with a request of2 credits.
- **ASSEMBLED_NO_TRANSFER:** same source and costs except the helper's final transfer opcode is a same-cost Const; checks that any output gain actually needs the relation.
- **SOLO_REQUEST_1 through SOLO_REQUEST_7:** the existing five-node B19 worker with a researcher-selected fixed request literal1..7, initial credits29 and no helper at t0 or ever. The generic policy is a fixed `Const r0=request; DrawCompute; Const output key; Const output value; Write` loop. **No policy is learned by CIL.**

The solo variants have **different code sizes** from the assembled founder (5 vs35 nodes) because the very question is the *total compute/storage burden of constructing organization*; they are an **efficiency baseline**, not a code-size- or description-length-matched control that proves a causal advantage of a mutation. Record code size and total occupied storage; never claim the smaller baseline differs only in a single mutated instruction from B22.

## Primary measures and preregistered interpretation

- Start with one actor and exactly29 total compute inventory. The assembled program pays to launch a second process, but native `GraphLaunch` transfers rather than creates the child's credits.
- Each initial program is run to the first blocked state before any environment packet; include **all charged bootstrap attempts** and any service outputs before resource influx, not just the post-start count. The question is long-horizon **resource access**, so report both *total service writes* and *service writes occurring after the first external packet*.
- For solo requests1..7, the founder has no initial world compute pool and cannot obtain positive additional credits. An insufficient `DrawCompute` with recoverable shortage **charges and retries** until initial29 credits exhaust; at that first block, no genuine service should have occurred.
- ASSEMBLED_RELAY must be checked against both schedules, including the outcome if the first neutral packet4 differs from B22's earlier packet3. A positive relation must involve a genuinely internally launched helper and actually transferred local credits at the helper's PC4, not simply a native graph event.
- Compare service counts and actual charged attempts under **identical initial compute29 and external total influx8**. **Universal organizational superiority is falsified** by a single solo program producing more genuine service writes under an identical schedule. A favorable solo outcome is not proof that the individual found the better request—it is investigator-provided and an evaluation baseline.
- Count traps/halts, residual pool credits, inactive but runnable processes, code/storage footprint, source provenance, and incomplete service cycles. Do not drop zero-output cases.
- These two packet schedules are **not** a statistically independent sample of all environments. There is no p-value or general population success claim. Treat them as a finite mechanical counterexample.

## Scientific consequence

The ability to build an elaborate helper is necessary for some cooperative organizations but **not sufficient for adaptive advantage**. B23 isolates the cost of internally constructing a relay from the benefit of a relationship, while asking whether a simpler prewired solo draw amount dominates under the very energy schedules used to demonstrate B22.

If a solo baseline wins, the next B3/B4 research hurdle is not merely adding more native graph edits: the actor must **originate or select a lower-cost policy from information and paid experience**, while control observations prove it did so without a privileged host fitness oracle. No changes to CL-0 world physics or researcher-selected winner after the fact are allowed.
