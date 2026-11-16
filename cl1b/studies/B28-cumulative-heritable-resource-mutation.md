# CL-1B / B28 — cumulative native genotype changes across full-program generations

**Prospective registration: 2026-11-16, before authoring B28 founder, native test, or independent outcome computation.** This is a **bounded mechanistic model of inherited variation and resource-pressure filtering**, not autonomous natural evolution, life origin, or open-ended adaptivity. The complete reproducer, mutation operator and mutation step are investigator-authored. Passing can support **cumulative code heredity under generic substrate physics**, not independent discovery of a good policy.

## Question and relationship to B27

B27 demonstrated full 12-node CIL self-copy through three native generations, but the copied genome was unchanged. B28 asks whether ordinary **paid CIL computation and generic graph edits** can create a **different full-replicator genome** at birth and whether the *new value itself becomes the descendant's starting genotype* for a subsequent mutation/reproduction attempt. This distinguishes **a one-off child phenotype change** (B26) from **cumulative executable heredity**.

## Frozen 19-opcode self-reproducer

The researcher loads **only process1** at t0, with two initial local compute credits; zero world pool. World laws are the existing `with_pool_paid_draw()` and `with_recoverable_draw_shortage()`; strict storage (16 cells, 64 max nodes/process, 128 total occupied units). No special reproduction, growth, mutation, fitness, or energy-creation opcode is added. No host child creation or targeted credits after initial loading.

The genome PC0..18 is:

| PC | Native CIL instruction | Next |
| ---: | --- | ---: |
| 0 | `Const r3=127` current request genotype | 1 |
| 1 | `DrawCompute amount=r3` | 2 |
| 2 | `GraphNew dst=r0` | 3 |
| 3 | `Const r1=0` copy index | 4 |
| 4 | `GraphCopyNode(graph:r0,source:r1,dst:r2)` | 5 |
| 5 | `Const r3=1` | 6 |
| 6 | `Add r1=r1+r3` | 7 |
| 7 | `Const r3=19` genome length | 8 |
| 8 | `Sub r3=r1-r3` | 9 |
| 9 | `Branch r3: nonzero→4, zero→10` | — |
| 10 | `Const r1=0` target child graph node0 | 11 |
| 11 | `Const r2=127` **current inherited base request**, same as PC0 | 12 |
| 12 | `Const r3=δ` **investigator-authored signed mutation step** | 13 |
| 13 | `Add r2=r2+r3` generates candidate Q+δ | 14 |
| 14 | `GraphSetConst(graph:r0,node:r1,value:r2)` edits copied node0 | 15 |
| 15 | `Const r1=11` | 16 |
| 16 | `GraphSetConst(graph:r0,node:r1,value:r2)` edits **copied node11** | 17 |
| 17 | `Const r1=2` child startup credits | 18 |
| 18 | `GraphLaunch(graph:r0,credits:r1,dst:r2,next:0)` | 0 |

The generic `GraphCopyNode` loop copies the **entire currently running 19-node executable** with unchanged absolute edges. The two native `GraphSetConst`s then update copied PC0 and PC11 to **the same newly calculated child request value**, so a descendant derives its *next mutation from its own inherited base* rather than repeatedly restarting from the original ancestor127. Each launched child retains the same 19-opcode replicator and mutation step δ; this is an authored deterministic mutation operator, not stochastic evolution.

## Exact per-birth compute feasibility

At bootstrap, each process with two initial local credits executes PC0 Const and one deferred Draw, leaving PC1 pending at zero credits. A pool-paid Draw for requested Q costs one world-pool instruction credit plus transfers Q credits into the process. The rest of the copying/reproduction program requires **125 attempted native instructions**:
- 2 instructions for GraphNew and copy-index initialization;
- 19 repeated graph node copies × six native instructions =114;
- 9 instructions for child request computation, two graph source edits and launch.

`GraphLaunch` additionally **transfers two existing local credits into the child**. Therefore Q must be **at least127** to fund successful reproduction. Exactly at Q=127, the parent ends at non-Draw PC0 with0 credits; the new child pays its own two startup charges and waits at Draw PC1.

This makes δ=−1 a registered deleterious counterexample: parent127 can create child126, but that child cannot complete the same paid program and transfer the required two credits, even if an environmental packet126+fee1 is available. The attempted launch must trap/stop without a new birth, and the failed child/allocated graph must remain in the audit denominator.

## Frozen 11-world experiment

Three fixed, **researcher-selected** mutation steps δ∈{+1,0,−1}, each with the same initial 19-node founder and first request127, under three identity-neutral pool-packet schedules:

- **INCREASING:** `[128,129,130]` (one packet per opportunity; total387).
- **FLAT:** `[128,128,128]` (total384).
- **DECREASING-SUPPLY:** `[130,129,128]` (total387; matched total to INCREASING, different order).

This yields **9 worlds**. Plus **NO_INFLUX** δ=+1 and no packets, and **COPY_DISABLED** δ=+1 under INCREASING with `with_native_code_edits_disabled()`; **11 worlds total**. All worlds are traced through the same finite observation windows, with no researcher selecting offspring or new program versions after inspection. Every packet is injected only into the generic shared world pool.

Predicted necessary outcomes to falsify explicitly:
- δ=+1 under INCREASING should produce **three** native birth events with source request genotype sequence **127→128→129→130**, and **both PC0 and PC11** of every child must match its inherited request. Under FLAT, a child whose request128 needs129 world-pool credits to run Draw, so its reproduction is delayed until enough packets accumulate; report the entire timing/unused pool, not just eventual births.
- δ=0 under FLAT should maintain a full **127→127→127→127** copied lineage with equal charged costs.
- δ=−1 under INCREASING can produce child126, but **must not** produce a grandchild after that child attempts the unfundable launch; report the native trap and unspent compute.
- COPY_DISABLED must not produce a real native child; silently making a placeholder process would fail.

## Causal evidence and bounds

- Native `GraphCreated`, **19 ordered native GraphNodeCopied events** from the current parent executable for every completed birth, two `GraphNodeEdited(edit="const")` at child graph indices0 and11, and a real `GraphLaunched(parent,child,transferred_credits=2,moved_nodes=19)` with `Origin::Internal(parent)`.
- Verify **full 19-opcode genome equality** to a logically expected source that differs only at inheritable PC0 and PC11 request literal values and preserves the generic copying/mutation program. Never equate a changed metadata label to inherited code.
- Verify every native attempted instruction decreases total local+pool compute by exactly1, each external packet increases it by its exact amount. Failed launch and trapped/stranded actors remain counted.
- Compare generation-by-generation reproduction opportunity and resource threshold across equal-total gradient/reversed schedules as well as flat supply. Do not infer statistically general fitness or intended selection from three authored timelines.
- Mutation rule δ and genome schema were explicitly encoded by the investigator. **No genotype arose by a discovered mutation operator or feedback-optimized strategy**, and neither increasing Q nor number of offspring necessarily means improved real-world function. "Open-ended evolution" and "cyberlife" remain unproven.

If B28 succeeds, the substrate has a **verified finite multigeneration native CIL lineage carrying cumulative executable changes**, stronger than B27's exact copying. The next actual CL-1B challenge is to permit **multiple competing mutational variants from the same seed** under shared resources, with native-created branching ancestry, paid resource competition, and no host-side choice of winners; ideally add CIL-visible, forge-resistant-enough functional evidence without privileged fitness semantics. No declaration of B3/B4 success on one deterministic hand-coded signed mutation ladder.
