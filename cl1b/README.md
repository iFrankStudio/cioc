# CL-1B — Continuous individual evolution

**Status: ACTIVE — initial causal feasibility phase, not a life discovery (2026-11-15).** Replaces population reproduction as the *default* research emphasis. [CL-1A](../cl1a/README.md) remains an archived, paused research route; original historical material remains under `cl1/`.

## Thesis and nonrequirements

Study whether **one logically continuous native digital organization** can maintain activity, alter its own executable information, recover from harmful internal changes, and accumulate effective adaptation under bounded compute/storage and environmental perturbations.

Birth, copy/launch, genetic inheritance, populations, aging, externally specified life rewards, and an immutable 'essential core' are **not necessary acceptance requirements**. Individual continuity is an empirically tested organizational relation, not process-ID equality, byte equality, uptime, or an observer's subjective attribution.

An organism-level conclusion must not be inferred solely from self-modifying code, an automatic rollback instruction, increasing output, a better hand-designed algorithm, or survival of a selected subset.

## Rules of the world

- Keep the accepted CL-0 Rust/CIL VM and generic, resource-accounted graph/information/compute primitives. No host `LIFE`, `REPAIR`, `FITNESS`, `OPTIMIZE`, or dedicated rollback oracle.
- Host snapshots/replay are **observer/laboratory instrumentation** unless a native agent itself constructs, owns, accesses and pays for corresponding recoverable information. Host test-harness restoration does not count as native recovery.
- Allow native code changes via existing CIL operations, including `PatchNext`, `PatchConst`, graph edits and normal writes. Trace internal vs external origin and debit every attempted instruction.
- A candidate may consist of several coupled processes or graph objects **within one causal organization**; do not define separate processes as separate lives by fiat. Account for all component state and storage.
- Researcher-authored seed code, descriptor layouts, protected monitors, repair scripts, and chosen perturbations must be labeled external. They may be used as *positive engineering controls*, never scored as spontaneous invention.

## Research phases / acceptance gates

| Gate | Exact question | Required evidence | Forbidden inference |
| --- | --- | --- | --- |
| B0 substrate | Can one CIL actor change its **own running** code, with causal behavior and finite charges? | single-actor self-edit vs mutation-disabled sham, intact actor identity, event/provenance/credit checks | that an authored edit is self-improvement |
| B1 functional continuity | Does a changing actor retain an independently declared function across multiple nontrivial internal code transitions? | matched initial worlds, measured pre/post function and code hashes, traps and losses retained | that a persistent PID equals persistent life |
| B2 native recovery | After a harmful perturbation, can internal operations recover relevant function without laboratory restore? | damage/sham/disabled-repair/resource-ablated arms, native snapshot provenance and complete resource accounting | that a host replay is self-healing |
| B3 cumulative adaptation | Do retained native changes improve persistence or task performance on **unseen environmental conditions**? | prospective training/holdout split, randomized order, matched edit opportunity/compute and negative controls | that externally selected best-of-N equals endogenous selection |
| B4 endogenous organization | Do the organization and adaptation policy arise from world-local interactions rather than a scripted repair/reward apparatus? | preregistered causal component ablations, multi-window self-maintenance, non-scripted provenance, independent replication | that a single good trace establishes life |

These gates are **sequential scientific claims**, not mandatory complexity of initial software releases. Negative results count. Do not quietly relax criteria or discard failed worlds.

## First implementation

- [Study B00 preregistered mechanism/negative-control protocol](studies/B00-self-edit-feasibility.md)
- [Native focused Rust mechanism test](../cl0/crates/world/tests/cl1b_b00_single_actor_self_edit.rs): a human-authored single-process CIL program patches the control-flow of its own executable instruction; compare with a same-cost opt-in suppressed-patch control. This is an **engineering demonstration**, not autonomous improvement.
- [B01 preregistration](studies/B01-multi-edit-functional-continuity.md) · [focused Rust test](../cl0/crates/world/tests/cl1b_b01_functional_continuity.rs): three function windows after two internally applied graph-edge changes; functional, suppressed, and explicitly damaging controls, five input cases. **Code committed; CI outcome not independently verified.** This is an authored positive/negative calibration, not evolved continuity.
- [B02 preregistration](studies/B02-native-repair-calibration.md) · [focused Rust test](../cl0/crates/world/tests/cl1b_b02_native_repair_calibration.rs): a single CIL actor reads back a damaged executable edge and restores it internally, compared against no-damage and mutation-disabled controls under 40 equal local opportunities. **Code committed; CI outcome not independently verified.** The safeguard was scripted by researchers and is not endogenous recovery.
- [B03a prospective protocol](studies/B03a-computed-self-variant.md) · [focused Rust test](../cl0/crates/world/tests/cl1b_b03a_computed_self_variant.rs): same-process native `ReadNext` + environment datum + arithmetic computes a self-code edit target, with transparent external delta and service-output/resource checks. **Not native trial-and-error or adaptation.**
- [B03b prospective protocol](studies/B03b-native-trial-feedback-selection.md) · [focused Rust test](../cl0/crates/world/tests/cl1b_b03b_trial_feedback.rs): a single CIL actor executes a trial, observes its result, and conditionally edits its future code path; matched patch-suppressed arm. This is a **human-authored decision policy**, not unscripted adaptive emergence.
- [B04 prospective checkpoint protocol](studies/B04-actor-owned-checkpoint-rollback.md) · [focused Rust test](../cl0/crates/world/tests/cl1b_b04_native_checkpoint_rollback.rs): one CIL actor writes an internally observed code edge to a storage-accounted cell, later uses that saved value to undo an external code-edge lesion. Damage, native-patch-disabled, checkpoint-corrupted, and no-damage controls; 20 deterministic worlds. **Only static path simulation checked; native CI not yet verified.** Checkpoint guard is researcher-authored.
- [B05 prospective protocol](studies/B05-reciprocal-monitors-throughput-paradox.md) · [native Rust test](../cl0/crates/world/tests/cl1b_b05_reciprocal_monitors.rs) · [independent simplified path audit](results/B05-static-path-audit.md): two **human-authored** monitors protect each other inside one process. In simplified path simulation, either single monitor lesion is repaired but double lesion bypasses both checks while **doubling raw service output (10 versus 5)** per 100 opportunities. The apparent throughput gain is a negative control for false life/fitness metrics, **not** scientific confirmation or Rust CI verification.
- [Focused B00–B05 workflow](../.github/workflows/cl1b-focused.yml): invokes only seven named Rust integration-test targets, without directly invoking archived CL-1A study binaries. Other pre-existing repository workflows may still run on pushes; this scoped workflow does not guarantee all historical CI is disabled. Successful CI and source-pinned data are mandatory before experimental verification. Rust formatting is not currently an acceptance gate in this narrowly scoped job.
- **Next independent-science obstacle:** B03b uses a programmed choice policy; B04/B05 use programmed self-maintenance monitors. In B05 the two-monitors-disabled actor is a *faster but structurally unprotected* negative control: future evaluations must independently measure service performance, recovery under withheld perturbations, recoverability of repair policy, and paid resources. To approach endogenous continuity, introduce candidate CIL-generated variants, CIL-owned candidate evaluation/rollback and held-out lesions; avoid optimizing against raw activity. The B3 gate still requires evidence beyond scripted adaptation.

Original CI workflows running pre-split `cl1` artifacts may remain as compatibility/regression checks; they are not authorization to continue CL-1A experiments. No autonomous experiment workflow or OpenAI API action should be enabled.
