# CL-1 Study52 — CI-verified retrospective source-opcode bottleneck: graph-copy result aliases source selector

**Scientific status: post-result, source-pinned diagnostic, ZERO new worlds. NOT LIFE.**

## Source, provenance and audit

- Retrospective registration was committed before this new opcode-ancestry audit but after both known Study49/Study51 primary outcomes.
- Original unaltered Study49 and Study51 SQLites were downloaded from CI runs [43556383858](https://github.com/iFrankStudio/cioc/actions/runs/43556383858) and [43556991490](https://github.com/iFrankStudio/cioc/actions/runs/43556991490).
- Independent audit [GitHub Actions **43557512186**](https://github.com/iFrankStudio/cioc/actions/runs/43557512186) **passed**, artifact `cl1-study52-copied-source-opcode-ancestry` ID `14098643715`.
- Revalidated both original preregistered Study49/51 Python grammar/pool/native outputs and **49,152 original SHA256 epoch snapshots**. Reused **512 original independent worlds** across two distinct cohorts; **zero new worlds**.

## Predeclared lineage results (ambient-general separately)

| Source copy opportunity | Study49 (256 original worlds) | Study51 (256 independent worlds) |
|---|---:|---:|
| Native child launches | 166 across 140 worlds | 171 across 142 worlds |
| Strict original-parent native copy→edit→launch | **1/256** | **3/256** |
| Strict path copied a source whose frozen initial opcode is `Write` | **0/256** | **0/256** |
| Strict path yielded child with an executable `Write` | **0/256** | **0/256** |
| Strict path child final control-flow edges locally closed | **0/256** | **3/256** |
| Actual native child shared `CellWritten` | **0/256** | **0/256** |

**Actual copied source opcodes in all four strict independent worlds**:
- Study49 seed **3,500,022** copied source node74, frozen original opcode **`CloneNode`**, and source node1, original opcode **`GraphCopyNode`**. Child final executable edges not closed; no child Write.
- Study51 seeds **3,600,139**, **3,600,160**, **3,600,172** all copied source node1, original opcode **`GraphCopyNode`**. Their child final executable edges were closed, but no child Write.
- **Zero** strict-path copied initial program `Write` opcodes; no copied source had unresolved original-versus-runtime mutation ambiguity according to the registered conservative classification.

## Mechanistic diagnosis

The frozen random CIL grammar uses the *same register2* as `GraphCopyNode.source`, `GraphCopyNode.dst`, and `GraphSetNext.node`. A successful native copy writes its new graph-local node index into register2, overwriting the previous original-source selection. This **aliases the source selector with the output node index**, allowing subsequent copy operations to preferentially copy a small early scaffold source index (such as parent node1) instead of retaining a randomly selected parent code node, including a potential Write. Stage52's complete event/initial-opcode ancestry is consistent with this mechanism. It does **not** prove the alias is the unique or necessary reason for the absent child Write.

**Next falsifiable prospective question:** keep the CIL graph-copy/launch operation semantics unchanged but compare two *generic register topologies* under new independent seeds: (a) original source/result alias in register2, versus (b) preserve source selector in register2 and write copied graph-local node index to a separate register0, with `GraphSetNext` referencing that result register. Every copy/edit/launch operation remains sampled stochastically; no repaired organism is preloaded. Fully audit whether the new topology actually produces copied initial-Write source ancestry and successful native child shared output (the latter remains strict).

This is a neutral **program composition/operand aliasing experiment**, not an engineered reproduction or a life definition.

**CL-1 remains OPEN.**
