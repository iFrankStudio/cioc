# C36-R1 — horizon-blind online amortization

Parent 4dc6feda3fc52af85e955cb1b2218360cacd3795; implementation 9da41240a495424289180885ff33d4eb9ca8d590. 128 paired scenarios (8 seeds, 4 temporal regimes, capacities 80/96, install tariffs 1/4), four arms, 512 abstract operation-model worlds. 12 announced epochs, coefficients learned via two identical paid basis observations per epoch. No policy knows future horizon. Static uses tighter packed mask-indexed direct paths and precomputed x+y; all dynamic graphs use same arithmetic. Always installs immediately if capacity allows; delay4 waits four requests; adaptive waits until accumulated hypothetical static-to-compiled savings cover estimated construction and retirement cost. Count paid metadata, policy, learning, dispatch, storage writes, install/retire and arithmetic; close with charged retirement. Capacity 72 abandoned before successful runs because adaptive boot+state+mask footprint is 74; 80 is the constrained condition. All 18408 requests per arm correct.

| arm | paid | best worlds | installs | capacity denials | peak |
|---|---:|---:|---:|---:|---:|
|static|728512|36|0|0|57|
|always|629298|88|1248|0|77|
|delay4|701189|4|398|450|83|
|adaptive|715401|0|330|592|85|

| regime | static | always | delay4 | adaptive |
|---|---:|---:|---:|---:|
|short|42192|67926|56752|56554|
|long|457516|355682|417018|426315|
|mixed|175864|159316|174154|177898|
|sticky|52940|46374|53265|54634|

Adaptive avoids all short-regime installs but pays decision/estimation cost; it delays profitable installation in long runs. At cap80, most adaptive admissions fail due to modeled controller footprint. At cap96 no admissions fail. No future information supplied: policy observes masks, tariffs, capacity, prior requests only; production correctness scored externally. No general guarantee: elapsed savings are sunk, not proof of future amortization.

## Matched footprint sensitivity

Re-executed all 512 worlds with base=56,state=6 for ALL arms, holding modeled initial footprint equal; same workload and arithmetic. Reproduction: replace the line beginning `const base={static:42` in the implementation with `const base=56,state=6;` (then run the script).

| arm | paid | best worlds | installs | denials |
|---|---:|---:|---:|---:|
|static|728512|80|0|0|
|always|697821|45|656|592|
|delay4|709543|3|328|592|
|adaptive|715401|0|330|592|

The adaptive rule remains nonwinning after matching footprint. Always advantage shrinks; static dominates constrained capacity. At cap96 and tariff1 adaptive 160762 vs always153054 vs static182128; tariff4 adaptive170844 vs always161595. This sensitivity reserves unused space for static and smaller policies rather than claiming their natural footprint is equal.

## Interpretation and next

Negative evidence for this specific authored online heuristic, not proof that self-directed modification cannot pay. Past dwell is not expected residual dwell; policy overhead and irreversible opportunity cost matter. Next test a cheap online predictor learned only from completed regime durations, versus fixed thresholds and an explicitly offline oracle, with held-out regime switches, matched capacity and charged memory/decisions. Consider no-construction policy when estimate/admission cannot amortize. Only then proceed to unannounced task changes and finite credit replenishment. Costs and boot sizes are abstract, host implementation not guest CIL; no native VM, CI, spontaneous task discovery or B3/B4 promotion.

Reproduce: `node cl1b/analysis/cl1b_c36_r1_online_amortization.mjs`.
