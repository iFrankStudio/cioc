# CL-0 Stage 5 — Pre-registered exploratory baseline (not emergent-life evidence)

## Goal
A reproducible **negative-capable baseline**, not an artificial organism or programmed evolutionary fitness competition. This experiment challenges the world's generic deterministic VM with researcher-generated arbitrary CIL and compares matched causal interventions. An apparently busy, persistent, or diverse trace has no life classification.

## Conditions (frozen before running)
The experimenter uses a documented 64-bit LCG to generate three independent 12-node CIL graphs in one world for each independent seed, with a fixed `Const` prefix, random choice among `Add`, `Sub`, `Write`, `Read`, `DrawCompute`, `Branch` and a terminating `Halt`. No hidden survival/repair/reproduction controller is embedded. All three graph structures and their initial eight credits per process are explicitly researcher supplied; random code diversity is therefore **not** evidence of internally generated novelty.

Each world has **three processes sharing the same world cells and scheduler**, each with eight initial compute credits. Processes have no predefined roles or cooperation policy. Each world uses `World::with_strict_capacity(24, 32, 64)`, which prices program nodes, shared data, graph headers, process headers and read grant metadata in abstract storage units. Initial process credits are finite; compute credits can be drawn from an explicitly externally supplied world pool through generic CIL operations.

Every seed forks one byte-identical initial world into:
1. `control`: no intervention, 48 attempts maximum.
2. `zero-credit`: externally set **all three** process credits to zero before execution.
3. `disable-after-8`: externally disable **process 1 only** after eight world-level instruction attempts, if the world reached that point.
4. `environment-compute`: externally supply 64 credits to the world compute pool at offset zero. The program may draw them **only if its code actually executes a draw instruction**. This is externally sourced energy, not spontaneous renewal.

The same seed and revision produce the same **three-process world**, schedule and event traces. Differences between processes may arise from ordinary shared-data competition and round-robin interleaving, not from host-specified social roles. Changes in shared cells, computed opcode diversity or instruction count are descriptive engineering outcomes. Contrasts are observational unless the exact matched intervention and causal estimand are specified.

## Execute and independently analyze
```bash
cd cl0
CL0_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl0-explore -- experiment.sqlite 24 54129
python3 analysis/analyze.py experiment.sqlite --json report.json --csv report.csv
cd console && npm ci && npm run dev
```
Open the generated `report.json` in the browser-only observer.

## Reproducibility tests
- Identical seeds generate structurally identical initial worlds and traces.
- Adjacent seeds generate distinct input programs (researcher variation only).
- Every stored run must pass exact SQLite replay verification, including persisted pre-intervention fork snapshots.
- Zero-credit controls generate no internal CIL instruction attempts across any of the three processes.
- Each baseline starts with exactly 39 strictly billed storage units (3 × [12 code nodes + 1 process header]).
- 24 seeds × 4 conditions = 96 reproducible records. A resource-supply treatment is explicitly `Origin::External`; activity differences cannot be counted as spontaneous energy acquisition.
- No run should exceed the specified storage or compute budgets.
- A timed action scheduled beyond inert termination is explicitly unperformed.

## Prospective CL-1 gate
Actual self-organization research must register *before observing outcomes*: an independently defined organizational closure statistic, sustained maintenance without external restoration, multi-generation novelty accumulation across independently seeded worlds, and interventions on hypothesized dependencies. Current CL-0 does **not** assert any of those observations.
