# CL-1 Study 31 — preregistered post-result register-dataflow audit of graph-code output addresses

**Registered 2026-11-14 after the Study27 2/192 source-sensitive outputs and Study28/29/30 nulls were already known.** This is *post-result corroboration only*. The two existing Study27 positive seeds (2300090 and 2300118) and their preserved numeric outputs (1 and 4) are known. Do not present this as prospective independent emergence, and add **zero** worlds to the independent ecological denominator.

## Falsifiable registered provenance question

When CIL process1 reads graph2's successful source-code tag (11 or 19), does the **actual register written by native `GraphReadTag`** remain a source-qualified **address-register value** until the **successful `Op::Write`** emits the observed `CellWritten` event in the same epoch? Or did the formerly defined chronological read/write association hide an intervening register overwrite or unrelated address source?

## Frozen corpus and stage

Reexecute **unmodified** Study27 generator, 192 independent seeds `2,300,000..2,300,191`, seven paired conditions, 8 ×12 attempts per branch, exactly 1,344 SQLite runs/10,752 hashed snapshots and full Rust epoch replay. Reapply original Study27 independent audit (strict primary graph2 output-address changed count must remain exactly two), with untouched/sham/exact-original/zero-credit/graph1 controls unchanged. Study31 reads those snapshots independently **without mutating VM, code, scheduling, compute or source definitions**. No new tool instruction or event type.

## Read-only dynamic def-use contract

Only process1 events with `Origin::Internal(1)` in epoch0, strictly increasing ticks, and their actual original serialized process1 `code.nodes[pc]` qualify. Track one conservative label set per native CIL register **starting empty at the epoch0 prefork**:
- Successful `GraphTagRead` uses actual `GraphTagRead {graph,owner,node,tag}` provenance and the actual original instruction's `dst`. The label `graph2-tag` is assigned only if graph2, owner2, node0 and tag11/19 exactly match that branch. A successful read of other code may be separately labeled but is never silently promoted to the graph2 source.
- Native successful `Const`, `Read`, `ReadNext`, `CloneNode`, `GraphNew`, `GraphCopyNode`, `GraphCopyFrom`, `GraphReadNext`, `GraphReadField`, `GraphLaunch`, `GraphEncode` and any other known register-writing instruction **overwrite its actual destination**; unqualified sources clear the graph2 label.
- Native successful `Add`/`Sub` propagate the union of pre-instruction labels on operands into `dst`. Native `Write` at its actual `address` operand may be a provenance witness **only if that address-register label includes the qualifying graph2 tag** and the emitted actual `CellWritten.key` matches the registered branch address. The actual corresponding numeric value must also match the complete paired dataset's original Study27 output.
- Native `Branch`, `GraphGrantRead`, `GraphSetConst`, `GraphSetNext`, `GraphTransfer`, `GraphDrop`, `GraphRevokeRead`, `Delete`, `DrawCompute`, `ReturnCompute`, `TransferCompute`, `PatchNext` and `PatchConst` do not by themselves create positive graph2-origin labels. If any encountered opcode/event might mutate process code, reorder register semantics, or has unsupported destination effects, **fail closed and do not award the witness**.
- A successful event is identified by its native success event variant (including `Executed`, `CellWritten`, `GraphTagRead`, `GraphCopied`, `GraphNodeCopied`, etc.), **not** a `Trapped`, `Halted`, externally attributed intervention, or an attempted but unsuccessful instruction. An event at a PC not present in immutable original process1 code is a technical audit failure.
- A fully source-qualified paired witness requires **both control and mutate-g2** arms of the same seed to satisfy the above (values 11→19, cells 11→19, same original numeric output). Count *all* 192 worlds; separately report the two already-known chronologically qualified Study27 worlds as an exposed set, and whether either/both satisfy stricter register-level address lineage. No cherry-picking, no retrospectively changing the Study27 outcome.

## Controls and scientific interpretation

Synthetic positive fixture: native `GraphReadTag.dst=0` then native `Write.address=0` without intervening overwrite. Negatives: intervening `Const.dst=0`, unqualified other graph read overwriting register0, `GraphCopyFrom.node=0`, wrong process, wrong PC, wrong origin, trapped `GraphReadTag`, and a write using a different register. All fixtures should be tested independently from seed-world results. Python read-only replay of exact event/code provenance and original Rust full-state replay are mandatory.

Even if two graph2 witnesses pass a stronger def-use test, the graph-code objects and initial CIL programs originated with the **researcher**, the graph mutation is **external**, and this is a **single-component early-epoch** functional dependency, not autonomous synthesis, multi-component closure, self-repair or persistent post-lesion recovery. The **CL-1 acceptance gate remains OPEN**.
