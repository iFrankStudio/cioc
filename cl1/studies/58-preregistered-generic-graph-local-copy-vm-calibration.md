# CL-1 Study58 — preregistered generic graph-local executable-edge relocation VM calibration

**Registration: PROSPECTIVE, BEFORE kernel extension or fixed fixture execution.** Earlier prospective Studies44/46/49/51/53/55/56 all failed to generate a native-launched child with a successful shared `CellWritten`. Studies44/45 and the hand-authored positive Study47 exposed a **code portability defect**: `GraphCopyNode` preserves a source executable node's *absolute parent-program control-flow edges*, which may be invalid in a smaller native child graph. Study57 also showed rare graph assembly paths have limited execution exposure. This is a **general-purpose language/virtual-world mechanism calibration**, not a life experiment or ecological emergence study.

## Frozen new generic CIL primitive, distinct from old opcodes

Introduce exactly one new opt-in opcode **`Op::GraphCopyNodeLocal { graph, source, dst, next }`**, ABI numeric descriptor tag **30**, as an alternative to existing `GraphCopyNode` with **identical register operands, ownership, allocation, credit cost and next-PC validation**.

- Resolve graph handle `G=registers[graph]`, source parent executable index `S=registers[source]`, copy the one source CIL node at `S` to the end of the actor-owned graph `G` at new index `D=G.nodes.len()`.
- One native attempted instruction still debits **exactly one** process compute credit; the successful copy reserves **exactly one** graph code storage unit, same as `GraphCopyNode`. All graph handle, ownership, source index, destination register, storage and program counter checks **must match** the existing native GraphCopyNode implementation. No host-side process restart, cell writes, success reward, auto GraphLaunch or extra energy.
- **Frozen generic destination-relative edge law:** for each outgoing execution edge `E` of the **source opcode**, write the corresponding edge in the copied opcode as
  `new_E = (D + E - S) mod (D + 1)` with Euclidean modulo (nonnegative result), where `D+1` is the graph's new node count **after** the copy. Apply this transformation to all `next`-bearing opcodes and to **both** `Branch.nonzero` and `Branch.zero`. `Halt` has no edges. A copied instruction's immediate destinations are therefore valid *within its destination graph at copy time*, regardless of opcode type. This is **explicit environment/VM-provided automatic edge closure**, not a native organism discovering repair. Do not selectively relocate only `Write` or `GraphLaunch` and never inspect a fitness/viability goal.
- Record successful local copy in a new, clearly distinct observer event `GraphNodeCopiedLocal {graph,owner,source_process,source_node,new_node}` with `Origin::Internal(actor)`. Old `GraphNodeCopied` provenance and old `GraphCopyNode` byte behavior remain unchanged. Add opcode tag30 to encode, decode, `opcode_tag`, `next_mut`, graph dispatcher. No existing opcode tag reassignment; existing ABI graph descriptors remain length5.
- Pin unchanged `WORLD_FORMAT_VERSION` and backward compatibility: all old serialized opcodes and statuses must decode and execute without changes. Source revision and experiment documentation must explicitly disclose that newly created children receive an *environment-assisted local edge representation*.

## Three fixed, explicitly human-authored mechanism fixtures, **zero new ecological worlds**

Fixture parent creates a graph via native `GraphNew`, then copies a fixed existing `Write(next=far parent absolute index)` into graph node1, and natively `GraphLaunch`es that graph with finite available credits. The source `Write` was **written by the researcher** and is not a discovered organism.

1. `absolute-stale-edge`: use old `GraphCopyNode`; child attempts copied `Write` and must trap `invalid edge` **without** producing child `CellWritten`.
2. `generic-local-edge`: change only the copy opcode at the same parent PC to new `GraphCopyNodeLocal`; copied source `Write` edge must relocate to a graph-local value, and child must successfully emit `Origin::Internal(child)` `CellWritten` with correct key/value. No external interventions after initial world construction. This **proves the generic local-copy tool works**, not reproduction, heredity, repair or spontaneous function.
3. `zero-child-credits`: use the same new local opcode but launch child with zero credits; even structurally valid child must not execute native Write. Positive/negative compute control.

Use deterministic fixed credits/graph capacities, fixed <=8 original native instructions per parent, only a few observation epochs, existing `Laboratory::run_epochs` and `verify_epoch_replay`. Archive original SQLite and independent Python output verifying actual five-word ABI tag30 roundtrip, exact source/copy child lineages, instruction edge, `CellWritten` success/trap and compute conservation. Add **two minimal Rust kernel tests**: generic branch two-edge relocation and unchanged original absolute-copy semantics; do not write hundreds of tests.

## Scientific decision gates

- A valid calibration must pass all three fixtures, the ABI roundtrip, exact native event chronology, original SQLite SHA256 snapshot replay and independently audited compute conservation, under a fixed source revision. If old virtual-world behavior differs, **REJECT** the extension and repair compatibility before running new ecology.
- Any success is **scripted, VM-enabled child execution**, not emergent cyberlife. Only after fixed calibration passes should Study59 prospectively expose **non-goal-directed randomized native CIL programs** on independent seeds to the new generic opcode versus original, with all source/operation selection probabilities held matched. Even Study59 positive child Writes would remain only an intermediate mechanism, not endogenous self-maintenance.
- Never interpret automatic graph-local edge closure as a process's native *self-repair*, never score fitness in the VM, do not inject additional compute, and never quietly change old study results.

**CL-1 remains OPEN.**
