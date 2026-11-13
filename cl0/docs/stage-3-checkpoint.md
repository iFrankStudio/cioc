# CL-0 Stage-3.1 verification checkpoint — 2026-11-13

## Scientific claim boundary
This increment adds general-purpose graph operations and partial shared storage pricing. It **does not** demonstrate self-maintenance, self-organization, individual identity or life. A test's constructed positive behavior is strictly a VM conformance fixture.

## Included
- `World::with_total_capacity`: optional aggregate limit for graph nodes + shared data cells.
- New CIL instructions: `ReadNext`, `CloneNode`, `PatchConst`, `Delete`.
- Bounded allocation on external program loading, internal node copy, internal cell creation and external cell insertion.
- Clone-capacity, code/data-capacity, cell reclamation and source/edge constraints in deterministic tests.
- Matched SQLite replay integration: same initial world, clone allowed versus experimenter-supplied zero compute credit.

## Failure record
- Initial Stage-3 CI failed because the test fixture attempted to clone an invalid node index and then asserted the wrong edge. The fixture was corrected, not dismissed.
- Subsequent CI demonstrated the corrected tests pass but Clippy found a collapsible-if issue in the new external storage guard. That issue was corrected without changing the capacity rule.
- Verified Stage-3 documentation commit `49024a6` in GitHub Actions run https://github.com/iFrankStudio/cioc/actions/runs/43397416909 : the workspace tests and strict Clippy completed successfully. The rustfmt command formats CI working files; it does not enforce a clean formatted repository checkout.

## Limitations
Worlds constructed with legacy `World::new` retain unbounded **aggregate** graph storage across processes; use `with_total_capacity` for a finite Stage-3 pool. No process creation, deletion, node erasure, byte-accurate memory charging, object capabilities, arbitrary graph synthesis, observer-independent claims of fitness, or direct evidence of spontaneous evolutionary innovation.
