# CL-1 Study70 — original-source CIL genotype uniqueness versus correlated seed-offset patterns

**Status: independently CI-VERIFIED POST-RESULT INTEGRITY AUDIT. ZERO NEW WORLDS. No life claim.**

## Frozen original and independently rechecked integrity

- Registered **after** Stage68 and Stage69's primary results but **before inspecting complete genotype fingerprints and alignment**: `cl1/studies/70-registered-retrospective-random-genotype-seed-independence.md`.
- Verified original two separate CIL source-pinned cohorts: Study68 [CI43569268106](https://github.com/iFrankStudio/cioc/actions/runs/43569268106), revision `49a74113de513b32984f3656733087e9ae3a8b74`; Study69 [CI43569769695](https://github.com/iFrankStudio/cioc/actions/runs/43569769695), revision `f06e71294c055e4ebff6bf543a5f0f324e3718a6`.
- The new independent [Study70 GitHub Actions CI43570386530](https://github.com/iFrankStudio/cioc/actions/runs/43570386530) **SUCCESS**; archived artifact `cl1-study70-genotype-seed-independence` ID `14103429526` contains new JSON only, no regenerated original worlds.
- Before any new diagnostic, both historical full native independent Python auditors passed anew: exactly **512 original numeric seed labels**, 4,096 matched native runs, **98,304 SHA256 epoch snapshots** independently verified and original source-sensitive primary counts unchanged (**broad79/256 vs short0** and **broad81/256 vs short0**). Study70 added **zero new independent ecology**.

## Exact original-program fingerprint analysis

CIL initial process code is compared as sorted-key canonical JSON (all 28 native nodes: 3 original interface, 24 randomized continuation, terminal Halt), excluding seed label, source cell and process credits. Original eight-arm programs were checked for *only* the registered code differences, exact LCG reproduction and Read-ablation negative controls.

| Full CIL executable genotype | Stage68 (256 seed labels) | Stage69 (256 new labels) | Exact cross-cohort overlap |
|---|---:|---:|---:|
| Broad backedge | **256 unique /256** | **256 unique /256** | **0 collisions** |
| Short backedge | **256 unique /256** | **256 unique /256** | **0 collisions** |

There were **no exact full-program duplicates** within either cohort or between the two cohorts. This is a reassuring integrity finding: the measured 79 and81 feedback successes are **not simply replayed copies of identical native executable genotypes**.

## Important remaining dependence: same-index source patterns are correlated

Among the 24 stochastic CIL nodes, compare matching offset `i` between the two cohort seed ranges (4,500,000+i vs4,600,000+i) to a preregistered fixed-permutation comparator `j=(73*i+19)%256`:

| 24 randomized native nodes | Same seed-index offset (mean equal) | Fixed-permuted index (mean equal) |
|---|---:|---:|
| Broad, **exact full opcode+operands** | **4.835938 /24** | 2.675781 /24 |
| Broad, opcode **tag only** | **5.710938 /24** | 3.511719 /24 |
| Short, exact full opcode+operands | **5.343750 /24** | 3.093750 /24 |
| Short, opcode tag only | **5.710938 /24** | 3.511719 /24 |

Within Stage68, neighboring seed labels also show a reproducible native tag-pattern structure (~3.99 matching opcode tags among 24 consecutive or seed+2 program positions). No statistical independence hypothesis test was registered for the fixed permutation and these descriptive values are **NOT p-values or effective sample sizes**.

**Conclusion:** complete genomes are unique, but the raw LCG seed-to-opcode sampling has **nontrivial cross-cohort alignment correlation**. Consequently, separate numeric seed ranges are **not automatically independent random CIL genotype samples**. Do **not** describe Study69 as an unequivocally statistically independent replication without this caveat or pool 79+81 as one independent-sample likelihood.

## Scientific interpretation and next action

The **within-cohort causal paired topology effects remain real**: initial programs differ only in sampled Branch backedges, and in each source-pinned cohort the broad topology yielded dozens of actual W2/W3 native source→Write loops while short did not, with strict controls and equal late native activity exposure. Original primary counts and full-genome uniqueness are preserved. However, broader generalization across random program populations requires an **independently preregistered new seed-mixing method**, not just another source-seed offset.

Next Stage71 should freeze `SplitMix64(seed)` *once before the original LCG*, preserving the exact opcode grammar and causal endpoint. Verify all source-genotype uniqueness across the new cohort independently and report broad/short W2+W3 source feedback without any outcome-selected seeds. SplitMix64 is deterministic random seed whitening, **not** native mutation, organism selection or fitness.

**CL-1 remains OPEN:** repeated information-dependent computation under researcher-provided I/O and finite credits is a useful enabling mechanism, not an autonomously maintained living organization.
