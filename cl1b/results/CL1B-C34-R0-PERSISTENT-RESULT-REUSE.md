# C34-R0 — persistent cross-request result reuse
Parent 5f3ebe666df860f118e23e87f6244019342fa0c9. C33-R4 costly admission. New task one required F(a,b,k) per request, z=b then k iterations az+b; a,b1..16, k1/2/4/8 can vary across requests, coherent during each. Same paid 16-node generated DAG. Cache records (a,b,k,result) at60..63 and64..67; zero a sentinel initialized, positive domain, all three keys compared via paid reads/sub/branches, no hash/alias. On miss actual full recurrence, save result and k (coherent input), actual bias/factor, factor last. One slot replaces last computed request; two slots actual FIFO replacement via paid PatchNext after each miss, hits do not move FIFO. Learn arm one slot, actual paid hit observation, on first exact hit retire activation probe, paid PatchConst flag, compile actual hit routing. Off suppresses only flag mutation, retains probe retirement, never reuses, still computes and stores; on/off first preactivation state/instruction prefix checked. Cache contents retained across requests; no guest verification of arbitrary corrupted state, cache data trusted and no external writer, coherent request, exact arithmetic kernel and intact validator/routing. This is a different task than C33, not same performance baseline or R3 fault tolerance. No native, no new VM op. Observer verifies actual cache records against full computation and every output, actual recurrence intermediate inputs, entry/clone counts, exact hit key, capacity. Strict16 cells/200 nodes/220 occupied/200000 credits. All source includes unused branches, baseline not minimum code storage. Shared and plain bypass cache lookups and stores but pay first empty sentinel initialization. Policies, FIFO, key grammar authored; no arbitrary algorithm invention.

64 designed schedules x6 arms=384 model worlds. Scales1/2/4/8, horizons24/96, eight patterns including changing k and nonlocal recurrence of keys. All correct outputs. 
| pattern | one vs shared | two vs shared | learned one vs shared | off vs shared |
|---|---:|---:|---:|---:|
|all|48906|81008|44774|-121696|
|stable|43924|43852|42420|-20096|
|alternating|-12480|42310|-12480|-12480|
|cycle3|-12480|-16328|-12480|-12480|
|burst|38760|37096|37412|-19264|
|drift|-12480|-16328|-12480|-12480|
|rare|-11554|-15386|-11666|-12672|
|k_change|27792|22336|26624|-19648|
|mixed|-12576|-16544|-12576|-12576|
One {"net":282709,"vsShared":48906,"wins":24,"paid":403345,"peak":198,"nodes":185,"reused":1260,"kernels":9657,"stores":2580,"evictions":2516,"activation":0,"lookupPaid":42120,"storePaid":51600}. Two {"net":314811,"vsShared":81008,"wins":32,"paid":371243,"peak":206,"nodes":189,"reused":1724,"kernels":7917,"stores":2116,"evictions":1996,"activation":0,"lookupPaid":68536,"storePaid":46552}. Learn {"net":278577,"vsShared":44774,"wins":24,"paid":407477,"peak":198,"nodes":185,"reused":1260,"kernels":9657,"stores":2580,"evictions":2516,"activation":32,"lookupPaid":46252,"storePaid":51600}. No storage rent beyond capacity, no explicit reclamation. Next consider paid capacity selection and safe cache publication/provenance; test no-locality loss and exact key k invalidation before native promotion. B3/B4 NOT_MET.
Reproduce node cl1b/analysis/cl1b_c34_persistent_result_reuse.mjs.
