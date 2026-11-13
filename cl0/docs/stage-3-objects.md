# CL-0 Stage 3.2 — Addressable executable graph objects (increment 1)

**Scientific status:** engineering construction only, not evidence of life.

## Object representation
A `GraphObject` has a monotonically assigned integer handle, an owning process ID, and a sequence of CIL `Op` nodes. Handles are not reused. A newly created empty object occupies one storage unit (metadata overhead); each appended node occupies another unit. Ownership is enforced at instruction execution; guessed integer handles are **not** capabilities.

Graph objects are manipulated only in worlds created with `World::with_total_capacity`. Legacy `World::new` remains compatible with historical experiments and traps object instructions rather than offering unbounded metadata allocation.

## General low-level CIL operations
- `GraphNew`: create an empty owned graph object and return handle.
- `GraphCopyNode`: append an instruction copied verbatim from the executing process's own graph.
- `GraphReadNext`: read a single outgoing edge from an owned object.
- `GraphSetNext`: rewrite one single-successor edge within an owned object.
- `GraphSetConst`: rewrite the literal of an owned `Const` node.
- `GraphDrop`: free the whole object, including header and nodes.
- `GraphLaunch`: move the nonempty object into a new process, transferring the requested number of **existing** compute credits from the caller. It does not replenish credits or duplicate code and has no repair/reproduction semantics. The graph header is released when consumed.

Every attempted operation costs one credit prior to its checks. Successful changes and deterministic traps appear in the existing event trace. An invalid handle, wrong owner, absent or unpatchable node, over-capacity append, or excess credit transfer has no object mutation. External experimenter edits still use `Origin::External`.

## Important limits
1. Stage 3.2 can **combine existing opcode material** into a new graph, not invent new instruction primitives. Branch-edge construction and full instruction encoding are not supported.
2. There is no access-control delegation, interprocess object sharing, or object transfer between processes. Object ownership is a static ID; when an owner stops, its objects persist but cannot be manipulated by another process.
3. The world counts graph-object headers and graph nodes in the shared allocation pool, but still does not price registers, process headers, instruction encoding bytes, or observer-side event logs.
4. `GraphLaunch` is a generic VM execution operation used in constructed fixtures. It is **not** a discovered copying/propagation strategy. A spawned graph's code is moved from its object; no automatic self-preservation or fitness function exists.
5. No bounds are yet enforced on wall-clock resource consumption for loading attacker-controlled SQLite snapshots; no secure deserialization contract exists.
6. Multi-version replay compatibility and immutable provenance require separate work.

## Verification controls
Tests cover finite allocation, graph creation/drop reclamation, owner access isolation, invalid index trapping, deterministic credit conservation on process creation, and the original 19 infrastructure tests. The SQLite replay suite must continue to pass unchanged. These tests do not test any emergence hypothesis.
