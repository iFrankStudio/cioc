# CL-0 Stage 3.1: Shared code/data allocation and CIL graph operations

Research classification: **infrastructure**, not organism behavior.

## Mechanism change
New `World::with_total_capacity(cell_limit, max_nodes_per_process, total_units)` enables one finite allocation ledger across **all CIL code graph nodes and public data cells**. One code graph node = one allocation unit; one distinct shared integer cell = one allocation unit. Updating either does not allocate. `World::occupied_units()` exposes the total for observation. Loading program nodes consumes capacity; cloning a node consumes one unit; creating a shared cell consumes one unit; deleting a shared cell reclaims one unit.

The existing `World::new(cell_limit, max_nodes_per_process)` remains a compatibility constructor with no shared aggregate ceiling. This is intentionally documented: historical Stage-1/2 experiments must not silently change. For bounded Stage-3 conditions always use `with_total_capacity`.

Storage occupancy equation in bounded worlds:

```
occupied_units = number_of_shared_cells + sum(process.code.nodes.len())
occupied_units <= total_units
```

This is *not* a byte-accurate model. Registers, process headers, interpreter allocations, SQLite/database size and observer-side event traces are **not** charged to this pool. CPU pricing remains exactly one process credit for each attempted instruction, including traps.

## Added CIL opcodes
- `ReadNext { dst, node, next }`: read the outgoing edge of the indexed graph node into a register. Only nodes with a single `next` edge are supported.
- `CloneNode { dst, source, next }`: copy one instruction node from the current program to the end of that program, writing the new index to `dst`. Cloning is a generic information-copy operation, **not** a reproduction instruction. Existing node types can be duplicated, but no new opcode type can spontaneously appear in this model.
- `PatchConst { node, value, next }`: overwrite the literal of an existing `Const` node using register-supplied values.
- `Delete { address, next }`: remove a shared data cell, releasing one occupied unit if it exists.

Previously implemented `PatchNext` edits one outgoing edge; `Write` can create/update shared data cells. Stage 3 therefore supports bounded alteration and extension of a program graph from pre-existing node material. Program graphs do not yet have general opcode construction, branch-edge mutation, graph deletion, or the creation of new executable processes.

## Atomicity, failure and attribution
All instructions must first validate node references, register indexes, execution edges and capacity before mutating the graph or public cells. A failed operation traps, stops the process, charges one compute credit, and leaves the graph/data unchanged. External loading and interventions have separate event provenance. Neither an opcode nor kernel scheduler detects a faulty program or chooses a useful mutation.

Cloned nodes retain their original edges, including edges to nodes in the same program. This is a literal node copy, not pointer rebasing or arbitrary whole-graph replication. A node's outgoing edge is not inherently its successor in a generated graph; CIL code must explicitly install or patch links.

## Negative controls and falsifiability
- Full shared pool blocks `CloneNode`, but the failed attempt still consumes credit.
- A program occupying the entire pool cannot create a new data cell.
- External `SetCell` has the same total storage guard as internal CIL writes.
- A cell deletion releases its one storage unit, allowing a later allocation.
- The laboratory can re-run clone-program conditions from an identical snapshot and contrast them with an externally imposed zero-credit control. Replay must match entire event traces.

These tests are constructed examples and provide **zero evidence** of spontaneous code invention, autonomous survival or self-organizing maintenance. All graph forms and goals are initially authored by researchers.

## Known limits / next gate
1. Replace fixed-size opcodes and process-local graph ownership with generic addressable instruction/data objects and proper capability semantics.
2. Price node deletion, data storage, and working registers through a clearly specified common model, without confusing researcher-side event storage with world resources.
3. Define first-class scheduled interventions at intermediate snapshots and test replay after every fork.
4. Add explicit object-byte representation and consistent cost accounting, without `REPAIR` / `REPRODUCE` / `FITNESS` host primitives.
5. Freeze serialization schema and lock dependency versions for long-lived cross-machine reproducibility.
