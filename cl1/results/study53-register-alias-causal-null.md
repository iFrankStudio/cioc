# CL-1 Study53 — prospectively verified null: separate graph source/result registers do not resolve functional-child bottleneck

**Status: independent paired causal register-topology trial passed CI; both predeclared endpoints were ZERO. No cyberlife.**

## Source, population and archived replay

- Frozen prospective registration: `cl1/studies/53-preregistered-graph-register-alias-causal-trial.md`, commit `9824060f2510ea5dd5c60a35ea76f9f85ab5ed39`, **before** code and seed outcomes.
- Native program source revision **`0cf22880fd212f5a74bbaeff74ce327f5065624f`**, independent seed IDs3,700,000..3,700,127; **128 independently selected paired seeds** across two intentionally different CIL initial program regimes, four externally controlled environmental arms/regime; 1,024 full native executions, **24,576 original hashed and exact-replayed SQLite epochs**.
- Independent GitHub Actions [43557895526](https://github.com/iFrankStudio/cioc/actions/runs/43557895526) **success**; artifact `cl1-study53-paired-graph-register-alias` ID `14098634727` includes both untouched SQLite archives and independent paired Python report.
- Rust CIL runtime/kernels unchanged. Independent auditor confirms exact initial-program diff consists only of `GraphCopyNode.dst:2→0` and corresponding `GraphSetNext.node:2→0` in the randomized continuation. Original random opcode choices, seeds, initial scaffold, world storage, quotas and environment/credit provenance otherwise match exactly. Two initial programs from each seed form **one correlated seed pair**, not two independent samples.

## Frozen primary results (ambient-general arm)

| Registered endpoint | Original source/result alias topology | Separate source/result register topology |
|---|---:|---:|
| **Complete native copy→edit→launch chain copying an initial Write source node** | **0/128** | **0/128** |
| Successful native-launched child CellWritten shared output | **0/128** | **0/128** |
| Wilson95 interval for each zero endpoint | [0,0.029137] | [0,0.029137] |

Paired primary-A discordance: alias-only0, separate-only0, both0, neither128. The treatment-specific prospectively declared success criterion **did NOT replicate**. A source/result register alias is an intelligible mechanistic obstacle, but its separation *alone* is **not sufficient** to create a successful Write-source assembly path in this cohort.

## Native opportunity controls

- Worlds with internally attributed child launch: **64/128** alias, **66/128** separate.
- Worlds with any strict original-parent graph copy→edit→launch: **1/128** alias, **0/128** separate.
- Worlds with non-GraphNew child opcode: **2/128** alias, **0/128** separate.
- Worlds whose launched child graph contained Write: **0/128** in both.
- No primary source Write-based native graph assembly, no child function, and no endogenous functional restoration in either register topology.

## Decision

The register-topology change did not causally increase the frozen endpoint. Avoid counting a structurally closed child graph or successful GraphNew as useful offspring. Next investigation should isolate *which native graph-copy opportunities and instruction-source types are even sampled before launch*, independent of whether a child can execute; do not keep adjusting random generator probabilities based on a few post-result seeds or claim that access to an extra register is a life mechanism.

**CL-1 remains OPEN.**
