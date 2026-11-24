# C29 original native evidence and R1 — decision freshness

Original native tested SHA ad3e3e196953e85132e2a13d7a988150814f90d1, [native run](https://github.com/iFrankStudio/cioc/actions/runs/45334638426) SUCCESS; [archive run](https://github.com/iFrankStudio/cioc/actions/runs/45334897966) SUCCESS. [Permanent original evidence](https://github.com/iFrankStudio/cioc/tree/cl1b-ci-evidence/run-records/ad3e3e196953e85132e2a13d7a988150814f90d1-45334638426-1), ZIP SHA256 c4657168a664f04c6fdaa137bbba1c23d8c77868e9c11ebdfb7570a4eb5add6c, economic-summary blob 42088b3687b73b92ed314a1cf9f8af5bd12852c0. Archive CI reverified all original native traces and remote ZIP hash. This turn read derived original summary; did not re-hash ZIP locally. 80 worlds, 219570 paid CIL, 5680 production, 288 trials; complete equal-service cutoffs. Focused historical regression run 45334638416 failed; dedicated pass does not imply full regression.

## Original native findings

| cohort | policy | economic saving | physical saving | wins/losses | unequal input pairs | equal-output unequal-input pairs |
|---|---|---:|---:|---:|---:|---:|
|calibration|snapshot|2626|1496|2/2|0|0|
|calibration|live|3617|2364|3/1|2|1|
|prospective|snapshot|-272|1486|3/9|0|0|
|prospective|live|1122|2570|3/9|4|3|

Economic comparator is snapshot initial-only, not matched live initial-only; physical effect is same-source off minus on. Local snapshot solves the declared between-trial input confound but is not atomic and does not predict future input. In new_same_product_swap, input (14,2)→(2,14) after A output: snapshot trial costs53/17 on identical (14,2), live costs53/53 on different inputs with equal output28. Snapshot correctly identifies old input's preferred candidate but applies stale choice, actual physical saving -866. Live avoids that edit by chance, not reliable causal inference. Native new_transient loses 1410 relative to initial-only, because old fixed stage24 evaluation occurs inside temporary reversal and never reevaluates again. Original source graph 167/159/164, peak occupancy179/169/176.

## R1 retrospective model

Test three authored source variants on all 16 revealed native schedules. No new native execution or blind prediction. Same paid sequential capture; environment changes only at declared boundaries; no concurrent capture atomicity claim. All instructions charged; physical-off suppresses only PatchNext. Original snapshot model reproduced all 16 native on costs in this turn.

- guard: after both trials, before output/cost decision, pay for fresh live input reads and compare each with retained input. Any mismatch abstains and keeps current source. No extra trial.
- retry: same guard with one retry allowed per evaluation episode, stored in ordinary cell47; on first stale result capture a fresh input and pay for both trials again, on second stale result abstain. No unbounded loop or horizon oracle. The current schedules only intervene once between trials, so repeated staleness is not tested.
- selective: perform freshness check only when the actual live dispatch would be physically changed, immediately before PatchNext. Reload dispatch register because checks overwrite it. Output equality and local candidate cost comparison still use captured input. If stale, abandon patch, no trial retry. Still subject to change after check or during capture outside declared environment contract. No evaluation admission or learned strategy; stage24 schedule retained.

| case | original native on | initial-only | guard | retry | selective | selective off |
|---|---:|---:|---:|---:|---:|---:|
|cal_swap|2109|2543|1347|1512|1391|1391|
|cal_different|1627|1535|1441|1556|1485|1485|
|cal_flip|2591|5157|2615|2621|2617|3526|
|cal_stable|1385|1103|1409|1415|1398|2853|
|new_equal|3033|2734|3057|3063|3033|3033|
|new_stable_a|2195|1819|2219|2225|2195|2195|
|new_stable_b|2193|1819|2217|2223|2206|4813|
|new_small_flip|2704|2537|2728|2734|2717|2879|
|new_large_flip|2255|4593|2279|2285|2281|3022|
|new_late|4689|4257|4713|4719|4689|4689|
|new_twice|4325|3948|4349|4355|4351|4402|
|new_transient|3591|2181|3615|3621|3604|2548|
|new_same_product_swap|2649|3741|1735|1914|1787|1787|
|new_same_product_factor|2445|2276|2319|2439|2375|2375|
|new_different_product|2384|2459|2126|2254|2170|2170|
|new_same_product_reverse|2639|2466|2595|2647|2652|2778|

Prospective-at-C29 (now revealed) summary:
```json
{
  "guard": {
    "nodes": 180,
    "peak": 192,
    "net": 878,
    "physical": 2644,
    "wins": 3
  },
  "retry": {
    "nodes": 187,
    "peak": 200,
    "net": 351,
    "physical": 2718,
    "wins": 3
  },
  "selective": {
    "nodes": 193,
    "peak": 205,
    "net": 770,
    "physical": 2631,
    "wins": 3
  }
}
```
Guard improves aggregate net from -272 to +878 on revealed scenarios, not a new prospective result. Retry adds cost, and selective guard trades placement/size against steady checking cost. New graphs must fit declared 200-node/220-total budget; all are model feasibility only. The improvement derives substantially from avoiding stale edits in injected between-trial transitions, not solving the transient reversal or stable evaluation overhead. No guarantee of positive gain in arbitrary environment. Preserve all negative cases. No native promotion on these data alone; freeze source, explicit coherence contract and fresh schedules including repeated stale events and changes between observation and patch, or separate protocol boundary tests. For next study, separate observation freshness from *admission* of trial; use charged evidence and no future horizon oracle. B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c29_decision_freshness.mjs`. Model body run in connector JS, no independent native CI for R1.
