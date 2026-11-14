# CL-1 Study42 — prospectively replicated native executable plasticity

**Status: CI-VERIFIED independent 128-world replication. Not functional repair, reproduction or life.**

Study42's frozen protocol was committed first (`026fdf333b4c38a2d79db736a96ceca5f0d6959f`), then it copied Study41's exact stochastic native code grammar, 24-epoch schedules, four arms, core VM and independent auditor while changing only the study identifier and **fresh seeds 3,200,000..3,200,127**. The executed source revision is `e2b74edb2f3b39f895f4d69410dde15431bc6309`. Independent GitHub Actions [43553947738](https://github.com/iFrankStudio/cioc/actions/runs/43553947738) **passed**, with original full SQLite and audit artifact `cl1-study42-native-code-plasticity`, ID `14097108465` (**512 runs, 12,288 independently hashed epochs**).

## Frozen predeclared outcomes

| Mechanism (ambient-general arm, 128 original worlds/replication) | Study41 | Study42 replication |
|---|---:|---:|
| Strict original-actor native executable code change attributed to one successful mutator in an epoch | **64/128** | **65/128**, Wilson 95% [0.422247, 0.592923] |
| Final initial-actor executable code differs from original | 80/128 | 75/128 |
| At least one native GraphLaunch with new child process | 52/128 | 51/128 |
| At least one successful child native operation | 12/128 | 17/128 |
| Child successful native ops in ≥2 distinct epochs | 9/128 | 15/128 |
| Original actor successfully continues after native executable mutation | 41/128 | 38/128 |
| Original actor performs successful native shared-cell write after mutation | 15/128 | 14/128 |
| W3 any native attempt | 44/128 | 57/128 |

The strict primary is **replicated** on a new independent seed cohort with unchanged acceptance grammar. Study42 reports **145** successful native mutator events, **58** native child-launch events and **59** native graph-node copies in `ambient-general`, all actor-attributed, with independent source lineage and no mutation/recovery preselection. Finite programmer-supplied random initial code and environment-supplied resources remain **external inputs**.

**Key limitation:** Mutating code does not mean the mutation preserves or improves its behavior; creating a child does not mean reproduction or heredity. `GraphLaunch` may create children that merely execute `GraphNew` in loops, consume finite storage and waste compute credits; a separate original-data retrospective audit must classify native children by successful opcodes, shared output, preservation of parent function, and later-epoch ability to work without a host starter. No claims of endogenous repair, persistent organization or life are justified.

**CL-1 OPEN.**
