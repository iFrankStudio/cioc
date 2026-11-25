# C33-R1 — full subprogram reuse and scale-dependent amortization
Parent 11d1b4e6e5efd15f363c4963da0b0d0b1432d8e5. R0 individual product reuse had negative economics. R1 replaces it with a real deterministic affine recurrence F(a,b,k): z=b then k times z=a*z+b. Two mandatory outputs F(a,b,k), F(c,d,k), coherent input per batch. Shared k from input58 (fixed within a world); a,b,c,d1..16,k1/2/4/8; safe integer domain. This is a different benchmark, not a directly matched improvement over R0. Each recurrence iteration executes the existing actor-generated 16-node binary multiplication DAG with live z, then adds b, decrements actual paid counter and loops; no synthetic delay or resource rewards. Plain parent loop, shared DD, static exact-match reuse and evolved exact-match reuse, matched activation suppression. First result cell11 already holds F; if both parameters equal, skip entire second recurrence and read cell11. No commutation assumed (recurrence is not symmetric). Actor observes equality, pays patch of decision to retire probe then actual gate activation; first match pays duplicate check. Rules authored, only installation endogenous. Off suppresses gate activation, retains probe retirement. No reuse across batches, no stale cache, no future information, no dynamic k change in this experiment. No semantic certificate by actor; observer checks each output, actual recurrence intermediate input at every kernel dispatch, entry addresses, clone count, exact kernel calls, continuation, safe integers and capacity. Exact initial prefix compares PC/registers/cells/current instruction/source length for every step until first activation, plus all mutations reconstructed by observer; not full code snapshot per step. Strict16 cells/200 nodes/220 occupied,200000 credit (explicit increase for real repeated work). No new kernel op.

4 scales ×2 horizons (24/96 batches) ×8 patterns ×5 arms=320 model worlds. All/half/quarter/rare/none/rise/fall/second-only mismatch; deterministic design, not blind. All production outputs checked, negative cases retained. Net vs shared includes all probes/guards/installation. 
| k | static vs shared | evolve vs shared | off vs shared | evolve wins/16 | kernel calls saved |
|---|---:|---:|---:|---:|---:|
|1|5296|5068|-4068|10|332|
|2|13716|13488|-4068|10|664|
|4|30556|30328|-4068|10|1328|
|8|64236|64008|-4068|11|2656|
| pattern | k1 | k2 | k4 | k8 |
|---|---:|---:|---:|---:|
|all|3732|6782|12882|25082|
|half|1415|2928|5954|12006|
|quarter|270|1028|2544|5576|
|rare|-803|-754|-656|-460|
|none|-840|-840|-840|-840|
|rise|1425|2948|5994|12086|
|fall|1429|2956|6010|12118|
|second_only_mismatch|-1560|-1560|-1560|-1560|
All evolve {"net":601089,"vsShared":112892,"winsShared":41,"paid":805499,"peak":146,"nodes":134,"reused":1328,"kernels":23820,"activations":48,"probeSteps":13872}. Shared {"net":488197,"vsShared":0,"winsShared":0,"paid":918391,"peak":146,"nodes":134,"reused":0,"kernels":28800,"activations":0,"probeSteps":0}. Distinguish reuse opportunity and governance costs; rare/absent reuse can remain negative even at higher work scale. If positive margin, next freeze fresh native schedules and bounded reuse certificate, or test cost-based admission with actual paired spans rather than fixed scores; no B3/B4 promotion. Non-native only.
Reproduce node cl1b/analysis/cl1b_c33_subprogram_reuse_scale.mjs.
