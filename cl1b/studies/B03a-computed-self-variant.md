# CL-1B / B03a — internally calculated executable variant, no selection claim

**Prospective micro-gate, registered 2026-11-15 before B03a test implementation/outcome inspection.** B03a is an engineered precursor to the independent B3 research gate: construction is not adaptation.

## Hypothesis

Within a single CIL process, an edited instruction next-edge can be **calculated from the executable's current own edge plus a publicly readable environmental delta**, rather than injected as an externally specified absolute next-edge target. The change must affect instruction cost and maintain two correct service outputs, without spawning another process. External input values are not "endogenous randomness"; the calculation algorithm itself remains researcher-authored.

## Frozen source graph

Exactly 13 nodes in one process, register semantics from accepted CL-0:

- 0 Const r0=9 →1 (self-code location to inspect)
- 1 ReadNext r1,node=r0 →2 (read PC9.next, initially 10)
- 2 Const r2=11 →3 (key of environment delta)
- 3 Read r2,address=r2 →4 (environment delta, either 0 or 1)
- 4 Add r1=r1+r2 →5 (construct candidate next edge as 10+delta)
- 5 PatchNext node=r0,target=r1 →6 (apply own code edit)
- 6 Const r2=7 →7 (input cell key)
- 7 Const r3=9 →8 (output cell key)
- 8 Read r1,address=r2 →9
- 9 Write r3,r1 →10 (candidate edge changes this next)
- 10 Const r0=0 →11 (no-op detour)
- 11 Write r3,r1 →12
- 12 Halt

**Explicit author correction before implementation:** the mutation target is **PC9 Write.next** rather than a Read. For delta=0 next remains10; for delta=1 next becomes11. The CIL code computes this target by reading the original code and adding the environmental delta. A positive delta skips one unnecessary instruction but does **not** constitute autonomous optimization: it was engineered in advance.

World: strict capacity64; one process id1, initially32 credits, no pool or process births. External initialization: shared key7 has service input; key11 has declared delta. Input cases `[-13,0,7,23,101]`, delta cases `[0,1]`, full factorial 10 deterministic worlds, no selection. Native self-edit trace enabled.

## Frozen predicted endpoints

- Both branches write the input correctly **twice**, at PCs9 and11, and reach natural Halt at PC12 with no trap or process launch.
- delta0: PC9.next remains10, no real edge-patch event, attempted `PatchNext` still charged; 13 attempted instructions, remaining credits19.
- delta1: PC9.next becomes11, exactly one `ExecutableEdgePatched { node:9,before:10,after:11 }`, 12 attempted instructions, remaining credits20.
- Exactly three external provenance events per world (load and two cell settings). Every other event is from process1; all instruction attempts charged; no pool credits supplied; final inventory equals initial32 minus attempts.
- No preselected successful sample filtering; no host fitness/reward. Delta is researcher-supplied public information and must be declared as such.

## Negative interpretation

This experiment is a proof of **native runtime genotype reading + computed code change with measurable cost effect**. It does not prove native evaluation, retention by selection, generalized adaptation, continual organizational identity, or spontaneous emergence.

## Next gate B03b (not yet preregistered)

Require at least two candidate variants evaluated via agent-owned native execution and charged local resource usage, a selection outcome *causally dependent on internally observable trial data*, counterfactual controls with decision disabled, and out-of-sample inputs or fault conditions. No outside Python/Rust-side candidate selection may count as the individual's adaptation.
