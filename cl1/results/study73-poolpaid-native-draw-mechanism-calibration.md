# CL-1 Study73 pool-paid Draw ignition — CI-verified generic substrate calibration

**Status: six human-authored fixtures, ZERO independent ecological worlds. Not cyberlife, endogenous repair, resource creation or survival.**

## Frozen registration and independently audited execution

- Pre-registration: `cl1/studies/73-preregistered-generic-pool-paid-draw-ignition-calibration.md`, commit `bd45a4e81ca9d7a570a2e926b75f188ad92723fd`, **before** the new Rust opt-in resource law and scripted fixture runner.
- Original frozen source revision `69a61b038e8bfaf23ca18ac87e2c6b373ef413a9`, independently verified GitHub Actions [43580314222](https://github.com/iFrankStudio/cioc/actions/runs/43580314222) **SUCCESS**, archived source-pinned SQLite and original independent JSON in artifact `cl1-study73-pool-paid-native-draw-calibration`, ID `14108300732`.
- **Six scripted original native runs, 24 epochs each, 144 independently SHA256-replayed snapshots**. Strict Rust world/kernel workspace tests, formatter, Clippy and six-mode independent Python event/credit/source/PC audit passed. Original CIL opcode ABI unchanged; opt-in policy bit is disabled by default and omitted from serialized old-world states; new successful/failed pool-paid attempts carry `pool_paid_attempt: true`.

## Mechanism

A process with **zero local credits** and next opcode `DrawCompute` may execute that very instruction if opt-in `pool_paid_draw` is enabled and the common world compute pool is positive. The **single attempted-instruction charge is deducted from the common pool**; the native CIL `DrawCompute` then transfers its register-specified positive quantity from the *remaining* pool. A stopped actor or an actor pending a non-Draw opcode is never eligible; old positive-credit actors still pay their own charge. A failed/insufficient-pool Draw **traps** and does not revive. Total inventory remains `original actor credits + original pool - total native attempted instructions` without external-inflow actions.

| Human-authored condition | Charged native attempts | Pool-paid native Draw attempts | Successful shared `CellWritten` | Native traps | Final pool |
|---|---:|---:|---:|---:|---:|
| Opt-in, pending native Draw, pool6 | **5** | **1** | **1** | 0 | 2 |
| Legacy policy disabled | 1 | 0 | 0 | 0 | 6 |
| Opt-in but next opcode not Draw | 1 | 0 | 0 | 0 | 6 |
| Opt-in but world pool0 | 1 | 0 | 0 | 0 | 0 |
| Opt-in, Draw asks more than remaining pool | 2 | 1 | 0 | **1** | 2 |
| Opt-in but actor already stopped | 0 | 0 | 0 | 0 | 6 |

The success fixture begins with one originally endowed local credit, used to execute native `Const` setting the amount register; at zero credits the actor invokes the new *world-paid entry* into native Draw and then spends the acquired three credits to execute `Const → Write → Halt`. Initial source code, native actions and the policy are **researcher-authored**. No process-specific host `SetCredits` occurs during the observation epochs.

## Interpretation and next critical gate

The observed Study36/67 zero-local-credit/pool-positive **ignition deadlock can be removed at the generic VM scheduling substrate level for processes already waiting at a reachable `DrawCompute`**. This is an environment law, **not** an organism initiating resource generation or proof of self-sustainability. Neither the availability of energy nor the policy guarantees a stochastic program reaches/chooses Draw or a useful feedback circuit.

A subsequent independent, prospectively registered **random-code ecological** trial must compare opt-in off/on on paired new genomes under *identical* ambient pool inflow, no host per-process starter, original Read-tainted output, late-window W2/W3 native Draw and function, intact vs Draw-/Read-ablated controls and exact `pool_paid_attempt` provenance. Existing separately frozen Study73 random ecology is not a trial of this opt-in policy unless its runner explicitly enables it; do not recast its results.

**CL-1 remains OPEN.**
