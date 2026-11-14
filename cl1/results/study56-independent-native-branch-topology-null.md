# CL-1 Study56 — independent prospective replication: no causal control-flow topology uplift

**Status: CI-verified prospective directional replication FAILURE, functional child still absent; no life.**

## Frozen source and full independent verification

- Study56 prospective protocol was committed at `3cc5a58e4f2aceb70d70a4250e5326f9e967cd4c` **after Study55's weak 2/0 causal direction was known but before any new Study56 code or sampling**.
- Study56 sampled **256 completely fresh independent paired seeds3,900,000..3,900,255**, with the exact unchanged Study55 two branch-backedge regimes `local index-2` and `uniform random earlier node`, all original 24-way opcode choices preserved by identical extra LCG draws, four energy/scheduler matched conditions per regime, 24 epochs×12 charged native attempts.
- Executed source revision `8983cd1e402ab181398ba3d13a58065e8d7a426e`; [GitHub Actions CI #43560259239](https://github.com/iFrankStudio/cioc/actions/runs/43560259239) **success**, artifact `cl1-study56-paired-native-branch-topology` ID `14100375019` (both original untouched SQLite archives plus independent paired report).
- **2,048 matched runs, 49,152 original SHA256-verified and exact native-replayed epochs**. Rust tests/strict lint, both independent stochastic grammar checks, exact permissible code differences, external energy/starter actions, graph/child native event provenance and 1-credit-per-attempt inventory audit all passed. Every one of the 256 matched seed pairs was included.

## Preregistered primary comparison against Study55 discovery

| Native `ambient-general` endpoint | Study55 (256 paired seeds) | Study56 independent replication (256 paired seeds) |
|---|---:|---:|
| Strict native same-parent nonzero source node copy→same-graph edit→launch, LOCAL branch topology | **0/256** | **3/256** |
| Same strict source-path, UNIFORM earlier-node branch topology | **2/256** | **3/256** |
| Paired uniform-only versus local-only | **2 vs0** | **0 vs0** |
| Paired both / neither | 0 /254 | **3 /253** |
| Native functional child `CellWritten` | 0 vs0 | **0 vs0** |

**The prospectively frozen directional replication criterion FAILED:** uniform-only0 is not greater than local-only0. In Study56 every strict source path appeared **in both code topologies for the same three seeds**. It is therefore scientifically incorrect to present Study55's 2 vs0 as a robust causal improvement or to retrospectively pool seed counts into a positive finding. The difference may be sampling fluctuation or occur through paths unrelated to modified Branch choices; that is a separate retrospective causal-exposure question, not a new positive endpoint.

In Study56 **139/256** worlds in *each* topology launched children, yet no child had an executable Write or successful native `CellWritten`. Native-launched child contained a non-GraphNew executable opcode in **4/256 local** and **7/256 uniform** worlds, respectively; those are mechanistic post-secondary counts, not functional life. Frozen initial `Write` source node copied as part of a strict path: **0/256** in both regimes.

## Scientific decision

Study55 and Study56 collectively demonstrate that stochastic native CIL control-flow/backedge changes can alter individual operation histories, but **a prospective increase in strict native copy→edit→launch probability has NOT been replicated**, and neither branch topology yields functional native child shared output. Do not continue indiscriminate seed proliferation or make `Branch.nonzero` a claimed life mechanism.

The next high-value investigation is to **read-only audit strict path temporal exposure to actually executed branch targets**, rather than assuming differing static program graphs necessarily caused the rare native ordering. A mechanism that changes no executed native branch is not an effective causal treatment for that seed-world. Subsequent ecology design should consider generic graph-local portability and temporal reuse with explicitly measured exposure, not cherry-pick successful initial programs.

**CL-1 remains OPEN.**
