# C31-R3 — paid sampled production-span feedback
Parent 9299276c16a7764c345511f94b189475a44adbf1. Non-native engineering model, B3/B4 NOT_MET. R2 exact input monitoring costly. R3 measures actual paid production span rather than comparing raw input. First service direct, next (and every16 services thereafter) sampled. Actor temporarily routes result to sample_out with actual PatchNext, uses paid SenseOwnCredits before service and after saving actual result to scratch54, obtains actual charged span by subtraction. Sample result is not recomputed: preserve it while comparing prior measured reference55, then restore normal result route and emit saved result. First sample triggers independent R1 graph-path search; reset reference after each search, next sample establishes new reference (blind interval). Subsequent unequal measured span triggers search; equal span retains both existing route patches. Decisions affect following services, not the already-computed sample. Only actual production, no external input monitor or fake trial. Sixteen-service countdown and sample rules authored. Total paid costs include countdown, temporary routing, saved output, sample cost and search; no host free feedback. Same-source off suppresses only both final route patches each search; after first divergence sample costs and search histories can differ (bundled policy effect, not identical per-decision causal contrast). Observer checks normal output, actual clone path/entry at each service, actual selection, measured reference transition and exact prefix before first activation. No R10 certification or guest numerical correctness trial. Strict16/200/220/30000 caps. No new kernel ops.

24 revealed R0/R1 schedules ×6 runs =144 worlds. Baselines plain, DD, initial-only and periodic R1, feedback on/off. No blind holdout. 
| family | DD net | initial net | periodic net | feedback net | off net |
|---|---:|---:|---:|---:|---:|
|all|57256|57688|41554|43879|40578|
|left|232|-80|-3026|-2389|-2389|
|right|19432|20872|17746|18515|16763|
|conflict|9232|10144|7288|7803|6635|
|flip|8782|7870|6556|6633|5953|
|alternating|9704|9392|6446|6999|6989|
|mixed|9874|9490|6544|6318|6627|
Actual samples 114, searches 35; off searches 34. Net relative to plain. Equal total incumbent span does not imply equal relative cost of alternative orientations; offsetting pair changes can mask opportunity. New reference not set until sample after modification, so adaptation can lag. Do not interpret fewer searches as evidence of profitable adaptation. If feedback remains inferior to initial-only, stop cadence tuning and move to task families with larger avoidable cost (e.g. reusable composed kernels where loop work dominates), or pursue cost amortization via code reuse. No native promotion or general evolutionary claim.
Reproduce node cl1b/analysis/cl1b_c31_production_span_feedback.mjs.
