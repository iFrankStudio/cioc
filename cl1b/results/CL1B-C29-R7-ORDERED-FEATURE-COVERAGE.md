# C29-R7 — shared capture and paid ordered feature coverage

Parent 751543cf7241af1e1617a64fd53f10f272246e57. **Non-native component model only.** No production, physical edits or new native evidence. B3/B4 NOT_MET.

## Shared capture

R6 had duplicate capture paths for trusted and fallback prediction. Move the common eight paid input-copy operations ahead of trust dispatch; then branch either to inference or trial. No change in instruction count per query, but remove eight static nodes. Executed all 300 original R6 worlds again with compact version and checked exact equality of costs, per-query outputs/paid costs/trust/measurement indicators, setup, validation and audit rows; node counts all reduced by eight. This is behavioral and accounting equality for tested schedules, not exact instruction traces (capture/trust check order differs). Environment held coherent within query; not claimed equivalent for arbitrary interleaving. Compact periodic graphs still exceed 200 nodes for audited variants.

## Ordered feature coverage

Replace fixed countdown with coverage cursor encoded in cell47 as target x+4 (range5..20); initial target1, phase3/4 still validation. For each trusted query after paid affine prediction, compare captured x to target by paid Sub/Branch. Only matching query triggers paid paired audit. On successful audit advance target by one, wrap16→1; mismatch revokes trust sticky and outputs actual difference as in R6. All comparisons and cursor updates charged, no new data cells, no host truth/failure-region input. This is coverage of one authored feature, not uncertainty estimation, not complete input-pair coverage. In particular cursor waits indefinitely for absent target. Model still assumes positive domain 1..16, fixed candidates, coherent input per query. Observer's reference checked against always-paired arm, all trial products checked. Model does not emit production or modify source.

## Experiments

Original R6 12 schedules×5 profiles×6 policies=360 worlds, 23040 outputs. Includes 300 matched compact replays and 60 coverage worlds. Additional four deterministic stress sequences per each of three nonlinear profiles ×6 policies=72 worlds, 4608 outputs: missing first class, clustered target, rare first class, missing middle class. Stress family deliberately constructed to expose cursor starvation; not blind. Same input schedules across controls.

| profile | coverage errors | audit4 errors | coverage revocations | audit4 revocations | coverage audits | coverage net vs paired | nodes / peak |
|---|---:|---:|---:|---:|---:|---:|---:|
|cross|0|0|0/12|0/12|319|-18913|209/226|
|unroll|0|0|0/12|0/12|0|-7848|215/232|
|switch5|22|24|6/12|5/12|84|1087|218/235|
|switch11|25|39|6/12|1/12|104|1726|218/235|
|switch16|27|24|4/12|5/12|127|438|218/235|

Across 36 localized failure worlds coverage emits 74 incorrect differences and revokes 16; periodic4 emits87 and revokes11. Coverage can improve some regions, not all; switch16 has27 errors vs24 periodic4. Coverage cross exact but costs18913 more than paired, due to clustered sequential audits and paid prediction/lookup. All coverage graphs209..218 nodes and peak226..235 exceed previous200/220 capacity even though cell count unchanged.

| stress | coverage errors | coverage audits | coverage revocations | periodic4 errors | periodic4 revocations |
|---|---:|---:|---:|---:|---:|
|missing_first|71|0|0/3|3|3/3|
|clustered|96|32|3/3|9|3/3|
|rare_first|189|3|0/3|9|3/3|
|missing_middle|93|3|0/3|93|0/3|

Missing-first stress never presents x=1, so cursor never audits any query. Rare-first and missing-middle also demonstrate starvation or late evidence. Ordered coverage is not reliable admission. All costs include training per world. Cheap wrong differences not valid optimization returns. No promotion, no relaxation of capacity.

## Next decision

Do not keep adding cursor heuristics to a predictor whose evaluation costs scale linearly with input. Separate resource-efficient model representation from reliability. Consider compact **bounded feature-indexed validation** (paid encoding and lookup, no free bitmap oracle), with explicit invalidation and no ordered starvation, or redesign prediction to reduce arithmetic cost. Compare with paired baseline and audited fixed reference on fresh schedule families; avoid selecting failure coordinates directly. Strongly distinguish valid cost inference from actual beneficial source edits. B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c29_ordered_coverage.mjs`. Model body executed in connector JS; original R6 300-world behavior/accounting match performed in this turn. Script preserves aggregate assertions; no native CI.
