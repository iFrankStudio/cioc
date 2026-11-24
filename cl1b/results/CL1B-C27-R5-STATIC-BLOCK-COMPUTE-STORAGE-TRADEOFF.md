# C27-R5 — static control-flow block scheduling: compute/storage tradeoff

**Retrospective non-native engineering model; no new Rust worlds, no independent native replay, no heldout or B3/B4 advance.** Parent 4b37f14298405d4994086ae1c792308d0f76aaee. Same 10 selected R4 scenarios and common external cutoff immediately after the Nth paid stage-counter Write. No future horizon or change time exposed to actor. Costs are instructions, not combined resource fitness.

## Design
R4 monitor2 charges a Sub and Branch every service to count down until sampling. Instead preallocate two blocks (candidate A and B), each containing k consecutive service slots. The physical PC100.next edit (101 or 131) selects the entire block. Each slot has the same six charged stage-count operations and same production semantics/cost as the original path; explicit graph edges move directly to the next slot, and the last slot reaches PC209 paid credit observation. No per-service cadence counter or new opcode. PC202–218 performs the original paid baseline and change comparison with the now-redundant countdown initialization removed. Mode data is used by ordinary production/trial paths, not the monitor. Evaluation still uses compact R3 paid candidate comparison. This is a researcher-authored static unroll, not actor invention or learned cadence.

Initial-only reference has the same number of allocated nodes, the same duplicated service slots, exits the block to PC100 instead of metrology, and skips paid monitoring initialization. Its costs equal the R4 initial-only reference in all ten selected scenarios. Each monitor on/off pair has identical initial graph and all PatchNext attempts charged, suppressing only physical effects in off. Note that downstream observations and trial decisions can differ; effects are bundled. At every stage-counter write environment changes only cell50 per the scenario; no changes are signaled directly to the monitor. All scenarios complete their N production writes. Transient regimes, late changes and baseline aliasing remain.

## Charged compute outcomes (initial-only and monitored on, net savings = initial-only minus monitored)
|Scenario|N|Once|k6|k12|k24|k12 net|
|---|---:|---:|---:|---:|---:|---:|
|A→B after 2|48|814|811|811|831|3|
|A→B after 17|48|754|775|751|771|3|
|B→A after 17|48|750|777|753|767|-3|
|A→B after 43|48|650|739|691|667|-41|
|A stable|48|630|719|671|647|-41|
|B stable|48|626|715|667|643|-41|
|A→B after 19→A after 21|48|638|783|735|655|-97|
|B→A after 19→B after 55|96|1346|1537|1441|1581|-95|
|A→B after 17|96|1522|1447|1375|1423|147|
|A→B after 17|240|3826|3463|3247|3223|579|

For 96 services, k12 improves from R4 monitor2 1574 to 1375 (199 fewer charged instructions) and now saves 147 vs once. For 240, k24 costs 3223 vs 3826 once, saving 603 (15.76%), compared with R4's 114 saving. At k12 the first two changed 48-service scenarios save just 3 each; reverse still loses 3. All stable/transient/twice cases lose vs once. At k12, summed over all ten selected cases, net savings are 414 CIL, dominated by the exploratory 240-service case; without it net loss is 165. Do not infer population performance from this selected mix or tune k per world.

## Capacity is not free
R4 uses 219 source nodes. The actual generated graph has 215+32k allocated nodes (including retained unreachable original nodes): 407/599/983 for k6/12/24. Relative to R4 this is +188/+380/+764. These exceed original C27's 170-node budget. Source occupancy and data capacity have not been charged in a common compute/storage fitness. Even if the k12 96-service benefit is 147 CIL, a one-time marginal cost of >=147/380 CIL-equivalent per extra node compared with R4 would consume this particular benefit (illustrative threshold using R4 size, not a native resource price or combined comparator). Matched unrolled once is equal-size, but a compact once controller does not need the extra nodes; real capacity limits can reject unrolling outright. A native experiment must explicitly measure code/storage occupancy and use declared feasible limits; no claim of net total resource benefit.

## Next gate
Prefer a compact source-native design before expanding to hundreds of nodes: remove unreachable source nodes, quantify reachable-node lower bounds and compare graph-footprint/compute Pareto frontiers; do not hide unused allocations. If a storage-aware tradeoff remains promising, freeze exact graph, finite capacity and novel schedule distribution before original Rust execution and independent replay, with stable/no-change/short/transient/multiple reversals and equivalent economic references. Consider runtime cost of cadence using existing stage accounting versus code-space; do not introduce a free specialized monitor opcode. B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c27_unrolled_block_monitor.mjs`. Evaluated JS model rows and all 30 same-source on/off comparisons are preserved via expected totals, exact measurement stages and edit traces. This script was transcribed from the evaluated model; no CI/native run claimed.
