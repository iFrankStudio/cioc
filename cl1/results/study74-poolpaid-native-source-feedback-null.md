# CL-1 Study74-P — independently verified prospective opt-in pool-paid native Draw causal NULL

**STATUS: full CI-PASSED, prospectively registered two-endpoint causal trial, no life claim.**

## Frozen population and integrity

- Prospective protocol committed before implementation and seeds: `cl1/studies/74-preregistered-poolpaid-native-feedback-causal-ecology.md`, commit `385eed2b04`. Fresh **128 independent** SplitMix64/LCG seed-genomes 5,000,000..5,000,127; **16 matched variants per genome**; exact 24 epochs×12 native attempt quota; complete **2,048 original runs and 49,152 exact SHA256 epoch-replayed snapshots**.
- Source-pinned [GitHub Actions run43581225480](https://github.com/iFrankStudio/cioc/actions/runs/43581225480) **SUCCESS**, original artifact `cl1-study74-poolpaid-native-feedback` ID `14108087207`: unchanged SQLite plus independently audited report. Original CIL generator, matched low/high source and Draw-ablation arms, both broad and short Branch topologies, true native PC/source/credit event provenance and EXTERNAL world-pool input were all audited.
- One process starts with **64** actor credits and ambient world pool96; +8 credits externally added to world pool on epochs1..23; **zero host `SetCredits` interventions** after initial setup. The only causal rule difference is `pool_paid_draw` ON/OFF. ON permits exactly a zero-credit, unstopped actor whose *next* opcode is actually `DrawCompute` to pay one attempted instruction charge from existing shared pool. It does not mint compute or revive a stopped actor.

## Frozen primary A: actual late native resource-acquired input→output function

| Registered W2 and W3 source-sensitive Read→Write with native Draw in W3 and cumulative native attempts >64 | Policy ON | Policy OFF |
|---|---:|---:|
| Broad-backedge native CIL | **3/128** | **3/128** |
| Short-backedge native CIL | **0/128** | **0/128** |

Broad paired discordance: ON-only0, OFF-only0, both3, neither125. No registered directional improvement from pool-paid entry. The 3/128 broad positive native joint-function worlds **also occur when the rule is OFF**: they demonstrate that some newly sampled random CIL genotypes can access compute *before exhausting initial credits* and produce source-dependent late output, **NOT an effect of pool-paid physics**.

## Frozen primary B: any native W3 execution, a separate substrate measure

| Native W3 charged attempt worlds/128 | Policy ON | Policy OFF |
|---|---:|---:|
| Broad-backedge | **10/128** | **10/128** |
| Short-backedge | **0/128** | **0/128** |

Broad paired discordance: ON-only0, OFF-only0, both10, neither118. Exactly **one of the 128 broad policy-ON worlds** emitted an independently attributed *actual* pool-paid native Draw attempt, without yielding extra W3 functional outputs or extra W3-active worlds. This is proof the new VM policy can be exercised under stochastic CIL, not proof it changes survival probability.

## Interpretation and next research gate

Two distinct prospective outcomes **fail to show causal improvement** from the narrow environment rule. This is scientifically consistent with the separately registered retrospective Study74-D finding that **no** original Study73 genome reached an eligible zero-credit-at-Draw state at **any saved epoch boundary**; it is not a formal proof that such a law can never help future native programs or unobserved instruction states.

**Important contrast across separate, NONPOOLED genotype cohorts:** Study73's original 128 genomes produced 0/128 strict coupled output, while Study74-P's completely new 128 produced 3/128 broad under BOTH policy variants. This is sampling variability plus source-independent native Draw opportunity and may not be interpreted as an ON effect. Do not aggregate across sources or change the predeclared gate.

**Updated decision:** neither a wider pool nor an additional special-case zero-credit gateway by itself creates an enduring closed functional organization. The next prospective experiment should vary **generic temporal wiring among ordinary Read/Write/ALU/Draw instructions**, with identical native instruction counts and matching source/energy controls, rather than targeting actor rescue or choosing lucky seeds. Any source-sensitive output must retain native Read→Write def-use, resource acquisition and genuinely late output; persistent function cannot be inferred from mere W3 charged attempts. **CL-1 remains OPEN** (no native construction of function, endogenous repair, heredity or open evolution).
