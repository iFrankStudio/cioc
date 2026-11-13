# CL-1 — Endogenous Organization Research

**Status: six preregistered feasibility studies completed; bounded negative/inconclusive findings; positive CL-1 milestone NOT accepted.** CL-0 is already accepted as laboratory infrastructure. CL-1 is a scientific program, not a new name for the VM.

[Verified outcomes and scientific boundary](RESULTS.md) · [Future positive acceptance gate](docs/next-experiment-contract.md) · [Green four-study CI](https://github.com/iFrankStudio/cioc/actions/runs/43411607915).

## Research registry
- [Study 01 preregistration](studies/01-preregistered-random-ecology.md) — 64 independent seeded worlds, five matched conditions per world; first negative-capable baseline.
- [Study 02 preregistration](studies/02-preregistered-reciprocal-causality.md) — 128 fresh seeds, six matched conditions, reciprocal state sensitivity audited with sham and zero-credit controls.
- [Study 03 preregistration](studies/03-preregistered-code-recovery.md) — 128 fresh seeds, targeted edge disruption, sham and explicitly external restoration controls.
- [Study 04 preregistration](studies/04-preregistered-open-compute.md) — fresh long-horizon worlds with external finite compute pools and replenishment controls.
- [Study 05 preregistration](studies/05-preregistered-exogenous-epochs.md) — independent environment epochs and full epoch-snapshot replay; [verified results](RESULTS.md#study-05--independent-external-epoch-environment-verified-2026-11-13).
- [Study 06 preregistration](studies/06-preregistered-internal-graph-ablation.md) — completed ablation of internally created graphs; [verified results](RESULTS.md#study-06--internally-constructed-graph-object-ablation-verified-2026-11-13).
- [Study 07 preregistration](studies/07-preregistered-cross-owner-information.md) — generic graph-to-graph code copying and explicit owner transfer under a new negative-capable cohort.
- [CL-0 → CL-1 handoff](../cl0/docs/CL-1-handoff.md) — scientific boundaries, controls and research acceptance rule.

**No life evidence has been established.** Across six distinct study cohorts (640 initial worlds and 3,264 matched runs), rare mutual state effects and external-compute-supported persistence were observed, but registered restoration and organizational closure were not. These four distinct generator families must not be pooled for statistical inference. See [results](RESULTS.md).

## Procedure
The preregistration is a separate prior commit. The runner and analyzer are implemented separately from preregistration commit `2b4e2ea`. CI must validate the complete run, replay and analysis before outcomes can be interpreted.\n\n```bash\ncd cl0\nCL1_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl1-study01 -- ../cl1-study01.sqlite\npython3 ../cl1/analyze.py ../cl1-study01.sqlite --json ../cl1-study01-report.json\n```
