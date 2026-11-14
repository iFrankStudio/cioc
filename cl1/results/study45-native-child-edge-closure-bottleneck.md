# CL-1 Study45 — verified retrospective native child graph edge-closure bottleneck

**POST-HOC RETROSPECTIVE, ZERO NEW INDEPENDENT WORLDS.** The edge-invalidating Study44 child Write witness was already inspected before this diagnostic was registered. No prospective scientific claim is made.

Study45 used the unmodified original Study41, Study42 and Study44 SQLite archives and replayed their full independently source-pinned auditors, then independently verified **36,864 SHA256 epoch snapshots** (384 reused distinct initial worlds, four matched arms each, 1,536 existing runs). CI [43554994755](https://github.com/iFrankStudio/cioc/actions/runs/43554994755) **passed**. The preserved diagnostic artifact is `cl1-study45-retrospective-graph-edge-closure`, ID `14097593653`.

## Final child executable graph topology (ambient-general arm)

| Structural observation | Study41 | Study42 | Study44 |
|---|---:|---:|---:|
| Native-launched children, each an actual new process | 60 | 58 | 74 |
| **Children with an out-of-bounds executable edge** | **49/60** | **41/58** | **56/74** |
| Children with every final outgoing edge inside their own code graph | 11/60 | 17/58 | 18/74 |
| Child program nodes with opcode `GraphNew` | 70 | 75 | 88 |
| Child program nodes with a non-`GraphNew` opcode | 1 `Const` | 1 `GraphCopyNode` | 1 `Write` |
| Native children with successful `CellWritten` | 0 | 0 | 0 |

Most launched child programs inherit **absolute `next` indices** pointing outside their much smaller executable graph. A **closed graph** is necessary for sustained execution but not sufficient for function: almost every closed child still consists only of GraphNew instructions and can repeatedly allocate more graph objects. The numbers above refer to **final-snapshot topology**, not necessarily exact at-launch code if a child mutates its own graph.

Study44's only native child with a copied Write was source-pinned seed3,300,121/ambient-general, child4, code `GraphNew(next1)` then `Write(next12)`. It performed native GraphCreated then trapped `invalid edge`. The independent audit separately validated a child native trap attributed to its out-of-bounds PC; do not infer that all 56 structurally open child graphs trapped for this exact reason (other native errors and capacity exhaustion also occur).

## Mechanistic result

Two separate bottlenecks now have empirical source evidence:

1. **Code-source diversity**: original graph objects often contain only copies of the scaffold `GraphNew`; Study44's random index expansion generated a `Write` child code in just one original 128-seed cohort.
2. **Executable edge closure**: absolute program `next` links inherited from a longer parent program are frequently invalid inside a short launched graph. A copied instruction's presence is not proof the instruction is runnable.

The next prospective opportunity study may increase the *generic* frequency of native graph copy/edge rewiring/launch primitives and allow random local edge targets **without inserting an explicit repair/reproduce/write program**. A child shared-cell output would still be non-goal-directed mechanism only; causal maintenance and hereditary functional continuity remain unproven.

**CL-1 OPEN.**
