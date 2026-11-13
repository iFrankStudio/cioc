# CL-1 Study 11 — preregistered post-scaffold non-scripted graph operations

**Preregistered 2026-11-13, before creating the Study-11 runner or examining its outcomes.** Study 10's engineered 192 copies and 192 transfers are known. This independent cohort asks a more restricted question: do *externally randomized but not hardwired* ordinary CIL continuations produce successful inter-owner information transitions under verified neutral opportunities?

## Frozen independent population
- **96** independent deterministic 64-bit LCG seed-worlds **1700000..1700095** inclusive. Multiplier 6364136223846793005, additive increment 1442695040888963407, wrapping u64. Initial generator state equals seed. No outcomes used to select seeds.
- Three processes IDs 1,2,3, 28 code nodes each, **56 credits per process**. Strict capacity 32 shared cells, 40 code nodes/process, total 192 abstract units. No externally funded compute pool and no agent fitness, repair, reproduction or survival indicator.
- The first **nine** CIL nodes are a *symmetric, investigator-authored opportunity scaffold*: node0 `GraphNew{dst:3,next:1}`; node1 `GraphCopyNode{graph:3,source:2,dst:2,next:2}`; node2 `Const{dst:0,value:i-1,next:3}`; node3 `Write{address:0,value:3,next:4}`; node4 `Const{dst:1,value:(i+1)%3+1,next:5}`; node5 `GraphGrantRead{graph:3,reader:1,next:6}`; node6 `Const{dst:0,value:i%3,next:7}`; node7 `Read{dst:1,address:0,next:8}`; node8 `Const{dst:2,value:0,next:9}`.
- **Critically, unlike Study 10, there is no prewritten `GraphCopyFrom` or `GraphTransfer` in this prefix.** Any postfork such successful event must come from the uniform randomized CIL continuation, though that continuation itself remains externally generated initial code, not autonomous goal selection.
- Nodes **9..26** are sampled with one `random()%12` draw per node: tags 0/1 `GraphCopyFrom{target:3,source:1,node:2,next}` (two equal-weight possibilities); tag2 `GraphCopyNode{graph:3,source:2,dst:2,next}`; tag3 `GraphSetConst{graph:3,node:2,value:0,next}`; tag4 `GraphSetNext{graph:3,node:2,target:2,next}`; tag5 `GraphReadTag{dst:0,graph:1,node:2,next}`; tag6 `GraphGrantRead{graph:3,reader:1,next}`; tag7 `GraphTransfer{graph:3,recipient:1,next}`; tag8 `GraphNew{dst:3,next}`; tag9 `Const{dst:2,value:0,next}`; tag10 `Write{address:0,value:3,next}`; tag11 `Branch{test:1,nonzero:index-2,zero:next}`. Node27 is `Halt`. Invalid handles, unproductive cycles and traps remain included. **No host online selection or mutation** of the generated code occurs.

## Fixed common pre-treatment history
Every world runs exactly **27 CIL attempts** under deterministic round-robin scheduling (nine per process). This must create three nonempty graphs and public handle cells 0..2, with owner-to-predecessor read authorization. The source prefix is investigator supplied. Failure to reach that neutral scaffold invalidates execution of the full cohort rather than dropping a seed. Record the identical resulting world as the matched initial snapshot.

## Five matched conditions, all from the identical fork
- `control`: no intervention.
- `sham-drop`: researcher attempts invalid `DropGraph{graph:0}` at epoch0.
- `target-drop`: researcher externally deletes graph handle1 at epoch0.
- `disable-owner`: researcher externally disables process1 at epoch0.
- `zero-credit`: researcher zeroes credits of all extant processes at epoch0.
Each branch receives **16 independent laboratory epochs ×12 CIL attempted-instruction opportunities**, at most 192 postfork attempts. This yields **96 × 5 = 480 matched SQLite runs and 7,680 hashed postepoch world snapshots**. Every action is attributed external.

## Prespecified outcomes and independent denominators
- `successful_random_cross_owner_copy`: a structured internal `GraphCopied` event *after the common fork*, executed at a random-continuation PC ≥9, with actor != `source_owner`, and actor == `target_owner`. Report total events and seed-world incidence/96. Successful `GraphCopied` at PC <9 would **invalidate the generator**, not be counted.
- `successful_random_graph_transfer`: internal `GraphTransferred` at PC ≥9, former_owner actor and new_owner != actor. Report event totals and seed-world incidence/96.
- `nonowner_effect`: final state of processes 2 and 3 differs between `control` and `target-drop`, excluding the directly deleted graph object and external event trace. This can reflect scheduling or a disrupted engineered opportunity; it is **not** proof of organizational dependence.
- `graph_modification_proxy`: any surviving original graph (IDs1..3) has nodes differing from its pre-treatment code under control. Source of random CIL is external; this does not measure endogenous meaningful novelty.
- `random_continuation_late_activity`: control has at least one internally attempted instruction in epoch8..15; report actual CIL attempts and traps without sample selection.
- `sham_equivalence`: control/sham simulated state must be identical excluding external events/ticks.
- `zero_credit`: zero postfork CIL attempts even with preexisting shared objects.
- All **96 independent initial worlds** are the denominator for seed-incidence/95% Wilson intervals. Do not count 480 paired runs as independent, and do not filter failed or inert cases. 480 exact SQLite replays, 7,680 snapshot hashes, matched initial hashes, strict capacity ≤192 and no compute minting are hard acceptance requirements.

## Scientific interpretation and stage boundary
This is a **post-scaffold general information operation study**, not an open-ended evolution test. Even a successful cross-owner copy/transfer is causally attributable to an *initial externally randomized instruction*. Neither generic graph changes, ongoing execution, nor a nonowner lesion effect constitutes self-maintenance or life. No independent functional regeneration or reciprocal maintenance is tested; both remain **NOT EVALUABLE**, and CL-1's positive milestone remains open. An observed zero under this exposed scaffold is a negative for the specified grammar/horizon, not proof of impossibility.
