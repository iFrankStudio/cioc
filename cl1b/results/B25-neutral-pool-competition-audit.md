# CL-1B B25 — neutral compute-pool filtering and deterministic scheduler confound

**Evidence:** [GitHub Actions run 43781054450](https://github.com/iFrankStudio/cioc/actions/runs/43781054450), tested SHA `af10e97c7e0c07422d5c357febc2dc57954591a1`, **full** campaign: all **26/26 native Rust CL-1B tests** plus **12/12 independent source-parsed analyses PASS**, separate evidence-publication job PASS. [Source-pinned raw summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/af10e97c7e0c07422d5c357febc2dc57954591a1-43781054450-1/summary.txt). [Prospectively frozen protocol](../studies/B25-neutral-compute-competition.md) · [Rust native test](../../cl0/crates/world/tests/cl1b_b25_neutral_compute_competition.rs) · [independent deterministic scheduler scan](../analysis/b25_neutral_competition_scan.mjs).

## Two fixed policies, equal code/endowment and no actor-targeted host replenishment

The investigator loads two **distinct, fully authored** five-node CIL processes, each with **two local credits** and its own pending Draw: one asks3, one asks4. Roles are swapped between process IDs1/2. They compete for a single world-pool resource fed by six equal or alternating **external identity-neutral** packets. Each real service is counted from the `Origin::Internal(pid)` `CellWritten(key9,value1)` event **at PC4**, never inferred from the shared cell's last value. Exact native compute conservation and no hidden process birth/host winner awards are asserted.

Four packet schedules × both ID assignments + solo controls for each request = **16 deterministic worlds**. Fixed supply sequences are not a random environmental population, and the solo comparisons have only one initial process/endowment rather than two.

## Native B25 output: request3 / request4 real service counts

| Shared environment | Contested PID1=request3, PID2=request4 | Contested PID1=request4, PID2=request3 | Solo request3 | Solo request4 |
| --- | --- | --- | ---: | ---: |
| Six packets of4 | **6 / 0** | **6 / 0** | 6 | 4 |
| Six packets of5 | **3 / 3** | **3 / 3** | 7 | 6 |
| `[4,5,4,5,4,5]` | **3 / 3** | **3 / 3** | 6 | 6 |
| `[5,4,5,4,5,4]` | **3 / 3** | **3 / 3** | 6 | 5 |

**Why six packets of4 is genuine resource filtering:** with zero local credit, `Draw3` needs4 units (requested3 plus one pool-paid instruction charge) while `Draw4` needs5. A contested request3 actor spends all4 each period, so request4 never has enough pooled compute, regardless of which process ID holds request3. In the solo request4 world, **unused packets accumulate**, allowing four real service writes by the final packet. Thus resource availability per event and *competition* interact; a solo outcome is not the counterfactual contested opportunity.

**Why six packets of5 is a scheduler confound:** both requests can execute when the shared pool has5; the deterministic scheduler alternates the first affordable candidate among packet windows, yielding 3/3 overall. Exchanging which request is process1 reverses *which numbered process* gets early access while preserving the strategy-level totals. This cannot be misreported as an intrinsic fitness edge or spontaneous fairness preference.

The mixed schedules also yield 3/3 contested service totals, and each process alternately seizes the shared pool with a deterministic ordering. These observations are **not biological natural selection**: no reproduction, inherited mutation or autonomous policy decision occurs in B25.

## Implication for B26 and later experiments

B25 establishes a **necessary ecological affordance** in the native CL-0 substrate: ordinary CIL resource-acquisition policy differences can be filtered by a **shared, limited neutral compute pool**, while deterministic scheduling and accumulated unused energy can substantially alter apparent winners. It is not enough to hand-load two arbitrary actors and call their output difference evolution.

The next requirement is **native executable lineage** (parent makes its own child and changes copied code) and eventually **multiple rounds of paid reproduction/mutation, environmental competition, retained variants and independent evaluation of causal origin**. B26 explicitly targets first-generation code lineage, but its mutation delta and generation recipe are still investigator-written. A positive B26 would be a mechanistic precursor, **not** a passing CL-1B B3/B4 or digital-life gate.
