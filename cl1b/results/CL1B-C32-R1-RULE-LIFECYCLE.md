# C32-R1 — sampled admission and miss-budget rule dormancy
Parent cc7b3383b8ab0f3038a704e84bb1ac06e44f2c1a. Non-native model only. B3/B4 NOT_MET. R0 permanently installs a rule on first match and probes unresolved rules on every service. R1 pays six-instruction service countdown, wakes every32 services (including first), probes only uninstalled/dormant rules. On match, initialize budget4, retire probe and independently patch actual gate. In lifecycle arm, each actual guard miss decrements corresponding remaining budget (paid); at zero restore probe edge and disable actual guard, then allow future scheduled wake to reactivate. Hits do NOT replenish budget: cumulative misses since last install, not consecutive misses or measured profitability. x precedence means y does not observe when x matches. No deletion of dormant code; only edge deactivation, no storage reclamation. All checks, countdown, patches, storage paid; no external event oracle or future schedule. Sampled-only arm same layout and wake schedule without miss path (initial counter write still paid). Off same lifecycle source suppresses only two activation PatchNext effects; probe retire still real, thus later histories differ and no deactivations in off. Exact initial prefix before first physical activation; total policy effect not same per-event comparison. R0 shared DAG and authored factor bodies intact; observer checks each output, actual generated entry, clone steps and applicability. No guest semantic certificate. Fixed32 cadence and4 miss budget authored, not learned, no general correctness or B3 claim. Strict16 cells/200 nodes/220 occupied/50000 credits.

36 revealed R0 schedules plus 2 newly designed return schedules ×7 arms=266 model worlds (not blind). Plain, shared DD, static both, R0 eager, R1 sampled-only, R1 lifecycle, lifecycle off. Return cases deliberately contain intervals of no match followed by restored equality to expose real deactivate/reactivate. All normal outputs exact. 
| family | static both vs shared | R0 vs shared | sampled vs shared | life vs shared | off vs shared |
|---|---:|---:|---:|---:|---:|
|all|22680|9580|5891|6379|-34121|
|revealed|21880|10610|6285|6726|-31279|
|x_stable|6259|4334|3712|3712|-3475|
|y_stable|3738|2733|3099|3174|-3475|
|none|-5568|-6496|-3485|-3485|-3485|
|both|6287|5271|3768|3768|-3447|
|x_to_y|6264|4088|3559|4499|-3473|
|y_to_x|6264|5248|4711|4701|-3473|
|rare|-5464|-6456|-4549|-3614|-3492|
|alternating|6264|5088|-914|-2450|-3466|
|mixed|-2164|-3200|-3616|-3579|-3493|
|return|800|-1030|-394|-347|-2842|
All lifecycle {"net":198254,"vsShared":6379,"winsShared":18,"paid":249369,"peak":192,"nodes":181,"factored":2605,"kernels":6515,"activations":58,"deactivations":31,"wakes":147}. Sampled {"net":197766,"vsShared":5891,"winsShared":18,"paid":249857,"peak":192,"nodes":181,"factored":2771,"kernels":6349,"activations":43,"deactivations":0,"wakes":147}. Reactivations 15; actual records in JSON. If cost remains dominant, do not assume fixed miss budgets learn economics; seek a smaller mechanism with a directly measured benefit/cost rather than tune cadence. Need richer tasks or larger reusable kernels to amortize management cost. No native run. B3/B4 NOT_MET.
Reproduce node cl1b/analysis/cl1b_c32_rule_lifecycle.mjs.
