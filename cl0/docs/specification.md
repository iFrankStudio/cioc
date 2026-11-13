# CL-0 Technical Specification — 0.1 (2026-11-13)

Status: normative architectural baseline. Features annotated **future** are not implemented by the Stage-1 kernel.

## 1. Epistemic contract

The host implements general physical laws of an information/computation/storage world, *not* biological analogues, survival objectives, repair controllers, replication schedulers, or a fitness oracle. Researchers declare hypotheses and independent observational metrics. Absence of these mechanisms is not itself evidence of emergence.

Three origins of state change are disjoint: (a) external experiment intervention, (b) internal CIL instruction, (c) kernel accounting/administrative effects. Each event records its origin. Experimenter-supplied initial graph code counts as an external construction.

No classification of alive/dead is exposed to CIL. Execution termination means only that the VM is not scheduled. No real-time semantics are exposed to CIL.

## 2. World state and transition

`W = (processes, shared_cells, storage_limit, execution_cursor, events, tick)`.
`W[k+1] = T(W[k], action[k], rules)`. One successful or rejected scheduler action is one logical tick. The kernel chooses the next runnable process in a stable round-robin order by integer id. No wall clock, nondeterministic iteration or hidden unseeded RNG is allowed. The initial implementation uses no randomness.

Any IO, resource injection, initial program load or code damage is an explicitly labeled external operation. For matched experiments, copy the identical pre-intervention state, apply different interventions and compare traces/results. An independent log is append-only from the observer's perspective; Stage 1 uses an in-memory log, **not yet durable or tamper-evident**.

## 3. Resource model

- Computation: each attempted instruction requires exactly one unit of process-local credit (Stage 1). If credit is zero, execution cannot proceed. Credits are created only by external initialization; no replenishment operation in Stage 1.
- Storage: finite shared key/value cell capacity; creating a new cell costs one slot, updating a cell reuses a slot. Removing a cell (future) releases a slot. Program nodes are bounded by a per-process immutable `max_nodes` at this stage; program allocation and storage bytes are **not yet unified**.
- Information: signed 64-bit integers only in Stage 1. Arithmetic wraps in two's-complement form, and branch decisions compare exact integers.
- Accounts: execution credits cannot be transferred in Stage 1. Future work must account for all transfers, allocations, deallocations and externally imposed resource flows.

Invariant: `sum(initial_credits) = sum(remaining_credits) + sum(executed_instruction_costs)` unless an explicit external injection/removal event is recorded. Storage never exceeds `storage_limit`. Invalid instructions must not alter storage or graph state.

## 4. CIL graph language

A program is a bounded vector of nodes; each node is an opcode and outgoing edges. `pc` points to a node index, with register bank `r[0..3]` of signed 64-bit integers. Stage-1 operations:

| Opcode | Behavior |
| --- | --- |
| `Const(dst, value, next)` | write literal into register |
| `Add(dst, lhs, rhs, next)` | wrapping integer addition |
| `Sub(dst, lhs, rhs, next)` | wrapping integer subtraction |
| `Read(dst, addr_reg, next)` | read public shared cell (missing key returns zero) |
| `Write(addr_reg, value_reg, next)` | write a public shared cell, unless capacity is exhausted |
| `Branch(test_reg, if_nonzero, if_zero)` | select graph edge |
| `PatchNext(node_reg, target_reg, next)` | generic mutation of the single `next` control edge on a target node |
| `Halt` | cease this process's scheduling |

An invalid graph reference, register index, or write violating storage capacity yields a deterministic trap. The attempted instruction is still charged. No `REPAIR`, `REPRODUCE`, `AUDIT`, `REBUILD`, `FITNESS` or target-specific contract opcodes exist. `PatchNext` is a generic control-edge operation, not repair semantics.

**Important:** currently only one `next` edge can be patched (not branch edges or node opcode/payload), and program node count is fixed; full code-graph editing, copying, creation and eviction are future milestones. Even if code mutation works, it does not establish autonomous repair.

## 5. Execution and fault semantics

All observable steps are serial and atomic; stable process-id selection, bounded registers, and wrapping arithmetic avoid platform-dependent behavior. A trap stops the offending process and records the failure. No host instruction evaluates whether the mutation or shared write is useful. A stopped process is not garbage-collected or called dead.

## 6. Interventions and measurement

Preregister condition, snapshot identity, exact world rules and initial programs. Baseline: two identical copies reproduce identical traces. Negative controls: `Halt`-only fixture, zero-credit fixture, exhausted capacity, invalid node target. Positive fixtures for graph mutation and shared storage are *engineering tests*, not life evidence.

Later causal experiments must compare identical snapshots with: program-edge removal, shared-object changes, resource withholding, independently labeled external restoration, common-mode fault and no-intervention control. Report total activity, storage usage, structure changes, causal differences, failed interventions and confidence uncertainty when applicable.

## 7. Reproducibility and record format

Record rules/schema revision, source Git commit, initial graph and state, external intervention history, ordered event trace, independent run condition, and deterministic final state. Reproducing a deterministic run does not provide independent evidence of emergence. SQLite experiment registry, versioned canonical serialization and hash chains are **future**; do not misdescribe in-memory traces as SQLite durability.

## 8. Security

The VM cannot call operating system APIs, issue network calls, access host files or arbitrary Rust memory. Storage operations go through bounds checks; no unchecked pointer access. Future persisted CIL must be parsed with strict size and depth limits, and world snapshots are untrusted inputs.

## 9. Deferred design decisions

Fairness under unbounded actor creation, dynamic allocation/garbage collection, resource exchange, copying and reconstruction of executable objects, access-capability isolation, stochastic entropy and seeds, SQLite transaction protocol, intervention API and novelty metrics are not yet implemented. They require separate falsifiable milestones before any life claim.
