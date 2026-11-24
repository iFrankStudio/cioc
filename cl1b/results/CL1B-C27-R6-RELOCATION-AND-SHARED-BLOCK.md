# C27-R6 — relocation-safe pruning and shared block topology

**Evidence tier: retrospective JavaScript CIL engineering model. No new Rust execution, no prospective holdout, no B3/B4 advancement.** Parent 1f315a1257ba9278f5b4cc213c0ef8ec91c1d69e. Uses the same selected ten R4/R5 scenarios, three intervals and two physical edit arms. External cutoff remains the Nth paid stage write, not actor Halt.

## Two variants

1. Source-specific pruning: start reachability from entry 0 and both legal mutable-edge targets 101/131. Traverse both branch arms and all next edges. Retain the union (including future legal edits), relocate all next/branch edges and the five known source-address Const operands at old PCs 93,94,163,165,168. This is not a general optimizer for reflective arbitrary CIL; it is valid only for this frozen finite mutation grammar. Physical edge addresses and raw address-valued registers differ under relocation. For each of 60 original monitor worlds (10 scenarios × 3 intervals × on/off), the model compares total debits, production/trial counts, exact original-PC sequence normalized through the relocation map, block observations, probe stages and normalized edits. All agree. This is model trace correspondence, not original native trace equivalence or a proof for unseen graphs.
2. Shared block: old roots 101/131 set r0 to candidate identity once per block. One shared unrolled production block uses an extra charged Sub comparing the actual ordinary service mode with r0 to choose the expensive path. Both code entries remain distinct, so ordinary paid PatchNext and ReadNext retain their roles. The monitor still never receives the environment mode directly. Compared with compact duplicated blocks, shared version preserves all probe stages and normalized source edit records in the selected on/off cases. Extra compute is one subtraction per service and one root constant per entered block. The source is researcher-written; actor does not discover sharing or optimize its own storage.

| k | R5 allocated | pruned duplicate | pruned shared |
|---:|---:|---:|---:|
|6|407|289|202|
|12|599|481|304|
|24|983|865|508|

Pruning removes 118 allocated nodes at every k for the monitored duplicated graph, with no additional execution charge. Sharing saves another 87/177/357 nodes. It is a compute/space tradeoff, not a free optimization. The original C27 budget of 170 nodes cannot fit any of these monitored variants.

## Fixed k12 results, equal production cutoff

| Scenario | initial-only | duplicate | shared | shared savings vs once |
|---|---:|---:|---:|---:|
|A→B after 2, N48|814|811|863|-49|
|A→B after 17, N48|754|751|803|-49|
|B→A after 17, N48|750|753|805|-55|
|A→B after 43, N48|650|691|743|-93|
|stable A, N48|630|671|723|-93|
|stable B, N48|626|667|719|-93|
|transient, N48|638|735|787|-149|
|two reversals, N96|1346|1441|1545|-199|
|A→B after 17, N96|1522|1375|1479|43|
|A→B after 17, N240|3826|3247|3507|319|

All complete the required production writes in the model. At N96, shared saves 177 nodes relative to duplicate at an extra cost of 104 CIL. Under a hypothetical one-time marginal storage price exceeding 104/177≈0.588 CIL-equivalent per allocated node, shared would have lower scalarized cost than duplicate, assuming no other resource differences. N240 threshold is 260/177≈1.469. These are illustrative sensitivity thresholds, not world prices. A compact initial-only implementation needs fewer nodes than the monitor and is still a meaningful economic baseline. No combined resource benefit is established.

## Next decision

Use the pruned duplicated k12 (481 nodes) and shared k12 (304 nodes) as candidates for a **bounded native engineering feasibility gate**, not an open-ended result. Before execution verify the actual Rust graph-length/capacity, node-address representation, validation, environment cutoff behavior and data-cell budget. Freeze exact relocated graph, source address mapping, affordable finite capacity and independent trace checker. Use the previously selected cases only for engineering calibration; freeze new task/change schedules separately before prospective testing. Keep initial-only, static baseline and identical-source physical-effect suppression; account for storage occupancy and all failures. A capacity limit between 304 and 480 can distinguish feasibility of the two candidates, but a deliberately chosen capacity is an experimental condition, not a universal resource advantage. A learned cadence or endogenous source-generation policy remains future work.

Reproduce: `node cl1b/analysis/cl1b_c27_compact_shared_monitor.mjs`. Full 30 interval/scenario rows, model equivalence assertions for 60 monitored cases, shared on/off comparisons, allocation counts and normalized edit records included. Executed the script body in the connector-side JS environment before commit; not CI or native.
