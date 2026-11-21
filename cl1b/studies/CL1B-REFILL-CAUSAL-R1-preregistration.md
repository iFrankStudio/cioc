# CL1B-REFILL-CAUSAL-R1: preregistered original-native surgical refill-control experiment

**Frozen before first original Rust native run** against `main 7e7b3c80d5aaa6e4c3791659a0abefc3848e8205`. Original F6 exact source, the 180 original CIL nodes and 16-data / 180-source / 300-occupied / 6000-private-credit law are unchanged. The original P4F6 evidence is [run 44862741694](https://github.com/iFrankStudio/cioc/actions/runs/44862741694), exact source SHA `26d94bb8720f57d0593f2b0618e07e1db977759c`.

## Causal question

Does *allowing quota restoration itself* improve behavior or reduce fully charged compute, compared with keeping F6’s same paid monitoring and same-source edits but suppressing only the actual budget-refill state effect? Previous whole-`PatchNext` suppression and F5/F6 cross-source performance cannot identify that estimand.

## Researcher-only opt-in original native control

At **PID1 PC174** only when the actual `Write` is register-resolved to **cell38, value0** and all ordinary VM validation succeeds, use a native host study-control that **charges the original CIL instruction** and advances original PC but does not mutate cell38. Audit as `NativeCellWriteSuppressed`. Original `PC152 Write38=1`, `PC172 Write39=0`, all task `Write11`, all other data-cell writes, `PatchConst` and `PatchNext` retain their normal action and credit charge. An invalid intercepted opcode/target/value must trap, never silently broaden scope. This is an investigator intervention, **not** a new actor ability, host optimizer or hidden task oracle.

**Placebo** configures the identical intervention machinery for **unreachable PC180**. It must produce the **same complete paid-instruction trace** as F6 original; zero placebo exposures are required. Actual F6-vs-suppressed trajectories must be identical until the first targeted `PC174`, then may diverge naturally. A no-exposure pair is not evidence of a null causal impact.

## Frozen, independently testable matrix

**45 genuinely new frozen 24-goal vectors:** five prespecified generating regimes × start0,1,2 × stride0,1,2; deterministic xorshift32 initial state `0xC11B4A71` gives each case a fixed 1-or-2 step perturbation. Families are widely separated persistent shifts with isolated disturbances; tightly spaced permanent shifts; fixed-seed sparse impulses; late temporary shift with reversion; and stable no-refill negative control. Each exact goal vector appears in the frozen JSON. Do not tune vectors or mixture after original native results.

Each fresh case has four completely independent original PID1 worlds: F6 ordinary; same-loaded-F6 `PC174` refill-suppressed; same-loaded-F6 unreachable-PC180 placebo; distinct F5 source as **secondary, noncausal** comparator (**180 fresh truthful native worlds**). Additionally **144 already-exposed P4F5 goal sequences** are tested with two original F6 arms (ordinary vs refill-suppressed; **288 exploratory truthful worlds**). Two adversarial environment-feedback schedules crossed with ordinary/suppressed (**4 worlds**). **472 original native worlds total**, 468 truthful, 4 adversarial, no across-world PID memory.

**Primary:** actual 24-stage truthful completion first, then paired `suppressed−original` privately paid CIL instruction cost *only where both complete the same true task count*, case-wise and by regime. A cheaper incomplete task is not a win. **Secondary:** actual paid service probes, suppressed replenishments, subsequent online `PC122` edits, monitor re-arm and disarm, memory cells, original credits, false-feedback controls. All full 472-world native traces must be independently replayed opcode-by-opcode, including PC, register, cells, exact charged credits, source edits `PC12,PC34,PC46,PC133`, occupancy and feedback; first divergence must be null. No extra priced probe overhead or post hoc environment reweighting.

**Fail closed:** original 99 native regression targets and 63 observers remain passing; source-shape/mutation preflight stays green; errors/failed CI logs preserved. Freeze survives any implementation fixes; result must be source-pinned in evidence branch after full native CI. **Interpretation:** a successful positive causal result validates the *researcher-authored* refill rule only in stated environments, not independent invention, generalized viability or cyberlife.
