# CL-1 — Endogenous Organization Research

**Status: nine preregistered feasibility studies completed; bounded negative/inconclusive findings; positive CL-1 milestone NOT accepted.** CL-0 is already accepted as laboratory infrastructure. CL-1 is a scientific program, not a new name for the VM.

[Verified outcomes and scientific boundary](RESULTS.md) · [Future positive acceptance gate](docs/next-experiment-contract.md) · [Verified nine-study CI](https://github.com/iFrankStudio/cioc/actions/runs/43419453968).

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
- [Study 10 design proposal](docs/study-10-neutral-opportunity-design.md) — method for an honest, non-teleological reachable substrate, **not** yet registered or executed.
- [CL-0 → CL-1 handoff](../cl0/docs/CL-1-handoff.md) — scientific boundaries, controls and research acceptance rule.

**No life evidence has been established.** Across nine distinct study cohorts (1,024 initial worlds and 5,184 matched runs), rare mutual state effects and external-compute-supported persistence were observed, but registered restoration and organizational closure were not. These distinct seeded generator families must not be pooled for statistical inference. See [results](RESULTS.md).

## Procedure
The preregistration is a separate prior commit. The runner and analyzer are implemented separately from preregistration commit `2b4e2ea`. CI must validate the complete run, replay and analysis before outcomes can be interpreted.

```bash
cd cl0
CL1_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl1-study01 -- ../cl1-study01.sqlite
python3 ../cl1/analyze.py ../cl1-study01.sqlite --json ../cl1-study01-report.json
```
