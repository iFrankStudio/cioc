# CL-1B / B22 — native CIL assembly and launch of a resource relay

**Prospective registration: 2026-11-15, before implementation/outcome inspection.** This is a *stronger engineered precursor* to organizational formation than B21, not endogenous invention or evidence of digital life. The investigator still supplies a complete **five-opcode helper template embedded as inert source nodes in the single loaded founder**. The host does **not** load the helper process, assemble it, allocate its execution graph, or directly credit the worker after setup.

## Discriminating mechanism

At initialization **exactly one** process id1 exists, loaded with one 35-node CIL founder and **29 local compute credits**. The CIL program (PC0–24) itself:

1. Uses native `GraphNew` to allocate an empty graph object, paid from local compute/storage.
2. Uses five `GraphCopyNodeLocal` calls to copy source template nodes PC30–34 into graph0–4; all five source nodes are **researcher-authored** and embedded in founder (no new algorithm inferred).
3. Calls native `GraphSetNext` four times to set the copied graph's forward edges 0→1,1→2,2→3,3→4. The final template node has the correct normalized backward edge4→1.
4. Sets child credit amount2 and performs `GraphLaunch`, **transferring** 2 credits from parent to new process id2, not minting compute.
5. The original parent continues as a worker (PC25–29): `Const r0=2`, `DrawCompute`, `Const r2=9`, `Const r3=1`, `Write cell9=1→Draw`.
6. The internally launched child runs its copied relay: `Const r0=4`, `DrawCompute`, `Const r2=1` (target parent PID), `Const r3=1`, `TransferCompute`→Draw. The target identity, source-node addresses, construction plan and transfer amount are **all investigator-authored in the founder**, hence this is actor-*executed* organization formation, **not actor-discovered cooperation**.

The copy operation's local-edge normalization is not assumed to reconstruct forward links in unfinished graphs. A real CIL `GraphSetNext` sequence edits the four forward edges after all copies. Both graph ownership and execution provenance must remain native.

## Frozen controls

All cases start with exactly one externally loaded founder, same initial29 compute, `World::with_strict_capacity(16,64,128)`, `with_pool_paid_draw()`, `with_recoverable_draw_shortage()`, same two externally supplied **world pool** packets3 then5, and no `SetCredits` after load.

- **BUILD_TRANSFER:** PC34 is `TransferCompute`. The parent builds/launched child, both burn their startup credit, packet3 brings parent to PC29 with zero credit and no work write, packet5 permits child to Draw4 then transfer1 to parent, parent performs the genuine PC29 write to cell9.
- **BUILD_NO_TRANSFER:** the parent still assembles/launched an otherwise identical 5-node child, but embedded template PC34 is a same-cost `Const r1=0,next31` instead of `TransferCompute`. No credit relay and **no parent PC29 write**; startup assembly, launch and resource packets are matched.
- **BUILD_NO_SECOND_PACKET:** identical BUILD_TRANSFER founder, only neutral packet3 is supplied. Parent remains at PC29 unfunded and zero output; no host repair or rescue.

Freeze source initially at 35 native CIL nodes, copy graph 5 nodes, child code 5 nodes, no more than 128 total storage units. Do **not** exclude traps or failing worlds. Compare actual first child creation time, `GraphCreated`, `GraphNodeCopiedLocal`, four actual `GraphNodeEdited`, native `GraphLaunched` source attribution, parent and child local compute balances, actual service write PC29 and helper PC4 transfer credit movement, all charged attempts and externally authored pool packets.

## Budget and causal requirements

The initial parent has29 credits. Assembly+launch executes **25 parent native attempts**, and `GraphLaunch` transfers2 credits to the new process. Two further parent attempts and two child attempts consume the remaining4 credits, leaving both at Draw with zero credits and pool0. Thus the initial world has exactly29 charged attempts and one created child.

Packet3 pays for one native worker Draw2 and its two setup steps, leaving worker PC29 without energy. In BUILD_TRANSFER the later packet5 pays for the child Draw4, three helper opcodes and one parent service Write, so total charged attempts = **37**, equal to initial29 + external packets3+5; all available energy is consumed. Negative variants are allowed unused balance or different step counts if one process becomes stranded, but must expose all that energy and its source.

The companion is not an investigator-loaded second process (contrast with B21) but is still based on an investigator-authored helper template and builder script. Passing proves **native CIL expressibility of actor-originated structural assembly and resource relay**, not spontaneous invention, cumulative evolution, persistent metabolic autonomy or open-ended adaptation.

## Validation

Native Rust test must verify genuine event provenance and local resource transfer at helper PC4 (not count generic Executed as proof). A separate source-parsed deterministic scheduler audit should independently reproduce the same full 3-arm campaign. Ensure the native results are retrieved with exact source SHA and a **non-cancelled** GitHub run before accepting mechanism claims.

The next gate is to remove the **embedded complete helper template** and test whether the continuing actor can generate/select a new relational graph from ordinary information and resource feedback on held-out neutral supply/competition, with no host selection of winners. If no such mechanism is expressible, report the missing generic information primitive instead of manufacturing an emergent-life claim.
