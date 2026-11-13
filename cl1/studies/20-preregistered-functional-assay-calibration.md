# CL-1 Study 20 — preregistered two-component functional assay calibration

**Registration: 2026-11-13, frozen before Study-20 runner, analyzer, outcomes or CI execution.** Studies 01–19 are known. This is **one externally authored mechanism fixture**, not a stochastic ecology, independent biological replicate, autonomous repair or life evidence. Do not pool its nine paired arms with the independent seed-world cohorts.

## Objective

Verify a deterministic CIL input/output function that reads two distinct internally *executed* graph allocations, detects loss under either single or double external ablation, responds to changed graph code content, and recovers the **same output capacity** under a same-object external `PutGraph` positive control. This is a mechanism-calibration observable, not longitudinal autonomous recovery. All initial process programs and opportunities are investigator specified, and execution by CIL does not make their design endogenous.

## Frozen common world

Strict capacity: maximum four shared cells, 16 process nodes, total 64 storage units. Load three process IDs 1,2,3. Processes 1 and 2 each receive researcher-authored six-node CIL programs and 6 initial compute credits; process 3 receives a nine-node researcher-authored reader and **zero** initial credits.

For owners 1,2, code nodes at PCs 0..4: `GraphNew` to r0, `Const(3)` into r1, `GraphGrantRead` from graph r0 to reader r1, `Const(0)` into r2 for owner1 or `Const(1)` into r2 for owner2, then `GraphCopyNode` using r2 as the source process-code index into own graph. PC5 is Halt. Ten common attempted instructions run before forking (five per owner, no reader instructions). Assertions: handles 1,2 exist and are nonempty with owner1,2 respectively; reader3 has explicit read grants; graph1 node0 has opcode tag 11 (GraphNew) and graph2 node0 has opcode tag 0 (Const); no shared output cell exists.

Reader3 code (nine nodes): `Const(r0,1)`, `Const(r1,2)`, `Const(r2,0)`, `GraphReadTag(dst=r3, graph=r0, node=r2)`, `GraphReadTag(dst=r2, graph=r1, node=r2)`, `Add(dst=r3,lhs=r3,rhs=r2)`, `Const(r0,90)`, `Write(address=r0,value=r3)`, Halt. Complete functional output is **shared cell 90 = 11**, attributable to successful reader CIL execution; a trapped or missing write is not success. If graph2's node is externally replaced by Halt (opcode tag 19), output must instead equal **30**.

## Nine matched, identical-prefork arms

All interventions occur at **epoch 0 before internal CIL**. All branches set process1 and process2 credits to zero. All branches except zero-credit inject exactly nine credits into process3 via externally attributed `SetCredits`. Conditions, with graph injections carrying the complete saved prefork graph object:

1. `control`: no graph intervention.
2. `sham`: rejected `DropGraph(0)`.
3. `drop1`: `DropGraph(1)`.
4. `drop2`: `DropGraph(2)`.
5. `drop-both`: `DropGraph(1)`, then `DropGraph(2)`.
6. `restore1`: `DropGraph(1)`, then `PutGraph(1, original_object1)`.
7. `restore2`: `DropGraph(2)`, then `PutGraph(2, original_object2)`.
8. `mutate2`: `DropGraph(2)`, then `PutGraph(2, object2 with node0 replaced by Halt)`. This is explicitly exogenous code mutation.
9. `zero-credit`: same owner-credit reductions, `SetCredits(process3,0)` instead of positive injection.

Run **3 independently clocked epochs × 12 CIL attempts maximum**, for exactly **nine complete SQLite paired runs and 27 hashed epoch snapshots**. No random seeds are sampled or implied. Every branch uses the same serialized initial world; native Rust must exact-replay all states/events and the independent read-only Python auditor must verify all nine schedules, snapshots, outcomes, provenance and storage/compute bounds. Sham and control simulated process/data/graph/pool state must match.

## Frozen outcomes and failure classification

- `control`, `sham`, `restore1`, `restore2`: cell90 must equal **11** from successful internally charged reader `CellWritten`.
- `mutate2`: cell90 must equal **30**, proving output value is sensitive to the supplied graph code descriptor, not merely graph presence.
- `drop1`, `drop2`, `drop-both`, `zero-credit`: cell90 absent; in drop arms reader is trapped, while zero-credit executes no CIL after fork.
- `PutGraph`, `DropGraph`, credit changes and code replacement must be explicitly external. No branch mints unrecorded resources; storage occupancy <=64.
- Any failed external graph injection, mismatched initial snapshot, incorrect source content, failed sham, missing event provenance, missing snapshot, non-replay or failed controls invalidates the **technical calibration**. Never silently remove failed arms.

## Scientific boundary

Even if this fixture passes, it proves only that an **externally scripted** reader can have measurable two-component information dependence and that immediate externally delivered same-object injection restores *capacity before the attempted read*. It does **not** demonstrate a lost function reappearing across post-lesion epochs, internal design autonomy, regenerative closure or self-maintenance. No CIL `REPAIR`, `FITNESS`, `REPRODUCE` or host-side autonomous maintenance policy is authorized.

A later independent ecological study must register **fresh seed-worlds**, unknown code continuation, matched untreated/sham/single and double lesions, matched resource controls, a feasible external functional restoration arm and at least two separated post-damage windows *before* implementation. Until then CL-1 acceptance remains OPEN.
