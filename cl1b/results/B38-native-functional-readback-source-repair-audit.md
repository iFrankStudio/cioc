# CL-1B B38 — native functional readback causes in-place code repair after actual task failure

**Verified native engineering result:** [GitHub Actions run 43826841272](https://github.com/iFrankStudio/cioc/actions/runs/43826841272), exact tested SHA `c5d1cb35daa87fe87acef4f1030f781a69d7991a`. Original [source-pinned full CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/c5d1cb35daa87fe87acef4f1030f781a69d7991a-43826841272-1/summary.txt): **38/38 native Rust integration targets PASS and 25/25 independent source-parsed CIL analyses PASS**, B31/B34 private ABI tests PASS, zero failures and independent evidence branch publication PASS. [Prospective preregistration](../studies/B38-functional-readback-conditional-repair.md), [native source and full 20-world differential](../../cl0/crates/world/tests/cl1b_b38_functional_readback.rs), [independent 96-step-per-world source-parsed CIL interpreter](../analysis/b38_functional_readback_scan.mjs).

## New causal mechanism compared with B34–B37

B34–B35 demonstrated resource-availability feedback. Those results alone never proved the actor could evaluate its **actual service**. In B38 a **single continuous native process** is handed an externally authored task: copy the actual value at shared cell7 into shared cell11. The source begins with a deliberately wrong executable Write path (writes constant0). The same CIL process runs a genuine `Read` of its output cell11, uses `Sub(expected−observed)` and `Branch` to decide whether to spend computation on **real `PatchNext` of its own PC3.next, from wrong Write PC4 to correct Write PC11**. The process retains that source edit across an in-world change of input value without replacement, reset, offspring or external code repair.

The host observer counts **actual native `CellWritten(key11,value)` events**, independently checking against the declared current input during each phase. A marker or process-supplied claim of success does not count.

## Frozen comparisons

Every world starts with exactly **96 local compute units**, no neutral resource injection, and exactly **48 native charged instructions per phase**. After the first phase, the researcher changes only the shared task input cell7 with an external `SetCell`, and the same process pays another 48 instructions. Four preregistered histories—`0→17`, `17→0`, `23→−9`, `−9→23`—cross five authored arms: READBACK, ASSUME_OK, FORCE_FIX, PATCH_DISABLED and compact STATIC_CORRECT. All **20** worlds are preserved. Every instruction, functional read, comparator, native source mutation and write costs one real compute unit; no special learning/fitness opcode exists.

| Two-phase real input | READBACK correct writes | ASSUME_OK | FORCE_FIX | PATCH_DISABLED | STATIC_CORRECT |
| --- | ---: | ---: | ---: | ---: | ---: |
| `0→17` | **7 + 5 = 12** | 7 + 0 = 7 | 5 + 5 = 10 | 7 + 0 = 7 | **12 + 12 = 24** |
| `17→0` | **5 + 6 = 11** | 0 + 7 = 7 | 4 + 5 = 9 | 0 + 6 = 6 | **24** |
| `23→−9` | **5 + 6 = 11** | 0 | 4 + 5 = 9 | 0 | **24** |
| `−9→23` | **5 + 6 = 11** | 0 | 4 + 5 = 9 | 0 | **24** |

Wrong writes are recorded separately and never treated as success. In the `0→17` world, READBACK's original wrong Write happens to be **correct while the task input is zero**, so **no edit is made during its first 48 native attempts** (PC3.next remains4). When input switches to17 the same actor now reads back a mismatched real output, executes its original CIL repair code, changes PC3.next to11 exactly once, and produces genuine correct writes. This cleanly distinguishes **feedback-triggered editing** from an arbitrary early deterministic timer-based patch in that comparison.

Under each nonzero first input, the actor's initial bad output triggers a real source patch during phase one. The patched source is retained when the externally supplied task input changes at phase two and correctly passes later values, including negatives. The source change is observed in both `Process.code.nodes[3]` and native `ExecutableEdgePatched{node:3,before:4,after:11}` provenance; code editing suppressed in the matched PATCH_DISABLED arm cannot improve the bad executable path.

## Negative and alternative explanations retained

- **ASSUME_OK** performs the same paid output `Read` as READBACK, but its deliberately wrong comparator ignores that read and always assumes no discrepancy. Its code stays broken and it produces zero correct nonzero outputs. So the feedback-driven branch genuinely matters to repair.
- **FORCE_FIX** pays the same number of native comparator instructions and changes the same code on a preauthored constant1 branch regardless of actual output. It also delivers working code, and on the initially correct `0→17` history it patches unnecessarily early. That shows that the source repair target and correction are *already known to the program's designer*. Relative READBACK gains in this finite set do **not** establish discovering unknown repairs.
- **PATCH_DISABLED** executes the original READBACK and attempts a paid `PatchNext` but the experiment world suppresses it. Zero real native edge changes and worse task behavior confirm the source modification itself is causal.
- **STATIC_CORRECT**, a compact four-node immutable copier with the exact same compute endowment, makes **24 correct writes** across every two-phase history. This is substantially more efficient than all 12-node self-repair sources. The simpler fixed strategy is still the strong practical baseline.

## Acceptance and scope

B38 is a meaningful **substrate+algorithmic mechanism** step: real *task-level functional state* (not resource-transfer success, not a self-written success marker) is read by an existing CIL operation; the result **causes conditional source repair**, and that code change persists across a real input shift inside the **same process**. It is stronger causal evidence for feedback-triggered self-maintenance than a single authored resource fallback.

It is **not autonomous discovery of a goal, a candidate code mutation or a new algorithm**. The cell addresses, task, expected result, wrong/working Write nodes, comparator and exact repair edge were all authored by researchers, and the repair pays more per useful output than a known static copier. The actual output cell can also become stale or externally modified between read and verification. Future work must test those measurement-timing and provenance limitations; it must not introduce a hidden host repair/fitness oracle or report this prewritten functionality as cyberlife.

**CL-1B B3/B4 and open-ended adaptation remain NOT MET.**
