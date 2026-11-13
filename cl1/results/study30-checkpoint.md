# CL-1 Study 30 — verified retrospective two-graph read/Add/write pipeline bottleneck

**Technical audit: PASS. Scientific finding: NO verified two-source Add-to-output pipeline. CL-1: NOT ACCEPTED.**

## Evidence and scope

- [Registered retrospective protocol](../studies/30-registered-retrospective-dual-source-pipeline.md) was recorded **after** the Study29 zero-of-256 endpoint was already known. This is **post-result diagnostic interpretation**, not prospective replication.
- The [successful targeted CL-1 CI run 43458724617](https://github.com/iFrankStudio/cioc/actions/runs/43458724617), source revision `1322789dde7e59fe89df0b1a5a89cecb8c5aa862`, reran **the exact Study29 generator and all seven branch interventions** through full native Rust SQLite epoch replay. The original read-only Study29 independent audit was repeated first and reconfirmed its frozen **0/256** source-sensitive dual-additive function endpoint.
- **256 previously sampled independent world IDs reobserved, 0 new seed-worlds.** All **1,792** matched runs and **14,336** hashed snapshots were verified. Synthetic native event unit fixtures do not add independent observations.

## Success-only program-trace attrition, untreated epoch0

| Registered native successful-execution stage | Distinct seed-worlds out of 256 |
| --- | ---: |
| Process1 successfully reads own graph1 source code | **38** |
| Process1 successfully reads foreign-owned graph2 source code | **45** |
| Process1 successfully reads *both* sources in epoch0 | **4** |
| Both-source read succeeded, followed by an executed native `Op::Add` at a source-verified original instruction PC | **0** |
| Both-source read → successful Add → later native shared-cell write by process1 (any target) | **0** |
| Both-source read → Add → prespecified cell22 write | **0** |
| Any successful native `CellWritten` by process1 in epoch0 regardless of graph reads | **116** |
| Process1 trap after both reads | **1** |

The `Add` success provenance joins the event's native actor/PC with the *actual prefork serialized program* and requires a successful `Executed` transition rather than a matching opcode text or an attempted/trapped instruction. All native reads require exact actual source graph, owner, node and tag. Untouched versus exact-source restored controls agree; zero-compute is inert. The same outcome is structurally expected to apply to the fixed paired source-edit branches only as their own descriptive exposure; they are not additional independent universes.

## Scientific interpretation

The original random CIL generator allowed both source-read instructions to occur, but under the frozen 18-node continuation and 12-attempt/epoch scheduling, **successful source1+source2 reads came together in only 4/256 independent worlds, and none reached a successfully executed Add afterward**. Successful writes occurred in 116/256 worlds, so native Write itself was not absent; it was **the registered *coupling* of source-read, Add and Write** that was not observed. This was an explicitly **post-outcome** bottleneck localization. It is not direct evidence that those two source reads would always compute the intended addition if more runtime were granted, nor that a less restrictive generator would produce a maintained organism.

The Study27 single-graph source-content-sensitive address result (2/192) remains valid but narrow. Study28 failed to establish source-qualified two-component dependence; Study29 and this reanalysis supplied no such function. The proper next step would be a **new independently preregistered** ecological operator-budget comparison, with transparent investigator chosen primitive distributions and no retrospective tuning, and eventual tests of internal construction/restoration rather than more unqualified graph-copy counts.

**Positive CL-1 scientific acceptance remains OPEN.**
