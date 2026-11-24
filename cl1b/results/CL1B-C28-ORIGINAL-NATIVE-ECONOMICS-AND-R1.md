# C28 native results and R1 paid trial-anchored baseline falsifier

## Original native results (confirmed)

Tested SHA f1fcc7db9247ff2c00dc8eb993f93528365971a2; [native + replay run 45320277423](https://github.com/iFrankStudio/cioc/actions/runs/45320277423) SUCCESS; [archive run 45320788057](https://github.com/iFrankStudio/cioc/actions/runs/45320788057) SUCCESS. [Original ZIP and economic summary](https://github.com/iFrankStudio/cioc/tree/cl1b-ci-evidence/run-records/f1fcc7db9247ff2c00dc8eb993f93528365971a2-45320277423-1), ZIP SHA256 3c64ce85d08b472e828b26b350d78c443d6308e59b6465371521e12d2d61007d, economic-summary.json blob a453d8aef0a5ae341792289607e03797681ad84c. Archive job re-downloaded, hash-verified, independently replayed and remotely re-read exact bytes. This turn read the archived economic summary; did not itself re-hash ZIP bytes. 60 original worlds, 91,552 charged CIL, 5,880 production services, 208 trials; all completed external equal-service cutoff, not actor Halt. Four model-known calibration scenarios and eight selected prospective change schedules, no population inference.

| Cohort/policy | on | off | initial-only | physical-effect saving | economic saving | economic W/L |
|---|---:|---:|---:|---:|---:|---:|
| Calibration duplicate |3550|3972|3656|422|106|2/2|
| Calibration shared |3810|4232|3656|422|-154|1/3|
| Prospective duplicate |13224|15276|13304|2052|80|3/5|
| Prospective shared |14238|16290|13304|2052|-934|2/6|

Peak occupied units: 489 duplicate, 312 shared, 119 initial-only. No common scalarized resource fitness. Duplicate economic gain 0.60% of prospective baseline, median per-case saving -62; shared -7.02%, median -185.5. Removing the single new_long scenario changes sums to -379/-1159. The physical-effect contrast remains positive in aggregate but both arms pay the monitor; new_early physical effect loses 58, despite duplicate beating initial-only by 69. No claim that isolated rollback causes these savings. Stable B physical benefit 528 is from initially enabled code change; no evidence monitoring helps. Focused historical regression run 45320277361 failed 28 observer checks; C28 dedicated pass does not establish full regression.

| Prospective case | duplicate saving | shared saving | observed on probe stages |
|---|---:|---:|---|
| early |69|-9|0,24|
| middle |135|18|0,36|
| late |-77|-168|0|
| stable |-125|-268|0|
| transient |-133|-224|0,36|
| twice |-47|-203|0,36,96|
| long |459|225|0,36|
| alias |-201|-305|0,24,60|

Both active variants have same probe stages and normalized edits in archived rows. In early (B→A after stage5), first block is already mixed; actual rollback occurs stage24. In alias, transient reversals cause costly full evaluations without edits. Late changes after the last complete monitored block cannot be acted upon. The block-total observation maps multiple distinct within-block mode histories to identical totals, so change timing and persistence cannot be inferred uniquely. The next intervention must treat monitoring as a decision under uncertainty, not simply increase frequency.

## R1 retrospective model: trial-anchored reference instead of first production block

A source-specific proposal preserves actual paid A/B trial spans and stores B in cell42 in addition to A in cell41. After each full evaluation, ordinary paid ReadNext observes the actual active dispatch edge (not the intended choice, important in suppressed controls), reads that candidate's measured trial cost t, computes 12*t using four Add instructions, and adds known researcher-authored framing (63 duplicate, 76 shared). This predicts unchanged-regime block cost from actual trial observations. The reference is stored in cell47; no need to accept a mixed initial production block as normal. All arithmetic, readback, reads/writes and source edits are charged. These constants are specific to the already known graph and k12, not learned transfer or a general world oracle. No new kernel opcode. Raw graph engineering model, not relocated native candidate. Extra allocated source and cell42 would need accounting before native run.

This model was run retrospectively on the already revealed 12 schedules, not a holdout. It preserves completion and produces these economic savings vs original initial-only baseline:

| Case | C28 original duplicate | trial-anchored duplicate | trial-anchored shared |
|---|---:|---:|---:|
| cal_middle |3|-29|-81|
| cal_reverse |-3|-35|-87|
| cal_long |147|115|11|
| cal_stable |-41|-57|-109|
| new_early |69|85|7|
| new_middle |135|103|-14|
| new_late |-77|-93|-184|
| new_stable |-125|-141|-284|
| new_transient |-133|-165|-256|
| new_twice |-47|-95|-251|
| new_long |459|427|193|
| new_alias |-201|-249|-353|

In new_early rollback advances from stage24 to stage12; its 48 additional saved production instructions outweigh the 32 extra paid initialization instructions. In new_alias, anchoring triggers an extra early evaluation with no edit. Prospective aggregate duplicate changes +80→-128, shared -934→-1142; this is a negative economic result, not a new native or independent replay. The evaluated code is transcribed in [cl1b_c28_trial_anchored_baseline.mjs](../analysis/cl1b_c28_trial_anchored_baseline.mjs). It asserts the known model totals; no CI claimed for this new script.

## Research decision

Do not promote the trial-anchored controller into native C29 unchanged. Better detection does not imply net benefit. Next isolate actual *persistence* evidence using a cheap confirmatory production sample before the costly A/B trial, compare fixed sample/confirmation policies with matched effect suppression and initial-only, count false-negative latency and false-positive probe costs. A single sampled block cannot guarantee that a transient persists, and without future horizon information a finite sequence can be adversarially indistinguishable. Freeze a new schedule distribution and all costs before native trials. Keep original C28 artifacts immutable. Neither C28 nor R1 demonstrates endogenous candidate invention, learned monitoring strategy or B3/B4. **B3/B4 NOT_MET.**
