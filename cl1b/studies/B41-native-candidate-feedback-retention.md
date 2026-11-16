# CL-1B B41 — paid native choice between unknown executable alternatives

**Protocol frozen before first native Rust CI result; the independent CIL model was executed first on 2026-11-16.** This is **not** a prospective preregistration with respect to the observer model. Do not label results native-verified until source-pinned CI succeeds. CL-1A remains archived; no reproduction mechanism is involved.

## Scientific question

Can one continuous individual pay for real task-function trials, use observed output (rather than a synthetic success marker), switch to a second **environment-supplied** executable alternative, and retain a source-level change in-place? How easily does a **single** trial overfit a task input that cannot distinguish two implementations?

The question is deliberately narrower than endogenous creation of executable hypotheses or a self-generated evaluator. The investigator supplies both candidates, task and comparator. The actor reads candidate-address data but does not invent the algorithms.

## Frozen world and native CIL mechanism

One PID1, unchanged 29-opcode CIL founder, **96 initial local compute credits**, no top-ups, host repair, second actor or reward. Finite storage; every native instruction paid. The environment sets `cell20=candidate A`, `cell21=candidate B`, `cell7=training input`; the actor writes actual task output to `cell11`. Candidate node14 is a **constant-zero Write**; node15 is a **live-input Write**.

CIL itself reads and patches its PC8 service router to point to candidate A, then executes that candidate. Both candidates route to a real CIL `Read` of the resulting output followed by `Sub` against the sampled input and `Branch`. Failure triggers a CIL-read alternative from cell21, native `PatchNext` to install it and an actor-written flag in cell22. If the second trial also fails, the actor `Halt`s instead of falsely succeeding. A successful trial uses native `ReadNext` and `PatchNext` to edit the selected candidate's own `Write.next` **9→5**, retaining it for later unverified service. No VM changes or host fitness instruction.

## Matched study and controls

Exactly **2 candidate orders × 3 training inputs × 2 held-out inputs × 4 source arms = 48 worlds**:

- Candidate order: `[14,15]` or `[15,14]`.
- Training: `0`, `17`, `23`; holdout: `-9`, `17`.
- At **48 instruction opportunities**, the host changes only cell7, then grants **up to 48** additional opportunities. A halted actor retains unspent compute; include this failure in counts.
- FEEDBACK: actual output readback and branching.
- ASSUME_PASS: same-cost `Sub(r0−r0)` comparator, blindly accepts first candidate.
- FORCE_FALLBACK: same-cost `Const(1)` comparator, blindly rejects; may halt after both.
- STATIC_CORRECT: four-opcode immutable copier, same initial compute budget, strong baseline.

Read true correct/wrong task writes from native internal-origin `CellWritten(key11,value)` events; verify causal `ExecutableEdgePatched` events, actor-generated flag22, termination and total compute. Do not judge solely from program's own comparator.

## Frozen observer-model predictions before native CI

The independent interpreter parses the Rust literal source and forecasts **48 model worlds**. Across 12 FEEDBACK cases it predicts 12 paid retention edits and four fallback flags. For nonzero training `17/23`, both candidate orders ultimately retain node15. For training `0` with first candidate14, the erroneous constant-zero implementation passes its single task trial and later makes **zero correct, nine wrong holdout writes** in each of two nonzero holdout conditions.

Across its twelve-world aggregate: FEEDBACK predicts 91 correct / 21 wrong holdout writes; STATIC_CORRECT predicts 144 correct / 0 wrong. FORCE_FALLBACK halts in all 12 worlds, spending substantially less compute than the others. Timing across the train/holdout boundary may cause a stale write even after retaining the right source. These are **observer predictions**, not native findings.

## Falsification and scope

Mechanistic success requires *native* evidence that true task feedback, and not an investigator's direct source choice, conditions an on-actor candidate switch and lasting paid edit. The zero-input overfit is an expected strong **negative** result, not an outlier to discard. Preserve all wrong writes, traps and halts; never report best-of-N as a learned life mechanism.

**Scientific boundary:** task, candidate inventory, candidate addresses and decision machinery are author-defined. No agent-originated novel algorithms, reliable multi-context validation, self-originated policy search or B3/B4 acceptance is claimed. Next discriminatory obstacle is multi-context evidence with genuine costs and evaluator integrity, not another unconstrained scripted guard.
