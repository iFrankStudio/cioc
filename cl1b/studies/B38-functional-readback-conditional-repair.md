# CL-1B B38 — same-actor functional readback, conditional source repair and changing real inputs

**Prospective specification, 2026-11-16; authored before coding or observing B38 outcomes.** This is the CL-1B **single continuing individual** route. B34–B35 delivered genuine *resource acquisition* success feedback, but resource success is not evidence that a program's **actual function** is correct. B12 demonstrated marker spoofing. B38 uses only existing native CIL `Read`, `Write`, `Sub`, `Branch`, `PatchNext` to let the actor compare **real output state** to its input. The performance objective is externally defined and authored: copying value at shared cell7 to cell11. **No privileged fitness opcode, energy award, additional actor, automatic code repair or endogenous goal discovery.**

## Registered 12-node self-repair source, fixed ablations

Exactly one externally loaded PID1 with **96 initial local compute units**, no neutral compute pool. Strict world storage16 cells, max64 executable nodes, combined capacity128; `with_native_executable_edit_trace()`. No custom VM opcode. Four same-code-cost arms for mutation causality, plus one compact immutable source:

- **READBACK:** PC5 performs native `Read(r1,cell11)`; PC6 `Sub(r1,expected r0 − observed r1)` and PC7 `Branch` on the actual discrepancy. When nonzero, PC8–10 pay for setting the target and **native `PatchNext` PC3.next from PC4 (wrong output) to PC11 (correct output)**. This is a source mutation of *the same individual*, not an external host edit.
- **ASSUME_OK:** same code, same paid PC5 `Read`, but replace PC6 with same-cost `Sub(r1,r0−r0)` that always returns zero. Native output is available but **causally ignored**.
- **FORCE_FIX:** same code, same paid PC5 `Read`, but PC6 replaced with same-cost `Const(r1=1)`; forces repair independent of function evidence. Shows an authored blind fix can equal or outperform measurement.
- **PATCH_DISABLED:** identical READBACK source; the native generic `with_native_patchnext_disabled()` control charges the exact same attempted instruction, validates it, and suppresses the edge edit. Shows actual source mutation, rather than the event flag, is necessary.
- **STATIC_CORRECT:** 4-opcode compact immutable CIL source `Const(key7)→Read(input)→Const(key11)→Write(input)→loop`. Starts with same96 compute, serves as the full-cost efficiency bound.

## Exact normal 12-node source (one programmed defect)

| PC | Native opcode | Next / purpose |
| ---: | --- | --- |
|0|Const r3=0|1, authored bad output literal|
|1|Const r2=7|2|
|2|Read r0 from cell[r2]|3, real task input|
|3|Const r2=11|**4 initially**, source edge that may be patched to11|
|4|Write cell[r2] = r3|5, initially wrong for nonzero input|
|5|Read r1 from cell[r2]|6, native readback of actual shared output|
|6|Sub r1=r0−r1|7, readback comparison|
|7|Branch(test=r1,nonzero8,zero1)|output mismatch gates repair|
|8|Const r1=3|9, source node to repair|
|9|Const r2=11|10, new control edge|
|10|PatchNext(node=r1,target=r2,next1)|real one-time source structural repair|
|11|Write cell[r2] = r0|5, correct output after source patch|

The cyclic program does not just write a self-reported success marker: it reads **the same actual output cell** and compares it to input, while an external *analysis-only* observer independently records each true `Origin::Internal(1)` `CellWritten(key11,value=X)` matching the actual input value at that time.

## Frozen changing-input histories

Exactly four two-phase histories with **48 native charged attempts per phase** =96 total per world:
- `0→17`: defect initially invisible (writing0 happens to be correct), then meaningful signal appears.
- `17→0`: defect visibly wrong first, then a later input that makes it correct.
- `23→−9`: wrong both phases with sign change.
- `−9→23`: wrong both phases with reverse order.

Use **five arms × four histories = 20 native worlds**. Host interventions are restricted to `SetCell(key7,value)` once **before each phase**, not to edit code or pay credits. Explicitly store snapshot of the actual current input at each phase. Count true output writes per phase using the native event stream and the phase input, and track mismatched writes, total writes, self-edit attempts, actual `ExecutableEdgePatched{node:3,before:4,after:11}`, `NativeCodeEditSuppressed` and final executable PC3.next.

An independent Rust-source-parsed CIL interpreter must reproduce the **same exact 96 paid attempts and phase input changes**, using real shared-cell read/write, native source edge patch and branch semantics—not an abstract success counter. Native Rust run must assert the independent model's full 20-world outcomes; negative cases cannot be omitted.

## Interpretation / scientific falsifiers

- **READBACK 0→17:** there must be **no repair before the environmental change**: writing0 equals input0. After new input17 appears, the actual output becomes visibly wrong; the actor must pay for a real PC3 edge patch 4→11, then use the repaired code to produce true correct writes. This is evidence-conditioned mutation **within one continuing process**.
- **READBACK 17→0,23→−9,−9→23:** after the first nonzero discrepancy, a real paid source edit is retained through the second environmental input; no host reset and no separate offspring.
- **ASSUME_OK** must never patch PC3 and must produce wrong writes whenever the source value is nonzero; its source is deliberately not responsive to actual readback despite equivalent read/write cost.
- **FORCE_FIX** may match READBACK in wrong-first histories, and may outperform in complexity terms because it does not need to use the measurement correctly. Preserve this **strong blind authored policy** counterexample, particularly for `0→17` (where it can patch unnecessarily early).
- **PATCH_DISABLED** repeats attempted repair but cannot mutate the real PC3 edge; repeated wrong outputs must remain observable.
- **STATIC_CORRECT** is expected to execute more correct copies per charged compute unit. Do not call the more complex self-repair program adapted in the sense of optimality or endogenous goal discovery if this simpler authored policy wins.
- The actor's input `cell7` is an **environment-controlled reference to which correctness is compared**. Real readback detection of wrong output is task-specific **authored evaluation logic**, not a general-purpose self-generated fitness criterion. It may be spoofed by other writers or by a defective evaluator and must be stress tested separately.

If positive, this is the first **direct native functional-state-driven source repair** in the B31–B38 sequence (not just resource-acquisition success). It is an engineering precursor to autonomous adaptation, not proof the actor independently invented a goal, generated candidate programs, chose among multiple unknown changes or achieved CL-1B B3/B4 acceptance.
