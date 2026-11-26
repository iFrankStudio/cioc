# C36-R2 — completed dwell prediction

Parent dd4fc7150c49088281279bb0e3e29faada5ebcb1; implementation b70b91dd6ef390863c137902991874689a1b8d76. 224 paired scenarios (8 new seeds, 7 regimes, capacities 84/96, install tariffs 1/4), 7 arms, 1568 abstract worlds plus 448 future-suffix intervention runs. 24 announced epochs; two paid observations each. Equal modeled base=58,state=8 for all arms (static also reserves unused controller space). Added short-to-long, long-to-short, alternating schedules. Past dwell counts requests since the most recent observed coefficient change, including unchanged epoch boundaries. Last uses previous completed dwell; EWMA uses floor((prior prediction+completed dwell)/2). Cold start declines installation. Both decide at observed coefficient change using predicted dwell * saved cost > estimated construction+retirement cost and capacity. Both pay per-request counting, update, comparison. Policy math uses abstract charges, not native instructions.

Oracle is explicitly offline, given exact contiguous future dwell; it decides whether installation pays conditional on same nonstatic setup cost and admission. It is NOT a global optimum across policies and can lose to static because static avoids planning costs. Neither online predictor reads future dwell or regime labels. Replacing epochs >=12 with a different suffix preserves all earlier predictor decisions in 448 checks.

All 76448 requests per arm correct.

| arm | total paid | online best worlds | installs | denied | peak |
|---|---:|---:|---:|---:|---:|
|static|3002652|128|0|0|78|
|always|2851607|92|2504|2296|89|
|delay4|2899132|4|1274|2296|89|
|adaptive|2921758|0|1273|2296|89|
|last|3027457|0|1257|2296|89|
|ewma|2974201|0|1568|2296|89|
|oracle|2815936|offline|1315|2296|89|

| regime | static | always | delay4 | adaptive | last | ewma | oracle |
|---|---:|---:|---:|---:|---:|---:|---:|
|short|89056|126942|117922|117922|122218|123632|115340|
|long|926356|831743|855311|862541|864749|866157|831743|
|mixed|371276|363216|369110|372061|396342|389566|356902|
|sticky|103180|100301|106049|107642|105161|105417|100269|
|short_long|501324|473758|480749|484246|490597|492007|467861|
|long_short|509756|480752|488019|491719|497035|499717|474853|
|alternating|501704|474895|481972|485627|551355|497705|468968|

At capacity84, static wins aggregate for both tariffs (750663 vs conditional oracle 785693/786143), highlighting the nonstatic planning cost under denied admission. At capacity96, immediate installation beats both predictors at both tariffs. Alternation particularly hurts last-dwell prediction. No predictive win among the tested online arms. Offline conditional oracle saves only 35671 (~1.25%) vs always aggregated, so the opportunity for extra prediction is small on this workload under this model. Equal footprint reserves unused memory; capacity regime is deliberately set to 84/96 after increasing state footprint. No universal bound on prediction value; workload distribution and policy charges are authored. Prefix check demonstrates invariance under the tested suffix edits, not general causal proof.

Decision: stop tuning dwell predictors as main route. Next investigate unannounced changes and paid information acquisition with finite credit ledger, comparing a minimal static learner to specialization under identical observation/feedback, including detection delay, wrong-publication cost, abstention and admission before expensive planning. Start native CIL feasibility audit; do not equate host JS accounting with guest agency. No native execution, CI, resource regeneration or B3/B4 promotion.

Reproduce: `node cl1b/analysis/cl1b_c36_r2_completed_dwell_prediction.mjs`.
