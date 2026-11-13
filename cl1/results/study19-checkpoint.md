# CL-1 Study 19 — verified external graph reinjection control (2026-11-13)

**Technical calibration PASSED. Scientific functional restoration: NOT YET EVALUABLE. CL-1 positive milestone NOT ACCEPTED.**

## Provenance and controls

- Preregistration committed **before** implementation: [Study 19 protocol](../studies/19-preregistered-external-graph-control.md), commit `2b7577f6688ed49c18179bbbecae8fd492073ab5`.
- Verified source revision: `239939312172101631565ae62daa4368fce9a70b`.
- [CL-0 Rust kernel CI success](https://github.com/iFrankStudio/cioc/actions/runs/43439224791) — workspace formatting, Rust tests, Clippy, Python and Vue acceptance.
- [CL-1 full historical research CI success](https://github.com/iFrankStudio/cioc/actions/runs/43439224969) — complete historical study suite, exact SQLite replays and independent read-only auditors.
- [CIOC benchmark CI success](https://github.com/iFrankStudio/cioc/actions/runs/43439224838).

## Verified intervention semantics

New `Intervention::PutGraph { graph, object }` is **experimenter-only** and cannot be executed as a CIL opcode. It injects a previously allocated, currently absent graph handle with the same node structure, owner and read permissions. It rejects zero/unallocated or still occupied handles, unknown owners/readers, empty/oversized objects and strict storage exhaustion **without partial state mutation**.

A focused native CIL fixture constructed and populated a graph with a read grant. The test captured that object, externally `DropGraph`-deleted it, and externally `PutGraph`-restored the identical object and handle. Original graph occupancy and total compute inventory were exactly conserved after the two external actions. The successful injection is explicitly marked `Origin::External`; it does not grant compute, restart a process or claim internally generated maintenance.

A separate matched `Laboratory::run_epochs` control applied external deletion in one epoch and same-object reinjection in the following epoch, with a paired untreated run. Both complete SQLite world/event/epoch replays passed at the verified source revision.

## Scientific boundary

**This is a verified *structural* restoration mechanism control, not a functional restoration positive control.** The output of an independently specified information transformation has not yet been shown to disappear on graph deletion and reappear on external reinjection, nor has two-component endogenous reconstruction been observed. Accordingly, CL-1 self-maintenance and organizational closure remain **NOT EVALUABLE** on this evidence.

A future study must predefine that same functional input-output relationship, demonstrate matched external *functional* restoration at equal budgets, then independently test internally initiated restoration after single and double ablations in two separated observation windows. No externally authored graph injection counts as life evidence.
