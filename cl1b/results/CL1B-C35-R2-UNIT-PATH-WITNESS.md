# C35-R2 — paid known-answer unit-path witness

Parent 5d2aaba1679c66c0845f9280b04e30c2d970203f. Actual non-native model run, 25 schedules x 7 arms = 175 worlds, 1512 requests per arm. Instead of independent repeated-add reference, execute first generated path with y=1; compare result against x using paid guest instructions. Paired production difference check remains. Unit probe shares the generated graph and is not structurally independent. Every-request arms omit redundant countdown (including parent1 for fair comparison). Repeat injection of unchanged edge recorded as noChange. 12 original, 12 seeded held-out earlier-edge reroute schedules (4 scheduled events each, seed 1..12), one nonrepairable opcode corruption.

| arm | original paid | original wrong | holdout paid | holdout wrong | opcode paid | opcode wrong |
|---|---:|---:|---:|---:|---:|---:|
|direct|57330|0|62808|0|1129|0|
|reactive|46891|114|55731|40|1417|20|
|parent1|73726|0|85580|0|6237|20|
|unit1|62851|0|72849|0|6193|20|
|unit2|61991|4|71820|17|4064|20|
|unit4|57933|8|67204|24|2849|20|
|unit1_off|136276|378|135458|379|6193|20|

Unit1 20+48 repairs on original+held-out; off zero repairs with matched prefix through first attempt. Unit1 cheaper than parent1 but still 15562 instructions above direct over repairable schedules. Neither witness repairs opcode corruption; bounded retry publishes 20 wrong pairs, despite 44 unit probes and zero repairs. No general correctness guarantee: unit probe assumes linear homogeneous arithmetic, stable faults across probe and production, trusted checker and input. Need fail-closed fallback and cheaper path-local verification; compare with optimized authored direct baseline, not only repeated-add. No native VM or CI, no B3/B4 promotion.
