# CL-1 — Endogenous Organization Research

**Status: research initiated; not accepted.** CL-0 is already accepted as laboratory infrastructure. CL-1 is a scientific program, not a new name for the VM.

## Research registry
- [Study 01 preregistration](studies/01-preregistered-random-ecology.md) — 64 independent seeded worlds, five matched conditions per world; first negative-capable baseline.
- [CL-0 → CL-1 handoff](../cl0/docs/CL-1-handoff.md) — scientific boundaries, controls and research acceptance rule.

**No life evidence has yet been established.** Study 01 can measure finite CIL behavior and state sensitivity but cannot alone verify organizational closure or self-maintenance.

## Procedure
The preregistration is a separate prior commit. The runner and analyzer are implemented separately from preregistration commit `2b4e2ea`. CI must validate the complete run, replay and analysis before outcomes can be interpreted.\n\n```bash\ncd cl0\nCL1_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl1-study01 -- ../cl1-study01.sqlite\npython3 ../cl1/analyze.py ../cl1-study01.sqlite --json ../cl1-study01-report.json\n```
