# B56-F1d1e — preliminary exact-source same-PID opcode closure audit

**Evidence class: manual source-level invariant inspection, NOT a new native experiment, NOT an independently executed observer.** Protocol was committed first at `3f1e4dbba7544500ada59418bda3da4244f03385`.

Inspected `cl0/crates/world/src/lib.rs` on main (blob `4d0a1e7b86c0bbae5c11724f4292252ee0993ff0`). In the current VM, the initial executable is supplied externally by `World::load` (line ~1324); subsequent actor-paid executable mutations are:
- `PatchNext`: changes only an existing instruction's single successor edge (~1804).
- `PatchConst`: changes only an existing `Op::Const` literal (~1890).
- `CloneNode`: appends a clone of an existing executable opcode (~1881). It cannot introduce an opcode type not already present.
- `GraphEncode` and `GraphReplace` decode data into **graph-object** nodes, not `process.code.nodes` (~2321, ~2495–2530).
- `GraphLaunch` moves graph-object nodes into `Process::new(child_id,...)`, where `child_id=self.next_process_id`, and advances the parent's own PC without replacing its executable (~2296–2311).

An exact-text scan of this source found one `code.nodes.push` site (CloneNode), no `code.nodes =` or `p.code =` assignments, and the only `code.nodes[index]` actor writes above are next-edge and Const-literal modifications. An external `Intervention::PatchEdge` changes only an edge (~1446–1456); external `load` is expressly excluded from same-PID actor-origin claims.

**Bounded inductive conclusion:** for an already loaded PID whose initial executable has no Branch(test1), any finite sequence of the existing actor-paid in-process executable mutations preserves the multiset of opcode types modulo duplication; `PatchConst` and `PatchNext` cannot change Branch's test register. Thus an initially absent Branch(test1) cannot become executable within that *same PID* through those operations. Generic GraphEncode can produce Branch(test1) in an owned graph object, but the existing transfer-to-execution path creates a **new PID**. This is a source-semantic reachability obstruction under the present API, not an impossibility theorem about future kernels or cross-PID organism lineages.

**Important limitation:** this inspection is not a machine-checked whole-kernel mutation proof; it must be checked by a separate source-derived observer, and native positive/negative controls should be added only if they distinguish a remaining uncertainty. The existing F1d1d native evidence already supplies positive PID2 construction and NO_ENCODE negative control. No new same-PID Rust execution or exact-SHA CI was obtained for F1d1e.

**Decision:** do not relax same-PID identity, relabel PID2 as PID1, or modify VM physics merely to satisfy the target. The scientifically useful next fork is either (1) separately preregister an explicit PID-spanning lineage identity and test inheritance/continuity with strict resource conservation, or (2) redesign the founder to achieve useful **within-PID opcode-preserving** adaptation and keep novel-opcode construction as a different claim. Strict original F1d1, N56-A/B/C, B3/B4 and cyberlife remain unachieved.
