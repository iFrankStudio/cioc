# CL-1 Study44 — prospective child-shared-output null under generic random source-index diversity

**Status: prospective CI-verified native-child shared-output NULL. Not life.**

Study44 was preregistered at `16f41c1d309b7511204a5427d95f65769b65f4c3` before its runner or seed outcomes. Its *one* randomized grammar modification compared with Study41 was to draw an additional uniform 0..35 source-node index into register2 when continuation choice8 was reached, rather than hardcoding zero; no host-defined repair, reproduction or favorable output. Seeds **3,300,000..3,300,127**, four matched arms, 24 epochs, 12 native quota, 512 matched runs, **12,288 SHA256 verified SQLite snapshots**, unchanged Rust world kernel and source-checked external resource/neutral ignition policy.

The original executed revision `1f83c2225f4f7ac681b957256b596b0683659520` passed independent GitHub CI [43554614365](https://github.com/iFrankStudio/cioc/actions/runs/43554614365). Preserved artifact `cl1-study44-native-code-plasticity`, ID `14097692596`, includes complete SQLite and independent auditor report.

## Frozen prospectively registered result

- **Primary child native shared-cell `CellWritten`: 0/128** in `ambient-general` (Wilson 95% [0, 0.029137]). All writes must have a successful internally sourced child actor, exact earlier native GraphLaunched lineage and an executable `Write` at its observed program counter. No artificially injected child or external write qualifies.
- Worlds with internally attributed child launches **61/128**, total **74** native launch events.
- Worlds with any successful native child operation **17/128**, all child successful events were **309** `GraphCreated`, with **zero successful child non-`GraphCreated` instructions**.
- Initial actor native executable self-code change **69/128** (separate secondary), final code changed **80/128**, mutated actor subsequently successfully worked **40/128**, native parent shared-cell Write after mutation **10/128**.
- W3 charged native activity **50/128** under externally supplied ambient energy plus generic environment starter; no evidence of child activity for two distinct W3 epochs without same-epoch host starter.

## Explicitly post-result mechanistic inspection (NOT primary evidence)

An independent local read-only examination of the original frozen SQLite found **74** launched children, of which **73** had exclusively `GraphNew` executable nodes. Exactly **one** child had a non-`GraphNew` instruction:

- Seed **3,300,121**, `ambient-general`, child process4, launched by parent1 at tick39 with graph1 (2 nodes).
- Its transferred program: node0 `GraphNew {dst:3,next:1}`; node1 `Write {address:0,value:3,next:12}`.
- Child successfully ran node0 at tick40, then **Trapped("invalid edge")** at tick43 on node1, because the copied Write retained its original *parent-program absolute edge 12* whereas the child graph had only 2 executable nodes.

This is a **post-hoc diagnostic witness**, not a preregistered explanation or proof that every possible child Write is impossible. It identifies a concrete graph portability and control-flow closure bottleneck. Merely selecting random source code nodes is insufficient to make them runnable after moving to a shorter graph.

## Scientific next gate

A separately registered retrospective **control-flow topology audit** must quantify how many launched graph-derived children contain edges outside their own executable graph, which child instructions trap because of stale absolute edges, and whether native `GraphSetNext` creates portable edge closure before GraphLaunch. Future prospective experiments may alter general **native graph edge construction opportunities**, but cannot add goal-directed repair/reproduce commands or simply engineer fixed successful child sequences. A valid functional child would still require functional continuity, lineage and genuine self-maintenance testing.

**CL-1 OPEN.**
