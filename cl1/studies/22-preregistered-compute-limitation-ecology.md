# CL-1 Study 22 — preregistered independent ecological compute-limitation test

**Registration: 2026-11-13, before Study-22 code, execution or outcome inspection.** Previous results Studies 01–21 are known. This is a genuinely **new independent ecological seed cohort**, not another investigator-authored temporal functional calibration. **No native repair/fitness/reproduction, no adaptive host restart, no life claim.**

## Falsifiable question and fixed null

Study 14 found early but no W3 successful cross-owner graph copies despite a 24-epoch observation horizon. Was late inactivity caused merely by exhaustion of limited compute credits, as opposed to stopped/trapped programs or disappearance of viable information-processing pathways?

**Registered causal exposure:** matched restoration of *compute credits only*, without restarting stopped processes, in W2 and W3. Compare the same-seed late charged instruction attempts, successful cross-owner `GraphCopied` events, and native `CellRead` and `CellWritten` events in `refill` versus `control` and `sham`. Every event requires `Origin::Internal` and successful, charged native opcode provenance. This is a **feasibility/energy confound test**, not a self-maintenance result; externally supplied compute must never be counted as endogenous resource acquisition.

**Independent seed worlds:** exactly 96 fresh, predetermined seed IDs `2,220,000..2,220,095`, disjoint from Study-14's `2,140,000..2,140,095`. Reuse Study-14's known graph ecology generator, LCG, three externally initialized 28-node process codes, 9-instruction neutral prefix, 18 independently randomized continuation instructions per process, 56 credits per process, strict world storage limits `(cells=32, nodes=40, total=192)`, 27 deterministic prefork attempted CIL instructions. Programs and initial graph opportunities are researcher-authorized; executed native actions are **not** inherently invented information designs.

**Epochs:** 24 × 12 charged attempts, windows W1 0..7, W2 8..15, W3 16..23; no extra postselection. Precisely **8 paired conditions/seed**, 768 total matched runs and **18,432 hashed postepoch snapshots**, all worlds retained in the denominator even if permanently trapped or inert.

## Eight frozen matched branches

1. `control`: no interference.
2. `sham-drop`: rejected `DropGraph(0)` at epoch0; simulated state must match control.
3. `refill`: `SetCredits(1,47)`, `SetCredits(2,47)`, `SetCredits(3,47)` at **epoch8** and again at **epoch16**, in ID order. No other action; stopped processes remain stopped. `47` is the predeclared normal residual per process immediately after 9 common prefix attempts.
4. `drop-g1-refill`: external `DropGraph(1)` at epoch0; the exact `refill` credit schedule follows; **no repair**.
5. `drop-g2-refill`: likewise `DropGraph(2)` at epoch0 plus `refill`.
6. `drop-both-refill`: drop 1 then 2 at epoch0, plus `refill`.
7. `restore-g1-refill`: drop graph1 at epoch0; at epoch8 *before any instructions*, externally `PutGraph(1,exact prefork graph1 object)`, then the three `SetCredits`; replenish again at epoch16. This is explicitly **external structural restoration**, not endogenous repair or guaranteed **functional** restoration. The separate Study-21 externally scripted fixture has already calibrated a genuine functional positive control, and must NOT be pooled or imported into this randomized cohort.
8. `zero-credit`: `SetCredits(1,0)`, `SetCredits(2,0)`, `SetCredits(3,0)` at epoch0 and **no later replenishment**. No internally charged instructions should occur postfork.

Do not modify conditions, seed range, initial generator, allocation budgets, refill amount or postepoch windows in response to results.

## Preregistered endpoints, denominators and classification

Report counts of the **96 independent worlds** with (a) W2 or W3 at least one charged postfork CIL instruction, (b) W2 or W3 successful cross-owner `GraphCopied` events, (c) W2 or W3 successful native `CellRead`, `CellWritten` or both (not automatically dataflow), and (d) W1/W2/W3 reciprocal copy marker by matched branch. Also report event counts and per-epoch attempts, preserving 0 outcomes. For the primary energy-limitation comparison, report paired discordance counts `refill only` vs `control only` for W3 activity and successful graph copy; calculate Wilson intervals for per-arm proportions but do **not** pool the 768 matched runs as independent.

A new internally generated nonempty graph after damage, if observed, is a **structural exposure**, not functional repair. A completed source-sensitive execution lineage must be independently auditable under frozen Study-18 rules; if a graph read/write value dependency is not linked, label **NOT EVALUABLE** rather than fabricate a functional causal chain. A restored externally injected graph is tagged *external* even if copied internally later.

## Integrity and stopping

Full deterministic Rust epoch-by-epoch and entire world+event replay at a source-pinned commit, SQLite hashed snapshots, Python independent read-only audit of all 768 runs, pre/postfork identical hashes within each seed's eight branches, strict storage/credit overflow checks, exact intervention schedules, sham simulated state equivalence and zero-credit null must pass CI. A failed control or missing world invalidates technical verification; never quietly remove it. A separate **mechanism functional positive control** is already verified in Study21; its readout cannot be attributed to these random programs.

**Stopping rule:** report the entire fixed cohort including null or trapped results. Exogenous recharge is *not* self-maintenance. Even W3 copies or paired change under refill does not meet CL-1's endogenous multi-component causal functional recovery in two separated late windows. The CL-1 acceptance gate remains OPEN.
