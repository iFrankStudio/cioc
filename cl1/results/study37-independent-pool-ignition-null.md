# CL-1 Study37 — verified independent environmental ignition result

**Scientific status: PROSPECTIVE NEGATIVE on W3, positive narrow native startup calibration; NOT LIFE.**

## Frozen source and independent verification

- Pre-registration committed before implementation/outcome at `67a144ad70633df2bbf654cc7e63015d8e81b64f`: [Study37 protocol](../studies/37-preregistered-native-pool-access-ignition.md).
- Actual code/source revision `47c168f675a0619760ebd0425ee9384581bc64dd`; unchanged CL-0 native VM/kernel, original nine-op scaffold and Study35 randomized CIL grammar; new seeds **2,800,000..2,800,127**.
- Independent CI [43550566473](https://github.com/iFrankStudio/cioc/actions/runs/43550566473) **success**. Artifact `cl1-study37-native-pool-ignition`, ID `14096290863`, includes source-pinned, hashed SQLite and independent `cl1-study37-report.json`.
- **128 new independent worlds**, seven matched conditions, **896 runs**, **21,504 fully hashed and replayed epochs**. The immutable registered intervention chronology, exact process/actor attribution, full 189-credit prefork inventory and all native attempt debits passed independent audit.

## Frozen primary: No W3 gain

| W3 active worlds (epochs16..23) | Count | Wilson 95% |
|---|---:|---:|
| Untreated `control` | **0/128** | [0, 0.029137] |
| Resource-conserving `pool-access` | **0/128** | [0, 0.029137] |

Paired discordance: policy-only **0**, control-only **0**, both **0**, neither **128**. **No prospective W3 improvement.** Generic environment-side transfer of 1 existing pool credit to a halted *scheduler opportunity* with a pending native DrawCompute does not overcome the late-epoch quiescence in this sampled grammar.

## Mechanistic secondary observations and controls

| Arm | W1 native activity | W2 native activity | W3 native activity | Any environmental ignition | Policy transfers |
|---|---:|---:|---:|---:|---:|
| `control` | 128/128 | 55/128 | 0/128 | 0 | 0 |
| `pool-access` | 128/128 | 55/128 | 0/128 | **7/128** | **7** |
| `empty-pool` | 128/128 | 16/128 | 0/128 | 0 | 0 |
| `pool-access-empty-pool` | 128/128 | 16/128 | 0/128 | **1/128** | **1** |
| `sham` | 128/128 | 55/128 | 0/128 | 0 | 0 |
| `external-refill` | 128/128 | 102/128 | **85/128** | 0 | 0 |
| `pool-access-zero-credit` | **32/128** | 3/128 | 0/128 | **32/128** | **82** |

The complete cohort had **90** explicitly external resource-conserving transfers (7+1+82). The independent audit verified each permitted transfer first debited one unit from the world pool then credited one *already stopped-by-the-scheduler, but not halted/trapped* process with `DrawCompute` as its exact next native opcode. All **90** resulted in successful corresponding native `DrawCompute`, tracked with event origin `Internal`. None of these environmental transfers minted energy; every native attempted operation still consumed one unit total inventory. In the initially empty-pool arm, 1 late transfer became possible because ordinary native `ReturnCompute` had subsequently recycled credits back into the pool, a legitimate negative-control nuance.

**Zero-credit source positive calibration:** 32/128 worlds were able to execute from an externally zeroed process balance only after **the external access rule** supplied an initial execution credit. This proves the deadlock is mechanically addressable under a host law, **not endogenous self-starting**. The direct host-refill arm's 85/128 W3 activity remains a **separate external subsidy** and cannot be counted as autonomy.

## Lessons and next registration gate

- A policy restricted to an actor whose *current next CIL instruction* is `DrawCompute` resolves some zero-credit access barriers but is not sufficient for W3 sustained execution under the existing finite-189-credit ecology.
- Do not claim that added pool credits alone revive actors that already cannot execute; replenishment and environment access are separate causal variables.
- The next useful prospective test should treat periodic **ambient pool input** and **generic zero-credit activation** as two orthogonal environmental laws, with explicit energy source ledger and no hardcoded fitness, repair, reproduction or selected actor reward. No amount of environmental rescue by itself is evidence of native self-maintenance.

**CL-1 remains OPEN.** Study37 meets an external resource-access mechanistic question only, not autonomous organization or life.
