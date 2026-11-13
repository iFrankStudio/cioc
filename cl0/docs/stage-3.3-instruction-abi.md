# CL-0 Stage 3.3 — Instruction encoding from ordinary world information

**Status:** implemented increment with deterministic checks. This document describes general VM mechanisms, not a life experiment.

## New capability
`GraphEncode { graph, address, dst, next }` reads **five existing integer data cells** at keys `base..base+4`, where `base` is obtained from the register indexed by `address`, the destination graph handle from register `graph`, and `dst` receives the new node index. These cells have ordinary world storage costs and may be written by ordinary CIL instructions or by explicitly recorded experiment interventions. No host-provided maintenance objective or desirability filter exists.

All five cells are mandatory. Input shape is `[opcode_tag, operand0, operand1, operand2, operand3]`. Unused operands have no effect for that tag. Index and register operands must be representable as nonnegative `usize` values; the `Const.value` literal is signed `i64`. The appended node is an `Op`, not a sequence of external Rust source code. Generated control flow is **not** automatically relinked or optimized.

## CIL instruction ABI — base tags 0–19 with additive extensions 20–29

| tag | opcode | operand0, operand1, operand2, operand3 |
| ---: | --- | --- |
| 0 | Const | dst, signed value, next, unused |
| 1 | Add | dst, lhs, rhs, next |
| 2 | Sub | dst, lhs, rhs, next |
| 3 | Read | dst, address, next, unused |
| 4 | Write | address, value, next, unused |
| 5 | Branch | test, nonzero, zero, unused |
| 6 | PatchNext | node, target, next, unused |
| 7 | ReadNext | dst, node, next, unused |
| 8 | CloneNode | dst, source, next, unused |
| 9 | PatchConst | node, value, next, unused |
| 10 | Delete | address, next, unused, unused |
| 11 | GraphNew | dst, next, unused, unused |
| 12 | GraphCopyNode | graph, source, dst, next |
| 13 | GraphSetNext | graph, node, target, next |
| 14 | GraphSetConst | graph, node, value, next |
| 15 | GraphReadNext | dst, graph, node, next |
| 16 | GraphDrop | graph, next, unused, unused |
| 17 | GraphLaunch | graph, credits, dst, next |
| 18 | GraphEncode | graph, address, dst, next |
| 19 | Halt | unused, unused, unused, unused |
| 20 | GraphGrantRead | graph, reader, next, unused |
| 21 | GraphRevokeRead | graph, reader, next, unused |
| 22 | GraphReadTag | dst, graph, node, next |
| 23 | DrawCompute | amount, next, unused, unused |
| 24 | ReturnCompute | amount, next, unused, unused |
| 25 | TransferCompute | recipient, amount, next, unused |
| 26 | GraphReadField | graph, node, field, next |
| 27 | GraphReplace | graph, node, address, next |
| 28 | GraphCopyFrom | target, source, node, next |
| 29 | GraphTransfer | graph, recipient, next, unused |

`GraphCopyFrom` copies an instruction node from an owned or read-authorized source graph into an actor-owned target graph, consuming one storage unit; the register selected by `node` receives the newly appended node index. `GraphTransfer` changes an existing graph object's owner to an existing process, without copying its contents or minting resources. An owner's previous owner privileges vanish after transfer unless a read grant independently permits limited access. In-place transfer preserves explicit read grants. Both operations are ordinary, non-teleological VM primitives, not special cooperation or reproduction actions.

Existing numeric tags are never renumbered; new tags are explicitly additive and require updating this table and the conformance tests. `GraphReadField` reads descriptor field 0–4 from an owned or authorized graph node into the register that previously held the field selector. `GraphReplace` overwrites an owned node from a validated five-cell descriptor without allocating another node. `DrawCompute`, `ReturnCompute` and `TransferCompute` transfer existing computation credits; the world pool can be replenished only through an explicitly recorded external intervention. A new tag requires an explicit *world ABI revision*. The opcode vocabulary is fixed at this stage; combinatorial reconstruction within this vocabulary should never be marketed as open-ended invention of new primitives. An assembled program can contain control-flow edges that do not yet point to valid nodes. If executed, invalid edges trap according to ordinary VM semantics.

## Accounting and atomicity
`GraphEncode` costs one process compute credit per attempt. Five descriptor cells already occupy five data storage units. A successful append consumes one additional graph-node unit. No allocation happens on insufficient data, unknown tag, bad numeric operand, missing object, wrong owner, or exhausted node/total capacity. No externally specified instruction repair/survival oracle or exception is introduced. Failed attempts stop the process as deterministic traps.

`GraphLaunch` remains a **move**, not automatic replication: it transfers a graph object's code nodes to a process, releases the object header, and transfers *existing* credits from its caller.

## Negative controls and experimental reproducibility
- A constructed descriptor of `[19,0,0,0,0]` yields a `Halt` node; it is a VM fixture only.
- Unsupported opcode tag and negative register/edge fields are rejected.
- Malformed descriptors cannot partially modify the object.
- Given the same snapshot, the normal condition can create and launch a described program, whereas zero-compute-credit control cannot.
- SQLite records both conditions and exactly replays complete world state and event order, including experimenter-injected descriptor data.

## Outstanding risks
There is no arbitrary host-code execution, version-independent ABI upgrader, delegatable or transferable graph ownership, calibrated byte-level memory pricing, or complete untrusted SQLite deserialization hardening. A strict allocation mode prices process headers and read-permission entries as abstract units. Five-cell interpretation is an abstract fixed tariff; it is not calibrated physical energy use. External initialization of descriptors remains an investigator intervention and is not evidence of spontaneous invention.
