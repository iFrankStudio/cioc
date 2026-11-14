# Study 34 — preregistration-versus-auditor grammar discrepancy

**Status:** documented source-level discrepancy, not a completed run-level replay or new prospective result. **0 new independent worlds.** This note supplements `study34-static-feasibility-finding.md`.

## Evidence and exact discrepancy

The frozen Study33 protocol `cl1/studies/33-preregistered-resource-linked-source-function.md` describes the two `GraphReadTag` alternatives in the 12-way random continuation as **`GraphReadTag(g1)` and `GraphReadTag(g2)`** and claims the generator is byte-for-byte identical to Study32.

The actual independent Python generator validator `cl1/analyze_study33.py::expected_program_continuations` enumerates:

- `choice == 3`: `GraphReadTag(dst=0, graph=3, node=2)`
- `choice == 4`: `GraphReadTag(dst=2, graph=1, node=2)`

Thus the validator describes **graph3/node2 and graph1/node2**, not the preregistered shorthand **g1 and g2**. It never directly generates a `GraphReadTag(graph=2,node=0)` continuation. The strict endpoint `trace_source_and_pool` requires the successful native event `GraphTagRead(graph=2, owner=2, node=0, tag=11/19)` to label an address register.

The shorthand `g1/g2` may have been an erroneous human summary of an unchanged Study32 generator; whether this is a substantive preregistration deviation cannot be resolved from these two documents alone. **Do not silently rewrite the original preregistration.** Preserve it, and source-compare Study32 and Study33 Rust generator implementations, immutable original Study33 runner commit, and recorded per-seed program nodes.

## Immediate consequences for interpretation

1. The existing 0/128 primary endpoints are still the reported outputs of the Study33 audit; this note does not replace or alter them.
2. No empirical inference about the probability of joint source2-read/resource behavior is justified **if** the complete actual executed code never has an eligible graph2/node0 read. This requires inspection of the **nine-instruction scaffold and any dynamic code edits**, not just the 18-instruction random continuation.
3. The statement that Study33 is prospectively protocol-conformant **requires a line-by-line Study32/Study33 source comparison** and an explanation of the `g1/g2` versus `g3/g1` mismatch. Until resolved, report **PROTOCOL-COMPLIANCE QUESTION OPEN**, rather than asserting confirmed noncompliance or dismissing the mismatch.
4. Study34 is retrospective and adds no independent worlds. Conditional gate probabilities with zero eligible read exposure must be labelled **NOT EVALUABLE**, not 0% efficacy.
5. The next prospective experiment must register exact machine-readable opcode alternatives (graph IDs, node indices, destination registers) in a separate prior commit, use fresh independent seeds and include positive/negative reachability calibration without constructing a guaranteed self-maintaining organism.

## Remaining verification

Read the original Rust source of Study32 and Study33 at their exact runner revisions, compare the scaffold, 64-bit LCG and opcode map, check runtime graph/program mutation possibilities, and rerun the source-pinned Study33 SQLite auditor. None of these checks has been executed by this source-level note.

**CL-1 remains OPEN. No autonomous self-maintenance or cyberlife evidence is established.**
