# CL-1 Study 38 — preregistered open-environment compute supply × generic process ignition

**Prospective registration.** Freeze this protocol on `main` **before** adding Study38 code, examining its seeds, or running simulations. Previous Studies35–37 establish two distinct resource barriers: finite closed inventory and a positive-credit scheduler gate. This is a **native-world environmental physics experiment**, **not a life acceptance study**.

## Research question

With ordinary CIL programs and no preprogrammed survival task, which independent, explicitly external physical conditions permit native actors to continue executing in W3? Compare ambient resource arrival versus the scheduler's access topology. Specifically, does ambient pool supply alone fail without a process able to run; and does a generic single-credit environmental starter (not restricted to a pending `DrawCompute`) permit sustained episodic native execution while conserving access transfers?

## Frozen independent native ecology

- **128 new independent seed worlds**, integer IDs **2,900,000..2,900,127**, all counted with **no selection**.
- Reuse Study37 exact 9-op native scaffold, 18-node random continuation per process, fixed 64-bit LCG and all twelve opcode choices **byte-for-byte**. Three processes, 56 credits each initially, 27 common prefork charged instructions, exactly 47 credits/process and world compute pool48 before fork. Same `with_strict_capacity(32,40,192)` and three original one-node graphs/permissions. No new native opcode, hardcoded instruction sequence, optimization or repair/fitness/reproduction.
- 24 epochs, 12 available native attempts per epoch (W1 0..7, W2 8..15, W3 16..23). Deterministic actor scheduler, native one-credit-per-attempt charge, same `Laboratory::run_epochs` and exact replay. Environment actions, including ambient supply and single-credit starter, must all be recorded as **External**.

## Environment mechanisms, frozen before outcomes

**A. Ambient pool input:** at the **beginning** of each epoch **1..23**, add exactly **8** researcher/environment-supplied credits to the *world pool*, using `SetComputePool { credits: current_pool+8 }`. This is a finite explicit **external energy source**, applied whether or not any process runs. Never credit a particular actor as part of ambient input. The total inventory must rise by exactly 8 each input epoch, minus charged native attempts. No ambient input occurs at epoch0.

**B. Draw-gated ignition:** Study37's generic external policy, after ambient/base interventions: only if **no process runnable**, choose the lowest-id existing unstopped zero-credit actor whose next instruction is a native `DrawCompute` and whose resolved register amount `q>=1`; require pool `P>q`, then `SetComputePool(P-1)` and `SetCredits(chosen_id,1)`. At most once per epoch. This is **external ignition**, not the CIL actor doing self-maintenance.

**C. General ignition:** after ambient/base interventions, if **no actor runnable** and pool `P>=1`, choose the lowest process id that exists, is unstopped and has 0 credits, with **no inspection of its code, future behavior or objective**. Transfer exactly one credit by two external actions in order: `SetComputePool(P-1)`, `SetCredits(id,1)`. At most once per epoch. No stopped process may be revived and no resource may be created by a transfer. This models a neutral resource-access scheduler, **not organism-specific rescue**.

All environment state-dependent action schedules are predicted on private cloned native worlds by the fixed rule for each epoch, then passed to `Laboratory::run_epochs`. Predictive simulation and replay must end in byte-for-byte identical native worlds for every arm. No intervention may depend on which seed or actor yields the best scientific result.

## Frozen arms (10 matched per seed, 1,280 runs, 30,720 epoch hashes)

1. `closed`: no input or ignition.
2. `closed-draw`: no input, draw-gated ignition B.
3. `closed-general`: no input, neutral general ignition C.
4. `ambient`: ambient A, no ignition.
5. `ambient-draw`: ambient A + draw-gated B.
6. `ambient-general`: ambient A + general C.
7. `ambient-general-empty-pool`: epoch0 `SetComputePool(0)`, then A + C.
8. `ambient-general-zero-credit`: epoch0 `SetCredits(id,0)` for all three, then A + C.
9. `external-refill`: direct host `SetCredits(id,47)` for processes1..3 at epochs8 and16, no A/B/C, an explicit **external positive control**.
10. `sham`: rejected `DropGraph(graph0)` at epoch0, otherwise same as closed.

All branches start from a byte-identical prefork snapshot per seed; external interventions and all their effects must remain observable and separately auditable.

## Frozen endpoints and exclusion criteria

**Primary prospective contrast:** W3 native charged activity in `ambient-general` versus `ambient-draw`, as counts out of 128 paired worlds; report discordance `general-only`, `draw-only`, `both`, `neither`, and Wilson 95% intervals. This tests a *generic world scheduler access law*, not endogenous maintenance. No minimum effect size is required for scientific integrity; report the exact null if 0.

**Secondary predeclared causal contrasts:** `closed` vs `closed-general` (access without new energy); `closed` vs `ambient` (energy without actor access); `closed-draw` vs `ambient-draw` (environment resource supply with draw access); `closed-general` vs `ambient-general` (ambient renewal under neutral ignition). Record native W1/W2/W3 activity, first/total environmental starter actions, executed native attempts per epoch, ambient input credits, total inventory after each epoch, halted versus zero-credit actors, actors successfully executing `DrawCompute` after draw-gated starter, and actor-world output changes. Never treat externally activated W3 activity as organism survival.

**Conservation:** for all non-`external-refill` arms, exact world total inventory per snapshot equals 189 − any epoch0 removals (48 for initial-empty-pool, 141 for zero-credit) + 8 × number of completed ambient epochs (1..23) − cumulative native charged attempts. Each access transfer is net zero. `external-refill` must have a separately exact `SetCredits` accounting ledger. Reject any invisible host action, wrong source revision, unexpected native program alteration, mismatched initial snapshot, missing hash, seed omission, actor-specific fitness reward, or unverified replay.

## Independent controls and archiving

Independently re-derive state-dependent environmental actions from **previous hashed snapshot**, registered base edits and actor eligibility, validate all 30,720 SHA256 epoch snapshots, native event/attempt correspondence, actor identity, exact 8-credit input at every scheduled epoch, policy transfer conservation, and matched sham identity. At least one focused test for each ignition mode's positive/negative candidate, zero-credit bootstrapping, and energy input order. Use separate new Rust runner and read-only Python auditor; no changes to the shared kernel or old frozen outcomes. Original unmodified SQLite and JSON must be archived.

## Interpretation

A positive `ambient-general` W3 result shows an explicitly **environment-supplied open resource stream** plus a generic **host-side scheduler** can sustain native operations longer than a draw-gated scheduler. It **does not** imply an actor has autonomously invented energy production, repair, heredity, organization, or life. Further experiments must require internally acquired resource capabilities and endogenous functional recovery after damage, not merely stochastic long execution under an engineered supply law.

**CL-1 remains OPEN unless entirely separate strict life acceptance criteria are met.**
