# CL-1 Study 21 — verified temporal loss and exogenous functional restoration

**Calibration outcome: PASS. Scientific conclusion: endogenous autonomous functional restoration NOT EVALUABLE. Positive CL-1 milestone NOT ACCEPTED.**

## Preregistration, source and independently verified results

- Frozen study design before code: [Study 21 preregistration](../studies/21-preregistered-temporal-restoration-calibration.md), commit `0b34a55c218b61b5ce9bfea2570d82a3c085fdab`.
- Validated running source revision: `dfb52f8d6920baaa73649dba66d84b223cbde748`.
- [Source-pinned CI run](https://github.com/iFrankStudio/cioc/actions/runs/43444126712); the `study21` job **passed** executing both the Rust runner's exact SQLite replay and [independent read-only Python audit](../analyze_study21.py). The CI artifact holds the actual SQLite and JSON audit for 30 days.
- **Eleven matched arms, thirty-three hashed post-epoch snapshots, three temporal windows. Exactly zero independently seeded ecological worlds.** The reader programs and graph-construction programs were **all designed by the experimenter** and loaded before the study.

## Registered information-processing output, by epoch

Cell values are from independently scheduled, externally authored CIL probe programs (processes 3,4,5) writing cells 90,91,92. `—` means *no successful CIL write of that epoch's registered output*, not a measured numeric zero.

| Paired branch | Window 0: observed damage | Window 1 | Window 2 |
| --- | ---: | ---: | ---: |
| Untreated | 11 | 11 | 11 |
| Rejected sham | 11 | 11 | 11 |
| Graph 1 ablated | — | — | — |
| Graph 2 ablated | — | — | — |
| Both ablated | — | — | — |
| Graph 1 externally restored only **after its failure** | — | 11 | 11 |
| Graph 2 externally restored only after its failure | — | 11 | 11 |
| Both graphs externally restored only after their failure | — | 11 | 11 |
| Graph 2 replaced with exogenous Halt opcode | — | 30 | 30 |
| Damaged but never repaired | — | — | — |
| No supplied probe compute | — | — | — |

The native events confirm probe3 trapped in the first epoch in externally restored conditions and was **never restarted**. Separate preloaded probe4 and probe5 independently read the code-bearing graphs in the subsequent two epochs; their executed `CellWritten` events are internally attributed as VM instructions, while the **probe software design, code choice, funding and `PutGraph` restoration intervention are externally authored**. All eleven arms share one prefork world hash. Graph/cell storage remained within the registered strict 96-unit cap; all stored epoch snapshots and results passed exact replay and independent integrity checks.

## What is and is not proven

**Proven:** The laboratory now measures loss of a source-content-sensitive two-component computation in an early temporal window, and can experimentally restore the registered output in **two later independent observation windows** using exact, explicitly external graph injection. Untouched, rejected sham, no-repair, content mutation and no-compute controls distinguish loss, persistent damage, externally changed content and budget limitations. This was the required temporal positive-control calibration **for a researcher-authored fixture**.

**Not proven:** Autonomous reconstruction, endogenously selected design, life-like self-maintenance, late recovery of a self-originated system, source-tainted causal closure, or open-ended evolution. No observed function existed as an emergent output of the Study-01–14 random ecology. In particular, externally supplying three separate readers and funding each at an epoch is **measurement instrumentation**, never self-propagation or independent biological replicates. Structural restoration and functional restoration are not the same as a *self-restoring* organization.

The next prospective ecological protocol must explicitly prevent the investigator-authored reader/copy/repair code and exogenous credits from being counted as part of a candidate life system. It requires fresh independent seed-worlds, matched sham/single/double lesions, no-repair and zero-credit controls, matched external functional-positive arm, and two late functionally verified windows **after actual loss** with a source-qualified *internally initiated* recovery path.

**Stage decision: CL-1 remains OPEN.**
