# CL-1 Study 26 — verified *post-result exploratory* graph2 dropout classification (2026-11-13)

**Technical audit: PASSED. Novel scientific acceptance: NONE.** This is an after-the-fact diagnosis of a **previously known** Study-25 observation, not a new confirmatory experiment or additional independent worlds.

## Source and integrity

- [Explicitly post-outcome protocol](../studies/26-preregistered-retrospective-dropout-audit.md), committed *after* the Study-25 three-world dropout result was already known. This distinction is essential: do not relabel the classification a pre-outcome test.
- Validated source revision `d1f29800765f9c2a3c58ca5e560311c4f5a1afe7`.
- [CL-1 CI run 43452937996](https://github.com/iFrankStudio/cioc/actions/runs/43452937996) `study26` **passed** and repeated the exact unmodified Study-25 registered program generator, 96 seed IDs, 864 paired condition runs, 20,736 complete hashed snapshots, source-pinned native Rust epoch replay and the original Study-25 independent audit **before** running [the new read-only classification](../analyze_study26.py).
- **0 new independent seed-worlds**, exactly 96 reobserved original worlds. Synthetic classification test cases and the repeated 864 branches do not increase the scientific denominator.

## Frozen classification of known dropouts

Three (and only three) independent worlds met the qualified untreated graph2 foreign tag-read followed by same-actor shared-cell output condition. All three had the same process ID **1** and output cell **11**, in **epoch 0**. In the externally code-mutated graph2 branch, each successfully read the changed tag **19**, whereas the original successful source read returned tag **11**. None subsequently wrote to original cell 11 in that epoch.

| Registered subtype | Distinct source-exposed worlds |
| --- | ---: |
| Changed graph2 tag read, followed by different successful numeric output in cell 11 | **0/3** |
| Changed tag, same successful numeric output in cell 11 | **0/3** |
| Changed tag successfully read, original output cell 11 not written | **3/3** |
| No successful target graph2 tag read at all, original output absent | **0/3** |
| Other / duplicate ambiguous output | **0/3** |

Source seed IDs (not selected prospectively): **2250022, 2250038, 2250059**. Native process1 `Trapped` events were present in the changed-source branch for **two of three** examples; the third did not show a trap. All three processes wrote to **at least one other shared cell**. Thus this is an **observed source-code-content-sensitive execution-path/output-location change candidate**, rather than simple failure to read the modified source.

**Strict interpretation:** The registered positive Study-25 paired result demanded a **successful value change to the same output cell** following a changed target code read in both branches; it remains **0/96**. The three observed missing-original-cell cases are exploratorily compatible with changed control flow or register state, but neither proves a specific register-value dataflow nor internally designed multi-component functionality. A native `Trapped` event is not proof of repair, and no recovery after a late lesion was observed. The originally investigator-initialized programs and external `PutGraph` mutation remain exogenous.

## Next scientific discriminator

To distinguish genuine **value-flow dependence** from incidental program path changes, any further confirmation must pre-register new disjoint seeds and an exact output-class criterion **before** those outcomes; a register-level, success-only observer trace would be useful but must preserve CIL computation, scheduling and resource accounting. Future stage acceptance requires externally controlled positive restoration for the **same emergent function** and internal rebuilding of at least two causally dependent components over separate postdamage windows.

**CL-1 remains NOT ACCEPTED.**
