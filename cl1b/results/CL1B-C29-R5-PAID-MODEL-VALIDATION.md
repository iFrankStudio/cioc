# C29-R5 — paid model validation, rejection, and a finite-validation counterexample

Parent fe16846b7a591cf875992793c8587485513ba40b. **Non-native component model only**, no native CI or physical source edits. Original C29 native record unchanged. B3/B4 NOT_MET.

## Actor mechanism

R4 affine coefficients are derived from charged trials on actor-authored anchors (1,1),(2,1),(1,2). Validate at two fixed disjoint points (3,1),(1,3). Prediction is emitted to ordinary cell53 before paired audit; any nonzero residual clears sticky trust cell55 via paid Sub/Branch/Write. Both checks executed even if first fails. Phase47 advances to 5. Trust determines subsequent behavior: if true, output estimated cost difference to cell54; if false, capture the live pair and perform actual paid candidate trials before emitting measured difference. No automatic restoration, online retraining or alternative model choice yet. Exact residual threshold, probe selection and affine family are researcher-authored. No host labels influence actor. The guest does not know profile or future batch length. Environment only changes live pair after output54; no atomicity claim under arbitrary interleaving. Positive bounded domain1..16. Trial products independently observer-checked. Control policies: unchecked affine (same three anchors, no validation); paired (no training, always actual measurement). Distinct source entry policy, not same-source physical effect suppression (no patch in any arm). Unused nodes remain allocated.

Profiles: cross and unroll from R4. Third profile switch8 has a real conditional execution path in B: if x=8 iterate x times instead of y; otherwise cross-loop. It charges the same 5 extra condition instructions, both paths compute multiplication. No free host flag. This profile was constructed expressly to falsify the fixed validation protocol; not blind discovery or real workload distribution. For all profiles the observer's independent formula is cross-checked against actually measured paired control on every test input, including switch8. Formula not guest-visible. Test excludes three training and two validation inputs, 251 pairs per profile, 16 independent batches grouped by x. 3 profiles ×3 policies ×16 batches=144 model worlds and 2259 emitted test differences (753 distinct profile-input combinations); all measured training, validation and fallback costs counted.

## Results

| profile | policy | accepted predictions | exact output difference | correct sign incl ties | paid CIL | candidate trials | nodes | peak |
|---|---|---:|---:|---:|---:|---:|---:|---:|
|cross|validated|251|251/251|251/251|27754|160|191|208|
|cross|unchecked|251|251/251|251/251|24074|96|185|202|
|cross|paired|0|251/251|251/251|30674|502|185|198|
|unroll|validated|0|251/251|251/251|37495|662|197|214|
|unroll|unchecked|251|28/251|204/251|24170|96|191|208|
|unroll|paired|0|251/251|251/251|30151|502|191|204|
|switch8|validated|251|236/251|245/251|28154|160|200|217|
|switch8|unchecked|251|236/251|245/251|24314|96|194|211|
|switch8|paired|0|251/251|251/251|31905|502|194|207|

Cross validation passes both points; unroll first predicts3 vs actual-2 and is rejected, second matches but sticky rejection preserved. All subsequent 251 unroll outputs measured (not successful predictions). Switch8 validation passes both points, but 15 unseen exact-difference errors and 6 sign errors remain; at x8,y1 prediction16 vs measured-5, opposite strict preference. Finite validation cannot certify arbitrary candidate control flow. Validation decision is correct with respect to finite evidence but does not imply calibrated confidence.

All three validated source footprints fit previous C29 node and total model limits (191/208,197/214,200/217); allocated cells16 at peak. Switch8 hits 200-node cap. No native capacity proof. Paired controls and unchecked variants also charged for retained unused nodes. Economic comparisons here concern cost-difference emission only, not productive service or edit effects. Accepted prediction can be wrong; don't count cheap wrong outputs as beneficial adaptation. Setup costs repeated once per batch; aggregate not single continuously learning individual. No claim of B3/B4.

## Next gate

Extend finite validation to paid ongoing residual checks and bounded trust revocation during deployment, with a matched always-measure control and disjoint prospective workload distribution. Need expose actual local trial evidence, not free truth oracle. A cheap predictor must earn enough savings after training, validation, periodic audits and error penalties. Before adding physical patching, verify the actor actually switches from prediction to measurement on a previously trusted profile encountering its failure region. Consider compact control graph/avoid excessive per-query overhead to preserve C29 storage. No further tuning on switch8 alone; freeze new failure regions and check false acceptance and false rejection, including delayed nonlinearity. B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c29_model_validation.mjs`. Executed the model body in connector JS with full paired-reference cross-check; script assertions freeze aggregate. Original native evidence unaffected.
