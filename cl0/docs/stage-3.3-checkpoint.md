# CL-0 Stage-3.3 checkpoint — 2026-11-13

## Research question and scope
Can generic CIL operations build a new instruction graph using **data** rather than just copying already available opcodes? This is a language expressiveness and provenance milestone, **not** a test of spontaneously emergent life or organizational closure.

## Implemented
1. Version-1 `Op::decode_instruction` covers every Stage-3.3 CIL opcode with a stable five-integer descriptor.
2. `GraphEncode` reads descriptors through ordinary public world storage and appends one node to an owner-controlled graph.
3. All instruction attempts consume compute credit; append requires one remaining code storage unit.
4. Incomplete/unknown/invalid descriptors and blocked resource allocations fail atomically.
5. Constructed CIL programs can assemble, launch and stop an executable graph from supplied descriptor data.
6. A separate fixture writes all five descriptor cells using ordinary internal CIL `Write` instructions, and then builds a new `Halt` node without external `SetCell` intervention. This shows internal **mechanical expressiveness**, not emergent behavior.

## Failures and regression safeguards
- Initial commit `8034387` placed `decode_instruction` outside an `impl Op` block; the Rust build failed. Fixed by `b3915ed`. Do not omit this failure from subsequent claims.
- The negative control deliberately withholds all computational credit. An ordinary initial program remains a hand-authored research fixture.
- A matched SQLite regression verifies identical baseline snapshots, the externally attributable descriptor initialization, and exact replay of positive and zero-credit conditions.
- In the internal-descriptor fixture, initial code is still designed by the experimenter. The test cannot be interpreted as an autonomous discovery of an instruction or a novel goal.

## Verification boundary
The authoritative verification record is the GitHub Actions run matching the source commit, **not** a prior green workflow. Commit `a56231d` passed CI: https://github.com/iFrankStudio/cioc/actions/runs/43400989531 (29/29 tests, strict Clippy). The subsequent internal-data descriptor fixture in commit `4429937` must be validated independently.

## Known limitations
Opcode types remain a finite vocabulary; register file and mutable graph construction semantics remain fixed. Graphs can store branch instructions but branch-edge mutation after assembly is not yet generalized. Graph handles are owned by one process ID, without delegated rights or revocation. Code/data node allocations are charged, but process metadata, working registers, detailed byte sizes and observer logs are not accounted for in the same world pool. No observation of autonomous self-maintenance, novel organizational closure, or open-ended evolution has occurred.
