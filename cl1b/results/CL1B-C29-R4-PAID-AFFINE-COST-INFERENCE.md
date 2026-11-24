# C29-R4 — paid affine cost inference on unseen operand pairs

Parent 944cdb3ff076e53fcbbbcc6773c6ee98e0786d67. **Non-native component model only**, not production adaptation or self-invention. Original C29 native artifacts unchanged. B3/B4 NOT_MET.

## Design

Bounded domain x,y=1..16. A multiplies via x repetitions of y. Cross profile B repeats x y times. Unroll profile B doubles y, then consumes two units of x per full iteration with a real odd-tail path loading original y. Both compute x*y, checked by independent observer on every training and audit pair. No artificial delay/padding. Profile is researcher-authored graph choice, not a guest-readable profile label. Learner instructions identical in both profiles apart from relocation; only B candidate body differs. Three actor-generated anchor inputs (1,1),(2,1),(1,2) stored via charged Const/Write in ordinary cells40/41. Actual SenseOwnCredits brackets each trial, outputs must agree before learner proceeds. Store d0=d(1,1), dx=d(2,1)-d0, dy=d(1,2)-d0 in cells48,49,52. No host calculation or coefficient injection. The model family is explicitly authored, not learned. Extrapolate d0+(x-1)*dx+(y-1)*dy via charged additions, countdowns and branches. Prediction written to cell53 BEFORE paid audit of both candidates on the same captured input. Audit writes actual difference to cell54. Audit never updates coefficients or feeds predictor. All actual trial products and arithmetic checked outside CIL. Model observer computes sign correctness (including exact tie classification), not an actor decision or physical patch. No production-service claim, no off arm because no physical edits; this isolates the learning component. Input coherent within capture/audit; environment changes after audit write only. Source storage charged by node count plus allocated ordinary cells and process header. External finite batch cutoff; actor not informed of horizon.

## Coverage

All 253 non-anchor input pairs for each profile. Sixteen independent batches per profile grouped by x, each independently trains once on the three anchors; 32 modeled worlds and 506 unseen-input predictions. Repeated training cost counted 16 times per profile. Domain enumeration is deterministic, not a real-world population or blind holdout. All test inputs excluded from coefficient estimation; this does not mean profile or model class was chosen blindly. Trial spans for cross are 11+3x / 11+3y. Unroll cost depends on parity, real work differs. No kernel change.

| profile | learned [d0,dx,dy] | exact | sign correct | MAE | max error | nodes | peak occupied |
|---|---|---:|---:|---:|---:|---:|---:|
|cross|[0,3,-3]|253/253|253/253|0.0000|0|136|152|
|unroll|[-3,3,0]|29/253|205/253|17.7075|35|142|158|

| profile | total paid | training | prediction | audit |
|---|---:|---:|---:|---:|
|cross|48527|3488|18092|26947|
|unroll|48105|3584|18092|26429|

Both fit previous capacity limits in model accounting (cross 136 nodes/152 total, unroll142/158; 15 allocated cells at peak). Cross learns [0,3,-3] from costs14/14,17/14,14/17; exact in whole bounded domain. Unroll learns [-3,3,0] from costs14/17,17/17,14/17; despite exact interpolation at anchors, extrapolation fails: x3,y1 predicted +3, actual -2; would prefer wrong candidate if used. 48 wrong signs including tie mismatch, not necessarily 48 strict wrong selections. Raw audit trace verifies computed cost difference after prediction; no confidence guarantee, no production savings. Prediction itself costs18092/253≈71.51 CIL per input; training amortization plus prediction is still substantial relative to paid audit cost. Audit includes verification and phase bookkeeping; comparing those totals alone is not a fair deployment economy estimate. No claim that inference is economical versus task-aware comparator or initial-only; actual full-model costs include training+prediction+audit.

## Next gate

Add actor-paid model validation on disjoint non-anchor probes; require rejection/abstention when residual violates threshold, compare equal-initialized no-learning and always-paired controls. Freeze validation inputs separately from untouched evaluation set; expose residuals as ordinary observations, not host reward or free labels. Model family selection remains authored; a new parity-aware hypothesis should be evaluated prospectively, not assumed optimal from these results. Only after demonstrating failure recognition and bounded prediction cost should couple predictions to actual source edits and evaluate lifetime economic benefit. B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c29_affine_cost_inference.mjs`. Model body executed in connector JS twice, exact aggregate assertions included, no native CI claimed.
