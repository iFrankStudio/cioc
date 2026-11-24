# CL1B C29-R0 — paired input-local arithmetic trials, initial engineering feasibility

**Evidence tier: non-native CIL model, not a native experiment or blind validation.** Parent 86334df18d10364bbc9a60f0f289818105708ffe. C28 frozen evidence untouched. B3/B4 NOT_MET.

## Real task and mechanism

Replace constant-output and artificial padding with integer multiplication on a declared positive bounded input domain x,y in 1..16. A repeats addition of y x times; B repeats addition of x y times. Both compute x*y; each loop iteration is a paid Add, Sub and Branch. Trial span measured between charged own-credit sensors is 11+3*x or 11+3*y. Actual production cost gap is 3*abs(x-y), theoretically 0..45 (selected family reaches 42). No specialized multiplication, compare, prediction or reward opcode. Initial source edge selects A. The actor copies both ordinary input cells50/51 to ordinary private-use data keys40/41 by paid Read/Write, uses this retained pair for both candidate trials, stores trial outputs separately, checks equality, then uses measured costs only in the bounded signed-difference comparison. Tie, disagreement or out-of-range abstains. Reads the real dispatch edge, avoids redundant edits and uses ordinary charged PatchNext to change or restore it. Trials operate on copies; actual production reads the live inputs. External observer validates every actual product and cutoff count. No oracle exposed to guest. Source graph size 167, peak model occupancy179. Fits a proposed strict 200-node, 16-cell, 220-total-unit world; not verified natively.

Snapshot is sequential two-cell copy, **not atomic against arbitrary concurrent changes**. In this engineering family inputs change only after service-counter write or after first trial output (after snapshot completion). No guarantee for writes during capture or for arbitrary invalid values; 1..16 positive domain is a hard experimental boundary. Production inputs are likewise held coherent per service in this family. Trial/production outputs are independently checked by observer only. Pair output equality is necessary but not general correctness proof. Candidates are authored, not invented.

Repeat policy evaluates at entry and once after production stage24, with all per-service gate charges counted. Once comparator executes only initial evaluation with no redundant gates. Same-source physical-off suppresses only actual PatchNext effect, charges attempt, no extra oracle. Live-input control removes snapshot capture and uses live inputs for both trials; distinct source control, not sole-effect suppression. All worlds stopped observationally immediately after Nth paid stage-count write, not actor Halt. No future cutoff input. New model not native or prospective holdout.

## Systematic selected family

Nine operand pairs, horizons 24/48/96, stable and single reversal at eligible boundaries5/23/37. 83 scenarios (27 stable/56 flip), 4 arms each, 332 runs plus 16 stress runs. Deliberately selected engineering mixture; no population inference.

| group | n | net vs once | same-source off-on | wins/losses |
|---|---:|---:|---:|---:|
| stable |27|-6615|10596|0/27|
| flip |56|9612|14424|16/40|
| total |83|2997|25020|16/67|

For (16,2) reversing after stage23, N96, repeat on 2591 vs once 5157 and off 3513. Same small-gap case (2,3) reversing after23, N96, repeat on2368 vs once2219 and off2584. Increased opportunity can support economic gain; not universal, fixed check timing and overhead still fail 67 cases. Different source node counts and data allocation accounted separately, no scalarization.

## Explicit confound: equal output does not imply comparable input

During initial trials, after A's output write, environment swaps (12,2) to (2,12). Snapshot A/B both use (12,2), cost 47/17, output24/24; live A/B use different operands, cost47/47, output24/24. The equality guard passes in both arms but live comparison wrongly reports a tie for the initial input. Local snapshot resolves this particular confound. Another transition (12,2)→(3,5) makes live outputs 24/15, triggering abstention; (2,12)→(3,8) preserves output24 but changes live cost pair 17/35 vs snapshot17/47. No general inference from output equality alone.

Importantly, snapshot's local conclusion can be stale immediately after environment change. In same_product_swap, snapshot initially chooses B based on valid old input, even though A is cheaper for new input; it reverts at stage24. Full snapshot repeat on/off 2109/1387 (negative physical effect), initial-only2543, live repeat1356. Thus local causal identification does not guarantee live utility. No claim that snapshot is universally better economically. A further native gate must preserve and test this distinction.

## Next gate

Before native promotion, freeze exact sources and resource limits plus new input pairs and schedules. Include snapshot and live controls, same-source edit effect suppression, compact once, stable and flip, same-product mid-trial interventions, and capture tearing as a separate contract test if allowing arbitrary host interleaving. Explicitly avoid declaring the sequential copy atomic. Verify original events and independent replay; require production correctness, actual private credit conservation, output comparison, measured pair consistency and real edits/rollback. Do not retune after prospective observations. Only then consider actor-paid admission decisions that compare evaluation costs to prospective benefit without a duration oracle. B3/B4 NOT_MET.

Reproduce with `node cl1b/analysis/cl1b_c29_paired_arithmetic_trials.mjs`. Full sources and scenario rows emitted; model body executed twice in connector-side JS. No native CI claimed.
