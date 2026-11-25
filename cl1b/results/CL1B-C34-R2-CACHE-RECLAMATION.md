# C34-R2 — paid actual result-slot reclamation
Parent fbd4d160455fa7b882aa90ba878f18bbead220f8. Isolate shrink from R1 growth: start two-slot FIFO, monitor only full cache misses (both keys missed, not second slot's unconditional nonuse), budget8 in mutable Const, decrement and PatchConst charged on each full miss, no extra data cell. Hits do not reset budget. At zero retire monitor via PatchNext, pay PatchConst flag, if allowed patch second-lookup bypass and stop rotation, patch store destination63, execute four native-shaped Delete instructions for keys64..67, preserving slot1; then original computation. Off suppresses only flag, still retires monitor, stays two-slot. All deletion effects paid and actual model map entries removed; no reallocation into another workload tested, no rent, no shrink of executable code. One-shot irreversible for this study, cumulative misses not profitability; may harm subsequent recurrence. Exact preactivation prefix and matched activation, off matches static two kernel count. Native Delete removes cell and last writer; model has no last-writer state. No new op, no host decision. Observer validates actual outputs, inputs, clone routes, full-key reuse and records, deletion count, no post-shrink second slot, strict16 cells/200 nodes/220 occupied/200000 credits. CellTime is sum of occupied data cells at request completion, not rent nor exact instruction-integrated occupancy. Authored threshold8 and FIFO. No native.
64 revealed schedules +32 designed transition schedules x5 arms=480 model worlds. Same source-family matched baselines (minor source/layout/branch overhead differences vs R1).
| pattern | one net | two net | shrink net | off net |
|---|---:|---:|---:|---:|
|all|48246|195198|198060|191510|
|original|43746|69068|88780|66468|
|shifts|4500|126130|109280|125042|
|stable|43908|43796|43748|43748|
|alternating|-13440|41750|41654|41654|
|cycle3|-13440|-18744|-14720|-19192|
|burst|38640|36388|36888|36092|
|drift|-13440|-18744|-14720|-19192|
|rare|-12498|-17778|-13778|-18226|
|k_change|27552|21360|24572|20992|
|mixed|-13536|-18960|-14864|-19408|
|alternating_to_drift|-16128|458|3602|10|
|alternating_to_stable|29556|51338|51242|51242|
|return|-16128|23404|3602|22956|
|stable_to_alternating|7200|50930|50834|50834|
Shrink {"net":198060,"vsTwo":2862,"wins":56,"paid":526656,"peak":211,"nodes":194,"reused":3196,"kernels":11037,"retirements":56,"deleted":224,"misses":540,"cellTime":84520}. Off {"net":191510,"vsTwo":-3688,"wins":57,"paid":533206,"peak":211,"nodes":194,"reused":3372,"kernels":10377,"retirements":56,"deleted":0,"misses":540,"cellTime":95784}. Future combine reversible growth/shrink only if management costs justified; use actual released capacity under competing storage demand. B3/B4 NOT_MET.
Reproduce node cl1b/analysis/cl1b_c34_r2_cache_reclamation.mjs.
