# C34-R7 — low-cost structural value tier
Parent 6fa145989d57521f4f440bde84c821ff441d5aa5. R6 exact comparison loops dominated. Same 60 revealed cases and caps14 cells/200 nodes/220 occupied/200000 credits; 8 arms=480 model worlds. No new op. Replace measured span with authored structural proxy based on actual request k: 1/2 => low1, 4/8 => high2; empty0. Guest paid reads, equality branches and PatchConst. Reject only low incoming vs high LRU victim; constant bounded comparison, no subtract loop. Metadata-only LRU and matched reject suppression, plus bounded one veto between insertions; same policies as R6 apart from value proxy. Recompute required output before admission. No actual measured cost or frequency estimate, k proxy ignores DAG depth and seed cost, not learned. Observer verifies tier from request domain, actual victim, exact output, cache record, intermediate recurrence and generated path, capacities, prefix up to first reject. Off always admits, same kernels/stores as metadata-only LRU. All normal outputs exact. No native.
| group | adaptive | FIFO | LRU | metadata LRU | tier | bounded | off |
|---|---:|---:|---:|---:|---:|---:|---:|
|all|420710|378998|413240|384941|384999|383373|379203|
|original|310050|296400|313173|290481|286051|283951|286051|
|mixed_cost|110660|82598|100067|94460|98948|99422|93152|
|primary_only|55588|56894|55202|55466|55430|55406|55430|
|secondary_only|59226|60720|59010|59430|59412|59400|59412|
|alternating_streams|27292|16672|29050|24478|23636|23240|23636|
|primary_heavy|37554|29904|37512|34578|34009|33739|34009|
|secondary_heavy|48570|46140|49623|48405|48122|47984|48122|
|phase_shift|55102|57470|55796|55904|55842|55806|55842|
|return|53214|54520|52900|52540|52400|52328|52400|
|no_locality|-26496|-25920|-25920|-40320|-42800|-43952|-42800|
|cost_asymmetry_primary|15346|8896|10322|8738|8416|8284|8416|
|cost_asymmetry_secondary|3684|3072|10168|8770|8572|8440|8572|
|scan_pollution|24894|2880|13450|11140|24140|17700|10524|
|bursty_secondary|11486|11980|11461|11182|8822|10740|11090|
|alternating_distinct_cost|37086|37326|36762|36846|36834|36826|36834|
|return_asymmetry|18164|18444|17904|17784|12164|17432|17716|
Tier {"net":384999,"vsLru":-28241,"wins":50,"paid":438379,"peak":208,"nodes":193,"reused":4360,"kernels":5460,"classified":1400,"comparePaid":5996,"classPaid":14940,"metadataPaid":13244,"vetoes":196}. Bounded {"net":383373,"vsLru":-29867,"wins":50,"paid":440005,"peak":208,"nodes":193,"reused":4460,"kernels":5563,"classified":1300,"comparePaid":5281,"classPaid":14127,"metadataPaid":13937,"vetoes":33}. Same revealed workloads, not prospective. A positive physical effect over off would not establish net superiority over LRU. Next prefer low-cost structural specialization/admission only if empirically beneficial; otherwise isolate frequency/cost tradeoff and review whether generic arithmetic ordering affordance is independently justified. B3/B4 NOT_MET.
Reproduce node cl1b/analysis/cl1b_c34_r7_structural_value_tier.mjs.
