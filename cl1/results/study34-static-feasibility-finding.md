# Study 34 — source-level feasibility finding (pre-execution diagnostic)

**Classification:** source-level static audit, not an executed 128-world Study34 outcome. Recorded after Study33 results and after Study34 retrospective registration. No new ecological worlds, no new experiment runs, no independently replayed SQLite snapshots in this finding.

## A decisive grammar-level obstruction

The frozen Study33 generator is independently restated in `cl1/analyze_study33.py::expected_program_continuations` and in the Study33 preregistration. For every seed and each actor, the only two `GraphReadTag` branches in its 12-way continuation grammar are:

- choice 3: `GraphReadTag {dst:0, graph:3, node:2, next:pc+1}`
- choice 4: `GraphReadTag {dst:2, graph:1, node:2, next:pc+1}`

There is **no choice that generates `GraphReadTag {graph:2,node:0}`**. The original nine-instruction scaffold must also be checked against actual recorded code before a universal *executed-program* impossibility claim is made. In the registered Study33 endpoints, the observer requires an actual successful native `GraphTagRead` from `graph:2, owner:2, node:0`, tag 11/19, followed by register-def-use-qualified output. The random continuation therefore cannot itself supply the required source2 read. Graph copies, graph object replacement and ACL changes do not rewrite the fixed `GraphReadTag` source operand in the process code. The external graph2 edit changes a graph object's code tag, not a `GraphReadTag` operand in the process program.

This is an **a priori structural non-exposure of the continuation grammar** to the strict source2 read endpoint, not a statistically informative failure of native source-resource integration. The Study33 zero numerators remain accurate under their frozen definitions; their 0/128 Wilson bounds must **not** be used as an empirical upper bound for a mechanism that the continuation grammar cannot express. In particular, a zero numerator cannot justify claims about how often joint source/resource behavior emerges under grammars that allow graph2 reads.

## Required Study34 follow-up

1. Verify all 128 persisted initial programs, including the nine-instruction scaffold, against the exact original source-pinned SQLite data. Record all actual `GraphReadTag` operands and successful source-specific events, including graph1 and graph3, without relabeling them as graph2.
2. Independently confirm the grammar-to-program identity and that no postfork native code replacement changes the executing actor's program before the alleged read. Fail closed if a dynamic-code exception is present.
3. If the complete recorded process1 programs indeed contain no graph2 read and no valid runtime replacement, mark graph2 source-read exposure **STRUCTURALLY UNREACHABLE UNDER THIS GENERATOR**, with conditional downstream success rates **NOT EVALUABLE**. Do not report zero conditional functional success as an informative failure rate.
4. Keep native Draw/Return evidence and pool accounting as valid separate measurements; neither can overcome missing code-source read reachability.
5. Any next new-seed experiment must be **preregistered before sampling**, and should make source2 reads *possible but not guaranteed* through an ordinary, unbiased CIL instruction grammar, while keeping explicit negative controls and externally endowed resources distinct from autonomous supply.

## Scientific boundary

This finding does not alter Study33 raw data or preregistered outcomes, and does not constitute a completed Study34 cohort audit. It is a source-level methodological diagnosis that should inform how the retrospective Study34 auditor is implemented and interpreted. **CL-1 remains OPEN, with no accepted self-maintenance or cyberlife evidence.**
