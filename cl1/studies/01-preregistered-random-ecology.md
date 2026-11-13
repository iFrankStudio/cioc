# CL-1 Study 01 — preregistration (frozen before experimental runner code or data)

**Registered:** 2026-11-13. **Status:** design registered; no CL-1 outcome has been examined at registration. CL-0 remains the verified world/VM implementation. This is a falsifiable first study, **not** a declaration that CL-1 is complete.

## Scientific question
Can a population of three *externally randomly initialized* generic CIL processes spontaneously exhibit resource-bounded persistence, internally produced graph changes, and shared-state-dependent behavior, without a special-purpose host repair/fitness/reproduction rule?

**Null hypothesis H0:** Any observed attempts, graph changes or perturbation differences are explainable by finite random CIL execution and exogenously supplied budgets. No evidence of internal organizational closure may be inferred without a prospective demonstration of distributed, bidirectional dependencies and autonomous maintenance.

## Frozen design, independent units and sampling
- Use deterministic 64-bit LCG (multiplier 6364136223846793005, increment 1442695040888963407), with independent seed inputs **610000 through 610063 inclusive** (64 distinct externally chosen initial worlds). Seeds are never selected after examining outcomes.
- Each world starts with **three** different investigator-generated CIL processes, 16 nodes each, an equal 28 instruction credits each. IDs 1–3. The initial generator and its opcode weights must be documented in source **before** running trials.
- Bounded strict world: 32 shared data cells, at most 32 nodes/process, total 160 abstract storage units. No host repair/fitness/reproduction policy. Programs may trap; traps and zero-activity outcomes remain in the denominator.
- Fixed 96-attempt horizon; analytic windows: after 0, 24, 48, 72 and 96 *possible* execution attempts. A window with no runnable process records its actual attempts, not a fictitious complete window.
- Compare five matched conditions from an identical initial serialized snapshot:
  1. `control`: no intervention;
  2. `zero-credit`: all process credits set to zero at attempt 0;
  3. `disable-p1`: externally disable process 1 after 12 attempts, if reached;
  4. `lesion-cell`: externally overwrite public cell key 0 with 9999 at attempt 12, if reached;
  5. `sham-cell`: same external overwrite value at distant cell key 1000000 at attempt 12, if reached. Sham has equal one-cell potential storage cost, but is **not** assumed behaviorally neutral if the CIL accesses that key.
- Run **320 conditions = 64 matched initial worlds × five treatments**. Every run must survive full SQLite replay verification; invalid/corrupted runs invalidate *technical acceptance*, not become selectively dropped evidence.
- Experimenter-supplied seed code, fixed initial credits and any world modifications are explicitly external. No on-line selection, breeding, favorable-world filtering or post-hoc parameter tuning is allowed. The observer does not affect the world.

## Primary descriptive and falsification measurements (fixed before outcomes)
For every run record exact attempted instructions, internal/external event counts, trap count, final occupied resource units, final shared-cell count, new process count, graph count, and **newly added executable nodes** above the 48 initial process nodes. These last two are not validated utility, heredity or novelty in the strong sense.

For each independent seed, count whether:
- `active_late`: control has *at least one* internal attempt after its first 48 attempts (equivalent to control total attempted instructions >48; **not** evidence of self-maintenance);
- `endogenous_graph_change`: final executable graph/node inventory differs from initial and internal graph-related instructions were attempted;
- `lesion_effect`: final semantic state of `lesion-cell` differs from `sham-cell` **excluding the directly manipulated keys 0 and 1000000** and excluding the external event logs. Without this exclusion, the contrast would be tautological;
- `p1_dependency`: the number of *other-process* internal attempts (process 2 or 3) differs between `control` and `disable-p1`, removing the trivial effect of process 1's forced stop.
- `closed_organization`: **NOT EVALUABLE in Study 01**. A single ablation and late activity cannot establish bidirectional internal maintenance, even if their proxies are positive.
- `novelty_accumulation`: **NOT EVALUABLE** from fixed-horizon one-shot random CIL worlds without independently traceable lineage/persistence across several externally defined observation windows.

Report the counts across all 64 seeds without exclusions, including zeros, and paired differences for the five conditions. The study must not set a threshold for “life” based on this dataset.

## Controls, integrity and failure rules
- A zero-credit condition must have exactly 0 internal instruction attempts.
- Any matched condition must have identical `initial_sha256` within its seed.
- All 320 runs must verify deterministic SQLite replay, including reached scheduled fork hashes; no silent reset.
- Storage never exceeds configured capacity; no undocumented compute minting.
- No condition may be classified a spontaneous recovery merely because external `SetCell`, `Disable`, or initial provisioning occurred.
- Summarization must be a read-only independent program, with deterministic output and a test covering the non-tautological state-difference rule.
- Any violation of an integrity condition means **study execution invalid** and requires a documented correction/revision and a full rerun, never selective exclusion.
- A negative or inconclusive result is a valid study finding; it does **not** close the CL-1 research objective.

## Technical execution plan (not data)
Implement `cl1/` with a dedicated Rust experiment binary in the existing locked `cl0-lab` crate, a separate independent read-only Python analyzer and a CI workflow archiving SQLite plus analysis. Freeze this registration in its own commit **before** creating the runner or seeing resulting data.

## Next study requirement
Before claiming CL-1 complete, launch separate prospective experiments that can actually test autonomous maintenance and bidirectional organizational dependencies. A Study 01 null result or mere structural variation is insufficient.
