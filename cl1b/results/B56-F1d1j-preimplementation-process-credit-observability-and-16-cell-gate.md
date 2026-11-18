# B56-F1d1j preimplementation credit-observability and rule-storage gate — negative source audit

**Evidence class: exact-blob manual source inspection, NOT a native F1d1j experiment, NOT a machine-checked reachability theorem.** F1d1i source-pinned native experiment [run44301654974](https://github.com/iFrankStudio/cioc/actions/runs/44301654974) is complete; F1d1j mutation-rule adaptation is **not implemented nor tested**. This is a research feasibility gate for the already committed [prospective F1d1j protocol](../studies/B56-F1d1j-actor-paid-mutable-mutation-rule-and-credit-normalized-retention-preregistration.md).

## Source identity and concrete observations

Read unchanged `cl0/crates/world/src/lib.rs` with Git blob SHA `4d0a1e7b86c0bbae5c11724f4292252ee0993ff0`. The complete current `Op` enum includes ordinary `Read`, `ReadLastWriter`, `SenseComputePool`, `DrawCompute`, `TryDrawCompute`, `ReturnCompute`, and `TransferCompute`, **but contains no `SenseOwnCredits` or other direct native instruction to read the currently executing process's remaining `p.credits`**.

Verified dispatch semantics in the exact kernel:
- `Op::SenseComputePool { dst, next }` reads `observed_compute_pool`, NOT `p.credits`. This observation is a neutral shared pool, which is zero/unused in the strict one-PID F1d1i world and cannot be repurposed as an actor's own compute accounting without changing the experiment.
- `Op::Read { dst, address, next }` reads `cells.get(&key).copied().unwrap_or(0)`, i.e. actor/environment shared cell memory, **not** process credits. Unless explicitly paid by actor, an instruction counter placed in shared cells has no automatic relationship to charged CIL steps. A host-injected credit counter would violate actor-origin constraints.
- `World::step` deducts `p.credits -= 1` before every process-paid attempted instruction (including trapped operations), in the strict no-pool configuration. The original F1d1i observer independently verified total charge=original instruction count, but this is **observer-visible cost**, not necessarily **PID1-accessible state**.

**Bounded conclusion:** under this current source and an unchanged kernel/no-pool world, F1d1j cannot simply implement exact PID1-local credit-based online cost decisions by calling an existing direct process-credit sensor. Research can still choose (1) an actor-maintained, actor-paid **estimated-cost** counter (with exact included counter maintenance costs and loss of precision disclosed), or (2) a re-preregistered versioned VM with a **new paid own-credit observation primitive**, clearly a different world specification, not an unchanged-world source-origin success. The pre-registered F1d1j path prefers (1) for direct continuity; it must NOT claim exact self-observed native credit balance.

## Simultaneous storage and source-code budget

F1d1i exact native source was157/180 nodes and used exactly16 address keys under max16: `7,11,20,21,31,32,33,50,51,52,53,54,56,61,62,63`. F1d1i native peak total storage was174/300. A durable mutation rule needs **paid persistent data** not overwritten by a virtual-operation accumulator or external feedback. There is **no free seventeenth cell** or more than23 existing-node headroom if simply appending to the frozen F1d1i source.

A proposed reuse of key21 (environment-provided edit gate) for a mutable rule descriptor is **unsafe by default**: if PID1 can change its value0→1/2, it may disable the suppression control and falsely count the failure of the ablation as endogenous capability. Any valid implementation must make the source-level permission bit immutable or separately protected, and source-identically preserve gate0 suppression through the entire run. Data-key repacking, paid deletion/reuse or source simplification must be demonstrated before native program results.

## Falsification plan and result classification

Before native F1d1j claims, commit an exact native 180-node/16-cell layout and source-derived independent liveness/capacity checker, together with valid source-level gate0 behavior, and report the difference between **external real credits consumed** and **agent-maintained observable estimated cost**. Require strict payment for every counter update, mutation-rule change, next program-slot change and fallback. An inability to retain rule descriptor or affordable counter under these caps is a legitimate **negative capacity result**, not grounds for silently extending the world.

**This source audit does NOT demonstrate self-modifying mutation rules, de-novo algorithm invention, N56-A/B/C, B3/B4, or cyberlife.**
