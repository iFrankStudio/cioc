# CL-1B / B27 — resource-paid multigeneration self-copying executable

**Prospectively preregistered 2026-11-16, before creating B27 native source or evaluating outcomes.** This study asks whether the existing generic CL-0 CIL VM can execute a **fully heritable self-copying program across multiple internally produced generations**. It is **not** a declaration of autonomous life or open-ended evolution. The entire replicator, request quantity, condition, and replication policy are investigator-authored. Copy fidelity and resource conservation are primary outcomes; a graph launch alone is insufficient.

## The 12-node native CIL reproducer

One host-loaded process1 starts with exactly2 local compute credits and a shared world pool of0, strict storage (16 cells, 64 nodes per process, 128 total occupied units), world laws `with_pool_paid_draw()` and `with_recoverable_draw_shortage()`. There is **no externally loaded helper, child, grandchild, code patch or individual credit top-up**.

| PC | Opcode | Next |
| ---: | --- | ---: |
| 0 | `Const r3=78` resource bundle | 1 |
| 1 | `DrawCompute amount=r3` | 2 |
| 2 | `GraphNew dst=r0` | 3 |
| 3 | `Const r1=0` source node index | 4 |
| 4 | `GraphCopyNode {graph:r0,source:r1,dst:r2}` | 5 |
| 5 | `Const r3=1` | 6 |
| 6 | `Add r1=r1+r3` | 7 |
| 7 | `Const r3=12` source program length | 8 |
| 8 | `Sub r3=r1-r3` | 9 |
| 9 | `Branch test=r3: nonzero→4, zero→10` | — |
| 10 | `Const r1=2` child endowment | 11 |
| 11 | `GraphLaunch {graph:r0,credits:r1,dst:r2,next:0}` | 0 |

**Key generic VM affordance:** `GraphCopyNode` (not `GraphCopyNodeLocal`) copies an existing executable node **preserving absolute control-flow addresses**. The CIL loop creates graph0..11 by copying the parent's **entire 12-node executable**, including its own graph copier and native launcher. The child starts at PC0 and is therefore able to repeat the same copying algorithm, not merely to produce a service output. All copies, graph allocation, and launches are charged; no privileged `REPRODUCE` opcode exists.

## Exact signed resource accounting

A process with2 initial local credits charges the PC0 Const and then one deferred Draw with empty pool. It becomes blocked at PC1, zero credits. A world-pool packet of **79** can pay the Draw instruction charge1 and transfer its requested78 credits. The actor then pays:

- PC2–3: 2 instructions;
- 12 copies × PC4–9, six instructions per copy: 72 instructions;
- PC10–11: 2 instructions.

This totals **76** native instruction attempts from the transferred78, with another **2 local credits transferred (not cloned) to the new child** by `GraphLaunch`. The parent ends with zero local credits at **PC0**, a non-Draw opcode and therefore not eligible for further pool-paid work. The child starts with2 credits, executes its own PC0 Const and PC1 deferred Draw (2 paid instructions), and becomes blocked at PC1 with zero credits. Thus the **single neutral 79-unit packet funds 79 charged native attempts**, one native birth and the new child’s own paid initialization. This pattern can repeat for the child if another identity-neutral pool packet arrives. This is a *scripted linear generational chain*, not unrestricted exponential population growth.

## Frozen five-arm study

After initial2 attempts to block the founding process, inject the following **externally attributed world-pool** packets, draining all eligible native instructions after each. Count every new child, graph node copy, provenance, per-generation credits and shared pool leftovers, including traps, halts and unused balance.

1. **THREE_GENERATIONS:** `[79,79,79]`. Expect three native births creating a chain `1→2→3→4`, each child with the **same full 12-node executable** including its own `GraphNew`, `GraphCopyNode`, and `GraphLaunch`. Each extinct/blocked older parent stays at PC0 with zero credits. Newest child waits at PC1, zero credits.
2. **SPLIT_THRESHOLD:** `[78,1,79,79]`. After the first packet78, no actor is eligible: no additional native event. The next packet1 brings shared pool to79 and allows the first birth. Later two packets79 produce subsequent births, with exactly the same source/lineage. Distinguish external supply and its threshold from native compute creation.
3. **SINGLE_PACKET:** `[79]`. Only one birth; child inherits reproductive algorithm but does **not** reproduce without another energy packet.
4. **NO_INFLUX:** no packets. No births after the founder consumes its first2 credits.
5. **NATIVE_CODE_COPY_DISABLED:** identical source, three packet79 supply events, but the generic `with_native_code_edits_disabled()` negative world law suppresses actual graph-node copying while still charging instructions. The launcher must **not** produce a child without copied code; a native trap or no-birth state is reported rather than discarded. This is a test of true native graph mutation, not mere host allocation.

All ordinary worlds use **identical initial source and initial compute**; only explicitly specified supply / copy-law controls differ.

## Native evidence requirements

- Actual `Origin::Internal(parent)` `GraphNodeCopied` for 12 complete ordered source node indices0..11 per successful birth, one `GraphCreated`, then `GraphLaunched(parent,child,transferred_credits=2,moved_nodes=12)` for each generation. No `Origin::External` process loads beyond initial process1.
- Every successful child executable must be **byte/structure identical** to the entire original 12-node Rust `Vec<Op>` and itself contain a working `GraphCopyNode` and `GraphLaunch`. Comparing a descriptor label or child process count without code equality fails this criterion.
- Parent-child event chains must be actual native process IDs `1→2→3→4` without host-created intermediate agents.
- Every charged step consumes one combined local+pool compute, every world-pool pulse increases total inventory by its declared amount. All action and storage costs, blocked instructions and traps remain visible.
- A full 3-birth run must verify at least **three generations of reproduction events** and each generation's power to copy source; this is not yet inheritance *with mutation*. No source program can be replaced or chosen by the observer.
- Critical caution: the CIL program was investigator-authored to be a **one-shot serial reproducer**, and parent nodes are deliberately stranded at non-Draw PC0 after copying. Repeating across three host-timed neutral supply packets is not evidence of autonomous ecological maintenance or open-ended evolution.

## Interpretation and next discriminator

If natively confirmed, B27 establishes that CL-0 generic graph/compute primitives can implement a **finite-resource, multi-generation self-copying executable with real hereditary code continuity** without a `LIFE` or `REPRODUCE` instruction. This would be a scientifically useful substrate affordance. It does **not** establish natural selection, novel genotype creation, self-generated replication policy, the ability to survive without environmental energy input, or cyberlife. The next experiment must introduce **native-computed, genuinely inherited variation of the complete reproducer** and compete mutated descendant lines on prospectively withheld neutral resource schedules, with no host-selected winners or prewritten complete variant templates.
