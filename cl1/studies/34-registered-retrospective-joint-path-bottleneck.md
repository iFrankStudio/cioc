# CL-1 Study 34 — registered retrospective joint-path bottleneck audit

**Registration status:** This protocol is written after Study33's strict 0/128 primary outcomes were known. It is a **retrospective, descriptive diagnostic**, not prospective confirmation. It reuses exactly the Study33 128 worlds (2,600,000..2,600,127), nine paired arms and existing source-pinned execution. **Zero new independent ecological worlds.** No new native opcode, seed selection, generator adjustment or endpoint substitution is permitted.

## Question

Study33 verified process1 native compute draws in 52/128 untreated worlds, but zero source2-register-qualified writes in the control arm. Determine **where the native read → register lineage → output path fails**, conditional on compute uptake, rather than interpreting the joint null as an absence of all component activity. This audit must not silently turn a post-result explanation into a new positive life claim.

## Fixed population and analysis

Use all 128 Study33 seeds and their existing original-control, mutate-g2, empty-pool, mutant-empty-pool, refill, mutant-refill, sham, exact-original-g2 and zero-credit arms. Reconstruct the original 18-instruction continuation from the Study33 frozen LCG and verify it against persisted source revision and original process code. Use actual success-only internal events with actor, PC, instruction, register provenance and native attempt ordinal. Never count a trap, mere opcode presence, host action or a successful event from a different actor as the same pipeline.

For each seed and each arm, record the **first successful native process1 DrawCompute** and the following mutually nested, time-ordered exposure gates:

1. Native process1 successfully executes GraphReadTag(graph2) in any epoch.
2. Native process1 executes such a read **after** its first successful DrawCompute.
3. Its actual destination register carries source2 provenance after the read, until overwritten or propagated by a successful native operation.
4. A later successful native process1 Write consumes that provenance in its **address** register (irrespective of address value).
5. That Write targets cell11 in the original branch or cell19 in the graph2-mutant branch, respectively.
6. Both matched branches satisfy their corresponding gates with the same epoch and preserved numeric output.
7. The matched qualifying Write occurs after process1 exceeds its original 47 postprefork native-attempt budget, without any external SetCredits.

Gates 1 and 2 must also be reported separately for the **reverse order** (read before Draw); the nested chain starts at gate 2. Distinguish successful read of the graph from a source2-labelled register that is actually consumed by Write. A source label is invalidated on overwrite, actor change, failed operation or uncertain def-use. For every failure, report the earliest failed gate, but do not infer why the generator failed from a mere correlation. Report all 128 seeds as the denominator and conditional gate denominators only when nonzero; use “NOT EVALUABLE” for empty conditional denominators. Report matched-arm discordance as paired worlds, not independent samples.

## Controls and verification

Reproduce the Study33 exact source-pinned SQLite snapshots and hashes before diagnostic analysis. Independently validate source graph ID, owner, node tag, ACL, actual instruction at PC, register overwrite and chronological event order. Confirm the sham and exact-original-g2 arms match control state, and zero-credit has no native successful execution. Reconcile all Draw/Return events with the recorded compute pool and process credits. Add only one targeted synthetic positive and a compact set of negative cases for wrong actor, read-before/after-order inversion, overwritten address register and failed Draw; avoid a combinatorial testing expansion.

The report must include the full fixed-seed table, gate counts, counterexamples, reproducibility commands, source revision and an explicit declaration that this analysis adds **0 independent worlds**. Do not infer causal benefit from resource draw without a prospective intervention with actual exposure. Do not revise Study33 preregistered endpoints.

## Decision rule for future work

If the earliest bottleneck is missing successful graph2 reads, the next *separately preregistered new-seed* study may investigate reachability under a frozen, non-goal-directed ordinary CIL grammar. If reads occur but provenance does not survive to Write, investigate register flow and scheduling without constructing a guaranteed working organism. If a complete source-sensitive function is observed, the next prospective test must still separately demonstrate endogenous origin, damage-induced loss, **internally initiated** restoration and function in two separated later windows, with matched controls and no host-credit subsidy. All future proposals must explicitly distinguish engineered opportunity from spontaneous organization.

**Scientific verdict cannot change on this retrospective audit alone: CL-1 remains OPEN.**
