# CL-1 Study 25 — preregistered prospective graph-content intervention in fresh random ecology

**Registered 2026-11-13 before Study-25 code, outcome or execution.** All outcomes from Studies 01–24 are already known; no code/probability changes will be made after result inspection. This is a *new prospective stochastic ecology* with **96 fresh independent seed-worlds numbered 2,250,000..2,250,095**. The historical Study-22 generator and 24 × 12 epoch/attempt schedule are used without retuning: identical researcher-authored 3-process nine-node neutral scaffold, 18 LCG-randomized continuation instructions/process, 56 initial credits/process, strict cells=32, max process nodes=40, total cap=192 and 27 charged common prefork attempts. Seed IDs are disjoint from Study-22 (2,220,000..) and Study-14 (2,140,000..). This experiment does NOT assume those preauthored code templates are endogenously invented.

## Falsifiable question

Does changing the executable **content** of a real cross-readable code-bearing graph in a randomly continued CIL world change a later registered **actor-owned shared-cell output** as predicted by a successful native graph-code read, rather than merely changing graph presence, number of copies or survival?

The two original prefork graphs are handles **1 and 2**. Their exact prefork objects must be saved and source-pinned before the intervention, including owner/reader ACL and executable nodes. Mutation is explicitly **external**: `DropGraph(graph)` then `PutGraph(graph, object with ONLY node 0 replaced by Op::Halt)`, at epoch 0. `Halt` has registered opcode tag **19**. Unmodified original graph identities are reinstated with `DropGraph` + exact `PutGraph` for the restoration-control arm. All manipulations are logged as `Origin::External`. No special host `REPAIR`, `FITNESS`, `REPRODUCE`, additional CIL capability, automatic code replacement, rerun of halted processes or hidden selection is allowed.

## Fixed matched conditions and resource controls

Exactly **9 arms × 96 seed-worlds = 864 paired runs** and **20,736 individually hashed epoch snapshots**:

1. `control`: no action.
2. `sham`: rejected `DropGraph(0)` at epoch 0; simulated state identical to control.
3. `mutate-g1`: replace prefork graph1 node0 with `Halt` at epoch0.
4. `mutate-g2`: replace prefork graph2 node0 with `Halt` at epoch0.
5. `mutate-both`: replace both graph1 and graph2 node0 with `Halt` at epoch0, graph1 first.
6. `exact-original-g1`: drop then externally reinject **unaltered** prefork graph1 at epoch0 (tests injection identity/effective content).
7. `refill`: untouched graph content, `SetCredits(process,47)` for processes 1–3 in order at **epochs 8 and 16**.
8. `mutate-g1-refill`: fixed external graph1 mutation at epoch0 plus exactly the `refill` credit schedule.
9. `zero-credit`: `SetCredits(process,0)` for processes 1–3 at epoch0, no subsequent instructions.

All arms fork from identical serialized prefork world per seed, never discard dead/trapped worlds and never change the previous random continuation generator. All native CIL work must debit credits, all PutGraph events are external, storage conserved relative to pre-intervention in equal-size graph replacements. A rejected structural intervention or missing owner/read ACL fails technical acceptance.

## Precommitted outcomes and definitions

For W1=epochs0..7, W2=8..15, W3=16..23, independently report per-arm distinct seed-world proportions and *event totals* of: charged activity, successful `GraphTagRead`/`GraphFieldRead` where `owner != actor`, successful `CellWritten`, same-actor read-then-write temporal opportunities, and successful cross-owner `GraphCopied`; separate exposure to targeted source graph IDs 1 and 2. All read and write events must come from `Origin::Internal(actor)`, with exact source/actor/PC, and successful instruction semantics.

The **primary narrow causal output contrast** requires paired same-seed `control` and `mutate-g1` OR `mutate-g2`:
(i) each has successful `GraphTagRead` of **that targeted graph by the same process**, with changed observed tag value; (ii) each subsequently has a successful `CellWritten` from **that same process to the same shared cell and in the same epoch**; (iii) the written numeric values **differ**. Exclude the initial common prefork history, event numbers are ordered by native tick, and output cannot be borrowed from another process or another epoch. This paired pattern is an **external graph-content sensitivity observation**, NOT proof of internal design autonomy, organizational closure or self-maintenance. If zero such paired exposures occur, the frequency of functional sensitivity among unexposed worlds is **NOT EVALUABLE**; report exposed denominator and failures separately rather than asserting function absent in all possible worlds.

Prespecified secondary contrasts: mutation induces loss of a write that was preceded by a targeted same-actor graph-code read in control; no added or removed storage/credits from same-size replacement; `mutate-both` cannot be counted as an independent replica of single-component effects; paired W3 exogenous refill activity vs untreated is descriptive only; `exact-original-g1` must be byte-equal in simulated final state to `control` or the intervention baseline fails.

## Audit / stopping / scientific boundaries

Exact every-epoch SQLite full-state/event replay, independent read-only audit of 864 runs and 20,736 hashed snapshots, exact intervention schedules, equal source SHA within each seed, strict resource budgets and successful sham/unaltered reinjection/zero-credit controls are mandatory. Preregister negative synthetic tests for misattributed readers, output-before-read, different actor, different cell or mismatched epoch. All paired-source comparisons use **96 independent seed worlds** as the unit; report Wilson 95% intervals, paired discordances, and raw exposure events without treating 864 branches as independent.

Even positive changes are induced by a *researcher-authored destructive code edit* of researcher-seeded executable machinery. There is no endogenous reconstruction, recovery after observed damage in two later windows, self-generated functional circuitry or autonomous acquisition of energy. **CL-1 positive scientific acceptance remains OPEN regardless of Study-25 outcome.**
