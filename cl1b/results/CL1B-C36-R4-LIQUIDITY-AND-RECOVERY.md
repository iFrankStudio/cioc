# C36-R4 — liquidity, exact admission and recovery

Parent e2f4fa081fcc48bf21b75febc7e2ced6c3bef852; implementation dc2d5ff5ee94b20bf8ae748aaf56d6f2a6c2033b. 576 paired scenarios, 6 arms, 3456 abstract worlds, 221184 offered requests per arm. Six new seeds, 384 requests per world, initial credits 100/300/1500/6000, reward 40/65/90, capacities 84/96, tariff1. Hidden fast, slow, fast-to-slow (nominal t48), slow-fast-slow (nominal t128..176) regimes. Duration selected at coefficient change; phase boundaries are nominal, not forced changes or notifications. No future information provided. All compiled arms emit directly into graph (no temporary recipe array). Exact admission first checks minimal feasibility, scans learned masks at charged cost, computes exact node count and install cost; validates actual install cost and storage. Conservative feedback_compiled retains R3 upper bound; exact_off pays same exact admission and installation but executes static mask path. No change to reward/feedback interface. Credit identity and capacity checked in all runs.

| arm | correct | wrong | survivors | end credit | net credit | installs |
|---|---:|---:|---:|---:|---:|---:|
|probe_all|146390|0|328|3043926|1906326|0|
|feedback|139590|13922|362|4162882|3025282|0|
|feedback_compiled|140782|13076|368|4609390|3471790|6333|
|exact_compiled|127880|9739|318|4148773|3011173|5048|
|exact_off|117940|8696|281|3410057|2272457|4067|
|periodic4_exact_compiled|126896|8563|314|3723589|2585989|5086|

Survival: no exhaustion through all offered requests and final credit >=16. Aggregate net includes extinct worlds, subtracts total initial credit. End credit alone is not survival or output quality. Exact admission allows 316 installs at cap84 vs 0 with conservative admission, but cap84 survivors fall from179 to148 (static feedback181); charged scanning and installation policy overhead not recovered. At cap96 exact vs conservative survivors170 vs189. Exact vs exact_off survivors318 vs281, not fixed trace ablation because credit changes later decisions.

## Liquidity vs operating loss

Fast regime, 12 scenarios per initial/reward combination (6 seeds x2 capacities). At reward40, all arms exhaust even with initial6000. At reward65 and initial6000, probe_all and feedback each survive12/12, but aggregate net credit -26726 and -56792; conservative compiled survives6/12, net -66613. This is finite-run survival funded by reserves, not resource self-maintenance. At reward90 probe_all survives12/12 at every initial level, net +88474 per initial level. Feedback at initial300+ also survives12/12 with +21708; conservative compiled initial1500+ survives12/12 with +7306. Exact compilation at initial6000 survives12/12 but net -34531; lower storage admission threshold is not sufficient for profitability. No infinite-horizon claim.

## Recovery across nominal phase boundaries

Each regime 144 worlds. Recovery defined as completed full horizon and end credit > credit at nominal end of stress phase; restoration defined as completed horizon and end credit >= credit immediately before nominal stress phase (initial credit for switch). Does not require actual prior depletion, so these are operational phase metrics, not proof of repair of every stress event.

| arm | switch reached t48 | switch recovered | switch restored | sandwich reached t176 | sandwich recovered | sandwich restored |
|---|---:|---:|---:|---:|---:|---:|
|probe_all|90|74|74|108|96|96|
|feedback|92|68|68|120|96|96|
|feedback_compiled|81|70|70|128|116|116|
|exact_compiled|62|55|50|128|114|114|
|exact_off|56|43|43|115|96|96|
|periodic4_exact_compiled|67|59|54|115|104|96|

No resurrection or external replenishment; rewards from correct production only. Some surviving trajectories recover reserves after nominal stress, with conditional advantage for conservative compiled in sandwich. Exact admission is a negative result as implemented, not a general impossibility of accurate admission. Next: isolate scanning overhead vs bad install decisions by moving exact node count into already-paid mask extraction and testing change-frequency-sensitive admission with a cheap static fallback; include phase-aligned actual loss/recovery criteria and verification-before-publication. Native CIL audit still outstanding. Host JS operation model, authored environment/policy, no native VM/CI or B3/B4 promotion.

Reproduce: `node cl1b/analysis/cl1b_c36_r4_liquidity_and_recovery.mjs`.
