# CL-0 Stage 5 — Pre-registered exploratory baseline (not emergent-life evidence)

## Goal
A reproducible **negative-capable baseline**, not an artificial organism or programmed evolutionary fitness competition. This experiment challenges the world's generic deterministic VM with researcher-generated arbitrary CIL and compares matched causal interventions. An apparently busy, persistent, or diverse trace has no life classification.

## Conditions (frozen before running)
The experimenter uses a documented 64-bit LCG to generate a 12-node CIL graph for each independent seed, with a fixed `Const` prefix, random choice among `Add`, `Sub`, `Write`, `Read`, `Branch` and a terminating `Halt`. No hidden survival/repair/reproduction controller is embedded. The graph itself and the initial credits are explicitly researcher supplied; random code diversity is therefore **not** evidence of internally generated novelty.

Every seed forks one byte-identical initial world into:
1. `control`: no intervention, 48 attempts maximum.
2. `zero-credit`: externally set process credits to zero before execution.
3. `disable-after-8`: externally disable the process after eight successful attempts, if the world reached that point.

The same seed and revision produce the same instructions and event traces. Changes in shared cells, computed opcode diversity or instruction count are descriptive engineering outcomes. Contrasts are observational unless the exact matched intervention and causal estimand are specified.

## Execute and independently analyze
```bash
cd cl0
CL0_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run -p cl0-lab --bin cl0-explore -- experiment.sqlite 24 54129
python3 analysis/analyze.py experiment.sqlite --json report.json --csv report.csv
cd console && npm install && npm run dev
```
Open the generated `report.json` in the browser-only observer.

## Reproducibility tests
- Identical seeds generate structurally identical initial worlds and traces.
- Adjacent seeds generate distinct input programs (researcher variation only).
- Every stored run must pass exact SQLite replay verification.
- Zero-credit controls generate no internal CIL instruction attempts.
- No run should exceed the specified storage or compute budgets.
- A timed action scheduled beyond inert termination is explicitly unperformed.

## Prospective CL-1 gate
Actual self-organization research must register *before observing outcomes*: an independently defined organizational closure statistic, sustained maintenance without external restoration, multi-generation novelty accumulation across independently seeded worlds, and interventions on hypothesized dependencies. Current CL-0 does **not** assert any of those observations.
