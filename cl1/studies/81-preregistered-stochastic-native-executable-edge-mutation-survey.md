# CL-1 Study81 — preregistered stochastic executable-edge mutation survey

**Status: PROTOCOL BEFORE STUDY81 ECOLOGY; ZERO Study81 stochastic runs at registration.**
Study80 Phase-0 only established with ten investigator-authored controls that the native VM can edit a damaged functional edge under credible mutation-disabled and host-repair controls (successful focused CI run 43591338076). Study81 addresses the next weaker but necessary prerequisite: do independently seeded, stochastic CIL programs ever execute **effective native edits to their own executable code**, and are those edits causally associated with later source-sensitive output? **No lesion/repair claims are permitted in Study81**, irrespective of results.

## Frozen prospective population and original program grammar

- Exactly 128 untouched seed labels 5,400,000 through 5,400,127, mixed using the same documented SplitMix64 finalizer / 64-bit LCG sampling as Study77; no filter or resampling after outcomes.
- For each seed, identical six-independent world laws are NOT used: instead the strict four registered matched arms are source low3/high8 × native executable edits enabled/disabled. Original CIL graph bytes and initial actor credits must be byte-identical across all four arms; only external source input and explicit disabled-by-default edit-ablation law differ.
- Exactly 28 program nodes: first three researcher-authored setup instructions, followed by 24 seeded stochastic operations at PCs3..26, then Halt at PC27. Setup: R1=8; R2=7 (source cell key); R3=9 (output cell key). Start R0=0. Each of the 24 random nodes independently selects one of eight 64-bit modulo operation bins (study-specific deterministic pseudorandom corpus, **not** asserted statistically independent across all seeds):
  0 native Read(R0, cell[R2]);
  1 native Write(cell[R3], R0);
  2 native Add(R0,R0,R1);
  3 native Sub(R0,R0,R1);
  4 Const(R0, random %8);
  5 Branch on R0, backedge target selected by modulo over *earlier* random-body code indices PCs3..(current−1), with PC3 targeting itself (the exact pre-existing Study77 broad-target implementation; not a claim of uniform unbiased samples), zero edge PC+1;
  6 DrawCompute(R1);
  7 native PatchNext(node=R1,target=R0,next=PC+1), targeting the executable code node PC8 and an actor-computed edge (may be invalid and trap in the ordinary VM). No other native executable edit operations in this grammar.
- No post-outcome edit to this grammar, seeds, scheduler, storage, pool rule, epoch count or measured event definition. This is an **operation-rich environment with a hardcoded edit affordance**, and so a positive event is not spontaneous goal formation or living repair.

## Frozen environment and matched worlds

- World strict capacity: max shared cells4; max process nodes32; max total allocated units64; load one 28-node Process id1 with 64 initial native credits; opt-in `pool_paid_draw` and `recoverable_draw_shortage` as Study77. Researcher-set shared source key7=3 or8; common pool starts with 96 credits.
- Run **24 epochs ×12 charged native instruction opportunities** each. For epochs 1..23, externally add exactly8 to the environment compute pool (absolute SetComputePool using previous simulated pool accounting). Every arm receives the same ambient *law*, not targeted process rescue; keep full source provenance.
- The native-edits-disabled control uses the identical loaded executable graph; a valid PatchNext still charges its single attempt and moves along its next edge but does not alter code, and invalid edits still trap under the original VM. Interception is explicitly human-designed environmental law and source-audited as such.
- Optional **observer-only native edit trace** may emit `ExecutableEdgePatched {node,before,after}` only on a successful actual edge change at a native PatchNext actor PC; never grants energy, changes actor registers, alters code or becomes visible to CIL. It is opt-in in all four arms and exact epoch-state/credit audited.

## Outcomes, all denominators and paired causal interpretation

1. **Native encounter:** fraction of original seeds with a charged native PatchNext attempt before trap, separately in each arm.
2. **Effective executable edit (primary):** for each seed/source arm, at least one **native-actor** `ExecutableEdgePatched` event with before!=after, matching real opcode PC, an exact actual code change at node8, valid actor-computed target and no external code intervention. Report edit-positive seeds/128 for low and high separately and paired source discordance; report all 512 world runs.
3. **Mutation-only difference (secondary):** matched genotype+same source edits-enabled versus edits-disabled first exact divergent event/PC, native code graph state and subsequent native CellWritten at output9. Report all seeds with *any* effective edit, matched controls and output delta; do **not** require convergence or normalise away traps.
4. **Source-sensitive output W2 (strict):** same seed native CellRead(key7) before successful CellWritten(key9) in both low/high arms, same actor and writer PC, with paired output high-low exactly5, not merely static sensor endpoints. Predeclare native event def-use constraints; an observed output value alone does not qualify.
5. **Cross-mechanism coexistence (exploratory):** strict W2 and at least one genuine preceding native executable edit and at least one successful native DrawCompute, with total actor charged attempts >64 before the qualifying output. Use exact writer/source ancestry; no statistical life claims.
6. **No lesion, no repair gate:** an effective code edit without external lesion and later functional restoration CANNOT be counted as self-repair, organizational closure or CL-1 acceptance. Study80 Phase-1 targeted lesion intervention may be attempted only under a distinct complete preregistered ecology with fixed pretreatment exposure and target selection.

Every world and seed enters all denominator counts even if it traps, never reaches PatchNext, has no function-bearing output or produces no native edit. Exact zero rates are meaningful negative evidence for this specific deterministic seeded grammar, **not** impossibility results for all CIL.

## Reproducibility and stopping

Source-commit the full generator before running the first seed, record unique initial genotype hashes and all original SQLite epoch JSON+SHA256 snapshots, target and branch PRNG provenance, per-actor native events, charged attempt counts, compute inventory, exact replay; independently audit every original snapshot and raw native edit provenance with Python. Report paired discordants and avoid treating 4 correlated arm worlds as 4 independent genotypes. Execute only dedicated scoped Study81 CI and never use autoresearch / external LLM research workflow. If full replay/audit fails, mark unverified and correct mechanisms transparently before concluding. **CL-1 remains OPEN**.
