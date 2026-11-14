# CL-1 Study58 — generic CIL graph-local node transport scripted VM calibration

**Status: full source-pinned CI success, ZERO new stochastic ecological worlds. Not life.**

A prior human-scripted Study47 native graph builder needed explicit `GraphSetNext` to repair a copied parent's absolute Write-next edge before launching a functional child. Study58 prospectively registered and then implemented an **opt-in generic CIL instruction**, `GraphCopyNodeLocal { graph, source, dst, next }`, with five-word descriptor ABI tag**30** and a distinct native-origin `GraphNodeCopiedLocal` event. Original tag12 `GraphCopyNode` and tags0–29 stay unchanged. The new VM law is generic for all source opcode outgoing edges, including both Branch targets: with source node S, target edge E, destination append index D, `newEdge=(D+E−S) mod(D+1)` (nonnegative modulus), bounded in target graph. Same credit charge, graph ownership and storage rules as old copy. **The VM automatically provides local edge closure; the child does not discover or repair this.**

Frozen prospective protocol: [Study58](../studies/58-preregistered-generic-graph-local-copy-vm-calibration.md). The three **human-authored** native builder fixtures share the same 9-node parent, differing only in old/new copy and child credit. Verified CI [43562437416](https://github.com/iFrankStudio/cioc/actions/runs/43562437416) **success**, executed revision `3f3415e20cd9f62fca739bdff082621950dc4d15`, original artifact `cl1-study58-generic-local-graph-calibration` ID `14100563118`. All **24 source-pinned SHA256 epoch snapshots** independently replayed, no post-initial host interventions, exact opcode event/child identity and resource conservation.

| Human-scripted control | Native copy event | Native child launch | Native child `CellWritten(key0,value0)` | Child invalid-edge trap |
|---|---|---:|---:|---:|
| Generic graph-local transport | 1 `GraphNodeCopiedLocal` | 1 | **3** | 0 |
| Original absolute copied edge | 1 `GraphNodeCopied` | 1 | **0** | **1** |
| Generic local transport, child credits0 | 1 `GraphNodeCopiedLocal` | 1 | **0** | 0 |

The copied source Write at original node6 with absolute next7 becomes graph-local node0 with next0 under the generic law. Old absolute copy retains next7, invalid in 1-node child; zero-credit local child cannot execute. These are **researcher-designed positive/negative mechanism calibrations**, **not spontaneous functional children**, endogenous code repair, ancestry of an evolved algorithm, energy independence or life. The separately preregistered Study59 stochastic program trial is needed to test non-goal-directed ecology and must not promote the human-scripted outcomes to ecological evidence.

**CL-1 remains OPEN.**
