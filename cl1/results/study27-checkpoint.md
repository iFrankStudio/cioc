# CL-1 Study 27 — prospective independent confirmation of graph-code-tag-directed output address

**Technical result: PASSED. Strict preregistered address-causality exposure: 2/192 independent seed-worlds. CL-1 scientific milestone: NOT ACCEPTED.**

## Source-pinned prospective evidence

- [Study-27 preregistration](../studies/27-preregistered-graph-tag-address-relocation.md) was committed **before** the new independent seeds, implementation and outcome inspection: `d457b60b89d2d849b1f9d3efcfb73c86ae4703a1`.
- Verified code revision: `d52e1c369b68807a26aeedbf3f935228b7f3eb44`. [GitHub Actions CL-1 run 43454204049](https://github.com/iFrankStudio/cioc/actions/runs/43454204049): Study-27 Rust fixture, all workspace tests/strict lint, 192 independent new worlds, exact SQLite epoch/state/event replays and independent Python audit **passed**. Archived raw SQLite/report retained 30 days.
- **192 distinct seed-worlds `2,300,000..2,300,191`**, disjoint from Studies 14, 22 and 25, **seven matched arms each**, **1,344 paired runs** and **10,752 hash-verified postepoch snapshots**, every seed included. The seven arms are not 1,344 independent statistical replicates.
- The researcher-authored positive-control CIL fixture independently confirmed that a successful `GraphTagRead` from code tag 11 vs 19 directly changes a later `Op::Write` **address**, but not the fixed written value; exact same-graph restore is separately verified. This fixture does not count as an ecological replicate.

## Precommitted primary causal address endpoint

The independent auditor requires both paired conditions for the same seed and actor1, epoch0:
1. Untreated graph2, owned by process2, successfully read as tag **11**, followed by a successful internally charged `CellWritten` to **cell11**, value `v`.
2. External `DropGraph(2)` + `PutGraph(2, identical handle/owner/readers but node0 replaced with Halt)` results in a successful graph2 read by process1 of tag **19**, followed by a successful `CellWritten` to **cell19**, **the same numeric value `v`**.

**Observed strict paired cases: 2/192 distinct seeds**, exactly **2300090** and **2300118**, each using native process1; the preserved numeric value was **1 for seed 2300090** and **4 for seed 2300118**. Wilson two-sided 95% descriptive interval for this *specific fixed program generator* is **[0.002861, 0.037179]**.

The single-target graph1 negative-source arm yielded **0/192** false-positive `GraphTagRead(graph2 tag11) → cell19` patterns under the corresponding fixed actor/epoch criterion. The rejected sham and externally re-injected **unaltered graph2** had identical simulated final states to untreated controls; zero-credit controls did not execute CIL. All source graph interventions were explicitly `Origin::External`, and no partial or unregistered graph edits were accepted.

## Scientific interpretation

This confirms **a narrow, causal source-content-sensitive information-processing capability** in two independently sampled random-continuation worlds: the write *address* changes in the prescribed direction when the actual code-bearing graph tag is externally modified, while the numeric value is preserved. It is stronger than a copy-count, temporal read/write co-occurrence or missing-output proxy. It is **not** an endogenously invented functional architecture: the input code, permission scaffold and available opcodes were researcher-supplied, and the graph mutation was performed by the experimental host. The read-to-write branch originates in the program continuation and is not evidence of adaptation or a repair response.

Both exposed events occur in **epoch0**; this cohort observed only eight early epochs and tested **no late postdamage internally reconstructed function**. There is no second causally necessary independently maintained component, no internal restoration after a demonstrated functional loss, no persistent closure, no resource autonomy and no evidence of self-propagation. Do not count the synthetic fixture or externally seeded programs as cyberlife.

**CL-1 remains OPEN**. The scientifically necessary next gate is a prospective source-verified, **two-component functional** intervention with an exact external positive control on the *same function*, followed by measured endogenous restoration in two separated late windows, without a built-in Repair/Fitness/Reproduce mechanism.
