# C29-R3 — bounded paid experience reuse versus novel input and task-aware reference

Parent 803485b5435ed30dfba70fdb31960ee0f7fc9399. **Non-native engineering model only.** C29 native traces immutable; R2 matched freshness initial-only remains the economic comparator. No claim of B3/B4 advance.

## Design

All trial-based variants retain R2 periodic24 novelty admission, with sequential snapshot and paid paired arithmetic trials. Cache uses one or two fixed three-cell slots (keys70..72 and73..75), storing exact operand pair and validated preferred candidate bit. The actor pays for each read, comparison, branch and write. Positive input domain1..16 makes zero-filled uninitialized slots nonmatching. On miss, run both candidate trials, output agreement and freshness checks, compare actual paid spans, and only cache strict non-tie preference. On hit, use stored candidate bit and actual dispatch readback; no trial or free host lookup. With two slots, copy first slot into second when inserting new experience (FIFO, not LRU). Same-source controls suppress physical PatchNext effects and charge attempts. No future horizon input. Trial-based cache hit uses the capture rather than a repeated freshness guard: within current specified stage-boundary / between-trial environment contract there is no mid-hit intervention, but arbitrary interleaving requires a new freshness protocol. No mutation of candidate bodies in this family: exact input preference cache would need source-version invalidation if code changes. Cache records are ordinary world cells, not hidden or access-protected private memory. All products checked. No claims of learned generalization or optimal replacement.

Task-aware reference compares operand magnitudes directly with paid bounded comparator and chooses shorter loop without trials. It exploits the researcher-known algebraic relation and is not endogenous discovery. Its graph still allocates unused trial nodes; no optimized storage claim. No guard on direct comparison beyond capture, and the C29 mid-trial intervention does not fire when trials are absent; hence direct's known-case numbers are recorded as diagnostics only, not a fair identical-environment contrast. The 108 family schedules use only production-stage changes and are comparable. Direct reference uses the same admission cadence and physical-off contrast; it is not a best possible task-specific algorithm.

## Family and results

Six operand pairs, 3 horizons96/144/192, 3 change intervals18/30/48. For each combination either alternate the same two exact operand pairs or deterministic novel sequence (54 each). 108 cases ×5 variants×2 physical arms =1080 runs; additionally 16 known cases ×10 arms. Each case same external service cutoff, no actor Halt claim. Artificial balanced family, not holdout/population inference.

| policy | total net vs matched initial | total net vs periodic | trials | hits | nodes | peak occupied |
|---|---:|---:|---:|---:|---:|---:|
|initial|0|65395|216|0|177|189|
|periodic|-65395|0|936|0|202|215|
|cache1|-73164|-7769|936|0|239|255|
|cache2|-65371|24|708|114|278|297|
|direct|-20653|44742|0|0|209|217|

| policy | recurrent net vs periodic | novel net vs periodic | recurrent trial count | novel trial count |
|---|---:|---:|---:|---:|
|initial|31994|33401|108|108|
|periodic|0|0|444|492|
|cache1|-3774|-3995|444|492|
|cache2|8243|-8219|216|492|
|direct|20535|24207|0|0|

Cache2 saves 8243 paid instructions in repeated family, loses 8219 in novel family; 24 aggregate savings vs periodic is effectively negligible relative to source/storage overhead. Cache1 never hits because novelty admission already suppresses immediate repeated input, while one slot is overwritten by next strict preference. Cache2 recurrent trials444→216 (114 hits, each replaces two trials). Same aggregate physical-effect savings 63996 in periodic/cache1/cache2/direct family; cache improves trial accounting rather than the selection on these schedules. Against matched initial-only, cache2 still loses65371; direct loses20653, but is 44742 better than repeated trial baseline. No generic mechanism shown to beat simple task-aware selection. All policies remain subject to sampling delay and future input uncertainty.

Capacity correction: periodic already 202 nodes > previous200; cache1 239 nodes/255 peak, cache2 278/297 peak, both exceed C29 200-node/220-total limits. Maximum occupied cells at measured peaks 15 and18 respectively (peak minus source nodes and strict process header); two-slot graph also exceeds 16-cell limit. These are model-accounted footprints, not native validation. Do not raise capacity merely to claim success. No scalarized compute/storage fitness.

## Research decision

Do not promote this uncompressed FIFO implementation. The mechanism shows conditional value of remembered exact preference, but not sufficient lifetime return. Next investigate compressed cache entries / reuse policies and task-independent validation of learned cost estimates on workload families where cheap hand-authored selection is not available. For meaningful open-ended progress, distinguish (i) choosing authored candidates (already demonstrated), (ii) learning reusable input→cost relations, (iii) inventing new executable candidates and verifying their correctness. Avoid more tuning of exact-input cache on the same multiplication task unless it improves an explicitly measured resource tradeoff. Use untouched family and matched task-aware references where appropriate. B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c29_experience_cache.mjs`. Model body executed in connector JS; exact aggregate assertions included. No new Rust CI claimed.
