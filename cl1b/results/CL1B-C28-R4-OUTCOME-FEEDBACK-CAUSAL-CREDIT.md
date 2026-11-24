# C28-R4 — paid post-evaluation outcome feedback and causal-credit failure

**Retrospective non-native engineering model.** Parent a48d6e6058dded741c3d954203e55c1e2b46815b. Original C28 native evidence unchanged; no new Rust run, no prospective validation or B3/B4 advance.

## Mechanism

Replace the stability-only exponential backoff with an actual post-evaluation cost signal. At a measured block mismatch, store its charged cost in ordinary cell49 before entering paid candidate re-evaluation. After the first measured post-evaluation block, write the new normal baseline, read the prior observed cost and compute the difference with ordinary paid CIL. For this specific duplicated k12 graph, scan the positive lattice 4,8,...,48 using charged Sub/Branch operations. Feedback1 requires >=4 measured improvement, feedback6 >=24; both choose zero skipped blocks on success and three on failure. Feedback_sparse requires >=4 but chooses one skipped block on success. The initial policy skips one after equal observations, and baseline is reestablished after evaluation. Actual skip count in cell48 is loaded by paid operations; countdown uses r0. The rule uses observed costs, not a host oracle, external environment-change notice or future horizon. This is a pre-authored outcome-conditioned rule, not learned causal credit or realized net return: observed pre/post block differences omit the evaluation fee from the *decision signal*, although the full lifetime accounting charges all instructions including comparisons and evaluation. Positive observed change can result from external regime changes. 526 relocated nodes exceed C28's 500-node cap, so this is a deliberately non-native feasibility model, not a deployable candidate. No scalarized storage pricing.

## Results

Same 438-schedule systematic family as R3 (144 services, 2 stable, 286 single, 126 transient, 24 reverse) and 12 already revealed C28 schedules. All on/off contrasts retain the identical initial source and suppress physical PatchNext only. Distinct-source initial-only economic reference. All cases complete. Model run count 438*(1+2*6)=5694 for family, 156 for known cases. No new heldout results, artificial scenario weighting.

| Policy | Known 8 net | 438 net | 438 wins | Worst net | Nodes |
|---|---:|---:|---:|---:|---:|
| once |0|0|0|0|113|
| original |80|-10592|178|-296|481|
| skip1 |114|-4850|190|-294|490|
| skip3 |-65|-2500|142|-281|490|
| feedback1 |-261|-26984|142|-404|526|
| feedback6 |-246|-23852|142|-404|526|
| feedback_sparse |-138|-19480|142|-368|526|

All feedback variants worsen the fixed-policy frontier. No policy except initial-only avoids loss across stable, transient and double-reversal families. Feedback_sparse is the least negative feedback variant but still loses 19480 across the family. No promotion.

## Explicit causal-credit counterexample

The model now records paid outcome decisions with prior cost, post cost, actual executable edge and chosen cadence. In the already-known alias case the observed first-block baseline is contaminated by a transient and the later block-cost decrease is not attributable to a successful code edit. Feedback1 active world: `{"on":1426,"off":1426,"nodes":526,"probes":[0,24],"observations":[[12,172,0],[24,148,172],[36,148,0],[48,148,148]],"edits":[],"outcomes":[{"stage":12,"prior":0,"post":172,"skip":1,"edge":101},{"stage":36,"prior":148,"post":148,"skip":3,"edge":101}],"offOutcomes":[{"stage":12,"prior":0,"post":172,"skip":1,"edge":101},{"stage":36,"prior":148,"post":148,"skip":3,"edge":101}]}`. In the suppressed family worlds, where *no* physical source change can occur, 87 post-evaluation observations nevertheless meet the feedback1 positive-cost-decrease criterion and select high-frequency sampling. Example records: `[{"case":"transient_0_12_1","stage":36,"prior":152,"post":148,"skip":0,"edge":101},{"case":"transient_0_12_4","stage":36,"prior":164,"post":148,"skip":0,"edge":101},{"case":"transient_0_12_12","stage":36,"prior":196,"post":148,"skip":0,"edge":101},{"case":"transient_0_18_1","stage":36,"prior":152,"post":148,"skip":0,"edge":101},{"case":"transient_0_18_4","stage":36,"prior":164,"post":148,"skip":0,"edge":101}]`. This is direct evidence that the before/after outcome signal is not a causal self-improvement reward. It is not a claim that such a decrease could never be useful as a predictive signal.

## Research decision

The C28 regime has at most 4 CIL/service benefit from two researcher-written candidates; monitoring and comparisons compete with that narrow gain. Stop incremental fixed cadence or before/after heuristic tuning on these revealed cases. Next separate (a) an actor-paid feasible paired local trial estimate under a controlled common input, (b) actual subsequent production costs, and (c) all evaluation/monitoring debts; do not confuse the paired comparison with future environment stability. Where no prospective benefit can amortize a decision, abstention is valid. First establish a larger and heterogeneous candidate-benefit family with finite resource bounds and matched controls, rather than adding a free specialized kernel oracle or selecting favorable schedules. Predefine a new evaluation suite before any native promotion. Preserve negative cases. Candidate invention remains unresolved; B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c28_outcome_feedback.mjs`. Exact generated sources, original schedule generator, model on/off costs, outcome observations and causal-credit counterexamples are emitted. Executed model body in connector JS; repository script includes aggregate assertions. No native CI claimed.
