# C34-R3 — reversible persistent result storage
Parent 12d5bd3644a2592b07986512616b5aefa63d38b3. To fit both growth and reclamation under 200 executable nodes, change representation to exact packed key (32*a+b)*16+k, injective for positive a,b1..16,k1/2/4/8; paid additions and reads, stored temporary59. Two cells per record (key,result), slots60/61 and64/65. Ghost key in mutable Const, no extra ghost data cell. Single-slot miss compares current key to previously evicted key, updates ghost via paid PatchConst from actual old slot1 key, on conflict grows via flag and patches route/store/rotation, computes and installs second record. Dual-slot full misses debit mutable Const budget8; at zero shrink flag, patch routes, delete both second-record cells, keep first slot, remember its evicted key before current computation; subsequent conflicts can grow again. Budget reset to8 each growth. Hits do not replenish. Flags remain one after successful first effect; repeated actual same-value PatchConst paid, subsequent transitions still branch on flag; matched suppression per site throughout. Off-grow never grows, continues ghost observation; off-shrink stays two after first denied shrink and retires monitoring. Prefix matches through first growth attempt; shrink-off follows same history until first shrink attempt (not claimed fully checked beyond first growth by script). No rent, no competing storage use, no actual reuse after process stop. No new op, authored rule and key grammar, coherent requests, intact code/key/value storage. Model observer checks exact recurrence input, output, record and hit provenance, paid state transitions, Delete, bounds. Shared bypass encoding; fixed capacities and grow-only share new layout; not directly comparable to old four-cell representation. 96 revealed schedules plus6 designed repeated transition schedules x7 arms =714 worlds; no blind holdout. 
| pattern | one | two | grow only | cycle | grow off | shrink off |
|---|---:|---:|---:|---:|---:|---:|
|all|-14726|196758|194542|190838|-60686|193310|
|stable|41500|41428|41444|41444|41444|41444|
|alternating|-16320|40086|38986|38986|-22000|38986|
|cycle3|-16320|-20640|-19680|-19680|-19680|-19680|
|burst|36180|34660|35760|35760|35760|35760|
|drift|-16320|-20640|-19680|-19680|-19680|-19680|
|rare|-15370|-19658|-18674|-18674|-18674|-18674|
|k_change|26376|24396|25536|25536|25536|25536|
|mixed|-16320|-20640|-19680|-19680|-19680|-19680|
|alternating_to_drift|-19584|-1734|-2834|-2722|-24496|-3282|
|alternating_to_stable|26476|48762|45742|45742|24156|45742|
|return|-19584|21300|20200|18828|-25376|19752|
|stable_to_alternating|3936|48642|47542|47542|-592|47542|
|repeated|-29376|20796|19880|17436|-37404|19544|
Cycle {"net":190838,"wins":60,"paid":648070,"peak":201,"nodes":187,"reused":3702,"kernels":12663,"grow":66,"shrink":34,"deleted":68,"cellTime":82196}; regrowth worlds 14. Next examine competing demand and actual allocation using freed capacity, and native proof. No native run, B3/B4 NOT_MET.
Reproduce node cl1b/analysis/cl1b_c34_r3_reversible_cache.mjs.
