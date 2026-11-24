# C29-R2 — paid novelty admission and matched freshness baseline correction

Parent 6de3a0157f83fdf6fef3d675773c121dbe2da860. **Non-native engineering model only.** Original C29 native evidence and R1 historical comparator remain unchanged. R1's reported +878 for guard used an initial-only baseline without freshness check; that contrast includes avoiding a stale initial edit, not just later adaptation. R2 compares all active policies against a snapshot initial-only policy with the SAME freshness guard. On the twelve revealed C29 scenarios the R1 fixed stage24 guard now nets -2354, not +878, a 3232-CIL comparator difference. No retrospective change to original data. For example new_same_product_swap original once cost3741, freshness-matched once1377. Its apparent re-evaluation gains largely vanish after fixing the initial choice. This is a corrected estimand, not an error in original measured costs.

## Admission mechanisms

All use the same input snapshot, actual paid two-candidate trials, output equality check, predecision freshness guard, live dispatch readback, and permitted physical PatchNext. Initial evaluates once then production. Fixed reevaluates at stage24. Admit checks live pair vs retained input pair at stage24 and abstains if unchanged. Periodic/periodic48 store next checkpoint in ordinary cell48, charge a per-service gate and paid comparisons at every 24/48 services; reevaluate only if the live pair differs from the last captured pair. Stage deadline is advanced after each check and reset on evaluation entry. No actor access to future horizon or observer correctness. Input novelty is not evidence of opportunity or persistence, and previous snapshot may be stale. All instructions charged, all actual products checked; same-source off suppresses physical PatchNext effects only. No change to kernel or original coherence contract. Fixed checkpoint logic is authored; admission decision is contingent, not learned. No guarantee of optimality. Periodic graphs 202 nodes exceed previous 200-node cap (peak215 within total220); no native feasibility claim. Model assumes valid positive inputs1..16 and coherent updates only at specified boundaries.

## Results

Known 12 revealed cases and new selected systematic engineering family: 7 operand pairs, horizons48/96/144, stable input, swaps after eligible stages5/23/37/73, transient swaps of 4/18/36 services with return before horizon. 21 stable, 66 single flip, 174 transient =261 cases; 10 arms per case incl on/off for all 5 policies (2610 family worlds, 160 revealed-world runs). Family is synthetic and unequally weighted, not holdout or population claim. Net = matched freshness initial-only on minus policy on. Physical saving = same-source off minus on including initial effects. Separate compute and storage accounting; negative outcomes retained.

| policy | known 12 net | family net | family wins | trials family | nodes |
|---|---:|---:|---:|---:|---:|
|initial|0|0|0/261|522|177|
|fixed|-2354|-121516|22/261|1044|180|
|admit|-1803|-102220|22/261|726|192|
|periodic|-451|-79424|33/261|966|202|
|periodic48|-1991|-89566|21/261|726|202|

| policy | stable net (21) | single flip net (66) | transient net (174) |
|---|---:|---:|---:|
|initial|0|0|0|
|fixed|-8826|19155|-131845|
|admit|-6237|22704|-118687|
|periodic|-9051|31989|-102362|
|periodic48|-8421|19741|-100886|

Novelty admission removes all extra trials in stable family (42 vs fixed84), but paid per-service gate still creates losses. Periodic24 improves long-horizon detection, including revealed new_late and new_twice, while all policies still lose in aggregate. Periodic48 cuts evaluations but misses more persistent changes. No candidate earns native promotion. In particular, comparing solely to suppressed source overstates continuing adaptation benefits, since initial edits also contribute. Need equal initial admission/freshness in future economic controls.

## Next research decision

Stop treating every novel input as a reason to pay for two trials. Separate actual marginal cost of checking from trial admission and persistence risk. A domain-specific direct compare of known multiplication operands could be cheaper than trialing; use such a simple selector as a *task-aware engineering baseline*, not as evidence of endogenous general adaptation. Establish whether paid comparison can compete when candidates' cost structure is opaque to the actor and repeated workload classes permit reuse of prior measurements; investigate cache reuse with bounded storage and charged lookup, comparing identical freshness-matched initial baselines. Avoid a host oracle or hidden free hash. This is a change of research direction, not a claim of solved admission. B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c29_novelty_admission.mjs`. Executed the model body in connector JS twice. No Rust CI for R2. Machine-readable summary and known rows accompany report.
