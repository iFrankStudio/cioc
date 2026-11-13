# CL-1 Study 28 — verified *post-result* second-source dependency falsification (2026-11-13)

**Technical audit: PASSED. Graph1 as a second read-mediated functional component: NOT DEMONSTRATED. CL-1 scientific milestone: NOT ACCEPTED.**

## Scope and exact-source integrity

- [Explicitly retrospective Study28 registration](../studies/28-registered-retrospective-second-component-audit.md) was committed **after** the two independently observed Study27 graph2-address-relocation cases were known, and **before this secondary-source analysis**. There are no new independent seed-worlds.
- [CL-1 targeted Study28 source-pinned CI run 43456985854](https://github.com/iFrankStudio/cioc/actions/runs/43456985854), code revision `480f7024356623879544c953f4675efbcab10233`: Rust exact 8-epoch replay of the unmodified original Study27 **192 seed IDs / 1,344 matched branches / 10,752 hash-verified snapshots**, original independent Study27 audit, then separate read-only Study28 audit and source/actor negative test fixtures. Study28 job passed; all paired source and sham/zero-credit/unaltered reinjection controls were inherited unchanged.
- The original strict Study27 positive endpoint is verified unchanged in the same **two seeds (2300090, 2300118)**. The two successful original output values are **1 and 4, respectively**, preserved within each changed-graph2 pair. An earlier narrative summary incorrectly said both were 1; the permanent Study27 checkpoint and RESULTS ledger have been corrected without changing outcomes, seed counts, code or source-pinned raw evidence.

## Second-component falsification

For each known graph2-dependent output address, Study28 compares `control`, `mutate-g1`, `mutate-g2`, and `mutate-both` at the same original process and epoch0. A second **read-mediated** graph1 functional source must demonstrate a successful native graph1 `GraphTagRead` or `GraphFieldRead` in the relevant actor before its output and a graph1-specific functional effect with graph2's source tag held fixed.

| Retrospective outcome | Independent worlds (out of 192) |
| --- | ---: |
| Known graph2 source-content-sensitive address relocation reverified | **2/192** |
| Graph1 mutation and combined mutation preserve required outputs | **1/192** |
| Graph1-only or combined mutation interferes with required output | **1/192** |
| Successful graph1 *source-code read* by process1 before the target output in either original pair | **0/192**, **0/2 exposed** |
| Read-mediated two-component functional dependency satisfying both exposure and intervention criteria | **0/192**, **0/2 exposed** |

Detailed outcome: **2300090** showed graph1-interference in at least one matched mutation but no qualifying native graph1 read by process1 before the output. **2300118** preserved the matched output with graph1 mutation and double mutation, also without such a read. Accordingly **a second read-mediated source-graph functional component is not demonstrated**. A graph1 edit can influence program behavior without a qualifying read via other VM operations or control paths; this audit does *not* prove graph1 is causally irrelevant, nor that its graph object is never accessed.

The specific original CIL generator predominantly exposes graph2 to process1 via registered `GraphReadTag(graph register1)`. It does **not** have a balanced opportunity for reading graph1's code by process1. Therefore these nulls are **exposure-limited** and do not estimate universal absence of multi-component dependence. This is a limitation in the ecological generator and the defined observer, not evidence for or against autonomous life.

## Boundary and next scientifically necessary step

Future prospective research should allow **unbiased access to both actual graph-code sources** in the stochastic continuation operator distribution (not impose a hand-authored repair pathway), ensure actor/source/PC and successful output attribution, and preregister single/double graph-content ablations plus exact-content controls. Every new-seed design must separate native source-dependent output behavior from endogenous construction, energy acquisition, graph self-repair and persistent two-late-window maintenance.

**CL-1 remains OPEN.**
