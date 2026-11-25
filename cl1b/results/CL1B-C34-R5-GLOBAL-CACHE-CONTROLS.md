# C34-R5 — stronger global cache controls
Parent b78358021d6d48838338095f2b553935858d6a97. R4 had fixed allocation baselines but lacked unrestricted global cache. Reuse R4 original actors unchanged and add exact same task/14 data cells/200 nodes/220 occupied/200000 credits global FIFO and LRU with same packed key, actual paid lookups, store destination mutable Const and rotation (LRU additionally patches victim on hit). Both slots global, no primary reservation, overwrite old key/value rather than unnecessary Delete. Transfer counts on global arms mean replacement across prior owner, not Delete; R4 counts actual deletion and allocation. Add global one-key ghost admission FIFO/LRU: on a complete miss, if equal to last rejected key, admit; otherwise remember key and compute uncached. Hits do not change ghost; successful admit leaves ghost key. All paid including PatchConst flag, store gate edits and lookup. Off suppresses every flag activation, never stores; exact prefix to first activation, later histories differ. This is a generic repeated-reference admission heuristic, not a measured-resource value policy. Independent observer verifies each output, exact key, domain exhaustiveness 2048 tuples, recurrence intermediate inputs, actual hit/miss and victim versus policy, actual generated nodes/entry, storage and paid cost. Different source layout, same task/caps; code size differences reported, no artificially padded baseline. No native.
48 original revealed R4 cases and 12 new authored mixed-cost schedules (not blind), 9 arms=540 model worlds. All outputs correct. 
| group | primary2 | split | R4 adaptive | global FIFO | global LRU | ghost FIFO | ghost LRU | ghost LRU off |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
|all|200928|186745|420710|378998|413240|83509|52748|-252130|
|original|122480|120763|310050|296400|313173|85903|65918|-202371|
|mixed_cost|78448|65982|110660|82598|100067|-2394|-13170|-49759|
|primary_only|56552|-24192|55588|56894|55202|-24192|-24192|-24192|
|secondary_only|-17280|60144|59226|60720|59010|59760|58068|-27042|
|alternating_streams|31336|31312|27292|16672|29050|14650|-484|-25152|
|primary_heavy|48277|-7017|37554|29904|37512|-24192|-24192|-24192|
|secondary_heavy|-10821|50820|48570|46140|49623|50373|48870|-26427|
|phase_shift|18728|17520|55102|57470|55796|16848|16020|-25602|
|return|18728|17520|53214|54520|52900|16848|16020|-25572|
|no_locality|-23040|-25344|-26496|-25920|-25920|-24192|-24192|-24192|
|cost_asymmetry_primary|22494|11076|15346|8896|10322|8402|-4968|-8384|
|cost_asymmetry_secondary|-508|10922|3684|3072|10168|2200|5238|-8384|
|scan_pollution|26078|2738|24894|2880|13450|-8064|-8064|-8064|
|bursty_secondary|-336|6254|11486|11980|11461|5796|5628|-8339|
|alternating_distinct_cost|15538|37612|37086|37326|36762|-8064|-8064|-8064|
|return_asymmetry|15182|-2620|18164|18444|17904|-2664|-2940|-8524|
Global LRU {"net":413240,"wins":50,"paid":410138,"peak":183,"nodes":168,"pHits":2259,"sHits":2186,"transfers":257,"admissions":0,"cellTime":79276}. Ghost LRU {"net":52748,"wins":26,"paid":770630,"peak":183,"nodes":168,"pHits":20,"sHits":2055,"transfers":10,"admissions":348,"cellTime":65652}. R4 {"net":420710,"wins":50,"paid":402668,"peak":200,"nodes":185,"pHits":2468,"sHits":1969,"transfers":505,"admissions":559,"cellTime":77976}. Interpret only relative to actual stronger baselines; do not promote organizational closure from advantage against fixed allocation. Next cost-aware admission/retention should use measured resource contribution and compare against these global controls, not solely R4. No native, B3/B4 NOT_MET.
Reproduce node cl1b/analysis/cl1b_c34_r5_global_cache_controls.mjs.
