# CL-1B B14 — native search-policy self-mutation, independent source-level audit

**Evidence category: independent lightweight instruction interpreter of the [frozen B14 founder](../../cl0/crates/world/tests/cl1b_b14_search_policy_mutation.rs). NOT a CL-0 Rust VM run or accepted life demonstration.** [Prospective protocol](../studies/B14-native-search-policy-mutation.md) was committed before test/source-level outcome inspection. Predictions were qualitative and frozen; the exact per-120-attempt counts below are post-registration descriptive compatibility targets.

Five independent service inputs `[-13,0,7,23,101]` × six conditions = **30 simulated single-process worlds**. Every world executes exactly **120 charged instruction opportunities**, with 240 initial credits and no source/marker/output externally seeded except declared source input and initial graph lesions.

| Arm | Effective actor-native executable edge changes (node6) | Actor's policy literal node29 | Correct service writes | Marker writes | Extra write key1 | 
| --- | --- | ---: | ---: | ---: | ---: |
| SHAM7 | none | 2 | 7 | 8 | 0 |
| LESION10 | 10→8→7 | **1** (changed from2 via PC22) | 5 | 4 | 0 |
| LESION10_POLICY_LOCKED | 10→8→6 | 2 (PC22 bypassed) | **1** then low-value self-loop | 0 | 0 |
| LESION10_PATCHNEXT_DISABLED | no effective edges; 4 paid suppressions | **1** (changed despite disabled PatchNext) | 0 | 0 | 0 |
| LESION11 | 11→9→8→7 | **1** | **3** | 3 | 1 |
| LESION11_POLICY_LOCKED | 11→9→7 | 2 (PC22 bypassed) | **4** | 4 | 1 |

The literal is modified by *executing native-like `PatchConst` (PC22)*, not by host intervention after boot. For LESION10, that effective code change is essential to avoid the subsequent candidate edge6 trap-loop under the fixed step2. A newly failed trial changes the search policy from decrement2 to decrement1, and the actor subsequently generates its next candidate using the updated literal. The actor remains the same single process throughout.

**Critical negative-control surprise:** the same policy change makes the LESION11 case **slower** than the externally locked baseline: 3 valid outputs vs4 within the equal 120-instruction window. The policy is therefore **not globally superior**, and a local counterfactual on one lesion does not establish generalized adaptation. This is a deliberately visible tradeoff, not an “evolutionary win”. The LESION11 route also generates a wrong-address extra write to key1. Both facts matter to finite resource accounting.

### What this adds and what it cannot prove

This is a native CIL expressibility target: **one process can mutate a real executable literal that controls its own future candidate-generation algorithm**, and the effects can be tested under multiple independently seeded damage conditions and negative controls. The repair rule, feedback check, step-change trigger, candidate neighborhood and definition of “correct service” are still all **researcher-authored**. The policy is not invented by the actor, nor does it autonomously optimize a robust held-out objective. An actor that learns to maximize the mutable success marker while losing actual functionality would fail B12/B13's independent-function challenge.

The [B14 Rust integration test](../../cl0/crates/world/tests/cl1b_b14_search_policy_mutation.rs) now checks process-local policy literal before/after PC22 native execution, ordered edge changes, marker and service outputs, harmful side effects, compute inventory and all six experimental arms across five inputs. **No Rust CI pass has been verified.** Success of the interpreter is a source-level sanity check only.

### Next gate

For a meaningful CL-1B claim beyond authored policy switching: separate training faults from **unseen fault combinations**, force the actor to maintain evidence that distinguishes true task work from corruptible marker output, measure whether policy changes improve recovery **out of sample** at equal resources, and require candidate evaluation and retention to be CIL-owned. Pre-register the failure/holdout allocation before any native experiment.
