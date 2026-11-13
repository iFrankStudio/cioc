# CL-1 Study 22 — verified causal compute-limitation experiment (2026-11-13)

**Technical verification PASSED. Scientific milestone: NOT ACCEPTED.** This is a new independent stochastic ecological cohort (not the scripted Study-21 positive-control fixture).

## Preregistration and exact provenance

- [Frozen Study-22 preregistration](../studies/22-preregistered-compute-limitation-ecology.md), commit `ce6f78bd8ac64cb5ccf2edcfaaef9f74658670b8`, before implementing and running the study.
- Source revision of the validated cohort: `7da289def9f5ed82b4c1e6af106bb4f94ee4fc76`.
- [Source-pinned CL-1 GitHub Actions success](https://github.com/iFrankStudio/cioc/actions/runs/43445242022) with complete Rust workspace tests/strict Clippy, 96 new worlds, 768 full matched SQLite epoch replays, independent read-only Python audit and archived raw SQLite/report. [CIOC benchmark success](https://github.com/iFrankStudio/cioc/actions/runs/43445242024).
- 96 **independent seed-worlds**, seeds **2,220,000..2,220,095**; 8 matched branches each, **768 runs**, 24 observation epochs per run, **18,432 validated hashed snapshots**. Branches are **not independent replicates**. All worlds, including trapped/inert ones, are in the denominator.
- Original generator, neutral externally authored prefix, 18 random CIL continuation nodes/process, 27 common prefork attempts, 56 starting credits/process and strict world resource cap were retained from preregistration. Investigator-supplied code and exogenous compute grants are **never claimed as endogenously designed**.

## Primary registered three-window outcomes

*Counts denote distinct independent seed-worlds out of 96, not copies/events or repeated branch counts.*

| Outcome | Control W2 | Control W3 | Exogenous-refill W2 | Exogenous-refill W3 |
| --- | ---: | ---: | ---: | ---: |
| ≥1 charged CIL attempt | 12/96 | **0/96** | 57/96 | **57/96** |
| ≥1 successful cross-owner GraphCopied event | — | **0/96** | — | **15/96** |
| ≥1 successful native CellRead | — | **0/96** | — | **23/96** |
| ≥1 successful native CellWritten | — | **0/96** | — | **0/96** |
| ≥1 window-local bidirectional copy pair | — | **0/96** | — | **0/96** |

Other verified contextual values: control W1 charged activity **96/96**; W1 successful cross-owner copy **68/96**; W1 reciprocal copy pair **2/96**; W1 successful native CellRead **23/96**, native CellWritten **41/96**, and both within W1 **4/96**. These are postfork *events*, not proof of graph-based functional dataflow.

The **paired W3 discordances**, using all 96 matched seed worlds, were:
- Recharge-only W3 charged activity: **57**; untreated-only: **0**.
- Recharge-only W3 successful cross-owner copy: **15**; untreated-only: **0**.

Preregistered Wilson 95% intervals: W3 exogenous-refill activity **57/96, [0.493739, 0.686547]**; W3 exogenous-refill cross-owner copy **15/96, [0.097037, 0.241915]**. These intervals describe seed-world proportions for this **specific** program generator and fixed ecology. No hypothesis-independent claim about arbitrary digital life is warranted.

## Causal interpretation and limits

External `SetCredits(process,47)` at epochs 8 and 16 **does not restart stopped/trapped programs**. It nevertheless permits late activity in 57 worlds that otherwise had none, and late inter-owner code copies in 15 worlds, supporting a distribution-specific **compute-exhaustion constraint** on late opportunities.

However:
1. There was **no successful native shared-cell write in W3** even under external compute recharge; successful reads (23 worlds) and successful graph copies (15 worlds) do not demonstrate a useful, source-dependent data transformation.
2. No reciprocal information-exchange pair was seen in W3 under recharge. This cannot be described as persistent multi-component maintenance.
3. `restore-g1-refill` explicitly injected the original graph as an **external structural control**; it is not internally rebuilt code and cannot be scored as native repair. The independent Study-21 **external functional** calibration applies only to that separately, hand-authored fixture.
4. No output-cell causal lineage to copied graph code was established in this prospective random cohort. **Source-sensitive functional recovery and organizational closure remain NOT EVALUABLE** rather than observed negatives on an inapplicable assay.
5. Nothing about these observations implies internal resource harvesting, autonomous reproduction, or open-ended evolution.

The sham was identical to control in simulated program/graph/cell/compute state, zero-credit never executed native CIL, all predeclared damage/injection schedules were independently checked, and strict resource bounds plus all SQLite snapshots passed deterministic replay.

## Next scientifically necessary step

The bottleneck has shifted: supplying compute enables late execution/copying but **not successful late writing or a validated source-conditional output function**. The next study should preregister an observer-only trace of the exact graph-code *read source*, and a prospective functional assay that differentiates externally prescribed probe code from internally originated program/data lineage. Any future claim requires two separately damaged, source-verified functional dependencies, an external positive control on that *same* registered function, and internally initiated restoration across two later windows.

**Stage decision: CL-1 OPEN, no cyberlife claim.**
