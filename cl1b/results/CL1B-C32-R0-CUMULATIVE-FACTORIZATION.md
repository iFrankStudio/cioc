# C32-R0 — cumulative guarded factorization of shared generated kernels
Parent 17a722ed5bb8f926bda925082b577c29669fcf26. C31-R0..R3 found monitoring/search costs exceed small direction-selection gains. New C32 tests a larger structural transformation: for x1=x2, x1*(y1+y2); for y1=y2, y1*(x1+x2). Otherwise original two products. Four input values1..16, aggregated multiplicand may be2..32 but DAG index remains1..16. Same paid cloned 16-node binary arithmetic DAG reused; no new kernel op. Factor transformation bodies and equality guards are authored, **not generated or discovered**, only independently installed by paid actor mutations. On first actual equality of each axis, paid probe retires itself (PatchNext) and independently installs corresponding guard (distinct PatchNext). Both guards persist; x has authored precedence when both match. Guard fallback preserves correct service when input changes. No future workload or external event oracle. Phase=1 and partial=0 reuse original sum/output continuation. Both transforms may coexist and second installation preserves first. No per-service numeric trial or R10 certificate; observer checks every service, actual entry and clone count, kernel call count and exact applicability, no incorrect results observed. Trusted grammar/guards, not arbitrary mutation proof. Strict16 cells/200 executable nodes/220 storage, 50000 private credits (explicitly larger for long workloads). Same-source off suppresses only final two guard activation effects; probe retirement still paid and real, identical first prefix and identical observed activation events ignoring clock/physical suppression. Subsequent timings diverge. Plain parent loop and shared generated DD bypass guards; x/y/both statically installed comparators (no probe). Unused authored source included in all arms; not minimal source storage baseline.

36 designed schedules ×7 arms=252 non-native worlds. Horizons16/64/128/256, nine equality-pattern families, not blind or population claim. Full economic cost includes generation, probes, installation, runtime guards and fallback. Distinguish kernel calls avoided from actual net savings.
| family | shared net vs plain | x vs shared | y vs shared | both vs shared | evolve vs shared | off vs shared | evolve physical |
|---|---:|---:|---:|---:|---:|---:|---:|
|all|174667|7342|8431|21880|10610|-30022|40632|
|x_stable|15784|6259|-2352|6259|4334|-2853|7187|
|y_stable|14937|-2303|5764|3738|2733|-2853|5586|
|none|15872|-3248|-3248|-5568|-6496|-6496|0|
|both|15896|6287|5841|6287|5271|-1944|7215|
|x_to_y|26776|1624|2552|6264|4088|-3104|7192|
|y_to_x|26776|1624|2552|6264|5248|-3104|8352|
|rare|15948|-3164|-3248|-5464|-6456|-5380|-1076|
|alternating|26776|1624|2552|6264|5088|-2104|7192|
|mixed|15902|-1361|-1982|-2164|-3200|-2184|-1016|
Evolve: {"net":185277,"vsShared":10610,"winsShared":24,"paid":223650,"peak":148,"nodes":140,"factored":2933,"kernels":5419,"activations":58,"probeSteps":21670}. Shared: {"net":174667,"vsShared":0,"winsShared":0,"paid":234260,"peak":148,"nodes":140,"factored":0,"kernels":8352,"activations":0,"probeSteps":0}. Distinct-time two-stage installations in 22 schedules. All outputs checked, negative economic results retained. Next examine amortization and selective admission of rules; if reduction in kernel calls not enough due to guard/probe tax, test more expensive repeated kernels or more compact guard integration rather than claiming successful open-ended evolution. B3/B4 NOT_MET, no native run.
Reproduce node cl1b/analysis/cl1b_c32_cumulative_factorization.mjs.
