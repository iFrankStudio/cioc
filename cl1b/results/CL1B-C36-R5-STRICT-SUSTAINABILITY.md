# C36-R5 — fused admission, success gating and strict sustainability

Parent 1aa54c986747144cc4710020356d4f21f9248d09; implementation 5e7b8eee339004e84d858712d262f18adae44e6a. 192 paired scenarios, 6 arms, 1152 non-native worlds, 98304 offered requests per arm. Six new seeds; 512 requests; initial300/1500; reward65/90; cap84/96; tariff1. Hidden fast, slow, mixed and sandwich regimes. Sandwich explicitly resets environment dwell at t160 and t224 (no notification to actor). Fused count charges branch/count increments within mask extraction instead of separate exact scan. Gated variant requires four consecutive accepted productions after learning before attempting install, pays policy counters; attempts once per learned coefficient change. Gated-off pays same construction and retains graph but uses static mask path. Conservative and fused emit directly, no temporary plan. All ledgers exact and storage bound enforced.

Strict criterion: complete 512 offers, no exhaustion, final credit>=16, end>=initial, positive net in last 256 requests, >=487 correct outputs (95% of all offers, not just publications). This is a finite-horizon operational test, not sufficient for indefinite life. Stress loss requires credit at t224 < credit at t160; restoration also requires completion and final credit >= t160 credit. Interpret stress metric ONLY for sandwich; other regime checkpoint losses are not induced stress.

| arm | correct | wrong | complete | sustainable | strict | net credit |
|---|---:|---:|---:|---:|---:|---:|
|probe_all|89522|0|168|168|168|1550728|
|feedback|78800|7852|166|166|56|2287930|
|feedback_compiled|77167|7070|161|156|56|2566668|
|fused_compiled|75106|6104|153|148|56|2513406|
|gated_compiled|74088|5633|145|144|56|2392059|
|gated_off|74021|5601|145|144|56|2035764|

At cap84 fused installs207 vs conservative0, but strict counts identical28/96 and completion80 vs82. Gating reduces total installations2902->1359 but loses completion and net resources. Gated vs gated-off net benefit356295, same strict56 and completion145, so resource improvement alone not additional quality-qualified viability. All feedback arms strict pass only 48 slow +8 mixed; none pass fast or sandwich. Probe-all strict168, including all48 sandwich, but less net resources. Different starting conditions and rewards from R4; not a direct temporal comparison.

| sandwich arm | measured losses | restored after loss | strict passes |
|---|---:|---:|---:|
|probe_all|24|24|48|
|feedback|24|24|0|
|feedback_compiled|32|32|0|
|fused_compiled|40|40|0|
|gated_compiled|40|40|0|
|gated_off|42|42|0|

All sandwich arms complete all48. More loss events is not superiority. Restore counts demonstrate actual observed drawdown and return to baseline, but correctness threshold excludes feedback strategies in this cohort.

## Initial native feasibility audit (source inspection, no new native execution)

Read CL-0 workspace manifest, world kernel at blob 72e8d66a1c25dad930cedd9935448a84336b803b, C26/C27 native reports. Native Op includes Const/Add/Sub/Read/Write/Branch, CloneNode, PatchNext, ReadNext, PatchConst, GraphEncode/GraphReplace, GraphDrop, SenseOwnCredits, TryDrawCompute, DrawCompute and ReturnCompute. C26/C27 already validate charged private credit sensing and source rollback with pre-authored candidates, not C36. Native CloneNode appends and enforces code/storage cap; Delete removes data cells, not executable nodes. Do NOT translate JS graph=[] retirement into free native code deletion. Graph object drop is distinct from deleting live executable nodes. Shared pool transfer is conservation, not production reward creation. Need explicit environment-side acceptance/probe and reward contract with logged provenance and resource budget; no free coefficient reads or actor-targeted hidden refills. Model comparisons >=, arithmetic counters, mask access, graph storage and dispatch must expand to actual paid CIL instructions; current abstract counts cannot be treated as native costs. First native scope should be bounded repeated hidden-coefficient changes with fixed reusable code slots, or capacity-accounted append only, static baseline and matched physical-effect suppression, predeclared independent replay. Further audit required for exact reward/event API and native test runner. No VM or CI executed this round.

Decision: prioritize reliable prepublication verification and native bounded mechanism gate rather than further tuning ad hoc installation heuristics. B3/B4 NOT_MET. No spontaneous task discovery or organizational closure shown.

Reproduce: `node cl1b/analysis/cl1b_c36_r5_strict_sustainability.mjs`.
