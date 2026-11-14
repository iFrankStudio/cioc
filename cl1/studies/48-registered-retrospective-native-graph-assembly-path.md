# CL-1 Study48 — registered retrospective native graph-assembly pathway audit

**POST-RESULT retrospective diagnostic. ZERO new independent worlds.** Study44 and Study46 both showed 0/128 successful child `CellWritten`; Study47's human-scripted fixture showed that the unchanged VM *can* produce child writes if a graph with executable, locally closed edges is assembled before launch. This protocol is registered **after those outcomes are known**, before implementing this retrospective source-path audit. It cannot establish prospective life or heritable function.

## Original unchanged population

- Exactly Study44 original 128 worlds seeds3,300,000..3,300,127, source revision `1f83c2225f4f7ac681b957256b596b0683659520`, original GitHub Actions run43554614365.
- Exactly Study46 original 128 worlds seeds3,400,000..3,400,127, source revision `6bed2ef33122584570502bbd644592248bf83e27`, original run43555307061.
- All four original paired arms for each seed, total 1,024 historical runs, 24,576 original SHA256 epoch snapshots. No regenerated worlds or seeds. Independently rerun the original Study44 and Study46 source-pinned Python auditors before extracting any new diagnostic.
- Original native CIL graph object events: successful `GraphCreated`, `GraphNodeCopied`, `GraphNodeEdited`, `GraphEncoded`, `GraphLaunched`, all with `Origin::Internal(actor)`, actor/PC/tick. Treat external interventions separately.

## Frozen descriptive assembly gates

For every native `GraphLaunched`, identify exact source graph ID and parent actor, launch tick and child ID. For each launched child record *prior events by the SAME original parent actor targeting the SAME source graph*:
1. An internally successful `GraphNodeCopied` of a parent code node *other than original GraphNew node0*, with source-node index and new node index.
2. An internally successful `GraphNodeEdited` before the child launch, with source graph and exact target node.
3. Both 1 and 2 before launch, regardless of order; separately require copy→edit→launch order.
4. At launch, child's executable nodes as observed from its original first appearance in subsequent native epoch snapshots if available; if impossible to reconstruct exact launch bytes from the final state, mark topology final-snapshot only, never infer successful control-flow closure from an event count.
5. Child code contains any non-GraphNew operation, contains Write, and has all outgoing edges within its own length; and actual native child `CellWritten` or `invalid edge` trap.

Report per-arm world counts for native launches, launch ancestry with copy, edit, copy+edit, copy→edit→launch, and native child output; report all three gate frequencies without selecting an individual success. Count final child code structural closure separately from actual runtime function.

## Integrity and interpretation

Validate all original SHA256 snapshots and source SHA, exact seed/arm population, monotone event history, native actor/graph owner identity, event tick ordering, parent identity and unique child ID. No host-inserted child, no background mutation or output may count as native. Include two minimal synthetic native event-path controls, one same-parent same-graph ordered positive and one unrelated graph/actor false-positive negative.

A zero or rare count of copy→edit→launch provides an explanation for missing functional child behavior in randomized CIL, **not** a proof of impossibility. Study47's scripted positive confirms the VM's capability is not intrinsically absent. A future prospective independent opportunity experiment should change only *generic sampling of graph-assembly action order/operands*, without hand-coded organism reproduction, and must preregister before running independent seeds.

**CL-1 OPEN.**
