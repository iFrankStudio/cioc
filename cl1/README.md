# CL-1 — Endogenous Organization Research

**Status: fourteen preregistered ecological feasibility studies executed; Study 14 negative checkpoint verified; Studies 15–17 are observer-only provenance calibration and lineage tooling, not evidence of life. Positive CL-1 milestone NOT accepted.** CL-0 is already accepted as laboratory infrastructure. CL-1 is a scientific program, not a new name for the VM.

[Verified outcomes and scientific boundary](RESULTS.md) · [Permanent Study-10 summary](results/study10-report.json) · [Permanent Study-11 summary](results/study11-report.json) · [Permanent Study-12 summary](results/study12-report.json) · [Permanent Study-13 summary](results/study13-report.json) · [Future positive acceptance gate](docs/next-experiment-contract.md) · [Verified thirteen-study CI](https://github.com/iFrankStudio/cioc/actions/runs/43427188730).

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
- [Study 12 preregistration](studies/12-preregistered-graph-lineage.md) — **completed** graph allocation, existing-process-code copying and edge-edit provenance; internally executed, not intrinsically invented, with [verified results](RESULTS.md#study-12--traceable-origins-of-internally-executed-graph-construction-and-code-edits-verified-2026-11-13).
- [Original Study 10 design proposal](docs/study-10-neutral-opportunity-design.md) — preserved as a pre-registration methodological plan, not itself the experimental protocol.
- [Study 13 preregistration](studies/13-preregistered-reciprocal-information-flow.md) — completed 96 new worlds × six matched arms, two nonoverlapping information-flow observation windows and targeted double ablations; [results](RESULTS.md#study-13--paired-graph-ablations-and-two-window-reciprocal-information-flow-verified-2026-11-13).
- [Study 14 preregistration](studies/14-preregistered-functional-restoration-gate.md) — verified 96-world 24-epoch study; [negative checkpoint](results/study14-checkpoint.md). **Source-sensitive functional output and true graph-restoration positive control remain NOT EVALUABLE**.
- [Study 15 preregistration](studies/15-preregistered-cell-io-provenance.md) — observer-only internal CellRead / CellWritten events and targeted success/trap tests; technical calibration, not a cyberlife finding.
- [Study 16 preregistration](studies/16-preregistered-descriptor-code-ancestry.md) — success-only GraphEncoded / GraphReplaced descriptor lineage, with [read-only last-writer audit](analyze_descriptor_ancestry.py); external/unknown descriptor sources must not become internal novelty claims.
- [Study 17 preregistration](studies/17-preregistered-graph-launch-ancestry.md) — resource-conserving GraphLaunched graph-to-process lineage, with no reproduction claim.
- [Study 18 preregistration](studies/18-preregistered-functional-lineage-audit.md) — observer-only [strict execution ancestry auditor](analyze_study18.py) with positive synthetic and adversarial controls. It is exercised retrospectively against replayed Study-14 SQLite; **functional causality and autonomous maintenance remain NOT EVALUABLE**. No extra ecological cohorts or life evidence are claimed.
- [CL-0 → CL-1 handoff](../cl0/docs/CL-1-handoff.md) — scientific boundaries, controls and research acceptance rule.

**No life evidence has been established.** Across fourteen distinct study cohorts (1,472 initial worlds and 7,616 matched runs), rare mutual state effects and external-compute-supported persistence were observed, but registered restoration and organizational closure were not. These distinct seeded generator families must not be pooled for statistical inference. See [results](RESULTS.md).

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

## Reproduce Study 12

```bash
cd cl0
CL1_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl1-study12 -- ../cl1-study12.sqlite
python3 ../cl1/analyze_study12.py ../cl1-study12.sqlite --json ../cl1-study12-report.json
```

For the archived result use source revision `f64f754be50fb1f5f0cfc993a6629836ce3bba7f`, not the later documentation commit. Create a fresh SQLite database, rather than appending to an existing cohort.

## Reproduce Study 13

```bash
cd cl0
CL1_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl1-study13 -- ../cl1-study13.sqlite
python3 ../cl1/analyze_study13.py ../cl1-study13.sqlite --json ../cl1-study13-report.json
```

Use a **fresh** SQLite path. To match the archived exact-replay report, check out source `bc670fafd26ea7c93b018f92c6bda62749036882`. The laboratory validates the full run and each epoch; the independent Python audit checks the recorded fixed cohorts and outcome semantics.
