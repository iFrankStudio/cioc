# CL-1B B09 — independent instruction-level audit (not native CI)

**Evidence:** deterministic lightweight interpreter for the **committed 19-node CIL program** and all five registered input labels × all five conditions, **25 simulated worlds**. Performed after [preregistration](../studies/B09-evidence-after-history-loss.md) and [native Rust integration-test source](../../cl0/crates/world/tests/cl1b_b09_trial_evidence.rs) were committed. The interpreter did not run the Rust VM, its state invariants, serialized replay, accounting or CI. Its results must not be reported as verified native experiments or autonomous improvement.

Every simulation used source input cell7; each arm executes 72 CIL instructions after declared external setup. Each correct service write is an interpreter-identified PC8 execution, **not** a host-generated fake output. The recovery path reads the runtime output cell9, compares it with expected source and calculates a candidate edge from current node7.next **minus one** inside CIL.

| Condition | Input 0: real service writes | Nonzero inputs: real service writes | Effective native-analogue edits | Outcome |
| --- | ---: | ---: | --- | --- |
| INTACT | 8 | 8 | none | original code works |
| BYPASS | **0** | 7 | 0 for input0; exactly 7:9→8 for each other input | zero=missing collision prevents recovery |
| BYPASS_PATCH_DISABLED | 0 | 0 | none; six suppressed patch attempts for nonzero inputs | recovery causally requires code edit |
| BYPASS_STALE_MATCH | 0 | 0 | none | stale matching output disguises absent real service |
| BYPASS_STALE_WRONG | 7 | 7 | exactly 7:9→8 per input | non-matching readback triggers repair even for zero |

All 25 source-matched simple path runs consumed72 modeled instructions, stayed in the single process loop, and had no unexpected control-flow trap. That is **not** a substitute for native VM validation, process inventory checks, event provenance, storage charges or reproducible SQLite replay.

## What this establishes / does not establish

- A **new observation** from a native-like service attempt can replace missing historical code information **only if the observation actually discriminates failure from normal function**.
- An absent output cell currently reads as integer zero; the actor's own valid zero result therefore aliases absence, and the hardcoded self-check cannot detect that failure.
- A pre-existing output equal to the expected value can mask **no new production at all**; merely reading stored output does not prove a fresh service attempt occurred.
- A mismatched stale output can trigger a correct edit in this **narrow engineered neighborhood**. It does not prove the program discovered the right fix, because the `current_edge-1` candidate and objective were human-programmed.
- The 4-register, ordinary-cell CIL substrate can express causal functional feedback; it cannot by this evidence be said to originate its own goals, accumulate unscripted adaptation, infer historical identity or achieve cyberlife.

## Next falsifier

An unbiased successor experiment must distinguish *fresh output produced by this trial* from *old matching output* without a trusted external observer injecting a success flag, and must maintain the distinction for input value zero. Introduce **native-owned attempt-specific challenge/response** and resource-accounted evidence, with adversarial stale-but-matching messages and damaged evidence protocol controls. A challenge whose expected output was directly given by the researcher is still a programmed diagnostic, not life.
