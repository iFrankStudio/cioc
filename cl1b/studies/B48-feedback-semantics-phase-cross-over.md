# B48 — Feedback semantics × execution phase cross-over (prospective registration)

Status: **PREREGISTERED DESIGN ONLY**. No B48 native result or independent dynamic verification exists at registration. B47's native negative result remains unchanged. B3/B4 NOT MET.

## Question and falsifiable hypothesis

B47's feedback actor beats blind feedback but loses to compact static code, while its supposed phase control has an entirely different architecture. Can *truthful task-feedback semantics*, rather than instruction phase, graph shape, scoring exclusions or realized exposure, explain a reproducible improvement **within the same mutable CIL architecture**?

H48 (mechanistic, **not** B3 acceptance): for each of two independently frozen holdout regime families, truthful feedback yields more **correct minus wrong** native service writes per common 144 attempted-instruction budget than both a same-opcode-cost semantically ablated feedback source and a feedback-source-permuted sham; the direction survives at least two deterministic starting phase offsets, and both arms have equal charged attempts. Any failure falsifies the full conjunctive H48. No claim of superiority to a compact immutable copier is implied.

## Frozen experimental arms

Use B42 founder ProbePlus as the source template, not a new host reward. A: genuine B42 node12 Sub{dst:1,lhs:0,rhs:1}. B: replace only node12 with Sub{dst:1,lhs:0,rhs:0} (same opcode class, next, cost, node count; source-blind). C: replace only node12 with Sub{dst:1,lhs:0,rhs:3} (same opcode class, next, cost, node count; register3 is overwritten to zero by node8 before evaluation, so this is a distinct but potentially correlated blind sham). D: immutable compact copier (engineering efficiency ceiling, excluded from H48). E: original A with native edits disabled (negative control; report early termination, never compare as equal realized exposure). A/B/C all allow identical ordinary CloneNode/PatchNext operations and charge their attempts.

**Phase cross-over:** duplicate each A/B/C arm with an extra one-instruction paid Const no-op before the first meaningful CIL instruction; preserve graph addresses by appending a new node and initializing the process at that node if the API permits. If the VM cannot support identical code plus a one-step phase offset without altering node IDs or feedback semantics, explicitly record phase intervention as infeasible and do not claim phase invariance. Alternative: one paid ordinary preamble with all branch targets mechanically shifted, validated by graph isomorphism; never compare an immutable four-node copier as the phase-matched sham.

Freeze the eight B47 sequences as replication block and eight new sequences as prospective unseen block: [23,0,-1],[-9,17,23],[1,-9,0],[0,23,17],[17,-9,-9],[-1,1,23],[23,23,0],[-9,0,1]. No dropping stopped or failed worlds.

## Primary measurement and disambiguation

Run 48 charged steps per regime, three regimes per world, same input transitions, capacities and 144 initial local compute per arm. Score every native CellWritten{key:11,value} at the actual Write event against the actual then-current cell7 value, **without the B47 restored/phase>0 exclusion**. Separately report post-restoration qualified writes, diagnostic writes, wrong writes, edits, suppression, traps, stopped worlds, total paid steps, remaining inventory and peak storage. Preserve raw trace identifiers (PC, source origin, phase, input, output), so a reader can reconstruct every classification. Track whether source reads or writes cell7 as part of a challenge, because input can be temporarily modified by the actor.

Freeze analysis before native CI: compare A–B and A–C within each seed/phase pair; compute aggregate net correctness and paired win/loss/tie counts for both holdout windows, and report per-144-budget throughput and per-actual-attempt throughput separately. Early stop is a scientific outcome, not a zero imputed at equal exposure.

## Independent observer and acceptance

A separate JS or Python interpreter must execute the subset of CIL opcodes used by B42, including dynamic CloneNode/PatchNext and ordinary writes, from source-parsed graph instructions. Compare native per-event trace with interpreter on identical seeds and phase offsets, including first divergence. Source-pattern assertions alone are insufficient. Verify Rust native run, observer, ABI checks and evidence against the **same executable SHA**; archive failed attempts. Changed-only is not a full-regression claim.

## Frozen directional predictions (not observations)

P48.1: A is likely to outperform blind B/C on some regimes, but this need not survive both unseen holdouts and phase offsets. P48.2: A will continue to lose raw correct writes per budget to D because its monitoring/construction consumes instructions. P48.3: raw versus qualified scoring will diverge around the actor's own diagnostic input mutation. P48.4: C may behave identically to B in the tested initial register conditions, exposing an insufficiently independent sham rather than independent replication.

**Interpretation boundary:** even a confirmed H48 is evidence for an engineered causal feedback mechanism, not self-generated objective, B3/B4 acceptance, open-ended evolution, or life.
