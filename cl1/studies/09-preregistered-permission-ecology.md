# CL-1 Study 09 — preregistered full generic graph-permission ecology

**Registered on 2026-11-13 before creating the Study-09 generator or examining any Study-09 outcome.** Previous Study-08 results were known, including a structural testability flaw: the original 14-opcode ecology did not contain `GraphGrantRead`, so no normal cross-owner source read could be authorized. Study 08's 0 successful cross-owner copy events are real observations under that grammar but **not an informative probability estimate for what happens when permissions are reachable**. This correction is prospective, not a reclassification of Study 08.

## Objective
Under a neutral richer generic CIL instruction distribution that includes **object creation, ordinary graph-node copies, read grants/revocations, cross-graph code copying and ownership transfers**, do successful cross-owner information exchanges occur and have causal influence on other processes? Even if yes, the observation tests only generic information interactions, **not** self-maintenance or life.

## Frozen independent cohort
- **128 independent initial worlds**, fixed LCG seeds **1490000..1490127** inclusive, never reused, selected before inspecting results.
- Each world has three externally generated 32-node processes, each initially with **56 credits**. Strict shared-data capacity **32 cells**, max code **48 nodes/process**, total abstract storage **160 units**. The initial three process code graphs and headers occupy **99 units**.
- Four initial constants exactly as Study 08: `r0 = random%5`, `r1 = random%101`, `r2 = 2+random%3`, `r3 = 1+random%3`. The remaining **27 instruction nodes (4..30)** are uniform `random%16` choices, with node31 fixed `Halt`.
- Case tags 0..13 retain Study-08's generator behavior in order, except that its fallback `GraphTransfer` is explicitly tag13. Tag14 = `GraphGrantRead {graph:3,reader:0,next}`; tag15 = `GraphRevokeRead {graph:3,reader:0,next}`. No host-defined cross-owner address book or handwired repair roles. Random invalid handles, permission failures and traps remain in the denominator.
- The generic 16-way grammar contains, notably, `GraphNew`, `GraphCopyNode`, `GraphCopyFrom`, `GraphTransfer`, `GraphGrantRead`, `GraphRevokeRead`. Object access is subject to VM permission checks; the generator is not a goal or selection mechanism.

## Common pretreatment and five matched perturbations
- Externally record an initial `SetComputePool{credits:128}`, then allow **64** CIL attempts as a shared common history.
- Select the **smallest nonempty internally constructed graph**, if any, *before splitting conditions*. Otherwise choose invalid graph0 and process9999 sentinels; no filtering.
- Five conditions from an identical serialized common state: `control` (nothing); `sham-drop` (failed external graph0 deletion); `target-drop` (delete selected graph); `disable-owner` (disable that owner); `zero-credit` (set all existing process credits to zero). All external actions at environmental epoch0.
- Each arm runs **32 external epochs × 8 maximum CIL attempts/epoch**, regardless of CIL progress: **640 matched runs and 20,480 persisted post-epoch snapshots**.

## Frozen primary and validity measurements
1. `eligible_object` and `successfully_deleted_object` seed counts out of **128**.
2. In the untreated *postfork* events only, count **successful** `GraphCopied` with `source_owner != actor` and valid `target_owner == actor`; and `GraphTransferred` with `former_owner == actor`, `new_owner != actor`. Report event counts separately from number of seed-worlds with ≥1 actual successful exchange and its Wilson 95% interval.
3. `nonowner_process_effect`: count matched worlds where `target-drop` changes the final process states of any process **other than the deleted graph owner**, excluding direct graph contents, host event logs and injected pool. Denominator 128; inactive/no-target worlds count zero.
4. `sham_equivalence`: 128/128 control-sham final simulated states identical excluding external event histories/ticks. Any violation **invalidates** the experiment.
5. `zero-credit`: zero postfork CIL attempts and zero successful graph-transition events in every world even when global compute pool remains nonzero.
6. `structure_change`: nonempty graph handles newly created after deletion, without labeling them 'repairs'. All failed/trapped/inert worlds remain.
7. Exact SQLite replay of all **640/640** run states and all **20,480** epoch checkpoints; independent read-only Python analysis of hashes, resource bounds, preregistered labels/offsets and event origins.

The 128 independent seed-worlds are the replication unit, **not** the 640 paired conditions. No positive threshold is chosen by observing the data. The sample is a particular deterministic generator, not a random sample from all possible algorithms.

## Interpretation gate
- Successful cross-owner exchange, if any, establishes that the **general information/permissibility mechanics can be exercised without a host hand-coded cooperation oracle**, not that a self-maintained organization has formed.
- Even a paired nonowner effect does not prove bidirectional, multi-epoch functional closure, repair or evolution.
- CL-1's positive milestone remains open until a **separate** preregistered functional interdependence/recovery test succeeds; failed or negative tests are published without altering denominators.
