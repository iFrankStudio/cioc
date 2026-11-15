# CL-1B / Study B01 — three-window functional continuity after two native self-edits

**Prospective registration: 2026-11-15, before B01 implementation/outcome inspection.** B01 is a **human-authored mechanism calibration**, not an endogenous adaptive process or life acceptance.

## Frozen question

Can the **same** single CIL process (without spawning descendants) perform an independently measurable function in three successive execution windows while undergoing **two distinct, successful internal executable-code edits**? Does a deleterious edit interrupt that function while a paid suppressed-edit control maintains it?

## Exact assay, fixed before running

Initial source is a 21-node CIL graph, one process id=1, starting pc=0, 40 credits, finite strict capacity (64 abstract units), no compute pool replenishment, no other processes. Public cell key 7 holds a predefined input; CIL must copy it to key 9. Three native `CellWritten(key=9, value=input)` events at actor PCs **3,11,19** define three functional windows. An observation at one PC is not interchangeable with a later one.

Two native `PatchNext` calls are attempted at PCs **6 and 14**, mutating executable Read nodes **9** and **17** respectively; the edits occur after the baseline functional window. Ordinary intended changes are `9.next: 11→10` and `17.next: 19→18`. Nodes 10 and 18 are harmless inserted detours that lead to the next functional write. They are in the same 21-node graph, not new processes.

Frozen arms, each with **identical source code within a matched pair** except the declared damaging-site target operand, and identical compute/storage limits:

1. **Functional edits:** replace PC9 successor with 10, PC17 successor with 18; both edits must actually occur and all three output windows must remain functional.
2. **Edit-disabled control:** same exact source as functional-edits arm; VM `with_native_patchnext_disabled()` charges the two patch attempts without applying them. Same three outputs, zero native edge changes.
3. **Damaging edit calibration:** human-author replacement value of the first next-edge target at PC5 to 20 (Halt) instead of 10; native edit at PC6 must cause loss of the second and third output windows. This is **not** a matched-code causal treatment and must never be claimed to isolate genotype fitness.

Independent **test inputs** predeclared: `[-13, 0, 7, 23, 101]`. These are separate deterministic test conditions, not stochastic independent organisms. Each input runs all three arms: **15 finite worlds**, with no post-selection. No external intervention after initialization.

## Frozen outcomes

- All arms: exactly one externally loaded process; no `GraphLaunched`, no unexpected external action, no traps; `attempts + ending local credits = 40`, and total compute inventory matches ending local credits.
- Functional edits: precisely 2 exact native `ExecutableEdgePatched` observer events (nodes 9 and 17), all three ordered functional writes with the input value, final executable edges 9→10 and 17→18, naturally terminal.
- Suppressed: exactly 2 `NativeCodeEditSuppressed` and zero patched events; same three ordered functional writes, edges 9→11 and 17→19, naturally terminal.
- Damaging: precisely 1 patched event (9→20); only PC3 output survives; PC11/19 absent, naturally terminal; report first missing functional window rather than dropping this arm.
- No increase in functionality vs unedited control is claimed. Both normal and suppressed programs are pre-engineered; any passing test proves only *compatibility* of two code edits with an externally specified function.

## Required follow-up

B02 must probe **internally performed recovery** from an induced break with a preexisting, *CIL-owned and resource-charged* recoverable representation. A host replay or external patch is forbidden as native recovery. B03 must measure accumulated improvement beyond the exact engineered edit locations and evaluate unseen environmental perturbations.

Archive raw run outcomes and source revision before promoting a scientific inference. If compilation or control assumptions fail, record as engineering failure rather than rewriting predictions to match.
