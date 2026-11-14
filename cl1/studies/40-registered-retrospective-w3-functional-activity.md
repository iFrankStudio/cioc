# CL-1 Study 40 — registered retrospective W3 native-function and external-starter audit

**Status: registered AFTER the known results of Study38 and Study39. RETROSPECTIVE, DESCRIPTIVE, ZERO NEW WORLDS.** Source cohorts were prospectively preregistered and completed independently, but none of the endpoints below are prospective Study38/39 primary endpoints. Do not reinterpret their W3 activity as life.

## Known baseline and question

The two independent environmental-physics experiments established W3 native charged-attempt activity in `ambient-general` versus `ambient-draw`: Study38 **97/128 vs33/128** and Study39 **93/128 vs36/128**. Their native W3 endpoint includes any charged attempted CIL instruction, including trap/halt. A crucial limitation is that the generic environment can inject exactly **one existing pool credit to an actor each epoch**. Determine how much of the late activity is (a) successful native computation, (b) actual shared-state output, and (c) dependent on a **same-epoch external process starter** rather than self-sufficient actor credit reserves.

## Frozen retrospective population

Reprocess exactly the **original source-pinned** Study38 run [43551254964](https://github.com/iFrankStudio/cioc/actions/runs/43551254964), source `02d0e9a017a26107edeca164c728d73e00998517`, and Study39 run [43551653280](https://github.com/iFrankStudio/cioc/actions/runs/43551653280), source `2071049cc8896ef6f1fc6d6e5f004fe152ae28e6`. Each 128 distinct worlds × ten arms ×24 snapshots; **256 source-worlds reused, 0 new worlds**. Never regenerate or change archived SQLite. Run the original independently verified analyzers `analyze_study38.audit` and `analyze_study39.audit` before this retrospective metric extraction.

## Fixed counts for each cohort and each of the ten arms

For each matched seed and all 24 epochs, derive events exclusively as the new suffix of cumulative hashed `state.events` between consecutive snapshots. Require exact monotone cumulative histories; all native events must have `origin={Internal: process}`, while `SetCredits(1)`, `SetComputePool`, direct host refills and sham actions have `origin=External`. Refer to `actions_json` to classify a registered external one-credit starter in each epoch and confirm action presence.

In W3 (epochs16..23), report:

1. **Charged activity:** at least one attributed native attempt; reproduce Study38/39 original W3 registered primary counts exactly.
2. **Successful native activity:** at least one internally attributed native event that is neither `Trapped` nor `Halted`; report median/range of successful native attempts.
3. **Native shared-state output:** at least one successful internally attributed `CellWritten`, regardless of target or putative organism fitness; record counts of events and seed-worlds.
4. **Temporal persistence:** successful native attempts in **at least 2** and **all 8** distinct W3 epochs, separately. An epoch with a trap only does not count.
5. **Without same-epoch external actor starter:** successful native attempts in at least 2 separate W3 epochs that contain **zero** `SetCredits(process,1)` environment starters. Ambient +8 pool input is not excluded, but must be separately labelled as **external environmental energy**. This does not mean absence of *earlier* external help.
6. **Host starter dependence:** number of W3 epochs with an external one-credit actor starter, successful native attempt in that epoch, and credited actor matching the first subsequent native event.
7. **Native compute uptake:** successful `DrawCompute` (confirmed from native PC/instruction `Op::DrawCompute` and event `Executed`) in W3, including event and world count. Native use of an externally endowed pool is not compute creation.
8. **Native graph construction/manipulation:** successful `GraphCreated`, `GraphNodeCopied`, `GraphTagRead` with strictly internal origin, by arm, descriptive only. Do not claim endogenous code evolution without ancestry and preserved function.

For every seed record at least the `ambient-general`, `ambient-draw`, `closed-general`, `ambient` and `closed` W3 successful-epoch counts, native outputs and no-same-epoch-starter counts; report both cohorts separately. No denominator substitutions or pooled prevalence inference.

## Controls and interpretation

Original independent auditor must verify the complete original source revision, seed grammar, intervention schedule, compute inventory, all **30,720 SHA256 snapshots per cohort**, and sham-closed final native state identity. The Study40 auditor must independently verify epoch event prefix, native event actor/PC legitimacy, absence of fabricated success, full eight W3 epochs, and exact starter intervention prefix. Include a compact synthetic false success from a `Trapped` event and a true native `CellWritten` with an unrelated external starter for calibration.

**Scientific interpretation:** W3 native output is at most evidence of functioning preloaded CIL under environment support. Even a completely successful eight-epoch W3 trace cannot by itself establish endogenous repair, internally acquired compute-source capability, multi-component causal closure, inheritance or life. Only fresh **prospectively registered** damage/repair and self-organization experiments may address the CL-1 acceptance threshold. CL-1 remains OPEN.
