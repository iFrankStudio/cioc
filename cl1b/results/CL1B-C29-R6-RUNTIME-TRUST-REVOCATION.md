# C29-R6 — ongoing paid residual audits and sticky trust revocation

Parent 31f8042409929d132009eaf637652928e512a8ee. **Non-native learning component model only**; no native execution, production service or source-edit claim. Original C29 evidence unchanged. B3/B4 NOT_MET.

## Mechanism and controls

Keep R5 anchor training, exact two-point validation and fallback. After validation, reuse ordinary cell47 (phase/countdown) rather than allocate new data. For period k, initialize it to 4+k. On each accepted prediction, the actor pays to distinguish validation phases3/4, decrement the runtime countdown, and either emit prediction or run actual paired trials at countdown5. At a runtime audit, compare measured cost difference with stored prediction in cell53 using paid arithmetic/branch. If residual nonzero, clear sticky trust cell55, emit measured result and use actual paired trials on all later inputs. If residual zero, reset countdown. Trust-zero fallback never compares against stale prediction. No external knowledge of profile, failure region, batch length or reward. On audit query the emitted result is the measured one, including on revocation. Observer checks products of every executed trial and all emitted differences against a matched always-paired run. No physical source patch in this component. Profile-specific B bodies are authored, not discovered; no adaptive probe selection or trust restoration.

Profiles cross and unroll retained. Replace prior switch8 by switch5/switch11/switch16, distinct candidate branches that execute a different mathematically correct multiplication loop when x equals the indicated value. Fixed anchors and validation points do not include these new failure regions. Twelve selected 64-query schedules (ascending, descending, modular permutation, four shifts each), disjoint from anchors and validation. Deterministic family designed before running these variants in this turn, not blinded or representative real workloads. Repeated training per world. All environments held coherent per query. Five controls/strategies: always-paired, initial validation only, audit each 4/8/16 accepted queries. Same test input sequence per profile/control. Total 5 profiles ×12 sequences ×5 policies=300 modeled worlds, 19200 outputs, all instructions charged. Model summary tracks both exact errors and sign/tie errors. Cost baseline always-paired has no training. No claim that cheaper wrong outputs are valid economic gains.

## Results

| profile | policy | exact | sign correct | paid CIL | net vs paired | runtime audits | revocations | nodes | peak occupied |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
|cross|paired_0|768/768|768/768|93108|0|0|0/12|185|198|
|cross|validated_0|768/768|768/768|67728|25380|0|0/12|191|208|
|cross|validated_4|768/768|768/768|96783|-3675|192|0/12|213|230|
|cross|validated_8|768/768|768/768|85647|7461|96|0/12|213|230|
|cross|validated_16|768/768|768/768|80166|12942|48|0/12|213|230|
|unroll|paired_0|768/768|768/768|91464|0|0|0/12|191|204|
|unroll|validated_0|768/768|768/768|96972|-5508|0|0/12|197|214|
|unroll|validated_4|768/768|768/768|99312|-7848|0|0/12|219|236|
|unroll|validated_8|768/768|768/768|99312|-7848|0|0/12|219|236|
|unroll|validated_16|768/768|768/768|99312|-7848|0|0/12|219|236|
|switch5|paired_0|768/768|768/768|96516|0|0|0/12|194|207|
|switch5|validated_0|726/768|758/768|68028|28488|0|0/12|200|217|
|switch5|validated_4|744/768|762/768|100503|-3987|127|5/12|222|239|
|switch5|validated_8|726/768|758/768|86427|10089|96|0/12|222|239|
|switch5|validated_16|726/768|758/768|80706|15810|48|0/12|222|239|
|switch11|paired_0|768/768|768/768|97284|0|0|0/12|194|207|
|switch11|validated_0|726/768|742/768|68028|29256|0|0/12|200|217|
|switch11|validated_4|729/768|744/768|98505|-1221|178|1/12|222|239|
|switch11|validated_8|729/768|744/768|87822|9462|89|1/12|222|239|
|switch11|validated_16|726/768|742/768|80706|16578|48|0/12|222|239|
|switch16|paired_0|768/768|768/768|97956|0|0|0/12|194|207|
|switch16|validated_0|726/768|728/768|68028|29928|0|0/12|200|217|
|switch16|validated_4|744/768|745/768|100350|-2394|131|5/12|222|239|
|switch16|validated_8|741/768|742/768|91167|6789|72|4/12|222|239|
|switch16|validated_16|741/768|742/768|86904|11052|36|4/12|222|239|

Across 36 branch-specific failure worlds, initial validation leaves126 wrong differences, audit4 leaves87 and revokes11, audit8 leaves108 and revokes5, audit16 leaves111 and revokes4. These totals include errors before detection and in worlds never detected. The deterministic cadence aliases with input schedules: switch5 audit8/16 never revokes, although wrong outputs continue; switch11 audit16 never revokes. More frequent audits do not establish correctness of unsampled queries. Cross is exact for all policies; audit4 costs3675 more than always-paired, audit8 saves7461, audit16 saves12942, initial validation only saves25380. Unroll is already rejected at initialization; extra audit control still costs instructions (phase dispatch) and all audited policies lose7848 relative to always-paired. Branch-specific net costs cannot be counted as successful adaptation when outputs are wrong.

Capacity: audited graphs213/219/222 nodes and peak230/236/239 exceed previous200-node and220-total limits, although data cells remain16. Do not promote or silently raise limits. No native CI claimed. This is a negative result for both fixed audit coverage and current implementation's capacity.

## Decision

Separate selecting *where* to validate from merely selecting *when*. Test low-cost coverage-aware input selection (including missed input classes), using only paid observations and a bounded state budget. Do not use a researcher-provided failure region, free hash, hidden scheduler entropy, or future horizon. First compact the shared prediction/capture/fallback control and compare exact traces before/after; retain an always-paired exact control. New schedules must contain clustered/rare cases, repeated input classes and adversarially aligned sequences. Measure error before detection, audit cost, false confidence and storage; do not interpret finite testing as proof of arbitrary control-flow correctness. Only consider native promotion after fitting declared limits and demonstrating nontrivial reliable economy. B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c29_runtime_trust_revocation.mjs`. The model body executed in connector JS twice, exact summary assertions retained. No native CI for R6.
