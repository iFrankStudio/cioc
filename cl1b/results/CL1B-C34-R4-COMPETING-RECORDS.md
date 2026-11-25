# C34-R4 — competing information-record allocation
Parent b64b8c9f8ba5a22de2977acbaeb57aa56fc09f5a. Two required activities share the same individual and generated multiplication DAG, but distinct recurrence semantics: primary seed b, secondary seed0, both k iterations az+b. Stream input57 is request kind, not resource oracle. Exact packed key (32a+b)*32+2k+t, disjoint namespaces and injective in domain. Tighten data cap to14 (4 input +4 scratch +1 key temp +1 output +4 record cells), 200 executable nodes/220 total, 200000 credits. Third full record impossible under cap. Primary slot60/61 reserved; contested64/65 can store either activity. Baselines no-cache shared, primary-only two-slot FIFO (secondary always computed), fixed split (one each), adaptive and matched off. Adaptive compares primary slot and contested exact key, then remembers evicted primary key in mutable Const; secondary compares contested exact key and retains last uncached secondary key in separate mutable Const. Conflict hit on either ghost authorizes paid flag mutation, Delete both contested cells (if allocated), full required computation and new key/result write to contested slot; primary default writes reserved slot, secondary default computes uncached. Both actual reclamation and reallocation, no free host decisions, no simultaneous third record. Repeated admission uses same flag; off suppresses every matching activation, never allocates contested record, so subsequent histories differ; exact prefirst-exposure prefix. Deleting empty cells paid, actual deleted count distinct. Transfer means actual previous contested owner differs from new owner, observer verifies admission. Ghost observations are authored and do not measure future value; no global optimality, no rent or arbitrary data corruption defense. Fixed split deletes before secondary replacement too; static policies not minimal code storage. Model observer checks every output, intermediate recurrence input, actual record/key/clone/entry, strict capacity. No native.
48 designed schedules x5 arms=240 worlds, scales1/4/8, horizons48/144, eight patterns; no blind holdout. 
| pattern | primary two | fixed split | adaptive | off |
|---|---:|---:|---:|---:|
|all|122480|120763|310050|-199385|
|primary_only|56552|-24192|55588|-35040|
|secondary_only|-17280|60144|59226|-25884|
|alternating_streams|31336|31312|27292|-284|
|primary_heavy|48277|-7017|37554|-24093|
|secondary_heavy|-10821|50820|48570|-26796|
|phase_shift|18728|17520|55102|-30396|
|return|18728|17520|53214|-30396|
|no_locality|-23040|-25344|-26496|-26496|
Adaptive {"net":310050,"wins":38,"paid":328706,"peak":200,"nodes":185,"pHits":1839,"sHits":1674,"transfers":369,"deleted":738,"admissions":411,"cellTime":61932}. Bidirectional owner transfer worlds 21. Need compare actual value-aware replacement and stronger shared FIFO baseline; no claim of organization closure, B3/B4 NOT_MET.
Reproduce node cl1b/analysis/cl1b_c34_r4_competing_records.mjs.
