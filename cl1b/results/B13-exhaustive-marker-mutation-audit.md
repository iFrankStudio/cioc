# CL-1B / B13 — first exhaustive marker-vs-service edge-mutation map

**Evidence type: independent restricted interpreter, NOT Rust VM or native CI.** Results use the [precommitted protocol](../studies/B13-exhaustive-marker-mutation-screen.md), [source-parsed Node scanner](../analysis/b13_edge_mutation_scan.mjs), and fixed [B10 CIL founder](../../cl0/crates/world/tests/cl1b_b10_freshness.rs). This is a **researcher-run observer screen**. The mutations are *not created, selected or evolved by the actor*, and the scanner is *not* a fitness oracle available to native CIL.

## Exhaustive finite sample

Enumerated all 19 instructions with mutable `next` fields × all 19 alternative addresses (0–19 except original next) × five source input values `[-13,0,7,23,101]` = **1,805 mutant worlds**, plus 5 unmutated baselines. Each condition runs at most80 interpreted CIL attempts, with halted/trapped conditions retained. Each baseline produces 5 valid service writes and 5 marker writes and no false acceptance. No prospective cases were discarded.

| Endpoint (binary per world unless noted) | Aggregate / 1,805 |
| --- | ---: |
| Any **false-positive acceptance** (PC11 reads nonzero marker, no correct service since PC2 reset) | **66** (3.66%) |
| Any **false-negative acceptance decision** (PC11 reads zero, correct service since PC2 reset) | **52** (2.88%) |
| At least one native-like marker written but **no valid service ever** in 80-step window | **218** (12.08%) |
| Valid service produced but **no marker ever** in 80-step window | **52** (2.88%) |
| Interpreter trap/unrunnable before80 attempts | **336** (18.61%) |
| Total false-positive readback *events* across all mutants | **381** |
| Total false-negative readback *events* | **82** |

**Warning:** binary categories can overlap; aggregate percentages should not be summed. The 218-world whole-window marker/service mismatch is **not** interchangeable with the stricter 66-world *at-PC11 accepted false positive* endpoint. The scanner records the active program's own marker read, not only output totals. Boundary censoring at80 instructions makes “any marker but zero service” a weaker structural indicator; the primary endpoint instead checks service since the actor's own marker reset.

### Input-specific differences

| Input | Any false-positive acceptance | Any false-negative decision | Trapped mutants |
| --- | ---: | ---: | ---: |
| −13 | 15 | 8 | 74 |
| 0 | **6** | **20** | 66 |
| 7 | 15 | 8 | 52 |
| 23 | 15 | 8 | 70 |
| 101 | 15 | 8 | 74 |

Zero-valued input is especially important. Valid-zero and missing-output ambiguity **changes which faults appear as false negatives**: input0 has 20 false-negative worlds against 8 for each nonzero input. Do not average input classes without retaining their individual outcomes.

### Reproducibility and independent audit

The committed Node script extracts the B10 founder directly from the Rust source, checks a fixed 20-op/19-next-edge shape and fails on unexpected opcode types. Its restricted execution model includes CIL reads, writes, deletion, branches, executable-edge reads/patches, bounded 4-register values and storage. An independently transcribed **Python** interpreter was separately used to check the primary aggregate totals (1,805 mutants; false-positive worlds66; false-negative worlds52; traps336), as well as per-input rates. Both simplified interpreters agree on these declared endpoints; the separate Python implementation is not yet committed as a canonical experiment artifact. Agreement is **not** a replacement for a Rust differential check.

The focused CI workflow has been configured to archive a `b13-observer-scan.json` artifact alongside original Rust experiment logs, if it runs. The GitHub Actions report branch remains unverified from this research session, so **no B13 native result or successful CI run is claimed**.

## Scientific significance and strict limitation

The B12 hand-picked marker/function mismatch **is not unique to those two edited nodes**. On a complete local single-edge mutation universe, numerous graph variations cause the *researcher-authored evaluator* to accept erroneous service states, while others wrongly reject genuinely performed work. Some variants are permanently unproductive or trap; all remain in the denominator.

This is an *evaluation-alignment defect in a human-programmed software mechanism*, **not evidence of evolved deception, new life, self-generated fitness or unscripted natural selection**. The next scientifically discriminating experiment is a **native VM differential** on frozen samples across both false-positive and false-negative classes, then an agent-executed method to obtain independent functional evidence and revise its own evaluator policy under held-out mutations — without the host using this scanner to select a winning code branch for the individual.
