# CL-1 Study 35 — preregistered independent replication of source-conditioned native address output

**Registration:** prospective. This document MUST land on main **before** the implementation, execution, analysis, or selection of Study35 seeds. It is a distinct fresh seed cohort, not a reclassification of Study33/34 outcomes. Scientific claim remains **NONE** until independent native replay and auditor output are complete.

## Motivation and question

Study34's retrospective reanalysis of Study33's 128 registered worlds exposed 25 native process1 successful reads of graph2/node0 and three *non-target* source-tainted address writes; those paired branches shifted native output addresses by +8 following an externally imposed source tag change (11→19), while the frozen strict target-address and joint-function endpoints remained 0/128. Because these observations were discovered **after** Study33 outcomes were known, Study35 must test them prospectively without changing the low-level CIL random generator or selecting productive seeds.

**Question:** Does a fresh, independent non-goal-directed CIL cohort produce reproducible native process1 register-mediated, source-conditioned address changes under a paired source-graph perturbation?

## Frozen independent population and common world

- **192 independent seed worlds**, IDs **2,700,000..2,700,191 inclusive**, used in numerical order with **no filtering**.
- Exactly the original Study33 3-process initial 9-op scaffold, 18 random continuation instructions per actor, fixed 64-bit wrapping LCG and exact 12-way opcode grammar **copied byte-for-byte** from Study33's Rust runner (apart from seed start/count and study identifier). Both `GraphReadTag` grammar operands are **register indices**, `(dst=0,graph-register=3,node-register=2)` and `(dst=2,graph-register=1,node-register=2)`, not static graph object IDs. Do not optimize these choices based on Study34's rare witnesses.
- Same strict capacities: 32 storage slots, 40 process code-node capacity, total 192 occupied units; initial 56 credits/process; 27 common prefork scheduled attempts; 48 researcher-endowed compute pool; graph2 initial owner2/tag11 and allowed reader ACL; exactly 47 native credits/process at branch point; 24 epochs x 12 native attempts/epoch.
- **Nine registered paired arms**, all seeds in each arm, exactly the Study33 schedule: `control`, `mutate-g2` (graph2/node0 tag11→19 via external replacement with Halt), `empty-pool`, `mutate-g2-empty-pool`, `refill`, `mutate-g2-refill`, `sham` (rejected drop graph0), `exact-original-g2` (external restore of byte-identical source), `zero-credit`. Epoch 8/16 host SetCredits=47 in refill arms are explicitly externally subsidized and are not part of the primary no-refill outcome.
- Every run persists all 24 state SHA256 snapshots, original source revision, initial/final state, intervention schedule and 24-epoch attempt ledger; Rust verifies exact native epoch replay and full capacity.

## Primary endpoint and causal audit

Analyze **all 192** matched worlds without post-result filtering. A seed qualifies as `paired_native_source_address_shift` only if:

1. In both `control` and `mutate-g2`, a **successful internally attributed process1** `GraphTagRead` resolves graph object2, owner2, node0, carrying tag11 in control and tag19 in mutant.
2. Within the same original process1 code ancestry, its destination register is source-tainted; any successful Add propagates taint, and successful overwrite clears it. The code's register operands are resolved dynamically, never confused with object IDs. Traps, external events, wrong actor and unrelated source do not generate qualifying provenance.
3. A subsequent **successful native process1 Write** consumes this taint in the **address operand** on both paired branches. Its actual cell key is read from the corresponding `CellWritten` event; no requirement is imposed that the key equal 11/19 (the old Study33 target endpoint remains a separate descriptive negative control).
4. The paired qualifying writes occur in the **same epoch** and preserve the **same cell value**, with `mutate_key - original_key = +8` (the preregistered external source tag difference); if multiple matches are possible, require an unambiguous uniquely selected witness: first native source-tainted Write in each branch by epoch/tick order, then require matching epoch/value/shift.
5. The matched pair must not involve external `SetCredits` after fork, `PutGraph` except the explicitly registered graph2 source mutation, or code edits. Its successful write must occur after its source read on each branch.
6. **Negative controls:** `sham` and `exact-original-g2` final native simulated state must equal control; `zero-credit` must have zero native attempts; initial-zero-pool arms and scheduled-refill arms are reported separately, never silently pooled with control.

**Success criterion for this narrowly defined scientific mechanism:** at least **one** independently verified primary paired native shift in the full frozen 192 seeds, with no violated controls or data-integrity checks. Report the proportion and Wilson 95% CI; if zero, report a genuine prospective null. Even a positive result demonstrates **externally source-conditioned native information flow only**, not life, energy autonomy, regeneration, or damage response.

## Secondary descriptive metrics (not substitutes)

For each arm report process1 native DrawCompute uptake, successful graph2 source reads, read-before-vs-after-first-Draw chronology, source-tag lineage reaching any native address Write, matched shifts that also have an earlier same-actor successful Draw in the **same epoch**, matched shifts after >47 process1 charged postfork attempts without host refill, and activity without refill in W1/W2/W3 (epochs 0..7, 8..15, 16..23). The pool is **researcher-endowed** and recycled: native DrawCompute is not energy production. Document conditional denominators and non-evaluable empty denominators. Do not reinterpret an isolated increase in event frequency as life.

## Independent validation

Implement separate Rust runner and Python auditor, each with its own fixed seed/generator verification. The auditor independently verifies all SQLite SHA256 digests and 24 epoch windows, registered interventions and process code, source-read actor/owner/PC, register taint def-use, event epoch, matching values, original 47-credit threshold, external resource ledger, sham and zero-credit controls. Use a **small** targeted positive/negative synthetic calibration for wrong source, overwritten address and mismatched epoch/value. Runner and auditor must agree, with CI passing and archived unmodified SQLite evidence. A source-revision mismatch or missing epoch hash means failure/NOT EVALUABLE, not success.

## Interpretation and next decision

A replicating paired shift would validate a limited causal function in **preloaded** CIL under **externally changed** source, not endogenous code, repair or life. Only a **new separately preregistered** later study may introduce native perturbation, self-initiated restoration, unassisted cross-window stability and broader functional closure. No engineered `REPAIR`, `FITNESS`, `REPRODUCE` or goal-directed guaranteed-success program is permitted.

**CL-1 remains OPEN.**
