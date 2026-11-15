# CL-1B / Study B00 — native self-edit feasibility (engineering preregistration)

**Registered before new outcome inspection: 2026-11-15.** Route B0 only. No spontaneous adaptation, recovery, or life hypothesis is tested here.

## Hypothesis

Within one process and without `GraphLaunch`, a native CIL `PatchNext` can alter the currently executing process's own code and cause a measurable later change in instructions executed. With the **same initial code** and the world’s opt-in `with_native_patchnext_disabled` law, the same attempted edit is charged but does not modify that code; the later outcome differs. This tests causal executable self-modification, *not* the emergence of a self-maintaining mechanism.

## Frozen miniature program

Instruction nodes:

- 0: `Const r0=3 → 1` (the target instruction index)
- 1: `Const r1=4 → 2` (the replacement next-edge)
- 2: `PatchNext(node=r0,target=r1,next=3)`
- 3: `Const r2=99 → 5` (will have next 4 only if edit succeeds)
- 4: `Const r2=42 → 5` (newly reached only when the edit succeeds)
- 5: `Halt`

One externally loaded founder, 12 initial credits, strict storage cap, no additional loaded process, no replenished compute. Two matched conditions: full normal VM and patch-disabled VM, otherwise identical. Run to natural terminal, max 12 CIL attempts.

## Predeclared endpoints

- Normal: exactly one internally sourced `ExecutableEdgePatched` event (3: 5→4), original process 1 still present, code node 3 next edge 4, register r2=42, no `GraphLaunched`.
- Edit-disabled: edit attempt is charged and suppressed, process 1 code node3 next edge5, r2=99, no `GraphLaunched`.
- Both: deterministic terminal, zero traps, **same external founder**, no resource minting, attempts match expected path 6 vs 5, all events originated from intended process except external load.
- A successful outcome establishes only a usable CL-1B substrate. A failed outcome is to be recorded and analyzed without changing the predicted outcome post hoc.

## Follow-up independent science

B01 must probe sustained function after **multiple internal** structural changes, sham changes and destructive edits, with preregistered function measurements independent of the human seed’s chosen code. B02 must distinguish paid native restore from externally restored host snapshots. B03 must distinguish within-individual adaptive gains from externally tuned self-optimization scripts.
