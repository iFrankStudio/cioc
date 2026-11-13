# CL-1 Study 03 — preregistered code-damage and internally initiated restoration audit

**Registered:** 2026-11-13. **Frozen before writing the Study-03 runner or viewing any Study-03 outcomes.** Study 01 has been inspected; Study 02 source is under CI. Study 03 uses new independent seeds **830000 through 830127** and is not selected based on previous outcomes.

## Research question and interpretive boundary
Can randomly initialized but generally code-editing-capable CIL processes reverse a targeted graph-control edge lesion **without the host restoring it**? A correct edge reversal is an **endogenous structural-restoration candidate**, not necessarily functional self-maintenance, a closed organization or life.

The experiment must not install a built-in REPAIR, FITNESS, reproduction or validation oracle. The observer detects changes after execution; it never selects which mutations to accept.

## Fixed population generator and resources
- 128 independent seed-worlds with three processes each, 16 nodes/process, 28 initial credits/process, no initial compute-pool replenishment.
- Strict world capacities: 32 shared data cells, at most 32 nodes per process, total 160 abstract units; 96-instruction-attempt horizon.
- Distinct deterministic 64-bit LCG seeds; initial code is **external**. Nodes 0–3 initialize registers using fixed roles: r0 shared-cell address uniformly from 0..4, r1 integer from 0..100, r2 *candidate target node index* uniformly from 0..14 and r3 *candidate edge index* uniformly from 0..15. Neither r2 nor r3 is initialized to a special damage-specific value. Later operations may alter these registers.
- For nodes 4–14 except node 8, uniform 12-way instruction choice among Add, Sub, Read, Write, Branch, PatchNext, CloneNode, GraphNew, GraphCopyNode, GraphDrop, DrawCompute, and a backward Branch. Every edge is fixed before outcomes. Node 8 is constrained to a uniformly sampled **single-successor** opcode with its original next edge equal to 9 so that a graph-edge lesion is well-defined, but it is not a special repair controller. Node 15 is Halt.
- No branch of the study selects populations based on whether they can recover.

## Five matched conditions, each from identical initial state
1. `sham`: at attempt offset 4 record a rejected `Disable { process:9999 }`; later rejected sham actions at offsets 24, 48, 72.
2. `lesion-only`: at offset 4 externally `PatchEdge { process:1,node:8,next:0 }`; sham actions at 24, 48, 72. This arm has **no host restoration**.
3. `external-restore`: same lesion at offset 4; at offset 24 externally `PatchEdge {process:1,node:8,next:9}`; sham actions at offsets 48, 72. This is a positive **instrumentation** control, not endogenous repair.
4. `disable-p1`: externally disable process 1 at offset 4; sham actions at 24, 48, 72. It checks that a nonexecuting target cannot autonomously edit its own graph under this configuration.
5. `zero-credit`: set all process credits to zero at offset 0, with no later actions.
- A scheduled action after global computation becomes inert is *unperformed* and must not be imputed. Report reached/failed actions independently. 128 × 5 = 640 runs, exact SQLite replay required for every run.

## Primary prespecified outcomes
- `lesion_exposed`: offset-4 patch reached and succeeded. Keep the full 128 seed-world denominator regardless.
- `restored_by_24`: among exposed lesion-only worlds, the edge at process 1 node 8 equals 9 in the **pre-sham snapshot at offset 24**, and no external restore occurred in this arm. This is possible only by internal code editing after the lesion, but it does not prove a maintained function.
- `restored_by_final`: exposed lesion-only world's final process 1 node 8 next edge is 9, with no host restore.
- `persistent_restoration_proxy`: exposed lesion-only worlds whose pre-sham snapshots at both offsets 24 and 48 **and** final state all show edge=9. All three checkpoints must have been reached.
- `active_after_damage`: process 1 executed at least one internal instruction after the reached lesion (count using event order, not total attempts).
- `functional_maintenance`: **NOT TESTED** by comparing a graph edge alone. A corrected edge can be inert or incidental.
- `closed_organization`: **NOT TESTED**: this is a one-target lesion, not proof of mutual regeneration of distributed parts.

The externally restored arm is counted separately; any correct edge attributable to the explicit external restoration must **never** be counted as spontaneous. The `disable-p1` and zero-credit controls remain in the denominator, including inert worlds.

## Reliability and uncertainty
- All 640 runs must pass exact SQLite replay, all checkpoints must verify saved SHA-256, initial hashes must match per seed and world storage must remain <=160 units.
- Report numerator/128 as well as numerator/exposed for descriptive context, and two-sided 95% Wilson intervals over **128 independent worlds**.
- Any nonzero restored candidate is a candidate for follow-up dynamic functional validation, not life evidence. Zero candidates means no such event was measured in the frozen program distribution at 96 attempts, **not** impossibility of cyberlife.
- No post-hoc exclusion, damaged-node redirection, threshold tuning or early stopping after observing a favorable seed.
- A protocol failure mandates a documented correction and full rerun, retaining failed CI history.

## CL-1 interpretation
Studies 01–03 may constitute a completed **bounded negative-capable CL-1 feasibility campaign** if all protocols and audits execute; they do not establish the *positive* goal of originating a self-maintaining, organizationally closed cyberlife form. Do not declare the discovery of life on the basis of any one proxy. A positive CL-1 claim requires separate functional and reciprocal restoration experiments with preregistered thresholds.
