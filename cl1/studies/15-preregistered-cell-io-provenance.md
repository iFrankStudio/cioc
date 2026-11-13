# CL-1 Study 15 — observer-only shared-cell I/O provenance calibration

**Registered 2026-11-13 before the new observation instrumentation or its execution.** Study-14 results are known: randomized graph-copy reciprocity occurred in 4/96 W1, 0/96 W2 and 0/96 W3; scientific source-sensitive output transformation was not evaluable.

## Fixed question

Can the Rust observer unambiguously record every *successful* CIL shared-cell read and write as a first-class, actor-attributed event without granting new capabilities to programs, changing the deterministic schedule, or treating temporal proximity as causal dataflow?

This is an **instrumentation test**, not an organism experiment. Its result is technical pass/fail, never a CL-1 positive scientific result.

## Event contract frozen before implementation

Add `CellRead { key: i64, value: i64 }` and `CellWritten { key: i64, value: i64 }` to existing `EventKind`, emitted **only after** successful native `Op::Read` / `Op::Write` execution, respectively. Preserve `Origin::Internal(process_id)`, actual instruction `pc`, credit charge and existing tick; no additional events or clock increments per instruction. Missing key reads are valid and yield 0. Trapped writes, e.g. storage exhausted, must never emit `CellWritten`. `Intervention::SetCell` remains an **external intervention event**, not internal writing.

One successful instruction gets exactly one normal outcome event (the new I/O event replaces `Executed` for that instruction); an unsuccessful instruction stays `Trapped`. No change to VM instruction semantics, resource accounting, or serialized process/world state apart from the event kind.

## Technical controls

- Focused Rust test: valid read (including absent-key zero), valid write, and rejected out-of-capacity write; assert actor/PC, values, and absence of fabricated success.
- World replay: the same history produces byte-equal states/events when independently rerun; historical snapshots must remain decodable because only additive event variants are introduced.
- Existing CL-0 regression and Studies 01–14 pass full workspace tests, format, lint and preregistered seed-world audits.
- 0 extra compute credits or storage units granted by instrumentation. Same world state including process registers, object graphs, cells and credit accounting.
- No retroactive relabeling of Study-14 output as functional recovery.

## Interpretive boundary and next gate

These events identify **who read/wrote a key and when**, but they do **not** establish a causal chain from a particular graph node to the value written, or an independently assembled maintenance mechanism. A future source-taint / intervention experiment must test whether specific input mutations cause changed outputs at a fixed resource budget. No life claim or CL-1 acceptance from Study 15.
