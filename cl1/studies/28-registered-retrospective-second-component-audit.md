# CL-1 Study 28 — registered retrospective second-component falsification

**Registered 2026-11-13 after Study-27 results are known.** Study-27 independently observed two graph2-tag/output-address causal witnesses (seed IDs 2300090, 2300118) in 192 worlds. Those cases are **known** and may not be relabeled as new positive discoveries. This is a **post-result secondary-source analysis** on the complete, unchanged Study-27 generator and seed cohort, with **zero new independent seed-worlds**. The source graph1 negative intervention and double-graph intervention arms were precommitted in Study27; the detailed comparisons here are registered before this secondary analysis.

## Falsifiable second-component question

Does graph1 contribute a second **causally necessary** component of the *same* source-dependent output-address function witnessed in Study27? Or does the behavior remain wholly attributable to externally seeded graph2 read access, random process1 instructions and the exogenous graph2 tag edit?

## Freeze and comparison

Rerun the **unmodified** Study-27 runner for all 192 independent seed IDs `2,300,000..2,300,191`, seven matched branches, eight 12-attempt epochs, 1,344 runs and 10,752 hashed epoch snapshots. Verify the original Study-27 independent audit and exact Rust epoch replays first, so historical observed successes remain exactly **2**. Do not add new native opcodes, credits, interventions, mutations or change source-code generators.

For every 192 seed-worlds, the independent read-only audit uses **epoch0**, native process1, the following unchanged registered target semantics:

- `control` and `mutate-g1`: successful `GraphTagRead(graph=2, owner=2, node=0, tag=11)` then exactly one successful `CellWritten(key=11,value=v)` later in the epoch.
- `mutate-g2` and `mutate-both`: successful `GraphTagRead(graph=2, owner=2, node=0, tag=19)` then exactly one successful `CellWritten(key=19,value=v)` later in the epoch.
- All comparisons require source tag, actor, shared-cell address, numeric value, epoch and event ticks; the same-seed graph1-only and double-mutant behaviors must be matched to the original control/graph2-mutant case, not to data from unrelated worlds. For each graph2 causal exposure, classify `graph1_not_necessary_observed` **only when** both graph1-mutant output and double-mutant output remain successful and carry the same numeric value as the corresponding control/graph2-mutant output. Classify `graph1_interference_observed` when either becomes absent or changes value. Otherwise `indeterminate`.
- Independently count native process1 successful `GraphTagRead`/`GraphFieldRead` **of graph1, owner1** in each arm and whether one occurs before its target output in epoch0. **Reading graph1 alone is NOT proof of its contribution** to the graph2-conditioned output.
- The strict second-component functional endpoint requires *graph1-specific functional loss or alteration in either comparison*, **and** observed process1 successful graph1 content read before the changed output (with graph2's source tag matched across that pair). Without graph1 source exposure, any output perturbation is not a demonstrated *read-mediated* two-component function; classify as `NOT EVALUABLE` rather than claiming a second component.
- All-world denominators remain 192; report separate **known graph2-exposed subset n=2** as retrospective, never estimate a new broad emergence rate from these two selected winners.

## Controls and integrity

Use the original Study-27 untouched/rejected-sham, exact-original external graph2 reinjection and zero-credit; the independent Study-27 baseline auditor must pass before any classification. Use synthetic logic tests with graph1 necessary/not-necessary and wrong-source negative events; source ownership, actor, tick, original output, mutation order, SQLite hashes and scheduled external provenance are independently verified. Rust must replay the identical 1,344 ×8 full worlds, with no changed CIL programs or state. Failure of controls invalidates the technical audit.

## Scientific boundary

A graph1 effect, even if observed, is not a free-standing native repair pathway: externally edited graph node0 and originally authorized graph read access are experimenter-controlled. **Neither common program setup nor code copying implies internally developed organization**. No self-initiated response to damage or two late functional recovery windows has been shown. This retrospective audit cannot alone close CL-1 and must never silently change the Study-27 primary endpoint.
