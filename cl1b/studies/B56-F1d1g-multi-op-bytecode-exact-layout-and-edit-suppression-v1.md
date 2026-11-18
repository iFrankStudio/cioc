# B56-F1d1g exact virtual ISA, register allocation, and matched edit-suppression addendum v1

**FROZEN BEFORE ANY F1d1g NATIVE EXPERIMENT SOURCE OR RESULTS.** This fixes details explicitly left open by the earlier B56-F1d1g preregistration. Both documents are binding. No source or VM physics change is authorized post-result.

## Identity, accounting, storage, and test matrix

One PID1 always, native kernel unchanged, `World::with_strict_capacity(16,180,300)`, exactly **1,000 initial credits**, no credit pool. All 24 worlds use **byte-identical original native founder source**, and have no GraphLaunch/GraphNew/GraphEncode nor code-field edits. They differ ONLY by external task/input/feedback and the **authorized initial control key21**. Across seed pairs `(x_initial,x_heldout)=(5,11),(7,-7)`, hidden function `IDENTITY(x)=x` vs `COMPOSED(x)=-x+2`, and feedback `TRUTH`, `BLIND_SUCCESS`, `INVERTED`, cross with `EDIT_ENABLED=1` and `EDIT_SUPPRESSED=0`: 2×2×3×2=**24** worlds. The environment interventions permitted: before first execution key21 control and key7 input; after first actual output11 key20 feedback and key7 new heldout input. Exactly four externally originated SetCell operations per world. No other environmental writes.

**True feedback** = first output matches hidden function; **Blind Success** always1; **Inverted** = 1−correct. First service is always identity and is wrong for COMPOSED on positive5,7. Key21=0 never changes actual data-program slots52 and53, even with negative feedback, but pays the same six Const/Write interpreter-edit instructions as enabled case using a **single shadow key60**, with last value2. This is a *byte-identical-source data-mediated inhibition*, not a native-source edit ablation. It uses one extra control cell and at most one shadow cell. If key21=1 with negative feedback, actor pays for real Write52(1) and Write53(2) while slot54 already equals RETURN opcode3. When feedback positive, neither path is entered and the program remains unchanged.

## Fixed virtual ISA / memory and native register roles

Native registers `r0=0` (constant zero), `r1=accumulator`, `r2=address or literal`, `r3=PC, opcode, data, or feedback`. Initial native source writes ordinary cells as follows: `cell50=0` virtual PC; `cell51=0` LOAD_INPUT; `cell52=3` RETURN; `cell53=3` RETURN; `cell54=3` RETURN; `cell55=2` ADD_LITERAL operand; `cell31=0` stage; no initial host code or virtual-program cells are injected.

The virtual opcode at `cell[51+cell50]` is fetched **dynamically on every interpreted instruction** using native `Read50`, `Const(51)`, `Add address=51+PC`, `Read indirect`. Opcode decoder compares `op==0` via `Branch r3`; if nonzero subtracts1 using `Const r2=1`, `Sub r3=r3−r2`, branches on r3 to dispatch `op==1`; subtracts1 again for `op==2`; otherwise takes RETURN path (only valid0..3 occur in frozen cohort).

- **opcode0 LOAD_INPUT:** read key7 into r1, pay Write56(r1) to record accumulator after the virtual instruction.
- **opcode1 NEGATE_ACC:** r1←0−r1, pay Write56(r1).
- **opcode2 ADD_LITERAL:** read key55 into r3, r1←r1+r3, pay Write56(r1).
- **opcode3 RETURN:** pay Write11(r1) and check stage31; first return routes to feedback, second return halts.
- Every non-RETURN operation increments cell50 by exactly1 using native `Read50`, `Const1`, `Add`, `Write50`, then loops to a new dynamic virtual opcode fetch. Return does not increment virtual PC. On first service the program `[LOAD,RETURN,RETURN,RETURN]` executes two virtual opcodes; after enabled negative feedback it becomes `[LOAD,NEGATE,ADD_LITERAL,RETURN]`, executing four virtual opcodes. **No complete COMPOSED service shortcut exists as a dedicated native output path.** All actual output service is the same fixed RETURN→Write11 native instruction.
- After any first return, PID1 pays Read20 feedback; when false it reads key21 and dispatches to exactly six native `Const/Write` mutation instructions to update two real program slots (enabled), or six shadow-key writes (suppressed). Both paths pay the same operation count. It then pays Write31(1), Write50(0), re-enters the interpreter on the updated external heldout input7. A legitimate native `Halt` occurs only after second return.

**Explicit simultaneous cell key upper bound:** `{7,11,20,21,31,50,51,52,53,54,55,56,60}` =13 ≤16. Graph ≤180 native source nodes and storage ≤300. No graph objects. The records must show *exact* first and second output, intermediate accumulator on NEGATE and ADD_LITERAL, program slot values before/after each mutation, exact native PC per opcode fetch, and every paid step.

## Causal predictions, adversarial null and limits

For ENABLED+TRUTH, all four task/seed pairs succeed on heldout: IDENTITY uses [LOAD,RETURN], COMPOSED uses [LOAD,NEGATE,ADD,RETURN]. ENABLED+BLIND_SUCCESS succeeds on the two IDENTITY pairs, fails two COMPOSED. ENABLED+INVERTED fails all four. SUPPRESSED+TRUTH succeeds two IDENTITY, fails two COMPOSED despite receiving identical true feedback and paying matching edit-branch instruction count. SUPPRESSED+BLIND_SUCCESS succeeds two IDENTITY only. SUPPRESSED+INVERTED succeeds two IDENTITY only, because requested bad updates are diverted into shadow60. Heldout values11 and−7 are deliberately not equal initial5 and7, and neither input is a functional alias between x and −x+2.

Successful **narrow claim**: same-PID feedback-dependent *multiple data-program-slot* edits and genuinely interpreted multi-opcode composition, under strict resources and actor-paid operations, with registered controls. **Not demonstrated** even on perfect scores: autonomous discovery of opcode semantics, open-ended program space, original F1c continuation, full B53 auth, N56-A/B/C, B3/B4 or cyberlife.
