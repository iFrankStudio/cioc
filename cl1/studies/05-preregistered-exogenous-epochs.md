# CL-1 Study 05 — preregistered exogenous epoch environment

**Registration:** 2026-11-13, prior to Study-05 runner implementation, analysis, or any Study-05 results. Studies 01–04 are already known; new seed inputs **1050000..1050063** are independent of those cohorts. This is a technology and falsification study, not a positive-life test.

## Question
Does a compute environment that supplies resources at **external scheduled epochs, independently of whether CIL remains active**, differ qualitatively from activity-triggered supply? The host must record the supply even when all CIL processes are inert, without granting a process compute credit directly. This tests the environmental clock limitation of CL-0 scheduling, not natural aging or spontaneous life.

## World and generator
Study 04's fixed external CIL generator and parameters are reused verbatim with fresh seeds: 64 worlds, three 16-node processes and 28 initial credits each, four initialized registers, uniform 12-way general CIL grammar; strict total storage 160, public cells 32, per-process node limit 32. Initial code is **investigator generated** and never selected based on outcomes.

## External epoch definition
An epoch is **one laboratory-administered opportunity**, not world logical time. There are **24 epochs indexed 0..23**. Every epoch can execute at most **8 CIL instructions**, and the next epoch happens whether or not any instruction executes. Thus there are 192 potential attempts and zero or more actual attempts. Environmental actions scheduled for an epoch occur **before** that epoch's 8-instruction opportunity. All environment actions have `Origin::External` and use `World::intervene`. Experimental actions are not CIL instructions.

## Four matched conditions (64 × 4 = 256 world runs)
1. `closed`: no environmental compute supply in any epoch.
2. `initial-pool`: set pool to 32 at epoch 0, never replenish.
3. `pulse-pool`: set pool to 32 at epoch 0, 4, 8, 12, 16, 20, including when no process has remaining compute credits.
4. `zero-credit-pulse`: all process credits set to 0 at epoch 0, plus pool set to 32 at epochs 0,4,8,12,16,20; this is a decisive **non-spontaneous-restart control**.

All conditions begin from the same byte-identical initial snapshot for each seed. Resource availability is an **external environmental variable**; the runtime does not magically assign it to a stopped/no-credit CIL process.

## Frozen outcomes
- `actually_executed`: aggregate CIL instruction attempts over 24 epochs.
- `pulse_events_delivered`: count externally logged `SetComputePool` events; must equal **6 for every pulse arm, including inert runs**. In the zero-credit-pulse condition, internal attempts must equal zero even though the external pool arrives six times.
- `active_late`: any internal attempt in epochs 12–23; this is **resource-supported execution**, not maintenance.
- `recovered_from_inert`: a later epoch has internal attempts after an earlier epoch had zero; preregistered indicator of possible renewed execution, **not** organizational restoration. If this occurs, report exact seed, actions and provenance without interpreting the activity as revival.
- `created_process`: a new process ID >3 appears; not counted as reproduction.
- `storage`: strict world occupancy remains ≤160.
- Paired difference of final states between initial-pool and pulse-pool, with direct external pool state excluded and event history excluded. Even a nonzero effect is not life.
- Report all 64 seed-worlds; binomial Wilson 95% intervals are optional descriptive and must use 64, not 256, as denominator.

## Verification and scientific validity
All 256 epoch-based runs, initial snapshots, action lists, **each of 24 post-epoch state hashes**, and final event order must be durably stored in SQLite with exact same-version replay. Explicitly audit epoch indices and scheduling upper bounds, rejected actions, overflow cases and tampered results. A failed integrity check invalidates **the full Study 05 dataset**, not only the failing record.

The CL-0 native world remains unchanged. The epoch clock is **outside CIL** and cannot evaluate program survival, recovery or usefulness. No external process is revived and no host repair/fitness oracle is added. The study cannot establish self-maintenance, organizational closure or endogenous novelty: the positive CL-1 milestone stays open regardless of how long a seeded process runs.

Results are published after this immutable preregistration commit. Study-05 source, CI, negative observations and limitations must be versioned separately.
