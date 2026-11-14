# CL-1 Study 37 — preregistered general pool-access ignition experiment

**Registration status: prospective and frozen.** This protocol is committed to `main` **before** implementing, executing, auditing, or inspecting Study37 outcomes. Study36's archived 138/192 stranded-pool worlds motivate the question but **none** of those seeds are reused. A positive result is about environment-facilitated use of finite credits, **not cyberlife**.

## Narrow causal question

Does a generic, deterministic, resource-conserving *environment-side access rule* that gives exactly one temporary execution credit to a stalled actor already pointing to native `DrawCompute` allow additional unassisted-by-`SetCredits(47)` activity in the late W3 observation window, compared with the unchanged baseline kernel?

This intentionally tests a **host/environment mechanism**, not an endogenous repair policy. The rule chooses a process only by publicly observable execution state and uniform actor ordering, never by success, organism viability, intended output, fitness, network identity, or observed previous seed outcomes. It moves one already-existing world-pool credit, and may run again at the next epoch if the same conditions recur.

## Independent seeds and exact native world

- Exactly **128 new independent worlds**, seeds **2,800,000..2,800,127**, all included, 3 processes in each, 24 epochs × 12 attempted instructions/epoch, no seed-selection and no stopping an arm early.
- Copy Study35's `seeded_world` and `prefork_world` implementations byte-for-byte apart from the study identifier and `FIRST_SEED`/`REPLICAS`. Same nine-instruction scaffold, identical LCG/12-opcode/18-node continuation, 56 initial credits per process, common 27-prefork native attempts, 48 **researcher-provided** world compute pool, 47 postprefork credits/process, resource and storage caps and graph permissions. The native CIL grammar **must not** be edited to contain more DrawCompute or guarantee a successful run.
- Never change the Rust kernel's `World::step`, `DrawCompute`, `GraphLaunch`, accounting, scheduler or interpreter. All new experimental actions are explicitly **External interventions**. Use `Laboratory::run_epochs` and `verify_epoch_replay` for every paired branch, with full hashed SQLite states and source revision.

## Frozen matched conditions (7 arms per seed, 896 runs, 21,504 snapshots)

1. `control`: no intervention.
2. `pool-access`: only the following environment policy, all epochs.
3. `empty-pool`: epoch0 `SetComputePool(0)`, no policy.
4. `pool-access-empty-pool`: epoch0 `SetComputePool(0)`, then identical policy.
5. `sham`: rejected `DropGraph(graph0)` at epoch0; otherwise identical to control.
6. `external-refill`: direct external `SetCredits(47)` for processes1..3 at epochs8 and16, and no pool-access rule; this is a **host subsidy positive control**.
7. `pool-access-zero-credit`: epoch0 externally set each process credits to zero, **without** emptying the initial pool, then identical policy; this tests whether environment-mediated ignition can start a process from zero credit, and any such start is **externally** enabled.

### Exact generic pool-access policy (only arms 2, 4, 7)

At the **beginning** of each epoch, apply the arm's base registered interventions first. Examine the current World, before any native attempt in that epoch.

- If **any** process is runnable (`!stopped && credits>0`), do nothing.
- Else enumerate existing processes in ascending ProcessId order, choosing the **first** process that is `!stopped && credits==0`, whose **current** code at its `pc` is `Op::DrawCompute { amount, ... }`; read `amount` strictly as a *register index*, resolve it to an unsigned quantity `q` from the current actor's register, and require `q>=1`, no overflow, and world pool `P>=q+1`. No process is selected if none qualifies.
- If selected, perform two and only two External actions, in this exact order, in the **same epoch**: `SetComputePool(P-1)` then `SetCredits(process_id,1)`. Their net effect conserves world total compute inventory and leaves the native `DrawCompute` as the process's own ordinary executable instruction, which then costs one credit to attempt. No participant sees an extra opcode or intrinsic repair. The seed/actor is not outcome-selected; at most **one** such transfer per epoch.
- No native execution occurs during policy selection. No pool-access action occurs between native instructions within an epoch. Each action, including rejected control actions, remains in recorded `actions_json` and native event provenance as `External`. If no candidate exists, do nothing. The rule must not create credits or revive a stopped process.

For compatibility with `Laboratory::run_epochs`, precompute each arm's action schedule by deterministically executing **the same frozen policy** on a private cloned prefork world for all 24 epochs (apply base actions, apply policy, then `run(12)`). Replay those explicit recorded actions through the existing laboratory without any additional researcher discretion. **Assert the action-plan simulation's final state is byte-for-byte identical to the laboratory result**, and invoke `verify_epoch_replay` on every run. An independent Python auditor must reconstruct the eligible policy actions from each preceding state/registered base action and reject any action inconsistent with the rule.

## Frozen outcomes and scientific interpretation

Primary paired descriptive endpoint: number of seeds with at least one **native charged instruction attempt in W3 (epochs16..23)** in `pool-access`, versus `control`, including the paired discordance counts `policy-only` and `control-only`; report each denominator as 128 and paired uplift, with Wilson 95% intervals for arm proportions. The **narrow null** is `policy W3 = 0/128`; a positive event shows only environment-side ignition can extend computation.

Secondary independent metrics: number of successful policy transfers per seed/epoch, number of actor-native successful `DrawCompute` after a policy transfer in the same epoch (success-only events), active W1/W2/W3 worlds, final total inventory and pool balance, stopped vs zero-credit actors, negative control state equivalence (control/sham and empty-pool/pool-access-empty-pool when no policy transfer occurs), zero-credit arm actor ignition frequency, world resource conservation, and comparison to explicit host refill. No functional restoration score or life claim is eligible from this experiment.

**Conservation integrity:** for `control`, `pool-access`, `empty-pool`, `pool-access-empty-pool`, `sham`, `pool-access-zero-credit`, at each epoch total compute inventory must equal the original prefork **189** minus removed one-time external credits (48 in empty-pool; 141 in zero-credit) minus cumulative native charged attempts. Policy transfer pairs must have **zero net compute inventory change**. External-refill uses an explicit recorded external ledger and must never enter this conservation-null group.

## Independent validation

Check every SHA256 epoch snapshot, all 7 matched arms per seed, exact independent initial LCG program, source revision and prefork invariants, base and conditional intervention schedule, external versus native event counts/actors, no account overflow, native event provenance, and byte-for-byte Rust replay. At minimum calibrate a synthetic positive case (zero-credit actor about to draw from a positive pool), a no-candidate case, a positive-credit runnable actor case, an insufficient pool case, and a stopped actor case. Avoid hundreds of incidental tests. Archive untouched full SQLite and independent JSON.

## Decision

Even if the W3 endpoint increases, **this is a researcher-supplied environmental access law**, **not endogenous code construction or repair**, and the initial finite compute inventory still bounds total native steps. A successful policy contrast would justify later research on **generic renewable environmental compute inputs and internally built capability-bearing organizations**, with fresh separate preregistration. Under no circumstances claim life from externally delivered execution credit, a matching output or positive CI.

**CL-1 remains OPEN.**
