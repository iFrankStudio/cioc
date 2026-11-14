# CL-1 Study63 — independently verified SCRIPTED native sensor feedback and compute-pool access feasibility

**Status: HUMAN-AUTHORED CIL MECHANISM CALIBRATION, zero independent ecological worlds. No digital life.**

## Frozen source and provenance

- Registered before runner/auditor at `cl1/studies/63-preregistered-native-feedback-resource-feasibility.md`, first commit `134d15b6a554294d84003bbf5f4b33e35f74f426`.
- Actual source pinned to **`78501017d52a822415cfe3066eda8a564137d67c`**, passed full [GitHub Actions CI #43566295425](https://github.com/iFrankStudio/cioc/actions/runs/43566295425). Archived original SQLite and independently produced JSON as artifact `cl1-study63-scripted-feedback-energy-calibration`, ID `14102840755`. Full Rust tests, format, strict Clippy and exact native epoch replay passed.
- Six **human-authored** eight-node process code fixtures; **48 original SHA256 epoch snapshots**. Exactly the registered CIL `Const → DrawCompute → Read → Const → Add → Write → Const` loop; low/high input and ablation/recharge controls were researcher-provided; no stochastic genome, no independent ecological world and no endogenous code construction.
- Original external setup `Loaded`, `SetCell(key7)` and `SetComputePool` events are all explicitly tagged `External`; only the `sensor-switch-native-pool` variant has one externally administered information stimulus `SetCell(key7,value8)` at epoch4. There is **no** host actor-credit intervention during any epoch.

## Independently verified exact outcomes

| Fixed human-scripted condition | Native attempts | Successful native DrawCompute | Native CellRead | Native CellWritten(key9) | Output values |
|---|---:|---:|---:|---:|---|
| Input3, world pool200 | 96 | **16** | 16 | **15** | only **11**, all epochs0..7 |
| Input8, world pool200 | 96 | **16** | 16 | **15** | only **16**, all epochs0..7 |
| Input3→8 at epoch4, pool200 | 96 | **16** | 16 | **15** | 11 before and at switch boundary, **16 from epoch4 onwards** |
| Input8, pool200, Read replaced by Const0 | 96 | **16** | 0 | **15** | only **8**, all epochs |
| Input8, **pool0**, initial actor credits3 | 3 | **0** | 0 | **0** | none; native DrawCompute trapped |
| Input8, **actor credits0**, pool200 | 0 | **0** | 0 | **0** | none; actor never became runnable |

Every successful native output is causally preceded by a native `CellRead(key7)` and passes a separate register2→register3 `Add`→`Write` def-use check (apart from the explicitly ablated Read arm). Native DrawCompute always transfers eight credits from researcher-endowed pool to actor and costs one credit for the attempt, preserving `initial process credits + initial pool − cumulative attempted native steps`. Full source, epoch actions and original snapshot hashes audited. A source change may be observed with a one-epoch-loop delay at the epoch4 boundary; it is correctly distinguished from independent external output.

## Scientific decision

This proves **the existing unchanged Rust/CIL primitives already support sustained native sensor-input-dependent output and actor-initiated compute harvesting from available world pool, once an actor is runnable**. It also reconfirms the zero-credit ignition deadlock with plentiful pool energy.

It does **NOT** demonstrate spontaneous feedback organization: the entire eight-op instruction order, input/output addresses, loop and initial credits were imposed by the researcher. The next independently preregistered Study64 tests whether randomized CIL instructions can naturally produce repeated source-sensitive Write behavior under an explicitly researcher-provided generic sensor/output interface and a matched cyclic vs linear connector, with finite initial endowment and no host starter. This will be an *intermediate feedback-opportunity assay*, not a life gate.

**CL-1 OPEN.**
