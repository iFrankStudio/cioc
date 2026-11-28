# C36-R8 — reversible sensing-path edits

Parent b64b19650084b78bf76b18e8d7f280aa9dca74e7; frozen Rust gate 4b5bada7dd582ffc40d5153ba8d731823ba62167. STATUS: JS instruction-level engineering preflight complete, native Rust outcome UNVERIFIED. R6/R7 workflow commit combined status and PR-only run query remain empty. Container cargo/rustc not found and GitHub DNS fails. No CI pass/fail inferred.

R7 producer/buyer protocol unchanged. Producer starts in fresh sensing. After each paid offset observation, native Read/Write/Sub/Branch compares previously cached offset with newly observed offset. Equal selects cached next offer; unequal selects fresh. Paid ReadNext reads live node33 edge, only if unequal to desired edge does charged PatchNext at PC46 physically change node33.next to 0 (probe) or 12 (cached production). Reuses same slot indefinitely; no cloning or allocation. No future horizon, no environmental phase flag, no free coefficient access. Rejection still triggers re-probe before publication. Same initial 47-node producer source for adaptive and edit_off; off suppresses only PatchNext effects, not instruction charge. Fresh/cache fixed controls use shorter 37-node producer. All have same 220/15000 private credits, eight cells, strict total capacity100, max nodes64, neutral pool0. Initial source footprint differs between fixed and adaptive arms; none reach capacity. Every attempt consumes exactly one combined credit; transfers explicitly checked. Test asserts correctness/ack before each formal publication, exact predicted metrics and matched native state prefix lengths. Host changes input only on completed offer. No kernel changes.

## Preflight (20 worlds, 5 schedules x 4 arms, 24 offers each)

| regime | arm | completed | paid | probes | invalid candidates | edit attempts/effects | producer end |
|---|---|---:|---:|---:|---:|---:|---:|
|stable|fresh|24|3311|24|0|0/0|820|
|stable|cache|24|2325|2|1|0/0|1445|
|stable|adaptive|24|2491|4|1|3/3|1350|
|stable|edit_off|24|3739|24|0|22/0|606|
|mixed|fresh|24|3311|24|0|0/0|820|
|mixed|cache|24|2797|6|5|0/0|1185|
|mixed|adaptive|24|3299|12|5|11/11|898|
|mixed|edit_off|24|3731|24|0|18/0|610|
|fast|fresh|24|3311|24|0|0/0|820|
|fast|cache|19|3999|20|19|0/0|0|
|fast|adaptive|24|3695|24|0|0/0|628|
|fast|edit_off|24|3695|24|0|0/0|628|
|fast-to-stable|fresh|24|3311|24|0|0/0|820|
|fast-to-stable|cache|24|3599|13|12|0/0|742|
|fast-to-stable|adaptive|24|3053|14|0|1/1|1009|
|fast-to-stable|edit_off|24|3693|24|0|11/0|629|
|stable-to-fast|fresh|24|3311|24|0|0/0|820|
|stable-to-fast|cache|24|3647|13|12|0/0|718|
|stable-to-fast|adaptive|24|3125|14|1|2/2|973|
|stable-to-fast|edit_off|24|3693|24|0|11/0|629|

All formal publications correct in preflight. Adaptive and edit_off each complete120/120; adaptive saves 2888 total paid vs off (15663 vs18551); fast no effect and complete trace identical. Pre-first-physical-effect matched state steps stable216,mixed216,fast3695 (all),fast-to-stable2064,stable-to-fast216. Actual reversible changes stable3, mixed11, fast0, fast-to-stable1, stable-to-fast2. Adaptive uses cached path in stable and switches back to fresh upon rejected candidate followed by changed observation. Adaptive costs 892 less than fixed fresh aggregate (15663 vs16555) at identical coverage, but loses to fixed cache in stable/mixed, loses to fresh in fast; no universal superiority. Edit-off repeated attempts are a consequence of suppressed effect and part of matched intervention total; not proof that editing beats equivalent ordinary dynamic branch. Need direct same-policy state/branch control with paid state, dispatch and matched storage. Need new independent randomized schedules and different offer/reward liquidity. Test is frozen after model predictions, not blind generalization.

Trusted verifier, protocol compliance, offer stability between validation and publication and finite buyer budget remain assumptions. No spontaneous candidate invention, no independent native replay, no native CI verified. B3/B4 NOT_MET.

Reproduce: `cd cl0 && cargo test --locked -p cl0-world --test cl1b_c36_r8_native_sensing_path_edit -- --nocapture`.
