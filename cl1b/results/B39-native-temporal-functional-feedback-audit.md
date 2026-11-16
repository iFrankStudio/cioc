# CL-1B B39 — truthful functional output readback can be stale and causally wrong

**Verified exact-source native result:** [GitHub Actions full run 43827711119](https://github.com/iFrankStudio/cioc/actions/runs/43827711119), tested SHA `0ea6f4f624ece356479eb74f0f2630c1d5eff7bd`. [Source-pinned original verification summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/0ea6f4f624ece356479eb74f0f2630c1d5eff7bd-43827711119-1/summary.txt): **39/39 native Rust integration targets PASS, 26/26 source-parsed independent CIL models PASS**, B31/B34 private ABI/serde tests PASS, zero failures, source evidence publication PASS. [Prospective registered B39 protocol](../studies/B39-temporal-task-evidence.md), [native original-source-reusing eight-world Rust assay](../../cl0/crates/world/tests/cl1b_b39_temporal_task_evidence.rs), [independent CIL model](../analysis/b39_temporal_task_evidence_scan.mjs).

## What B39 tests—and what the previous B38 success did not

B38 verified that a continuous single CIL process can use actual task-output `Read`, arithmetic comparison, and a charged `PatchNext` to correct its authored executable path, rather than trusting a marker. **B39 shows that even this functional feedback has a temporal consistency limit:** the same process may compare the actual current output to an **older input value** saved before the environment changed.

B39 reuses **exactly** B38's investigator-authored READBACK and compact immutable STATIC_CORRECT source through the native test module import, with 96 initial local compute credits and no shared pool or host repair. There are eight frozen worlds: task-input changes `0→17` and `17→0`, arriving either **before any native `Read(cell7)`** or **immediately after the first real CIL `Read(cell7)`**; the original source and later instruction execution remain identical. A host-side `SetCell(key7)` is the *environment input*, not a fake reward, code repair or per-process credit replenishment.

## First true CIL feedback checkpoint

The observer stops B38 READBACK immediately after its first actual PC7 discrepancy `Branch`, which follows a true PC4 Write and a true PC5 output read. Both the native actor's chosen PC and the actual current input/output cells are inspected. The source PC3 edge has not yet been modified, and the first branch can be checked against the **current** externally supplied input (which the actor cannot magically know).

| Task source transition | Input update timing | Real output at first check | Correct against *current* task? | Actor's first READBACK decision |
| --- | --- | ---: | --- | --- |
| `0→17` | BEFORE_READ | 0 | **wrong** | discrepancy → repair PC8 (**correct judgement**) |
| `0→17` | AFTER_READ | 0 | **wrong** | no discrepancy → repeat PC1 (**false success**) |
| `17→0` | BEFORE_READ | 0 | **correct** | no discrepancy → repeat PC1 (**correct judgement**) |
| `17→0` | AFTER_READ | 0 | **correct** | discrepancy → repair PC8 (**false failure**) |

**This is a genuine causal mismatch:** the CIL task-output read did not lie, nor did the arithmetic or branch opcode malfunction. At AFTER_READ, the environment changed task cell7 only *after* the actor had already acquired the old expected input in r0. The subsequent actual Write output0 can match that earlier r0 yet fail the **current** task, or mismatch r0 despite already being correct for the current task. In the `17→0 AFTER_READ` case the same individual follows an **unnecessary paid source-repair path**, despite its service output being currently valid.

At the exact checkpoint, B38's original source code is **not yet edited**; later successful correction cannot retroactively validate an earlier false judgement.

## Entire 96-step outcomes (all eight worlds)

| Transition and update timing | READBACK correct / wrong writes (after update) | READBACK actual code patches | Compact STATIC_CORRECT correct / wrong writes |
| --- | --- | ---: | --- |
| 0→17 BEFORE_READ | **12 / 1** | 1 | **24 / 0** |
| 0→17 AFTER_READ | **11 / 2** | 1 | **23 / 1** |
| 17→0 BEFORE_READ | **14 / 0** | 0 | **24 / 0** |
| 17→0 AFTER_READ | **13 / 0** | **1 unnecessary** | **23 / 1** |

Each cell is grounded in genuine native `Origin::Internal(1)` `CellWritten(key11,value)` provenance compared with the externally current cell7 task input after its injection. Exactly96 ordinary CIL steps were charged per world. The simple STATIC_CORRECT program is itself vulnerable to writing an old value **once** when the input changes after its own input read; it has no verification or repair overhead, yet remains far more throughput-efficient and correct on later cycles. This isolates the general asynchronous sampling issue from the more specific failure of B38's self-repair decision.

## Interpretation and next falsifier

B39 rejects a tempting overclaim from B38: **truthful task-level feedback does not guarantee correct decisions about a changing task**. A self-modifying program needs some notion of *input/output consistency*, versioning, atomic sampling, freshness, or repeat verification if concurrent environment changes are possible. A sensor or feedback register alone is insufficient; at the same time, introducing an investigator-supplied version token or hidden host fitness score would predefine part of the goal and could fake success.

An admissible next study is to let the *same CIL individual* pay for rereading the actual task input **after its output is written**, then compare the output to the fresher input without any new VM instruction, and check both true judgement and extra compute overhead versus READBACK and compact STATIC_CORRECT. No instantaneous consistency guarantee can be claimed for unobserved changes after the final read. 

The mismatch, source repair, input update and fixed function definition all remain investigator-authored, and **CL-1B B3/B4 endogenous adaptation/open-ended cyberlife are not achieved**.
