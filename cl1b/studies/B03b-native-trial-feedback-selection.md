# CL-1B / B03b — one-actor native trial-feedback selection calibration

**Prospective registration 2026-11-15, before implementing B03b test or inspecting outcomes.** Calibrates CIL-visible trial feedback -> candidate path decision -> self-code edit. **Not endogenous discovery or accepted B3.** Original program and choice policy are *researcher-authored*.

## Hypothesis / question

Can the *same* native CIL actor execute a provisional candidate computation, compare the trial result with locally known input, then mutate an instruction's next edge to select a future execution path — without spawning a helper actor and without host choosing a winner? Disabling native patching in an otherwise identical seeded world should preserve a different branch path.

This is **decision dependent on internal observations**, but a scripted fitness proxy/correctness test is embedded in the founder. It does not establish native discovery, unscripted goals, generalization or cumulative improvement.

## Exact fixed program layout

One 26-node CIL process id1, 40 credits, strict world storage64, shared input cell key7 and publicly supplied trial-offset cell key11, no pool supply.

- 0 Const r2=7 →1; 1 Const r3=9 →2; 2 Read r0 from key7 →3
- 3 Const r2=11 →4; 4 Read r1 offset →5; 5 Add r1=r0+r1 →6
- 6 Write result r1 to key9 →7 **TRIAL**. A wrong trial output remains in the trace and must not be discarded.
- 7 Sub r1=r1-r0 →8; 8 Branch r1 !=0 ? 10 : 9 (local error signal)
- 9 Const r1=21 →11 (choose candidate B shortcut **only after correct trial**)
- 10 Const r1=20 →11 (choose baseline A if trial wrong)
- 11 Const r2=19 →12; 12 PatchNext node=r2,target=r1,next13 **internal candidate selection**
- 13 Const r2=7→14; 14 Read r0,input→15; 15 Const r3=9→16
- 16 Const r2=11→17; 17 Read r1,offset→18; 18 Add r1=r0+r1→19
- 19 Const r2=7→20 (selected path: mutated next21 for B, unchanged20 for A)
- 20 Const r2=7→22 (harmless extra instruction on A)
- 21 Write r1 to output9→25 (B)
- 22 Write r0 to output9→25 (A)
- 23 Halt; 24 Halt (unreachable sentinels)
- 25 Halt (natural finish).

Only one actor exists throughout. On offset0, B's trial equals input and the actor should patch node19 next20→21, performing the shorter correct final write at PC21. On offset1, B's trial differs, the actor retains node19 next20 and uses baseline output PC22; this correctness check is part of the human-designed seed program.

## Frozen matrix and readout

Independent deterministic input labels `[-13,0,7,23,101]` × public offsets `[0,1]` × arm `[normal, patch-suppressed]` = **20 complete worlds**. Offset is an **external environment input**, not internally generated novelty.

- All worlds: exactly one source actor id1, two CIL-origin cell9 writes (trial at PC6, final at PC21/22), no traps or births, natural Halt, three external events (load and two public data cells), total credits initial40 minus all attempts; no pool credits.
- Offset0 normal: writes `[(6,input),(21,input)]`, native edit `19:20→21`, last edge21; **21 attempts**.
- Offset0 suppressed: writes `[(6,input),(22,input)]`, one charged `NativeCodeEditSuppressed`, last edge20; **22 attempts**.
- Offset1 normal: writes `[(6,input+1),(22,input)]`, no effective edit, last edge20; **22 attempts**.
- Offset1 suppressed: same outputs as normal, one charged suppression, last edge20; **22 attempts**.

No hardware performance claim: any saving of one attempted instruction is in this exact engineered path and after extra trial overhead. A correct final answer is not an independently evolved agent objective. Negative, failed and suppressed conditions must stay in the denominator.

## Scientific boundary and next question

If accepted, the substrate permits **single-actor local trial, internal feedback, conditional self-edit and later continuity**. It is still a programmed control loop. Stronger next gate must test internally generated candidate *sets* and multiple round trips, **agent-owned checkpoints**, held-out tasks unknown to the founder, and resource/comparator ablations. An environment-dependent if-branch alone is not adaptation or life.
