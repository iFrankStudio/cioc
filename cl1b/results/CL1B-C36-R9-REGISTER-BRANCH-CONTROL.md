# C36-R9 — same-policy register-branch control

Parent 319cf6b37264207d5a2d71b177f4e6087e27b4ab; executable model aedd6159bd12f023d1176f342cf20fe424fffbac. 65 paired scenarios, 325 abstract instruction-model worlds. Five R8 schedules replayed; 30 new seeded temporal/input schedules, each run under two paired initial/reward configurations (220/100 and 500/85), 48 offers each. 3000 offered per arm. Same R8 buyer and producer protocol; fixed fresh/cache, adaptive source-edit, matched suppression and new register branch. Fourth register r3 retains signed difference between newly observed and prior offset; no production instruction clobbers it. Probe payment temporarily uses r3, and probe exit recomputes difference. The static dispatch Branch tests r3 after every completed offer. Same observation/decision rule, no PatchNext/ReadNext, no extra data cell. 40 functional nodes padded to 47 to match adaptive's modeled initial source footprint; equal peak86 including buyer, eight cells and process headers. Padding not executed. Fresh/cache have peak76. No native VM run. Credit conservation, publication validation and equality of decisions, probes and rejections over common completed offer prefixes asserted; both adaptive arms finish all 65, so match entire offered sequences. No timing-dependent environment or mid-offer mutation.

| arm | complete worlds | correct completed | paid steps both actors | producer end credit | probes | actual edits |
|---|---:|---:|---:|---:|---:|---:|
|fresh|65|3000|413935|76100|3000|0|
|cache|40|2299|322863|68588|1028|0|
|adaptive|65|3000|390375|95896|1664|413|
|edit_off|65|3000|464471|50832|3000|0|
|reg_branch|65|3000|372635|104766|1664|0|

No wrong formal publications. Adaptive and reg_branch each produce 189 invalid candidates (all rejected); exactly same sensing decisions and coverage. Register branch saves 17740 combined paid steps (~4.54% of adaptive), and 8870 producer instructions (195220 vs186350). Both arms have identical probe fees and production receipts. Cache lower paid total is not comparable at reduced coverage. Adaptive still saves 74096 steps vs effect suppression, but the stronger nonediting strategy saves 91836; this is evidence against attributing the policy's advantage specifically to executable mutation.

| regime | adaptive paid | reg_branch paid | fresh paid |
|---|---:|---:|---:|
|stable|62303|62007|82787|
|mixed|80343|77771|82787|
|fast|92387|85161|82787|
|fast-to-stable|77133|73265|82787|
|stable-to-fast|78209|74431|82787|

Register branch wins63/65; edit wins2/65, both seed3889 stable under two budgets (same temporal/input schedule, not independent discovery), 4849 vs4865, six probes/five edits. Thus very long dwell can amortize editing, but the tested rule's read/compare/patch overhead plus protocol wait costs outweigh dispatch savings on most schedules. R8 old cohort branch14955 vs edit15663; new60 branch357680 vs edit374712. No universal impossibility theorem; stronger self-modifying implementation or different workloads may differ. New schedule seeds are chosen in this study and not preregistered blind tests. JS interpreter follows subset of CIL, not independent native replay. No additional Rust test added until existing R6-R8 execution evidence can be obtained. Current main workflow commit status query empty; local cargo/rustc and github.com DNS unavailable. Do not infer CI success/failure.

Decision: stop tuning simple sensing-path mutation as a primary life-research route. Separate policy adaptation from executable adaptation. Next investigate bounded construction of previously absent task-specific computational structure versus equally informed data-driven interpreter (not merely selecting existing routes), using reusable storage, paid verification, and full resource/quality accounting. Audit native graph construction/launch lifetime and code reclamation; if native evidence remains unavailable, explicitly label engineering model and avoid promotion. B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c36_r9_register_branch_control.mjs`.
