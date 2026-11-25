# C34-R6 — paid measured-cost cache admission
Parent b8868c912ca8d3d9d4dc77321a9d5d9b24dcb1ab. Same R5 task, 60 revealed schedules, 8 arms=480 model worlds. Preserve original shared, adaptive, FIFO, LRU. New measurement-only LRU observes actual production span between SenseOwnCredits instructions, after paid start Const patch and before publication, records per-resident measured cost in mutable Const. No extra recurrence or data cells. On global miss measure actual required production; compare incoming cost against LRU victim stored measured cost by actual paired decrement/zero branches until one exhausted; accept if incoming >= victim (empty cost0 accepted), else skip caching and retain victim. Bounded variant allows one veto between insertions (not time-based; hits leave veto bit intact), then forces next miss admission. No frequency estimate; cost is recomputation span, not net lifetime benefit. Matched off suppresses only rejection flag; same paid decision until first divergence, always inserts, same kernels/stores as measurement-only LRU. Source includes dormant paths. All instructions, including measurement, comparison and metadata updates charged. Observer checks exact span, incoming/victim metadata, actual replacement, output and recurrence input, generated node/entry, caps14 data/200 nodes/220 occupied/200000 credits, exact initial prefix. No native, no data corruption defense or arbitrary value discovery. 
| group | adaptive | FIFO | LRU | measured LRU | value | bounded | off |
|---|---:|---:|---:|---:|---:|---:|---:|
|all|420710|378998|413240|388755|-477796|-210019|-348712|
|original|310050|296400|313173|293631|-444490|-197202|-329072|
|mixed_cost|110660|82598|100067|95124|-33306|-12817|-19640|
|primary_only|55588|56894|55202|55502|55490|55466|55490|
|secondary_only|59226|60720|59010|59448|59442|59430|59442|
|alternating_streams|27292|16672|29050|25072|-95776|-112912|-99046|
|primary_heavy|37554|29904|37512|34983|-138160|-57090|-48337|
|secondary_heavy|48570|46140|49623|48612|31036|31262|10571|
|phase_shift|55102|57470|55796|55958|51960|51924|51960|
|return|53214|54520|52900|52648|-6936|40962|36490|
|no_locality|-26496|-25920|-25920|-38592|-401546|-266244|-395642|
|cost_asymmetry_primary|15346|8896|10322|8996|-63484|-69914|-61942|
|cost_asymmetry_secondary|3684|3072|10168|8842|-5244|-7824|-5676|
|scan_pollution|24894|2880|13450|11388|13314|6940|-10636|
|bursty_secondary|11486|11980|11461|11216|-4282|7347|7692|
|alternating_distinct_cost|37086|37326|36762|36862|36858|36850|36858|
|return_asymmetry|18164|18444|17904|17820|-10468|13784|14064|
Value {"net":-477796,"vsLru":-891036,"wins":29,"paid":1301174,"peak":204,"nodes":189,"reused":4212,"kernels":6102,"measured":1548,"comparePaid":864286,"measurePaid":12384,"metadataPaid":2970,"vetoes":1278}. Bounded {"net":-210019,"vsLru":-623259,"wins":36,"paid":1033397,"peak":204,"nodes":189,"reused":4264,"kernels":6423,"measured":1496,"comparePaid":575484,"measurePaid":11968,"metadataPaid":11264,"vetoes":472}. Next use measured-cost decision overhead to motivate structural/low-cost value estimates or generic VM ordering affordance only if separately justified; do not optimize threshold on revealed grid. B3/B4 NOT_MET.
Reproduce node cl1b/analysis/cl1b_c34_r6_measured_cache_value.mjs.
