# Study 34 correction: GraphReadTag graph operands are registers, not graph IDs

**Status:** Corrects a false static impossibility statement in `cl1/results/study34-static-feasibility-finding.md` and `cl1/results/study34-rust-generator-crosscheck.md`. Preserve original reports for auditability; do not silently overwrite them.

## Root cause

The Rust interpreter `cl0/crates/world/src/lib.rs` handles `Op::GraphReadTag { dst, graph, node, next }` with:

```rust
let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
let index = usize::try_from(reg(node)?).map_err(|_| "invalid node")?;
```

The `graph` and `node` instruction fields are **register indices**, whereas successful `EventKind::GraphTagRead { graph, owner, node, tag }` fields are **resolved runtime graph IDs and node indices**. Consequently, absence of a literal `GraphReadTag { graph: 2, node: 0 }` in initial code does **not** make source graph2/node0 unreachable. The inherited Rust/Python random continuation may access graph2 through a register, for instance `GraphReadTag { graph: 1, node: 2 }`.

## Archived evidence, unmodified

From GitHub Actions run [43543465053](https://github.com/iFrankStudio/cioc/actions/runs/43543465053), archived artifact `cl1-study34-retrospective`, original 128 seeds x 9 conditions, full 27,648 epoch snapshots and Study33 baseline auditor passed. An independent *local* retrospective scan of original final-state native events (no new worlds; descriptive only) found **25/128 original-control worlds with process1 successfully reading resolved graph2/node0**, contrary to the previous 'structurally impossible' claim. A witnessed example is seed `2600004/control`, process1 tick 50, PC16, opcode `GraphReadTag { dst: 2, graph: 1, node: 2 }`, with runtime `GraphTagRead { graph: 2, owner: 2, node: 0, tag: 11 }`. This is a native source read, not source-dependent cell write, self-maintenance or life.

A correction to `cl1/analyze_study34.py` now resolves source exposure from **actual successful graph events** with actor/PC attribution. The report must be cross-checked in CI against all archived runs before treating the new exposure count as verified. The original strict Study33 source-write/joint primary outcomes remain zero; a new read exposure must never be substituted for their endpoint.

## Interpretation

The original static feasibility diagnosis was wrong; the source2 read opportunity is real. The useful question is now **where the successful read fails to produce an address-dependent native Write**, especially among the same-actor DrawCompute cases. Follow the already-frozen Study34 retrospective bottleneck gates; do not invent a favorable new success criterion or confuse externally endowed pool transfers with endogenous resource generation.

**CL-1 remains OPEN.** This correction retracts one methodological conclusion, **not** the Study33 negative joint-function observation.
