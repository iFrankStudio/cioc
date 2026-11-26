# C36-R0 — runtime synthesis with unknown coefficient epochs

Parent a8fc4e4b7b23675f756c54d07ef42f2e3a95ce35; experiment 84047fbc6e87e69345eb18c8faa17a3ed111f76b. 48 paired scenarios, 5 arms, 240 non-native operation-model worlds. Eight announced epochs, coefficients a,b in 0..63 hidden until two paid labeled basis observations; signed held-out production inputs. Task family a*x+b*y known at boot, concrete coefficients and operation recipe not. Same observations across arms. Shared bit extraction; fixed generic loop, unrolled static flags, recipe interpreter, runtime compiled chain, and installation-off recipe interpreter. Explicit metered observations, arithmetic, control, data, synthesis, installation and retirement; 96 modeled storage units, four registers. No production answer fed to learner. All 8160 requests per arm correct.

| arm | paid | setup | peak occupied |
|---|---:|---:|---:|
|generic|569114|14158|42|
|plan_interp|605336|27120|63|
|compiled|233909|37103|80|
|compiled_off|612644|34428|65|
|static_table|467076|14158|68|

| requests/epoch | regime | compiled paid | static_table paid |
|---:|---|---:|---:|
|1|drift|8499|5396|
|1|stable|2976|3459|
|4|drift|12210|13457|
|4|stable|6285|11364|
|16|drift|26892|45682|
|16|stable|21140|43759|
|64|drift|80793|173167|
|64|stable|75114|170792|

Compiled beats all three implemented static controls in 42/48 paired scenarios; frequent coefficient change with one request/epoch is the six-world losing group. Savings derive from runtime specialization/amortization, not spontaneous evolution. Static code footprint and primitive charges are authored model assumptions, not native measurements. Host JS implements the learner/compiler with explicit abstract charges rather than guest-executed CIL; compiled dispatch/termination is modeled as VM overhead, and installation-off does not retain physical graph storage. This is not an equal-storage causal VM ablation. Task family, training samples and epoch signals are supplied; no unknown task family discovery, no fault survival, no resource replenishment or native CI. No B3/B4 promotion. Next: validate cost/storage sensitivity and a tighter static baseline, guest-executed specialization, adaptive compile-vs-interpret selection without known future horizon, then unannounced task changes. No claim of universal superiority.

Reproduce: `node cl1b/analysis/cl1b_c36_r0_unknown_coefficient_epochs.mjs`.
