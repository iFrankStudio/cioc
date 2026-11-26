# C35-R4 — stronger static and fused production controls

Parent 6950c14aa3066b22c4f8efdc85ebe441e46413ec; experiment 85b81dc33ea855cfca9573020ddd2a00e1375a15. 41 schedules x 6 arms = 246 non-native model worlds, 2024 requests per arm. Prebuilt control starts with identical 16-node DAG and dispatch/repair anchors installed at initialization (no paid generation); matched generated/static arms have identical production, witness, maintenance and quarantine instructions, faults and output. Fused variants compute first product and derive second using +y; relation check becomes tautological, so the unit probe is the operative witness. No change to workload contract: both results still delivered. Direct controls use trusted parent and are not exposed to generated-region faults; matched static DAG controls ARE exposed. Preinstalled setup cost not charged; all peak storage counted. Same 24 repairable and 17 opcode schedules as R3.

| arm | all paid | wrong | repairs | quarantines | peak occupied | repairable paid | opcode paid |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
|direct|159411|0|0|0|135|120138|39273|
|direct_fused|90858|0|0|0|138|68265|22593|
|generated|185076|0|68|17|171|135700|49376|
|static|181140|0|68|17|171|133396|47744|
|generated_fused|140499|0|63|16|174|108118|32381|
|static_fused|136563|0|63|16|174|105814|30749|

Every generated/static matched pair returned identical outputs, repair count and quarantine request; generation costs exactly 96 extra paid instructions per world (3936 total), with no subsequent advantage in this closed known task. Fused runtime saves substantial work but is still costlier than trusted fused direct. One opcode scenario never needs corrupted node 16 in fused path (not evidence of full fault coverage). The fused direct control has an explicit trusted-parent advantage; equal fault exposure comparison is static vs generated DAG. Static code authored with foreknowledge and setup cost excluded; not a universal impossibility proof for runtime construction. The current graph is a predetermined template, not adaptive discovery. No native VM, CI or B3/B4 promotion. Next: stop incremental arithmetic tuning as primary route; design unforeseen task/constraint changes with equal initial information, matched static online optimizer, paid discovery/installation, held-out generalization and accounting for correct production and resources. Test whether construction enables capabilities not supplied at boot.

Reproduce: `node cl1b/analysis/cl1b_c35_r4_static_fused_controls.mjs`.
