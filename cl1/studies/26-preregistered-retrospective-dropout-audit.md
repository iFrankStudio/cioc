# CL-1 Study 26 — preregistered retrospective *exploratory* dropout mechanism audit

**Registered on 2026-11-13 after Study-25 results were already known**. The Study-25 independent cohort, output contrast and the **three** observed `mutate-g2.output_missing_after_exposure` worlds are **known**. This is a **post-outcome exploration**, not a new independent confirmatory study, not a scientific preregistration preceding the original data, and not additional seed evidence. Do not cherry-pick identities or call the result prospective.

## Question

Why did three fresh Study-25 worlds lose a control-branch candidate graph2-read-followed-by-cell-write upon explicit external graph2 code replacement, while the strict positive paired numeric value-change endpoint remained 0/96? Separate source-exposure elimination, actual changed-source reads, stopped/trapped programs, relocated output writes, and ambiguous unobserved pathways **without redefining the earlier outcome or suggesting internal recovery**.

## Frozen reexecution and sampling

Use the **exact unmodified** Study-25 runner under the source-pinned current revision: seed-worlds `2,250,000..2,250,095`, all nine matched arms, same 24 epochs × 12 quotas and external graph-code/compute schedules. Reproduce 864 paired SQLite runs and 20,736 hashes, verify full Rust replay and rerun the independent Study-25 audit. Study26 then reads that SQLite **read only**; no subsequent modifications to source worlds, interventions or prior reported outcomes. The paired unit remains **96 independent worlds reused**, 0 newly seeded worlds.

## Strict candidate and classification

For each world, identify every **control** `GraphTagRead` with source `graph=2` and `owner != actor`, followed later in the *same epoch* by a successful `CellWritten` from that same actor to a specific cell. Use the earliest successful target-source read event and **exactly one** successful corresponding write per actor and cell (exclude multiple writes as ambiguous). Compare the `mutate-g2` branch at the same seed/epoch/actor/cell, preserving genuine process identity and event chronology.

Use the following **mutually exclusive**, predeclared rules in order, per qualified control (seed, epoch, actor, cell) match:

1. **Changed numeric output after target read:** the altered branch has a successful graph2 `GraphTagRead` of a different `tag` by the same actor, followed by an unambiguous successful write to the same cell with a different numeric value. (This replicates the original strict positive endpoint and should remain 0 if unchanged.)
2. **Same numeric output after changed target read:** changed target tag is observed before an unambiguous same-cell same-actor write but numeric value unchanged (source may be ignored).
3. **Altered source read, output absent:** changed target tag is actually read, but same-actor same-cell successful write is absent in that epoch. Separately report whether the actor **trapped** during that epoch and whether it wrote a different cell.
4. **No altered-branch target read, output absent:** altered branch never successfully read graph2 via that actor in that epoch. Separately report actor trap/nonexecution, alternative target `GraphFieldRead`, and any write to another cell; classify **path-exposure mismatch**, not a proven graph-tag-caused lost output.
5. **All other cases:** indeterminate or duplicate output/read; preserve raw cases, do not silently discard.

If the altered branch has an unchanged tag despite a claimed external node replacement, classify **audit failure** (source mutation must be effective if the exact node is read). All counts must include the **total 96-world denominator** and the separately reported **qualified-control-exposed-world denominator**. Do not call individual events independent worlds. Recheck the unchanged source-reinjection and sham matched final simulated-state equivalence and all external provenance controls.

## Evidence and scientific guardrails

Report per-class distinct world counts, unique control-exposed worlds, duplicate/ambiguous cases, source read totals, changed-source reads and actual trap evidence. No causal proof may arise merely from a missing output; any observed changed tag preceding a trap is a negative structural/behavioral sensitivity, not semantic source-to-output value dependence. Graph edits, credits and restoration remain externally scripted. Do not relax CL-1's accepted multi-component causal dependence + internally initiated recovery after observed loss over two later windows; no innate Repair/Fitness/Reproduce.
