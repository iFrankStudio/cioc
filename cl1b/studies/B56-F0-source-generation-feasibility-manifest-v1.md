# B56-F0 — executable feasibility and provenance freeze (prospective; no native result)

Frozen 2026-11-17 before any B56 native run. Parent: `B56-source-generated-monitoring-topology-and-meta-policy-provenance-preregistration.md`. This is a **feasibility falsifier**, not an assertion of B3 or novelty.

## Hypothesis and predictions

H56-F0: a single continuous PID1 can build at least one **reachable executable monitoring control graph** from individually inert generic CIL instruction templates, within strict fixed capacity, without a complete preseeded NEVER/EVERY/FOURTH monitoring subgraph. A positive result requires charged actor-origin CloneNode/PatchNext/PatchConst events and a before/after graph witness. Prediction P1: transplanting B55b's 178-node founder unchanged fails the novelty gate even if it fits 180 nodes; P2: merely changing node2 to route to dormant complete policy fails; P3: clone/edit disabled cannot produce a new reachable control graph. These are prospective predictions, **not native measurements**.

## Locked feasibility envelope

- One PID1; no replacement, no compute top-up, no host source patches after start.
- Capacity: `World::with_strict_capacity(16, 180, 300)`, initial compute 4,000 charged instructions; no budget extensions in this F0 run.
- Deterministic paired identifiers: 101, 211, 307, 401; two task schedules A `x-2 → -x+3 → x-2` and B `-x-3 → x+2 → -x-3`, alternating identifier parity. No task label to PID1.
- Trial inputs `[5,-3]`, heldout `[11,-7,0]`, 24 scored service opportunities per stage, three stages. Missing outputs count as failures; no exclusion of early-halting worlds.
- Generic source grammar restricted to existing CIL CloneNode, ReadNext, PatchNext, PatchConst, Add, Sub, Branch, Read, Write and ordinary control flow. No new privileged opcode.
- The founder must be newly inventoried, source-hashed and shown to contain **no dormant complete monitoring-policy subgraph**; the B55b 178-node founder is explicitly ineligible. Individual inert instruction templates are allowed; their identities, initial edges, constants and reachability must be frozen with the actual native source before execution. No claim that a valid founder has yet been built.

## Strong controls and decision rules

F0-A: actor assembly with paid feedback. F0-B: identical founder and compute/storage but blind constant success feedback. F0-C: identical founder with native CloneNode/PatchNext/PatchConst disabled (attempts still charged or counted as failed per documented VM semantics). F0-D: identical founder, no assembly route. F0-E: static EVERY and FOURTH, separately tagged **preauthored baselines** under identical capacity/compute; never label them generated.

The Rust experiment must export pre/post full code graphs, charged instruction trace, node allocations, changed edges/constants, origin PID and cost, plus first reachability of each generated controller. Independent JS must reconstruct every paid step without using native next-state as input. An independent graph auditor must reject structurally equivalent rerouting to a dormant completed policy. Count source edit attempts separately from actual graph changes, including no-ops.

F0 passes **mechanical feasibility only** if A produces a new reachable functional monitor absent in the inventoried founder, with every constituent edit actor-paid, while C/D cannot; B is not required to lose for F0. If no new topology fits, record feasibility rejection under 180/300/4000, not a post-hoc budget increase. N56-B and N56-C require **separate** later experiments: paired useful-throughput superiority over best fixed baseline, then paid edits to the generator itself after a hidden change. F0 success alone is authored assembly, **not B3/B4/cyberlife**.

## Execution gate

This file freezes a proposed protocol, **not executable source or measured results**. Next commit must supply the source-inventoried founder and a Rust capacity/load test; a native CI run and independently replayed evidence must be source-SHA-pinned before any positive result. Do not replace the frozen B55b negative utility result with B56 predictions.
