# B55a executable-resource manifest v1 — prospective, NOT native results

Frozen before any B55a Rust/CIL implementation on 2026-11-17. This manifest instantiates the B55a protocol using **observed B54-compatible ceilings**, not an assertion that the proposed B55a actor fits them. If it does not fit, record a feasibility failure and revise as a new numbered protocol, never silently enlarge limits after outcome inspection.

## Exact fixed constants

- Seeds (paired across A–G and E1–E3): `[101, 211, 307, 401, 503, 601, 701, 809]`. These are **experiment identifiers**, not claims that the deterministic World has an RNG. Seed-dependent input permutations must be implemented by the laboratory's fixed deterministic schedule and exposed identically to all arms; no hidden RNG may change the source.
- One PID = 1, no replacement, process starts with **4,000 local compute credits**, no replenishment, no pool draw or additional actor; **4,000 maximum actual paid CIL instructions** per world.
- `World::with_strict_capacity(16, 180, 300)`: maximum 16 data cells, 180 code nodes, 300 total occupied storage units, as used in B54. **Feasibility must be tested** against the new B55a founder and its clones. The B54 124-node source fits; a B55a menu may not.
- Input training probes `[5,-3]`; scored heldout probes `[11,-7,0]`, repeated in that order. Mandatory heldout service opportunities: **24 per stage, 3 stages = 72** per world. Missing writes are counted; extra outputs are not a substitute.
- Two hidden affine families, alternating by seed index parity: A `(s,k)=(1,-2)` → `(-1,3)` → `(1,-2)`; B `(-1,-3)` → `(1,2)` → `(-1,-3)`. The actor receives input and paid feedback, **never** a task label. For E1 all three stages use the first task; for E2 the middle stage changes and returns.
- E3 uses the E2 task schedule but current-nonce success feedback is overwritten for truly wrong outputs at the **same native pre-Read boundary** as B54's LATE_BIT_FLIP. The intervention is externally authored and has access to laboratory truth; never attribute it to PID1. Report the number of interventions separately.
- Two prospective trial periods: the first 2 × 2 training probes; then actor must choose once through charged CIL operations and be scored on the same 72 heldout opportunities. The 3 preauthored candidate policies are NEVER, EVERY, FOURTH. The schedule of their trial order must be identical across all arms and seeds. Source-identical actor must not know whether it is in E1, E2 or E3.
- A–G arms, utility, acceptance and mandatory negative controls are unchanged from `B55a-monitor-choice-executable-protocol-freeze.md`. No additional reward or hidden oracle is available to the actor. All comparisons are paired by seed, environment, input stream and initial compute/storage ceiling.

## Pre-execution feasibility and provenance gates

1. A native feasibility test must assert that **all seven arms** load under the exact strict capacities with identical initial opportunity, and that all trial and 72 mandatory service windows are scheduled without a host-selected source patch. If not, B55a is BLOCKED at this manifest; **do not silently increase 180/300/4000**.
2. Before a scientific acceptance test, the actor's own paid CIL operations must perform the trial observation, policy choice, and executable-edge mutation. A host-side comparison of U or host-chosen route **fails** the core hypothesis.
3. No post-hoc substitution of a policy menu, schedule, input set, trial lengths or seeds. If the implementation cannot perform 72 outputs within 4,000 paid steps, count all missing mandatory writes as M, not as excluded worlds.
4. Emit the SHA-256 of this manifest's exact UTF-8 bytes in the native trace and independent JS replay, along with the actual source SHA, paid steps, actor PID, source-edge edits and resource deltas. A SHA-256 digest is not yet calculated here; no fabricated digest is frozen.
5. Record actual counts `(C,W,M,P)` and `U=(C-2W-M)/max(1,P)` for every arm × ecology × seed. H55a is rejected unless every E2 paired A−B and A−C difference is strictly positive and every E1 A−C difference is nonnegative. A CI PASS alone cannot establish H55a.

## Explicit prediction (not measured)

A preauthored monitor-selection menu may fail to fit the B54 code-node/storage cap, and a 4,000-instruction budget may fail to accommodate trials plus 72 scored outputs. Both are engineering falsifiers, not grounds to delete bad worlds. A seeded fixed oracle may outperform actor choice. Current-nonce lies remain indistinguishable without a separate trustworthy signal. **B3/B4 NOT MET; no B55a native evidence yet.**
