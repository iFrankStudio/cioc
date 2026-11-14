# CL-1 Study 36 — registered retrospective compute-access and ignition-deadlock audit

**Timing:** registered after Study35 completed; all Study35 outcomes are already known. This is a **retrospective, explanatory audit**, not new prospective evidence. It reuses **exactly** Study35 original 192 independent seed-worlds, nine matched arms, 1,728 runs, 41,472 hashed epoch snapshots, source revision `9ee57f2536b8cd4eed2a84817f2e7e777d9bcc00`. **New independent worlds: 0.** Do not regenerate or replace the archived data merely to obtain a favorable statistic.

## Central question

Study35 showed 0/192 untreated world activity in W3 (epochs16..23), despite a finite endowment and some native DrawCompute. Is that exclusively because total compute is fully exhausted, or do scheduling and access rules leave residual compute stranded in the world pool and/or stopped processes?

## Frozen mechanism and ledger

The Rust world `World::step` schedules only processes where `!stopped && credits > 0`; every charged native attempted opcode subtracts **one** process credit before execution, even on trap. `DrawCompute` transfers existing pool credits to the actor; `ReturnCompute` transfers actor credits to the pool; `TransferCompute` and `GraphLaunch` transfer existing process credits to another process, never mint them. The only direct sources of new credits are experimenter-controlled `load`, `SetCredits`, `SetComputePool`, or other registered externally injected resources. In the closed no-refill branches, let `I = world.compute_pool + Σ process.credits`. Absent an externally changed credit balance, each successful *attempt* reduces `I` by exactly one. Credit movement inside native CIL preserves `I` after that charge.

If a world has no process satisfying `!stopped && credits > 0`, the scheduler cannot execute even a native DrawCompute, regardless of how much compute remains in the pool. This is a **compute ignition barrier**, not proof of biological death or intrinsic inability to form life in an open environment. Finite initial inventory alone bounds the total count of native instructions in any closed run; it does not prove the exact epoch at which work must cease.

## Fixed analysis

Read the archived Study35 SQLite artifact **read-only** and verify SHA256 of all initial/final worlds and all 41,472 epoch snapshots before inference. Require the same 192 seed IDs, nine arm labels, 24 snapshots/run, and the exact source revision. Do not count external interventions as native attempts. Validate the `attempts_json` ledger, including zero-credit 0 attempts.

For each `control` seed record `I0`, final `I24`, total charged native attempts; assert exact `I0 - I24 = attempts` in the absence of after-prefork external compute endowment. At end of epoch15 and epoch23 record: total remaining inventory, compute_pool, stopped processes, total credits in stopped processes, active processes with positive credit, active processes with zero credit. The primary descriptive outcome is **no runnable process with strictly positive compute pool at the start of W3**, with separate categories:
1. At least one unstopped process, but all such processes have 0 credits and pool >0 (ignition deadlock).
2. All processes stopped while pool >0 (terminal stopped).
3. Pool =0 and at least one unstopped zero-credit process.
4. Pool =0 and all stopped.

Report counts, medians, minimum/maximum remaining inventory, the fraction retaining nonzero world inventory, and the corresponding category in exact control, sham, mutant and exact-original matched branches. Compare W3 native activity in untreated control and explicitly external `refill` branches; do **not** count refill as spontaneous self maintenance.

## Integrity and controls

The original Study35 independent auditor and CI replay are authoritative preregistered primary outcome evidence. This audit must independently verify snapshot hashes, credit ledger, no-future native events after terminal quiescence, exact treatment population and W3 native attempts, and either match the original Study35 result or fail closed. A small synthetic negative and positive control for the scheduler runnable predicate is sufficient. No new opcode, source grammar, fitness function or repair mechanism is permitted. Do not modify Study35's preregistration, data or endpoints. Archive the separate result and report **0 new worlds**.

## Interpretation and next experiment

If positive inventory remains but no process is eligible to execute, the next *prospectively preregistered* experiment should isolate the compute-access topology and bootstrapping constraint, not merely add more external compute credits or report host refills as native autonomy. If all credit is exhausted, then energy conservation is the immediate bottleneck. Both outcomes must preserve a strict distinction between environment-supplied energy and internally managed access.

**CL-1 remains OPEN.**
