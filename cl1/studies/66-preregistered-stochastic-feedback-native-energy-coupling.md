# CL-1 Study66 — preregistered coupled stochastic CIL sensor feedback and native compute uptake

**PROSPECTIVE UNEXECUTED REGISTRATION.** Study63 SCRIPTED fixture proved native `Read→Add→Write` plus repeated native `DrawCompute` can work if prewired, not spontaneous life. Study64 randomized CIL with researcher-provided ring and I/O interface produced real W2+W3 source feedback in 5/128 genomes vs0/128 linear; exact independent Study65 replication produced 9/256 vs0/256. Those studies used **300 researcher-provided credits** and contained NO random DrawCompute. Study66 tests whether genuinely randomly sampled feedback and repeated native resource acquisition may coexist under a lower initial credit endowment and externally passive world-pool resource input. Not a life assay.

## Fixed independent population

- **256 new independent seed genomes 4,400,000..4,400,255**, zero outcome filtering/pilots; **eight matched arms per seed**, 2,048 full native runs and 49,152 original SHA256 epoch snapshots, each 24 epochs ×12 charged native attempts. One original process per world, ID1.
- Same unchanged Rust CIL/VM, strict 4 shared cells/32 code nodes/64 total storage. 16 researcher-endowed initial actor credits (vs Study64's 300), no researcher-issued runtime `SetCredits`, no generic host starter or direct process rescue.
- Three **researcher-provided generic interface opcodes** exactly PC0 `Const(r1=8,next1)` (candidate native harvest quantity and generic Add/Sub operand), PC1 `Const(r2=7,next2)` (sensor address), PC2 `Const(r3=9,next3)` (output address). These are not autonomously built infrastructure.
- Randomly generate **24 executable CIL nodes PC3..26** using exactly Study64 64-bit wrapping LCG `state=state×6364136223846793005+1442695040888963407`, one draw per node choice and **one additional draw ONLY for choice4 Const**. Uniform **seven-choice** grammar `random(state)%7`, all generic CIL:
  0 `Read(dst0,address2,next:i+1)`;
  1 `Write(address3,value0,next:i+1)`;
  2 `Add(dst0,lhs0,rhs1,next:i+1)`;
  3 `Sub(dst0,lhs0,rhs1,next:i+1)`;
  4 `Const(dst0,value:random(state)%8,next:i+1)`;
  5 `Branch(test0,nonzero:(i==3?3:i-1),zero:i+1)`;
  6 `DrawCompute(amount1,next:i+1)`.
- Every program ends at PC27 with the same researcher-provided **generic cyclic connector** `Branch(test1,nonzero3,zero3)`. **No** CIL PatchNext, GraphNew/GraphLaunch, native child, prearranged metabolic pathway, Repair, Fitness or Reproduce. No functional-repair or self-construction inference permitted.
- External sensor cell7 initialized to3 or8 before first epoch; external initial pool=48 for `ambient` arms and0 for `closed` arms, explicitly attributed to `External`. For every `ambient` arm the world receives **+8 credits to pool ONLY** at beginning of epochs1..23 via the minimal absolute `SetComputePool` action computed from independently replayed preceding state. This is **EXTERNAL ENVIRONMENT ENERGY**, not native production or host actor aid. No `SetCredits` at any epoch.

## Eight matched conditions

1. `ambient-low` and 2. `ambient-high`: intact seven-choice genome, ambient resource input, sensor cell7=3 or8.
3. `closed-low` and 4. `closed-high`: exact same intact genome and input variants, but **zero world pool** and no replenishment. A native DrawCompute that requests credits without pool legitimately traps/stops (not a success).
5. `ambient-draw-ablated-low` and 6. `ambient-draw-ablated-high`: identical sampled genome choice sequence, but every choice6 DrawCompute replaced with a one-attempt neutral `Const(dst1,value8,next)`; no native energy uptake. Still receives external ambient pool energy, which remains inaccessible via native action.
7. `ambient-read-ablated-low` and 8. `ambient-read-ablated-high`: replace every sampled choice0 Read with `Const(dst0,value0,next)`, leaving DrawCompute and all energy rules unchanged. Source-taint false positive control.

Each seed's eight initial worlds are intentionally different only in the **registered sensor value, pool credits and exact ablated opcode substitutions**, with all random 7-choice nodes and extra Const draws otherwise identical. Denominator is **256 independent genomes**, not 2,048 independent organisms.

## Frozen causal endpoints

**Primary A — coupled late native resource access and recurrent feedback:** a genome qualifies in `ambient-low` and `ambient-high` only if both low/high programs, in each **W2 epoch8..15 and W3 epoch16..23**, successfully perform native `Read(key7)` that taints register0 in **that same window** and successfully `CellWritten(key9)` at an identical original Writer PC with value difference **high−low=5**; in addition the actor must have executed at least one successful internally attributed `DrawCompute` no later than the first qualified W3 output. No `SetCredits` or process-specific external action may occur. Report worlds/256, Wilson95, exact source input and native Draw source tick/credit origin.

**Secondary stricter gate B — recurring resource renewal:** among all 256 genomes, count those qualifying A **AND** successful native `DrawCompute` in **each of W2 and W3** by the same original actor in both input variants, before at least one matching qualified output in that window. A positive is a finite resource-fed native program feedback mechanism, not autonomous organism. Zero is scientifically valid.

**Frozen negative/secondary gates:** `closed` no successful native DrawCompute and no source feedback with native Draw; `ambient-draw-ablated` no native Draw and cannot meet A or B; `ambient-read-ablated` no native CellRead or tainted source-dependent Write, and paired low/high native Write(event PC/value) trajectories must match. Report W1/W2/W3 activity, W2/W3 newly source-tainted outputs, successful native draws by window, trajectories with no runnable process despite positive pool, and original source-origin split. Never count external +8 pool injections as actor credit acquisition.

## Full source-pinned independent audit and constraints

- Two focused synthetic tests: ordinary native `Read→ALU→Write` taint and `DrawCompute` versus external replenishment provenance; no arbitrary suite growth.
- Reconstruct exact fixed seven-choice LCG from every original seed and eight program variants; verify all initial code/data/pool/difference constraints, no program mutation, complete 2,048 matched run population, original initial/final SHA256 and all 49,152 epoch hashes and cumulative native events.
- Independently derive from original per-epoch event ledger: each external action's **exact +8 pool increment** and no actor-target intervention; each successful native `DrawCompute(amount8)` with `Origin::Internal(1)` transfers 8 pool→actor and spends one charged attempt, with `pool_after=initial_pool+8×ambient_events−8×native_draw_successes`, `actor_credits=16+8×native_draw_successes−cumulative_attempts`; preserve exact finite compute inventory. A trapped Draw is a charge with zero transfer.
- Validate actual native PC and register0 def/use: `Const(r0)` clears source taint, `Read(r0)` sets it, Add/Sub r0±8 preserve it, Branch/Write leave it unchanged. Match per-seed source-high/low Writer PC in both late windows and new Read same window. No matched seed outcome can be manufactured by an initial external SetCell alone.

## Scientific decision

This is the next *minimal coupled process* after the replicated Study65 feedback result and scripted Study63 harvesting feasibility. If the source and pool channels coexist under random CIL, a subsequent independently preregistered study must replace the externally provided cyclic connector/interface with **natively assembled graph-local code/data organization**, then later evaluate causal damage/repair. If no joint native source+compute loop occurs, do not tune lucky seed weights; diagnose exposure and resource topology with frozen original archives before any new independent cohort. Neither positive A nor B is self-maintaining life because the ring connector, sensor interface, initial energy and externally replenished pool are engineered.

**CL-1 remains OPEN.**
