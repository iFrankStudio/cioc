# CL-1B / B15 — Exhaustive cross-lesion generalization of a mutable candidate-policy step

**Prospective protocol frozen 2026-11-15 before B15 implementation and outcome inspection.** This is an **observer-run evaluation** of a researcher-authored policy switch, not endogenous invention or natural selection. The B14 35-node founder and its real CIL `PatchConst` at PC22 remain the native mechanism.

## Question

B14 showed that a hand-coded step-size switch in the individual’s future code-edit candidate generator helps when the service edge is damaged to PC10, but makes PC11 recovery slower than a policy retaining step2. Across **all possible one-edge lesions of PC6.next**, how often does the switch actually help, harm, or make no difference under equal compute?

## Fixed founder and cost-matched ablation

Use the [B14 founder](../../cl0/crates/world/tests/cl1b_b14_search_policy_mutation.rs) unchanged in the *adaptive* arm. In the **cost-matched no-switch** control, alter exactly one **researcher-provided initial opcode literal**, PC21 `Const r3=1` to `Const r3=2`. Both paths still execute all the same CIL diagnostic and `PatchConst` operations, pay for those instructions, and write the candidate-policy literal at PC29; the control's `PatchConst` simply writes **2→2**, while the adaptive arm writes **2→1**. No host bypass edge or global disabled-edit rule is needed. This is source-different by a single literal, openly acknowledged, not a genuine within-life natural selection comparison.

One process, initial compute 240, `World::with_strict_capacity(16,64,128)`, input key7, marker key13, service key11; no pool or births. External initial `PatchEdge(process1,node6,next X)` only, with X varied exhaustively among the integers0..34 except healthy7. Execute **up to 120 charged CIL attempts**; retain trapped/stopped outcomes in the denominator. The candidate's own second-failure policy rule is still predefined.

## Mutation universe and partitions

Five fixed inputs `[-13,0,7,23,101]` × **34 damaged successors** × **2 policy arms** = **340 native worlds**, with five healthy7 × two policy baseline worlds (10 further, total 350). Before inspecting outcomes, tag:
- **Development lesions:** X∈{10,11}, reused from B14. Must not be called holdout.
- **Holdout lesions:** remaining 32 damaged next edges. These are **new tests of a frozen B14 rule** (not unseen from the investigator's theoretical ability to inspect code). No arm is filtered for viability, traps or low output.
- **Healthy controls:** original next7, both policies.

## Primary measurements and precommitted interpretation

- First, independently count actual native CIL writes `CellWritten(key11,value=input)` across the fixed attempt window. Never use marker count alone as function score. Also record marker writes separately, unintended key1 writes, native code-edge edits, actual execution of `PatchConst`, self-policy literal at end, attempted instructions, process state, traps and total compute inventory.
- For each matched input×lesion pair, compare **correct service count**: switch wins / loses / ties (no cherry-picking). Count separately development lesions and holdout lesions. Equal *initial compute, code size, and opcode count per guarded cycle* do not imply equal executed trace lengths, because lesions can bypass arbitrary instructions; report realized attempted steps.
- **Universal-benefit hypothesis is falsified** by *any* paired lesion/input in which the switch produces fewer correct outputs than the no-switch variant. The narrower interesting claim would be improved median held-out outcomes without excess traps, but it is **not assumed** in advance and must be tested; an aggregate win does not establish spontaneous life.
- Both policies should be inactive on healthy PC7 if no CIL failed trial occurs. If healthy baseline differs, fail the control.
- Zero-input outcomes remain in the denominator, and zero-valued output is valid service only when an **internal CIL Write event** occurs.
- Explicitly count cases where a variant traps *before* reaching120 attempts. These do not get zero padded into fake attempts, nor silently rescued by the observer. If a failure is caused by self-modification, keep it.

## Verification and limitations

Implement a separate **source-parsed, restricted independent interpreter** of the committed B14 founder, then a native Rust integration test reading the same founder with the one-literal counterfactual. Since the native CI has not been verified, observer results must remain labelled non-native until a source-pinned Rust test artifact is retrieved. Do not mix the adaptive-vs-no-switch comparison with host selection of an individual winner. Do not retrofit the next policy based on B15's observed outcomes under the same label.

Scientific objective after B15: stop relying on single hand-designed diagnostic signals and test whether **the individual itself** can acquire independent functional evidence and internally revise criteria under *prospectively withheld* disturbances, with cost and provenance accounting. Self-modifiable fixed heuristics are a mechanism precursor, not autonomous open-ended evolution.
