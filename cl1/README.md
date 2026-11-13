# CL-1 — Endogenous Organization Research

**Status: eleven preregistered feasibility studies completed; bounded negative/inconclusive findings; positive CL-1 milestone NOT accepted.** CL-0 is already accepted as laboratory infrastructure. CL-1 is a scientific program, not a new name for the VM.

[Verified outcomes and scientific boundary](RESULTS.md) · [Permanent Study-10 summary](results/study10-report.json) · [Permanent Study-11 summary](results/study11-report.json) · [Future positive acceptance gate](docs/next-experiment-contract.md) · [Verified eleven-study CI](https://github.com/iFrankStudio/cioc/actions/runs/43422732727).

## Research registry
- [Study 01 preregistration](studies/01-preregistered-random-ecology.md) — 64 independent seeded worlds, five matched conditions per world; first negative-capable baseline.
- [Study 02 preregistration](studies/02-preregistered-reciprocal-causality.md) — 128 fresh seeds, six matched conditions, reciprocal state sensitivity audited with sham and zero-credit controls.
- [Study 03 preregistration](studies/03-preregistered-code-recovery.md) — 128 fresh seeds, targeted edge disruption, sham and explicitly external restoration controls.
- [Study 04 preregistration](studies/04-preregistered-open-compute.md) — fresh long-horizon worlds with external finite compute pools and replenishment controls.
- [Study 05 preregistration](studies/05-preregistered-exogenous-epochs.md) — independent environment epochs and full epoch-snapshot replay; [verified results](RESULTS.md#study-05--independent-external-epoch-environment-verified-2026-11-13).
- [Study 06 preregistration](studies/06-preregistered-internal-graph-ablation.md) — completed ablation of internally created graphs; [verified results](RESULTS.md#study-06--internally-constructed-graph-object-ablation-verified-2026-11-13).
- [Study 07 preregistration](studies/07-preregistered-cross-owner-information.md) — completed cross-graph code copying and ownership transfer with matched negative controls; successful cross-owner action **not evaluable** at the original event granularity.
- [Study 08 preregistration](studies/08-preregistered-instruction-provenance.md) — completed source-attributed successful graph-transition audit on 128 fresh seed-worlds; no successful cross-owner exchange observed in this cohort.
- [Study 09 preregistration](studies/09-preregistered-permission-ecology.md) — generic permissions became available, but zero/128 pretreatment worlds contained an eligible nonempty object; results not causally exposed.
- [Study 10 preregistration](studies/10-preregistered-neutral-opportunity.md) — **completed** neutral reachability calibration: 64/64 exposed graph worlds; all successful cross-owner events externally scaffolded, [results](RESULTS.md#study-10--reliable-neutral-graph-interactions-are-reachable-but-externally-scripted-verified-2026-11-13).
- [Study 11 preregistration](studies/11-preregistered-postscaffold-ecology.md) — **completed** new-seed random postscaffold graph interactions, real inter-owner causal exposure, but no maintenance or closure evidence; [results](RESULTS.md#study-11--random-cil-continuation-produces-cross-owner-exchange-without-prewired-exchange-instructions-verified-2026-11-13).
- [Original Study 10 design proposal](docs/study-10-neutral-opportunity-design.md) — preserved as a pre-registration methodological plan, not itself the experimental protocol.
- [CL-0 → CL-1 handoff](../cl0/docs/CL-1-handoff.md) — scientific boundaries, controls and research acceptance rule.

**No life evidence has been established.** Across eleven distinct study cohorts (1,184 initial worlds and 5,984 matched runs), rare mutual state effects and external-compute-supported persistence were observed, but registered restoration and organizational closure were not. These distinct seeded generator families must not be pooled for statistical inference. See [results](RESULTS.md).

## Procedure
Each study protocol was preregistered in a prior separate commit. The Study-10 protocol is commit `02092cdf`; its validated runner revision is `9bf20b08`. CI validates complete experiments and read-only analysis. Use fresh SQLite filenames rather than appending experiments to an existing database.

```bash
cd cl0
CL1_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl1-study01 -- ../cl1-study01.sqlite
python3 ../cl1/analyze.py ../cl1-study01.sqlite --json ../cl1-study01-report.json
```

## Reproduce Study 10

```bash
cd cl0
CL1_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl1-study10 -- ../cl1-study10.sqlite
python3 ../cl1/analyze_study10.py ../cl1-study10.sqlite --json ../cl1-study10-report.json
```

To reproduce **exactly the archived** experiment revision, check out `9bf20b088036479d28dcc9442c23097d478234e3` before running. The programs' scaffolded transfers/copies are externally specified initial code, never evidence of emergent organization.

## Reproduce Study 11

```bash
cd cl0
CL1_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl1-study11 -- ../cl1-study11.sqlite
python3 ../cl1/analyze_study11.py ../cl1-study11.sqlite --json ../cl1-study11-report.json
```

Use a fresh SQLite file for each reproduction. The exact archived source revision for the cited Study-11 outcomes is `833b6a609ebbf5a7d5d354588bb4d0770a3c595a`.
