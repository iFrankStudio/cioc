# CL1B-REFILL-GATE-G1: strict-180-slot native contextual refill gate (preregistration)

**2026-11-21 UTC. PRE-NATIVE freeze.** Parent source: [R1 full native 101/101, 63/63, 472-world independent replay](https://github.com/iFrankStudio/cioc/actions/runs/44906052116), exact original F6 180-node CIL from the R1 frozen JSON. Parent PC155 observational-alias audit verified by [#44911175424](https://github.com/iFrankStudio/cioc/actions/runs/44911175424). That retrospective analysis establishes only a weakness of an *early irrevocable binary choice*; no learned gate or downstream benefit.

## Scientific question

Can a **researcher-authored but truly executed and paid CIL conditional refill** make a better total-compute tradeoff than identically implemented unconditional allow/deny controls, *without extra source slots, data cells, permissions, or extra host hints*? This is a feasibility/falsification study of an **actor-side** decision, NOT an autonomous discovery claim.

## Locked founder and 4 arms

Use the same original F6 source as the parent and exactly 180 source instructions. Four independent single-PID1 worlds per identical 24-goal schedule:

- `F6_parent`: exact frozen original 180-node source.
- `G1_conditional`: construct a full-length 180-node source from F6, touching **only original PCs 171..179**, all source resident from birth. At original monitor threshold after PC170, reset paid cell39 at PCs171/172, then `PC173 Branch(test=r3, nonzero=174, zero=176)`: nonzero last *actually attempted* task output allows a paid `Write38=0` at PCs174/175; zero output bypasses that write, with all paths still disarming the monitor through paid `PatchNext PC178`, then using already-existing `PC126` to read current stage. The decision is computed by a normal paid Branch and uses actor-owned r3; no external task true value, oracle or experiment tags. This is a deliberately simple **authored candidate** chosen before the new worlds.
- `G1_always`: identical 180-slot G1 graph and price, but `PC173` has both branches route to PC174, enabling all refills.
- `G1_never`: identical 180-slot G1 graph and price, but `PC173` has both branches route to PC176, skipping all refills. The native VM must never intercept a cell write in these arms; differing source instruction pointers encode the policy.

Exact new gate suffix from PC171 onward (others bit-for-bit F6):

    171 Const r2=39 ->172
    172 Write cell[r2]=r0 (=0 at successful PC170 threshold) ->173
    173 Branch r3, nonzero=174/176, zero=176/174 (arm-specific)
    174 Const r2=38 ->175
    175 Write cell[r2]=r0 (=0) ->176
    176 Const r1=133 ->177
    177 Const r0=62 ->178
    178 PatchNext node[r1].next=r0 ->179 [real paid disarm of own PC133]
    179 Const r2=32 ->126 [reuse original stage-read PC126]
    126 Read r1=cell32 ->62 [original node, unchanged]

The raw initial code variants are **researcher seeded**, NOT source invented by PID1. The split only evaluates whether a conditional actor-local primitive can work under strict limits; it does not yet measure environmental future utility or learn a rule. In particular, using numeric output r3 is not justified as a statistically predictive feature.

## Exact constraints and external feedback

Original world `World::with_strict_capacity(16,180,300)` and `Process::new(1,source,6000)`; original unmodified task channel: real pid CIL `Write11` at PC3 then researcher environment sets cell20 to 1 iff current r3 equals frozen hidden stage goal, 0 otherwise. PID1 must autonomously discover correctness by reading the paid ordinary cell20. Real task completes only after true goal is solved; assert 24/24 and honest truth/claim. All ordinary CIL, PatchNext, PatchConst, source edits, environment feedback and memory billed under original native VM physics. No opt-in native Write suppression, no oracle, no extra compute, no free graph capacity.

**New prospective 45 goal vectors**, generated once using exact xorshift32 state `0xC11B6A7E` and frozen as a literal JSON manifest *before* the first G1 Rust native execution. Five predetermined schedule families, each `start0,1,2 × stride0,1,2`: stable rotation, wide permanent double shift, short two-pulse reversion, late regime inversion, deterministic sparse impulses. Every schedule 24 stages, each goal in {0,1,2}. The full manifest contains those 45 literal goals, the exact F6 and all 3 G1 sources, the explicit arm assignments. No post-result tweaks, environment-family reweighting, or reused R1 vector counted as novel.

**4 additional invalid feedback control worlds**: inversion and forced-zero feedback for conditional and always, so a falsely claimed success must not count as true work.

## Primary, secondary, and falsifiers

**First endpoint:** actual 24/24 true completed stages for each truthful arm. If any actor fails or exhausts paid CIL, it is **not a cost saving**. Conditional vs same-source G1 always/never is primary; F6 only a different-code historical reference. Compare total charged native instructions paired by goal vector, report per-family/overall, no posthoc favorable mixing. Do not subtract task probes as an invented additional price. Actual actor-paid PC173 decisions, PC175 allowance, PC178 disarm, PC122 online edits, emitted source edits, task probes, used cells, occupied storage and credits are secondary.

Mandatory falsifiers:
- G1 conditional different decisions both observed under some schedules; without this no context-dependent behavior shown.
- Exact strict 180-node source length and 16-data /300-occupied /6000-credit law, no host edit or extra PID; original VM error if violated.
- For every case, initial G1 sources differ **only at PC173** (conditional/always/never), and F6 original remains exactly frozen.
- All truthful worlds honest 24 tasks; deceptive feedback worlds must be marked untrustworthy and not promoted as learned competence.
- Native original trace, including registers, PC, memory and source edits, independently replayed by a separate Python interpreter using **frozen loaded CIL**, not Rust summary predictions.
- A negative or regime-specific result stays negative, does not justify new threshold tuning. Historical negative aliases are already exposed and MUST NOT be rebranded as new holdout successes.

This work is an actor-side conditional rule **authored externally**. **B3/B4 remain NOT MET** unless an additional, independently preregistered experiment shows a persistent individual inventing/revising its policy from paid native evidence with appropriate cross-regime transfer.
