# CL-1 Study 04 — preregistered open-resource information ecology

**Registered:** 2026-11-13. Frozen before implementation or any Study-04 trial. Independent seeds 940000..940127, not reused from Studies 01–03. Results of previous studies (including Study-03 zero structural restorations) were known at registration and must not be used for selecting Study-04 outcomes.

## Why a separate study
Under only 3 × 28 = 84 initially supplied CIL credits and no environmental compute, all code execution must eventually cease. A finite-credit closed baseline cannot fairly test **sustained** activity across many observation windows. Study 04 introduces a **generic externally replenished compute pool**—not a biological fitness or recovery oracle—whose supply is logged as an external intervention. Running longer after this intervention is evidence of access to externally provided computation, not of spontaneous self-maintenance.

## Fixed initialization and instruction grammar
- 128 independent LCG seeds, 940000 through 940127 inclusive.
- Three process IDs 1–3, 16 preauthored nodes each (four constants, eleven generated instructions and Halt), 28 compute credits each. Shared cells ≤32, max nodes/process 32, strict storage budget 160.
- Four initial registers: r0 shared-cell address `random %5`, r1 signed value `random %101`, r2 requested compute amount `2+(random%3)`, r3 arbitrary process id `1+(random%3)`. External randomized values are **not** internal discovery.
- For each node 4–14, a uniform 12-way LCG choice among Add, Sub, Write, Read, backward/forward Branch, DrawCompute, ReturnCompute, TransferCompute, GraphNew, GraphCopyNode, GraphLaunch, and CloneNode. Use valid register indices and sequential or bounded backward graph targets. No host checker selects better programs.
- Note: GraphNew/GraphCopyNode/GraphLaunch operate on general graph objects and may fail; random combinations that trap remain in the denominator.

## Matched conditions and supply schedule
Fixed horizon: 512 attempted CIL instructions, each branch starts from an identical saved initial snapshot.

Five conditions per seed:
1. `closed`: no world compute-pool supply.
2. `initial-pool`: externally `SetComputePool{credits:256}` at offset zero, no subsequent replenishment.
3. `replenished`: externally set pool to 256 at offsets 0, 128, 256, 384 **only if the world can reach that number of CIL attempts**. Setting a pool to 256 is an external reset, which can add *or remove* remaining pool balance. It is never internally created energy.
4. `disable-p1`: same pool256 offset zero and external `Disable{process:1}` at offset 32.
5. `zero-credit-pool`: pool256 at offset zero plus external zeroing of all three process credits at offset zero. Pool access requires executable instructions; zeroed CIL cannot draw energy by itself.

128 × five = **640** SQLite matched runs, all exact-replayed and hashed, plus independently audited schedule checkpoints.

## Prespecified measurements
Per independent seed and condition:
- Total internal attempted instructions and resource occupancy; number of traps and host-supplied pools.
- `beyond_initial_budget`: >84 attempted instructions. In `closed`, this is a **hard invalidation** (unless an unexpected internal compute creation bug exists); in pooled cases, it is a *generic resource-acquisition proxy*, not self-maintenance.
- `late_256`: >256 attempted instructions; `late_384`: >384. A run must actually reach these instruction counts; if not, it remains a zero.
- `created_process`: a new process ID >3 appears internally; code construction/launch does not imply reproduction.
- `occupied_node_growth`: final code nodes + graph object nodes >48; a structural variation proxy only.
- `response_to_disable`: paired final *non-target processes* state differs between `initial-pool` and `disable-p1`; this is a causal sensitivity candidate, **not** reciprocity or organizational closure.
- `source_transparency`: every compute-pool replenishment appears as `Origin::External`; no additional energy is credited to endogenous creation.
- Report all counts over 128 independent worlds and Wilson two-sided 95% intervals for each binary proxy. Never use 640 runs as the binomial denominator.

## Controls, integrity and epistemic rules
- Every world must respect total abstract storage ≤160 and exact SQLite replay; every seed must have five byte-identical initial hashes, with no dropped runs.
- `closed` attempts ≤84, `zero-credit-pool` 0, scheduled events only at reachable offsets.
- If the world becomes inert before a scheduled replenishment, **no** replenishment occurs because scheduling counts internal attempts, not external wall time. This is an explicit feature and a limitation; Study 04 does not model a physical independent clock.
- Any long-lived process must still be classified as **finite resource-supported computation**, never intrinsically alive.
- Zero or very rare long activity is a legitimate negative result; the experiment does not tune seeds or mutate the initial generator in response.

## Status and scope
Study 04 tests whether the CL-0 world can support extended generic activity **when external compute exists**, and exposes the difference between finite-resource exhaustion and organization. It does **not** by itself operationalize a full self-maintained organizational boundary, intergenerational novelty or emergence of life. Such positive claims require new preregistered structural/functional experiments.
