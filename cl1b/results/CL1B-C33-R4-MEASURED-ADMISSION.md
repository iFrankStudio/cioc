# C33-R4 — measured production-span admission
Parent cbb164371faefdcf0f864de1db9abfa6180a6eea. R3 fused guest-side provenance remains intact. On first exact input match, do NOT reuse yet: execute the required second recurrence normally, measure its actual paid span using two SenseOwnCredits operations, storing start59. Span includes setup, recurrence and return routing, not hypothetical saving or actual counterfactual comparison. Then paid bounded decrement comparison against authored48 or96; admit iff measured cost strictly exceeds threshold. On admission retire probe and PatchConst flag followed by compile of gate, but publish already computed result from state41. On rejection retire probe permanently, leave gate to second recurrence. No additional recurrence trial, no free observer decision; all setup/measurement/comparison/control edits charged. Later actual reuse subject to R3 provenance checks. No estimate of future hit frequency, no retry after rejection. The threshold is not learned and is not equivalent to actual future net benefit. Paired on/off flag suppression, same measured decision; initial prefix checked. Observer checks exact measured credit span, threshold decision, outputs, actual generated entry and clone counts. Strict16 cells/200 nodes/220 occupied/200000 credits. No native; R3 fault coverage not re-run for R4, no expanded trust claim.

16 revealed R1 cases plus32 designed grid cases (4 scales, 2 horizons, dense/rare/none/fall), 6 arms: shared, R3 eager, 48 and96 admission, matched off each=288 model worlds. All outputs exact. 
| cohort | R3 net | threshold48 | threshold96 | off48 | off96 |
|---|---:|---:|---:|---:|---:|
|all|24871|13634|6444|-18560|-23105|
|calibration|1068|543|-180|-974|-1163|
|prospective|7354|4165|1956|-3814|-5298|
|designed_grid|16449|8926|4668|-13772|-16644|
|dense|21798|18464|14896|-2154|-3100|
|rare|-4692|-7072|-6752|-4472|-5452|
|none|-5160|-5160|-5160|-5160|-5160|
|fall|4335|2526|1516|-2154|-3100|
Threshold48 {"net":13634,"wins":18,"paid":505429,"measured":38,"admitted":32,"comparePaid":7280,"reused":392,"peak":184,"nodes":171}; threshold96 {"net":6444,"wins":13,"paid":512619,"measured":38,"admitted":19,"comparePaid":11890,"reused":241,"peak":184,"nodes":171}. Separate actual cost comparison instructions from total economics; fixed threshold may reject useful cheap-but-frequent reuse or admit expensive-but-rare reuse. Do not optimize threshold on same revealed grid. If underperforms eager, pursue coarse structural admission/low-cost repeated-hit evidence or cross-batch reuse, not per-step metering. B3/B4 NOT_MET.
Reproduce node cl1b/analysis/cl1b_c33_r4_measured_admission.mjs.
