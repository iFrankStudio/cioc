# Study 34 — verified Rust Study32/33 generator comparison

**Scope:** read-only inspection of current repository Rust runner source and Study33 Python auditor; **not** a historical commit-pinned binary/SQLite rerun. No new seeds or executions.

Compared:
- `cl0/crates/lab/src/bin/cl1-study32.rs`, `seeded_world`
- `cl0/crates/lab/src/bin/cl1-study33.rs`, `seeded_world`
- `cl1/analyze_study33.py`, `expected_program_continuations`
- Frozen `cl1/studies/33-preregistered-resource-linked-source-function.md`

**Confirmed:** both Rust runners use the same 9-instruction scaffold, including `GraphNew`, `GraphCopyNode`, `Const`, `Write`, `GraphGrantRead`, `Read`, and no `GraphReadTag`. Their 18-instruction random continuation uses the same 64-bit LCG and 12-way opcode map; the two `GraphReadTag` choices are exactly `(dst=0,graph=3,node=2)` and `(dst=2,graph=1,node=2)`. The Study33 independent Python generator matches these Rust choices. The changed seed cohort is 2,500,000..2,500,191 in Study32 and 2,600,000..2,600,127 in Study33.

**Confirmed discrepancy:** Study33's frozen human preregistration calls those alternatives `GraphReadTag(g1)` and `GraphReadTag(g2)`, whereas actual Study32/33 Rust and Python specify graph3 and graph1. This is a **human protocol shorthand error relative to the unchanged Study32 generator**, not evidence that Study33's runner silently changed its inherited generator. Preserve the original registration and explicitly note this correction.

**Stronger static result:** neither the original scaffold nor the randomly generated continuation contains a direct `GraphReadTag(graph=2,node=0)`. This makes the strict graph2 native source read unreachable **in the fixed initial process code**. A universal execution-level impossibility still requires ruling out runtime code modification/replacement and inspecting the archived execution data at the exact Study33 source revision. The strict Study33 0/128 outcomes remain recorded, but must not be treated as evidence of the prevalence of joint source/resource function in a grammar with eligible source2 reads.

**Next gate:** historical SHA-pinned Study33 execution and SQLite event audit, including dynamic code ancestry. Do not introduce new seeds or claim a completed retrospective cohort analysis from this static finding. **CL-1 OPEN.**
