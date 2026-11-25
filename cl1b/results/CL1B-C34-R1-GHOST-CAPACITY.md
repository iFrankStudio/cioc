# C34-R1 — ghost-key-driven paid capacity expansion
Parent f40bb11854ef2d581d7300542fbd3dbb0f944352. Start with one complete result record. Copy evicted key only (3 actual cells, no cached value) into second slot on miss, before overwriting first. Compare complete live (a,b,k) to first record, then ghost on miss. Exact ghost match is observed capacity conflict, not predicted future benefit. Pay PatchConst activation (matched off suppresses only flag), compile second lookup and store/rotation edges; compute required result normally, store into second slot, retain first. Thereafter FIFO two slots. Off disables ghost probing/copy and remains one-slot, leaves ghost cells allocated; no second result slot. No shrink/reclamation, no storage rent; only capacity. Ghost is three quarters of second slot already, not cheap capacity discovery; permanent expansion may be wrong for cycle3. Initial duplicated code exceeded 200 nodes; compact single write path, slot address rotation uses actual PatchConst and 127 minus final slot pointer. All actual paid instructions, mutations and storage accounted. Source includes dormant paths; authored policy/threshold, no arbitrary discovery. Strict16 cells/200 nodes/220 occupied/200000 credits. Correct all outputs, actual generated entry and recurrence inputs, actual complete record and cache hit provenance. Observer checks first matched prefix and exact activation evidence; not guest corruption defense. No native.
64 revealed R0 schedules x5 arms =320 worlds. Same task and schedule, new common store implementation and route overhead, matched baselines in this study. 
| pattern | one net | two net | ghost net | off net |
|---|---:|---:|---:|---:|
|all|43746|71184|58260|3858|
|stable|43908|43804|43740|43740|
|alternating|-13440|41766|40410|-13992|
|cycle3|-13440|-18264|-22104|-22104|
|burst|38640|36448|37536|37536|
|drift|-13440|-18264|-22104|-22104|
|rare|-12498|-17306|-21018|-21018|
|k_change|27552|21480|24120|24120|
|mixed|-13536|-18480|-22320|-22320|
Ghost {"net":58260,"wins":32,"paid":393991,"peak":214,"nodes":197,"reused":1716,"kernels":7947,"expanded":8,"ghostCopies":2116}. Next investigate cheaper conflict observation or real deletion if expansion loses to fixed two-slot. B3/B4 NOT_MET.
Reproduce node cl1b/analysis/cl1b_c34_r1_ghost_capacity.mjs.
