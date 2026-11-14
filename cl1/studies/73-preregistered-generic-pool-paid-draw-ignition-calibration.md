# CL-1 Study73 — preregistered generic pool-paid native DrawCompute ignition feasibility

**Status: PROSPECTIVE MECHANISM CALIBRATION; zero ecological worlds.** This is a substrate-physics intervention written by the researcher, **not** a programmed organism, evolutionary change, endogenous energy creation or native self-repair. Register before kernel modification.

## Motivation and known limitations

Study36 showed 138/192 closed worlds with positive shared compute pool but every unstopped actor at zero local credits. Study67 found 207/256 Study66 ambient worlds similarly stranded despite later pool input; 49/256 had irrecoverably trapped attempting to draw from a temporarily empty pool. Study63 *human-scripted* CIL repeatedly exercised native DrawCompute + information-sensitive Read->Write under initial local credits; 0-credit scripted control remained completely silent despite a full pool. Study71 independently verified non-prewired stochastic Read→Write recurrence in 63/256 broad-backedge genomes, but with **300 externally supplied initial actor credits** and no renewable compute coupling. Therefore, the next scientifically necessary ability is a **generic physically defined pool-paid entry into a native resource-acquisition instruction**, not a fitness-selected host process restart.

## Exact opt-in kernel change

Add a new **opt-in, disabled-by-default** world-policy bit `pool_paid_draw`, initial false. Preserve previous 0..30 opcode ABI and all semantics for default worlds. Under the bit:
- The deterministic scheduler considers a zero-credit, unstopped actor **only if its current valid PC holds exactly `Op::DrawCompute { amount, next }` and the shared pool has at least one credit**. It never selects an arbitrary zero-credit actor, performs a goal/fitness check or revives a stopped actor. Keep existing ProcessId cursor ordering among eligible actors.
- When this actor is actually selected, debit **exactly one compute credit from the shared pool** as its attempt cost, rather than subtracting from its zero local credits. Then execute the native CIL `DrawCompute`, which attempts to transfer the register-selected amount from the **remaining** shared pool into the process, with unchanged edge/operand validation. A failed native opcode still traps the actor and incurs the cost; insufficient pool must never mint credits, forgive a trap or run an unrelated opcode.
- A positive-credit actor always pays its charged attempt from its **own** process credits under ordinary old rules, even if the next opcode is `DrawCompute`. Nothing else is eligible at zero local credits.
- The exported epoch/event observer must identify which attempts were pool-paid, either by an explicitly serialized default-suppressed native `Event` field or by a rigorously source-pinned provenance log. **Never label the pool-paid cost as autonomous energy generation.** No new "repair", "fitness", "reproduce" or process-saving operation.
- All charged attempt energy is conserved: `total inventory at end = initial total + external ambient pool input - total native attempted steps`. The pool debit pays the attempt, then native Draw can move credits from pool to actor but not increase total.

## Frozen human-authored calibration (no stochastic population)

Test at most six fixtures, all with exact original CIL code, source pin, independently checked event/credit ledger and 24 epochs:
1. Zero actor credits, positive pool, `DrawCompute(amount=register1)` at PC0 and valid `Const/Read/Write` later: opt-in on → successful native Draw and subsequent ordinary native code execution; pool-paid cost exactly1.
2. Identical original world with opt-in off → no native attempt, even with positive pool.
3. Opt-in on, zero local credit, next opcode NOT Draw → no native attempt.
4. Opt-in on, zero local credit, correct Draw but pool0 → no native attempt.
5. Opt-in on, zero local credit, Draw operand demands more credits than pool after paying entry cost → one paid native attempted Draw, actor traps and is never revived; loss exactly1.
6. Opt-in on, stopped actor has pending Draw and positive pool → no native attempt.

Include normal positive-credit actor Draw as an old-compatibility assertion. Confirm no after-start intervention SetCredits, no host goal-directed action, exact native origin and pool cost marker. The test must fail if an unrelated zero-credit instruction executes, if total credits rise, or if an earlier successful native program is mislabelled as life.

## Interpretation and sequencing

Passing this calibration only shows that an opt-in environmental resource-access law can remove the particular **ignition deadlock for actors already attempting native Draw**. It does not show that a random agent chooses appropriate amounts, avoids future traps, finds a functional feedback loop or maintains its organization. Next register a **fresh** 256-genome stochastic ecology comparing this bit OFF/ON under equal ambient energy and *no host actor-specific starter*, requiring both real source-sensitive feedback and continuing successful native Draw in later windows; include no-pool, Draw-ablation and non-read controls. Do not reuse Study71 positive seeds to manufacture survival.

**CL-1 remains OPEN.**
