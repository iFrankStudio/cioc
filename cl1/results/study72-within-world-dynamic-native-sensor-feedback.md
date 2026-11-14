# CL-1 Study72 — CI-verified prospective within-world dynamic native sensor tracking

**Status: independently verified prospective genuine native response to a runtime input change, not organism life, self-maintenance, repair or evolution.**

## Frozen population, source, full audit

- Prospective protocol `cl1/studies/72-preregistered-within-world-changing-sensor-feedback.md` was committed **before code and independent raw seed execution**. Exact original Rust native source **`e4dd0737ed107fd6489ba842083dc5106d952c06`**; the initial failed audit in CI43579049799 was caused by comparing unaligned absolute native event `tick` after an extra **External** `SetCell` at epoch12. The analyzer was corrected to subtract that exact **one registered external-event tick** for the dynamic ablated control, without changing protocol, source genome, native output, source evidence or registered primary endpoint. Entire native cohort was rerun unchanged under the corrected source revision.
- Independent [GitHub Actions43579435147](https://github.com/iFrankStudio/cioc/actions/runs/43579435147) **SUCCESS**; artifact `cl1-study72-changing-sensor-causal-feedback`, ID `14108025379`, retains original **1,195,896,832-byte SQLite** and independent `cl1-study72-report.json`. **256 independent full unique SplitMix64-mixed CIL genotypes**4,800,000..4,800,255, 12 original matched arms each, **3,072 native executions and 73,728 SHA256 independently audited exact replayed epoch snapshots**, all 24 observation epochs and unfiltered seeds.
- Program: researcher-provided three CIL register interface opcodes (r1=1, sensor-cell address7, output-cell address9), 24 random six-way CIL nodes with broad vs predecessor-only Branch backedge, terminal `Halt`; **no hand-written loop or cyclic tail**. The program and its 300 initial actor credits were externally initialized. No host actor-specific credits, code edits, resource acquisition or organism reward.
- Dynamic branch only: `SetCell(key7,value8)` at the **start of epoch12** after initializing same input3 as `static-low`. Matched `static-high` starts at input8. The `Read`-ablated branch replaces each random Read with `Const(r0,0)`, without changing any other sampled opcode/operand/branch draw. All source external actions and native events are independently traced.

## Frozen prospective primary (ambient-independent single-process source tracking)

| Exact PRE epochs8–11 + POST epochs12–15 native input tracking | Broad-backedge random CIL | Short-backedge random CIL |
|---|---:|---:|
| Qualified original seed genomes | **78/256** | **0/256** |
| Wilson95 | **[0.251536,0.363614]** | [0,0.014784] |
| Paired discordance | **broad-only78**, short-only0 | both0, neither178 |
| Exact two-sided paired sign test | **6.6174 ×10^-24** | descriptive, one prospectively fixed contrast |

All 78 require a **fresh successful native `CellRead(key7)`** in both PRE and POST; real source-tainted native `CellWritten(key9)` at the **same writer PC** in each window; dynamic PRE equals low3 and dynamic POST equals high8; matched `static-high−static-low` native values equal +5; actual POST native Read occurs after registered external epoch12 switch; all 12-arm Read-disabled controls contain **zero native Reads** and input-invariant native write trajectories after aligning the one extra external event tick. **No evaluability-based sample removal.**

### Source-pinned first actual witness, seed 4,800,000

- Broad native original writer **PC6**, PRE epoch8 native `CellWritten(key9,value3)` at tick99 in dynamic and static-low, value8 in static-high; PRE native Read(key7) in epoch8.
- POST epoch12 native `CellWritten(key9,value8)` at tick149 in dynamic, versus at tick148 static-low value3 and static-high value8. The one tick difference is exactly the extra **External** SetCell before the native POST event, not a native computation or causal achievement. POST `CellRead(key7)` is newly executed in epoch12; no intervening r0 taint-killing Const. Read-disabled programs cannot produce the source-dependent behavior.
- Verified original initial full-genotype uniqueness **256/256** and all original native event-PC/source and credit ledgers.

## Why this is scientifically meaningful, and what it does not show

Prior Studies68/69/71 only varied initial source state across matched worlds and detected separate late windows. Study72 now shows an unoptimized random CIL native program can respond **within one execution history to a changed runtime input**, with contemporaneous CIL sensing and matching output. No native life machinery, auto-repair goal, or artificial positive seed is inserted. **The generic broad random backedge makes causally useful recurrent feedback reachable**, and the original short-branch control demonstrates that W3 raw execution alone is insufficient (253/256 W3-active in each non-ablated topology despite only broad78 functional worlds).

It remains a **finitely endowed, externally sampled one-process random program**, not a self-assembled, resource self-replenishing, heritable or self-repairing organism. It has **300 researcher-endowed credits**, no native compute draw and no code mutation. The environmental source SetCell is **External**. No claim of natural digital life or autonomous organization is justified.

## Next scientific gate

The subsequent separately preregistered Study73 (already frozen **before** any new Study73 seed run) isolates **native resource acquisition and persistent source-sensitive function beyond 64 initial actor credits** using generic eight-credit DrawCompute and broad non-prewired Branch code, with depleted pool and draw-disabled negative controls, zero external process starter and exact source/pool ledgers. The actual resource supply is necessarily **environmental**; the question is whether *native CIL* can access and use it while retaining a function. Only then should lesion-induced native function repair be assessed.

**CL-1 remains OPEN.**
