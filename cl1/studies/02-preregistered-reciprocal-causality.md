# CL-1 Study 02 — preregistered reciprocal causal-sensitivity audit

**Registration:** 2026-11-13. **Frozen prior to Study-02 experimental runs.** Study-01 results (9/64 lesion-vs-sham indirect-state contrasts; 0/64 other-process attempted-instruction contrasts) are already known and cannot be used to tune this preregistration. Independent Study-02 seeds **720000..720127** are not reused from Study 01. The full Study-01 initial program generator, weights, horizon and initial resources are unchanged.

## Scope and hypothesis
Test whether independently seeded CIL process populations display **bidirectional causal influence on process state** when one of the three processes is disabled. Such reciprocal sensitivity is only a candidate prerequisite for internal organization; **it cannot prove maintenance, functional dependence, or life**.

## Frozen sample and environment
- 128 externally independent seeds 720000 to 720127, three 16-node processes per world, same 64-bit LCG and uniform 12-way opcode selection as Study 01. No generated population is selected for performance.
- Same strict world: 32 shared cells, 32 maximum nodes/process, total 160 abstract units. Every process gets 28 initial credits, no world compute-pool injection.
- Fixed 96 world instruction-attempt ceiling; matched intervention offset 12.
- Six conditions per independent seed:
  1. `control`, no intervention;
  2. `disable-p1`, `Disable { process: 1 }` at offset 12;
  3. `disable-p2`, `Disable { process: 2 }` at offset 12;
  4. `disable-p3`, `Disable { process: 3 }` at offset 12;
  5. `sham-disable`, rejected `Disable { process: 9999 }` at offset 12;
  6. `zero-credit`, all process credits externally set to zero at offset 0.
- 128 × 6 = 768 SQLite conditions, each requiring exact replay, common initial hash within its seed, strict capacity and zero-credit validity. A scheduled action not reached because the world stopped is recorded **as not performed**, never counted as successful treatment.

## Independent endpoint statistics
For a process ID j, define `state_j(W)` as its serialized final process state (code graph, program counter, registers, credits, stopped). It excludes globally recorded event logs and the directly disabled process i.

For each seed and each ordered pair i≠j:
`causal_sensitivity(i,j) = [state_j(control) != state_j(disable-pi)]`,
**only if the disabling action at offset 12 actually occurred**. Otherwise mark pair `not exposed` and keep that seed in the denominator.

A **reciprocal sensitivity candidate** for unordered pair (i,j) requires both `causal_sensitivity(i,j)` and `causal_sensitivity(j,i)` in that seed. Count each seed once if **at least one** reciprocal pair exists; separately report individual pairs, exposed and unexposed counts, directional counts, and 95% Wilson binomial intervals. The denominator is always the 128 independent seed-worlds; **do not** treat 768 matched conditions as independent replicates.

Instrumentation checks:
- `sham-disable` must yield identical final process states to `control` (apart from external events/ticks), or the test is invalid.
- `zero-credit` performs zero internal CIL attempts.
- Reached interventions must be attributed `Origin::External`; unmatched initial hashes or failed replays invalidate the technical dataset.
- Disabled process state is **not** used as evidence of dependency.
- Same 96-attempt horizon is large enough to exhaust three initial 28-credit accounts in the absence of free credits; scheduling shifts can still affect interactions, so reciprocal sensitivity cannot be equated with an internally maintained organization.

## Interpretation rule
- If zero exposed reciprocal candidates: no evidence of bidirectional causal sensitivity in this frozen initial distribution.
- If any candidate: report it as reciprocal **state sensitivity** only, not autonomous maintenance or organizational closure. No life claim can be made from this study. Effect sizes and uncertainty are reported even if the total is zero.
- No seeds, runs, or unperformed interventions are excluded after outcome inspection. Failures, traps, and inert runs remain in the denominator.

## Prespecified next-stage necessity
A separate prospective **damage-and-recovery** protocol must verify internally initiated restoration without host repair, matched sham damage, external compute-source attribution, and time-separated persistence before CL-1 may claim any maintenance evidence. Study 02 by itself is insufficient to accept CL-1.
