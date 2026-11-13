# CL-1 Study 23 — verified successful source-graph read attribution

**Technical result: PASSED. Scientific milestone: NOT ACCEPTED. No new ecological seed-worlds were sampled in Study 23.**

## Registered before implementation

- [Study 23 preregistration](../studies/23-preregistered-graph-read-provenance.md), commit `b13a6f05a93f19f37bb6b08e2d2d33c406fd34a4`.
- Verified source revision: `483cd15798025a05afbf3a0d6d3d7e29fb26513b`.
- [CL-0 kernel and targeted Rust tests passed](https://github.com/iFrankStudio/cioc/actions/runs/43446182035).
- [All seventeen source-pinned CL-1 study jobs passed](https://github.com/iFrankStudio/cioc/actions/runs/43446182100), including independently replayed earlier stochastic ecology and the precommitted Study-20/21 functional controls and Study-22 energy-limitation cohort.
- [CIOC benchmark passed](https://github.com/iFrankStudio/cioc/actions/runs/43446181952).

## Exact new native observer outcomes

`GraphTagRead { graph, owner, node, tag }` records a successful CIL `GraphReadTag` with *source owner from the actual graph object*. `GraphFieldRead { graph, owner, node, field, word }` similarly records the exact decoded word of a successful `GraphReadField`. The containing event retains `Origin::Internal(reader_id)`, PC, tick, process and credits-after-execution. Neither variant creates new graph data, new compute, new permission, a scheduler decision, or an executable CIL instruction.

The preexisting permission-granted foreign graph-tag read fixture now asserts source graph 1, owner 1, reader 2, node 0 and actual tag 0. A separate narrowly targeted field-read fixture verifies owner1's copied `Const` node with its encoded field 2 equal to 42 is read successfully by permitted reader2 at PC3, recording `GraphFieldRead { graph:1, owner:1, node:0, field:2, word:42 }`. The register receives exactly 42 and all ten charged instruction attempts consume the same compute as before. Existing denied/revoked read tests continue to trap without successful source-read events. Full workspace tests and CI pass.

## Strict scientific limitations

This closes one **observability** gap: an auditor can identify **which graph was read and whose code it contained at the read moment** without inferring source from a later write or owner identity. It does not prove that the read information caused a later output value; proper register-level dataflow dependence still requires preregistered content-mutating paired controls and an independently specified output function. In particular, Study-22's 15 externally recharged W3 worlds with cross-owner copies and 23 with native cell reads remain *functional non-exposures* because **zero W3 worlds successfully wrote a registered native shared-cell output**. No external recharge or graph injection becomes an endogenous organism action.

**CL-1 self-maintenance, open-ended adaptation and organizational closure remain unaccepted.**
