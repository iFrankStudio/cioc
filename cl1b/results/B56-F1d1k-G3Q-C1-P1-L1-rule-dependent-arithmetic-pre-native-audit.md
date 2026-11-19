# CL-1B B56-F1d1k-G3Q-C1-P1-L1 — paid retained rule influences original-PID arithmetic

**Status (2026-11-19):** precommitted exact **179-node CIL founder**, native Rust test code and independent source/aggregate checker have been committed to `main`; **no confirmed exact-SHA native CI PASS or native event-by-event replay yet**. This study is **NOT** full C1-P1 hidden task learning. It is a narrowly falsifiable intermediary beyond P0's readback-only bridge.

## Narrow causal question

Can a single original PID1 that has paid to certify all64 authored meta-rule descriptors (24 accepted,40 rejected) later pay to deploy its own saved three-word bank and make a **new arithmetic output** dependent on a dynamically paid descriptor-byte Read, instead of only echoing a stored word?

Unlike P0, the final `Write11` is **calculated** from two original PID-owned values. This still does **not** implement a genuine three-word interpreter for hidden task execution, and no hidden task input is provided.

## Precommitted source and code-size accounting

- Exact [frozen 179-node CIL source](../studies/B56-F1d1k-G3Q-C1-P1-L1-frozen-179-node-original-rule-dependent-arithmetic-v1.json) committed **before** the new Rust test, commit `4c398fafbf24f08e7c8196d8c2ba0ab2dbe9a280`.
- C0's original redundant `Read11` after its structural certificate was removed: `Write11` leaves the same verdict in source register3, so direct `Branch` retains verdict semantics without those two instructions. Control-flow targets were address-renumbered. This is a **fresh researcher-authored source refactor**, not the original C0 executable source self-editing.
- The final original C0 Halt and descriptor enumeration exit are replaced with source-paid post-coverage bank deployment, then a paid arithmetic service. Total **179/180** CIL code nodes; exactly 16 potential data keys and original strict storage bound `World::with_strict_capacity(16,180,300)`.
- Original structural certificate remains `Write11` at PC106. After all64 descriptors and safe-only retention, active bank data come from `Read51/52/53` at PCs162/166/170 and paid `Write62/63/64` at PCs164/168/172. Existing stored bank is `[1,3,2]`. At PC174, source reads actor-generated final descriptor index cell65=63. At PC175, it pays `Add` on index63 and the **last dynamically deployed rule word**2, then at PC177 emits arithmetic `Write11(65)`. The two meanings of `Write11` are distinguished by **source PC and phase**, not by a posthoc output label.
- **No external input** is injected; index63 is an endogenous task-like operand and **not a hidden or heldout task**. The arithmetic is researcher-authored, not discovered by PID1. Neither genuine source-native three-word meta-rule reexecution nor fitness feedback is present in this L1.

## Independent exact frozen-source predictions (not native measurements)

- Original founder: **23,939 charged CIL instructions**, 64 structural certificates, 72 actor-paid safe-bank retention writes, paid bank reads `(51,1),(52,3),(53,2)`, paid active writes `(62,1),(63,3),(64,2)`; endogenous operand63 plus paid active tail2 produces output **65**.
- A **researcher-authored pre-load source mutation control** changes only source PC169's bank-tail-address constant from 53 to51. With the same 23,939 paid instructions, the third active word becomes1, and the output becomes **64**. Original retained bank and index63 remain unchanged. This demonstrates a sharply localized dependence on the original source `Read` path if independently confirmed in native Rust. The mutant is *not* PID1 self-editing.
- Private starting credits32,000; predicted ending credits8,061; total peak occupancy bounded by 179 CIL nodes +1 PID +16 cells = **196/300**. No PID2 or restart, and no host code/data writes after the initial load.

## Implementation and reproducibility

- [Native Rust integration and positive/negative source-mutation arms](../../cl0/crates/world/tests/cl1b_b56_f1d1k_g3q_c1_p1_l1_rule_arithmetic.rs) committed after frozen source.
- [Independent Python source interpreter and exact-source native-summary comparison](../analysis/g3q_c1_p1_l1_native_aggregate_replay.py). Run `python3 cl1b/analysis/g3q_c1_p1_l1_native_aggregate_replay.py` for source-only results. Add path to original native result JSON for aggregate crosscheck.
- Focused CI now registers **74** Rust native targets, including this P1-L1, and executes its independent aggregate check if selected; existing 63 per-event observer targets remain separate. **This source/aggregate check is not equivalent to a full independent replay of each original native instruction**.
- A valid native success claim requires an exact-SHA Rust run actually passing plus original source-pinned evidence and independent comparison; until this is observed, all figures here are **source interpreter predictions**.

## Non-claims and next stronger gate

P1-L1 does **not** demonstrate a hidden task, a genuine task-bytecode interpreter, different probe inputs, truthful/false/absent feedback, learned task choice, complete three-meta-word program reexecution in the task phase, conditional safe fallback, original PID1 source self-mutation, new ISA grammar, Hk, N56-A/B/C, B3/B4, or cyberlife.

Next stronger P1-L2 must make **all three** deployed meta opcodes causally drive a paid task program and output (including a RESET/intermediate accumulator witness), then introduce an actual hidden task shift, source-identity-matched controls, and task-feedback-conditioned adaptation. If this cannot be encoded under 180 nodes, report a bounded capacity failure instead of moving interpretation to the host or widening limits.
