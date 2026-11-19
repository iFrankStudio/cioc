# CL-1B G3Q-C1 — frozen C0 native-source capacity preflight

**Status: STATIC PRE-FLIGHT ONLY, not native C1 success.** This is a prospectively reproducible source-graph inspection of the *already verified* G3Q-C0 164-node frozen founder. The proposed C1 cannot silently outsource certification, inject actor data, or enlarge the existing physics.

## Exact measurements

The frozen C0 graph contains **164/180** code nodes, **164 CFG-reachable** nodes, **15 branch nodes**, and a final Halt at PC163 entered from PC154. All **16/16** allowable data addresses are used: `7,11,20,21,32,33,50,51,52,53,56,61,62,63,64,65`. Original recorded peak occupancy was **181/300** (164 source + PID1 + 16 cells). Adding 16 source nodes without allocating new data would yield at most **197** occupied units; thus **source-node and data-address constraints, not total occupied storage, are the immediate bottlenecks**.

A straightforward bank-to-active copy of three descriptor words using four nodes per word (`Const(source-address), Read, Const(dest-address), Write`) consumes **12 of 16** spare source nodes, leaving only **4**. This is a *particular simple copying construction*, not a proof that 12 is the global ISA minimum, nor a proof that all C1 designs are impossible. Four nodes do not give us a demonstrated paid task-bytecode evaluator, input-output service, scored two-probe feedback, hidden regime shift or distinct certificate/service provenance. Hence simply appending tasks to the C0 founder would not constitute the promised C1.

## Constructive direction for an exact C1 founder

- Keep the strict `World::with_strict_capacity(16,180,300)` physics; one PID1 remains uninterrupted throughout the **new** C1 world. This will be a **fresh researcher-authored source lineage**, not mutation of C0's original source.
- Reduce full 64-descriptor enumeration to an investigator-preregistered **targeted accepted/rejected set**, but require the actor's own CIL program to **compute** certificate flags and to pay to retain the accepted descriptor. The harness must never inject validity decisions.
- After certificate completion, overwrite genuinely dead per-trial cells, not still-live bank slots. Establish an exact staged address table (generation/certification -> bank retention -> deployment -> task evaluation -> feedback -> heldout service) and a control-flow proof that no overwritten cell is subsequently read as its earlier role.
- Freeze a complete instruction graph **before** any original Rust-native test, including every branch target and register/address liveness, then fix exact credits, two seeds, task schedule, feedback regimes, and source-identical controls; record all prior failed attempts.
- Preserve two different provenance-tagged meanings of `Write11`: C0 structural verdict versus C1 actual task-service. If only a certification verdict executes, C1 does **not** pass.
- Distinguish **earned paid rule deployment** from **useful task-conditioned learning**. A fixed safe bank already matches full-coverage task success, so real feedback-dependent choice and cost improvements need their own falsifying comparisons.

## Feasibility gate

A valid C1 implementation must demonstrate actor-paid source-native Reads of retained bank values, actor-paid Writes into active interpreter slots, subsequent instruction fetch, actual task service, verified task feedback and source-identical controls. If the exact 180-node program cannot contain that sequence, report the capacity-limited attempt as a negative finding. Do not claim original PID1 learning from a host-side oracle or source-rewritten C0.

Reproduce the bounded static check:

`node cl1b/analysis/b56_f1d1k_g3q_c1_source_capacity_preflight.mjs`

The checker asserts the frozen C0 source length, CFG, permitted cells and measured C0 peak. **No native C1 run, replay evidence, new CI pass or cyberlife result is claimed.**
