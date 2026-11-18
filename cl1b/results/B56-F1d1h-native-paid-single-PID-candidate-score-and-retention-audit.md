# CL-1B B56-F1d1h — exact frozen 166-node source, native paid single-PID candidate generation/score/retention, 36-world causal controls

**Evidence status:** native-executed, source-frozen, FULL CI verified, independently reexecuted **14,781 actual paid CIL instructions**, zero first divergence. Accepted only as **finite authored-grammar PID1 candidate generation, execution, local scoring, winner restoration and held-out retention**. Not autonomous invention of a primitive, not an open-ended algorithm generator, not original B51c/F1c identity or cyberlife.

## Evidence chain and prospective protocol freeze

- [F1d1h initial candidate-search preregistration](../studies/B56-F1d1h-paid-same-PID-generated-bytecode-candidate-search-preregistration.md) committed before code at `cbcae11c90fe1e512a01849fecf9b2385699e90d`; [score, multi-probe, feedback and causal-oracle addendum](../studies/B56-F1d1h-frozen-candidate-score-causal-oracle-addendum.md) also committed before any F1d1h native implementation at `8ad1a4bc5676794468483b2d556f6e4e60714493`.
- [Exact 36-world test design and static cell-key union manifest](../studies/B56-F1d1h-frozen-exact-source-and-36-world-controls-v1.md), SHA-256 `149f39656b4412353861e1b645a697c61e96a2ecaaea6dc0b5dc5edb89263d72`, plus [exact all-166-instruction source/branch edges JSON](../studies/B56-F1d1h-frozen-exact-CIL-source-v1.json), SHA-256 `27ec853e84c918b14d989867b976b9811e8477bb6a0693961f0a0e704682475e`, were **both committed FIRST** at `7e1afa15eb2b43b40d00a1c026280b017de5c02b`; Rust source subsequently added at `f662244fe1f73a20c34239fb01f3633240849e8d`. Pre-CI Rust assertion fixed a frozen-only score count error and preserved code/manifest semantics at `855edce0b312056bd24df92eda345ef1e3781b72`; do not portray that edit as an algorithmic discovery.
- [FULL CI run44297799709](https://github.com/iFrankStudio/cioc/actions/runs/44297799709) tested exact code SHA **`85465205a37861d29d44465d7efa4308eebc02e8`**, **66/66 Rust CL-1B integration targets PASS**, **57/57 independent replay/analyses PASS**, mandatory B31/B34 Rust library ABI tests PASS, `failed_target_count=0`, `failed_observer_count=0`. Evidence job independently published original runs.
- [Native Rust CIL test source](../../cl0/crates/world/tests/cl1b_b56_f1d1h_paid_candidate_retention.rs) · [separately written source-parsed JS interpreter](../analysis/b56_f1d1h_paid_candidate_retention_replay.mjs) · [raw native step/event JSONL](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/85465205a37861d29d44465d7efa4308eebc02e8-44297799709-1/b56_f1d1h_native_paid_candidate_retention.jsonl) · [complete original independent replay report](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/85465205a37861d29d44465d7efa4308eebc02e8-44297799709-1/b56_f1d1h_paid_candidate_retention_replay.json) · [original full-test scope and observer summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/85465205a37861d29d44465d7efa4308eebc02e8-44297799709-1/summary.txt).

## Mechanism: what native PID1 actually executed and paid for

All **36 isolated strict worlds** use the SAME **166-node original Rust CIL founder** (native executable cap180), strict `World::with_strict_capacity(16,180,300)`, exactly **1,000 finite starting credits** per one-PID world, no compute pool or second PID. Frozen actor CIL initializes its data program and all three candidate score cells61–63 through ordinary paid CIL Writes. The sixteen source-limited potential cells are `7,11,20,21,31,32,50,51,52,53,54,55,56,61,62,63`: **exactly 16/16 simultaneously possible**, and the independently observed maximum total occupancy is **183/300**. The frozen machine-readable specification declares the 16-key union; this is not presented as a universal static program-analysis proof about future code.

The fixed native interpreter pays for each virtual instruction fetch: **Read virtual PC50 → Add(51+PC) → Read virtual opcode from the resulting key**; it pays for LOAD_INPUT, NEGATE_ACC, ADD_LITERAL2 or RETURN, writes actual accumulator checkpoints to key56, advances virtual PC and pays actual output Write11. Source authoring and operation semantics are externally supplied at initial PID1 load; after that the actor runs, edits data and scores under strictly paid native rules. **No GraphLaunch, external executable edits, privileged state repair or PID relabeling.**

The initial data program A is `[0,3,3,3]`, function `x`. PID1 executes A on two distinct probes `-7`, `5`, reads the environment feedback after each actually executed service, then stores one score increment for every real Read20 equal to1 in **score cell61**. To generate B, PID1 increments its candidate index and pays a native Write52(1), making program `[0,1,3,3]`, function `−x`; two more real probes and paid feedback update score cell62. To generate C, PID1 pays Write53(2), making program `[0,1,2,3]`, function `−x+2`, and similarly scores cell63 after two probes. This is **sequential mutation of the same data program by one PID**; it is not three host-injected finished programs.

PID1 then *itself* scans its three real score cells: first it checks score2 in index order A,B,C, then score1 in index order, otherwise defaults to A; this implements a bounded score 0..2 and smallest-index tie break. It pays to reinstate both slots52/53 of the winner, reuses candidate31 as retained winner, sets probe stage32=2 and virtual PC50=0, and reexecutes a **genuine held-out service** at input11. No winner identity or program bytes are externally injected. The native test asserts candidate-specific historical service counts/score consistency, exact same native source after every paid instruction, same PID1, finite credits and same-source controls; JS independently reconstructs the exact source from Rust literals and compares to the precommitted 166-opcode JSON.

## Frozen 36-world matched experiment and held-out results

Three hidden tasks `IDENTITY: x`, `NEGATE: −x`, `COMPOSED: −x+2`; two training probes **−7 and5**, heldout **11**; four feedback regimes `TRUTH`, `BLIND_SUCCESS`, `INVERTED`, `ABSENT`; three generation gates **1=enabled**, **0=suppressed**, **2=frozen A only**. Each world uses EXACT same CIL founder graph. External interventions are strictly limited to gate key21 once, input key7 at trial start and after each real probe service, plus feedback key20 only in non-ABSENT arms. The actor is never given task identity, fitness function, winning score, winning code, extra credits or externally updated code/data slots.

| Feedback regime | 3 generated candidates, enabled | Generation suppressed (A executed under all three candidate labels) | A only frozen baseline |
|---|---:|---:|---:|
| **TRUTH** | **3/3** heldout correct | 1/3 | 1/3 |
| **BLIND_SUCCESS** (feedback bit always1) | 1/3 | 1/3 | 1/3 |
| **INVERTED** | **0/3** | 1/3 | 1/3 |
| **ABSENT** (feedback cell20 never exists) | 1/3 | 1/3 | 1/3 |

Full-generation **TRUTH** exact native-origin outcomes:

| Withheld task | A, B, C stored paid scores | Actor retained winner | Held-out output | Paid instructions |
|---|---|---|---:|---:|
| IDENTITY | `[2,0,0]` | A | `11` | 555 |
| NEGATE | `[0,2,0]` | B | `−11` | 580 |
| COMPOSED | `[0,0,2]` | C | `−9` | 607 |

Candidate training output vectors, reconstructed from **actual paid service outputs** in the native original, are A `[-7,5]`, B `[7,-5]`, C `[9,-3]`, distinct at both probes and heldout. Even the source-identical suppression gate0 pays two writes, but to existing literal/shadow key55 rather than newly generated opcodes; thus all three candidate indices execute A, an **explicit generation ablation, not a valid three-candidate ranking arm**. Gate2 executes *only A* and two probes, then heldout; it has different service and total cost, and must NOT be called a matched paid-compute comparison. Order is necessarily A→B→C in this frozen monotonic mutation grammar; **order bias was not tested or randomized**.

`BLIND_SUCCESS` is an uninformative PRESENT feedback bit, **not missing feedback**. In true **ABSENT** arm the environment never creates cell20, and existing Rust `Read` returns0 for the missing cell. The actor cannot distinguish absence from a valid negative bit, and no authenticated freshness/correctness mechanism is established.

## Exact event-by-event replay and resource ledger

The JS observer separately parses every original CIL source node from Rust, verifies 166 structural opcodes against frozen preregistration, reproduces the stable source snapshot fingerprint, and executes **every native paid instruction and every external SetCell event** in deterministic order. It rejects any different native PC, register, charged credit, cell contents (including absent), code source, PID, event kind or occupancy and includes every real service, score and virtual fetch. Original replay log is source-pinned SHA `85465205a37861d29d44465d7efa4308eebc02e8`.

| Raw native observation | Independently reexecuted count |
|---|---:|
| Isolated source-identical PID1 worlds | **36** |
| Original finite paid CIL steps | **14,781** |
| External gate/input/feedback interventions | 366 |
| Paid Read20 feedback operations | 168 |
| Actor-paid per-candidate score Writes | 84 |
| Paid reads of score memory | 199 |
| Actor-paid new bytecode-slot Writes | 24 |
| Cost-bearing program-generation suppression shadow Writes | 24 |
| Dynamic virtual opcode fetches | 484 |
| Real paid intermediate accumulator checkpoint Writes | 280 |
| Real actual service Write11 outputs | 204 |
| Per-world paid instruction range | 194–607 |
| Maximum total occupied storage | **183/300** |
| Independent replay first divergence | **null** |
| Actor PID changes / native executable code mutation | **none** |

For all standard gates (enabled/suppressed), the actor executes six paid probe services and one heldout (7×24=168 services). For A-frozen gate2, two probes and one heldout (3×12=36 services), total204. Every service was executed in the original Rust world and accounted, not inferred from a source branch or host oracle.

## Scientific conclusion and next essential falsifier

**F1d1h closes a narrow gap after F1d1g:** the actor not only performs a single known two-slot edit, but now actually **generates, executes, evaluates, compares and retains multiple different program configurations in one continuous PID1 under actual finite actor-paid storage/compute**. The correct winner depends on real feedback bits, and inverted/absent controls show failure rather than magic oracle access. This is a mechanical capacity for small candidate search; the entire candidate grammar `A→B→C`, primitive ISA, scoring rule, tie break and probe count were **preprogrammed by researchers**. The original F1c lineage has not learned or incorporated this mechanism by itself.

A discriminating next experiment should test **actor-chosen candidate mutations depending on observed scores**, not only monotonic candidate enumeration, with generated program length/cost tradeoffs, repeated regime shifts, real validation/rollback and order/budget adversarial controls. If the actor always follows a researcher-authored fixed candidate order, do not call that open-ended self-discovery. Need preserve 16-cell/180-node limits; a negative capacity result is scientifically useful. Host-provided feedback remains spoofable; prior B53 correlation requirements separate.

**NOT MET:** authenticated B53 receipt source origin for original B56-F1d1, N56-A/B/C, B3/B4, open-ended instruction semantics, original F1c founder continuation, autonomous cyberlife.
