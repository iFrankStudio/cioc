# C33-R0 — cross-task common-subexpression reuse
Parent 7f343aefffa5ad63534caceadd2efd5295e5029c. C32 found rule-local per-hit ledger destroyed economics; shift to structural reuse between multiple outputs. Batch inputs a,b,c,d,e,f,g,h (1..16), outputs a*b+c*d and e*f+g*h. Both are always required, coherent input for whole batch; only after second output does environment change. Four products share paid generated 16-node binary DAG, actual return continuation patched before every invocation. First product stays in existing scratch41 after first dot product emitted; if direct equality (a=e,b=f) or commuted equality (a=f,b=e) then reuse it as second dot product's first term and skip an actual kernel invocation. No extra cache cells, no persistence across batches, no stale use, no future input. Actor detects direct and commuted equality independently; first match pays probe retirement and guard activation, both persist with direct precedence. Rules and implementations authored, not discovered; only installation endogenous. Off suppresses only activation effects, not paid probe retirement. Static direct/swap/both arms and shared DD baseline; plain parent loop same call/return pipeline. All actual clones/edges/entry/continuation/output/call counts checked by observer; no actor semantic certificate, no native. Strict16 cells/200 executable nodes/220 occupied,100000 credits (larger explicit budget for 4 products per batch). No added world op. Unused authored source included; not minimal storage baseline.

36 designed schedules ×7 arms=252 worlds; 16/64/128/256 batches, nine relation families, input values change every batch. Not blind. Observer checks first and second output independently, actual skip count and domain. Matched prefix snapshots captured only for first batch (if first exposure later, this is NOT an exact full preactivation proof); activation event identity ignoring clock/suppression checked. After divergence histories differ. No native promotion.
| family | shared net vs plain | direct vs shared | swap vs shared | both vs shared | evolve vs shared | off vs shared |
|---|---:|---:|---:|---:|---:|---:|
|all|283119|-6765|-10078|-4852|-18580|-36816|
|direct|31300|3451|-3712|3451|-333|-4712|
|swap|31300|-3712|2987|203|-797|-4712|
|none|31300|-3712|-4060|-6844|-7772|-7772|
|both|31300|3451|2987|3451|2379|-2000|
|d_to_s|31345|-129|-361|1830|-634|-3392|
|s_to_d|31255|-132|-364|1824|752|-3392|
|rare|31285|-3647|-4060|-6755|-7779|-6452|
|alternating|32605|-87|-319|1914|650|-2192|
|mixed|31429|-2248|-3176|-3926|-5046|-2192|
Evolve {"net":264539,"vsShared":-18580,"winsShared":10,"paid":413584,"peak":182,"nodes":170,"reused":2937,"kernels":13767,"activations":52,"probeSteps":28464}. Two-stage schedules 16. If guard/probe cost dominates, quantify break-even with larger batches or expensive repeated subexpressions; do not assume reuse is profitable. Next investigate fused equality dispatch/probe (avoid duplicate checking), and coarse actual-cost trial before installation only if opportunity justifies overhead. B3/B4 NOT_MET.
Reproduce node cl1b/analysis/cl1b_c33_cross_task_reuse.mjs.
