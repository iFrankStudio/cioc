# CL-1 Study43 — POST-HOC native child-function ancestry audit (registered before coding, AFTER initial observation)

**Epistemic status: EXPLORATORY, RETROSPECTIVE.** This audit proposal is committed *after* inspecting Study41/42's final results and a preliminary Python inspection that already indicated their successful child native events were all `GraphCreated`, with **no child `CellWritten`**, and many trapped child operations. This is **not** a pre-registered discovery outcome; do not conceal prior inspection or claim independent hypothesis confirmation. Study41 and Study42 themselves were separate fully prospective cohorts.

## Frozen exact source cohorts (0 new worlds)

- Study41: original CI run `43553705027`, original artifact `14096049859`, source revision `d52840573e4ee0ebee889ab008d964f5d1272e62`, exactly 128 seeds 3,100,000..3,100,127.
- Study42: original CI run `43553947738`, artifact `14097108465`, source revision `e2b74edb2f3b39f895f4d69410dde15431bc6309`, exactly 128 seeds 3,200,000..3,200,127.
- Reuse **unaltered** complete 512 native matched runs / 12,288 immutable SHA256 snapshots from each cohort; 256 original worlds total and **0** new independently drawn worlds, 1,024 arms, **24,576 hashed epochs**. Never rerun the old seeds to manufacture new evidence.

## Retrospective diagnostic

Before extracting child metrics, run the original source-pinned Study41 and Study42 independent full auditors on their respective SQLite archives. Then, for all 128 seeds and all four arms per cohort, verify every original epoch hash, native actor provenance, monotone event history, original graph creation and launch events, and source revision.

For each successful internally attributed `GraphLaunched` event record source owner, parent, graph handle, child process ID and tick, and require child ID >3, unique within the world and present in the final world `processes` collection (unless the VM has an explicitly logged native child deletion; current VM does not). Every child event must occur after its corresponding launch and have `origin=Internal(child)`; parent-coded or externally injected child events are excluded.

For the full `ambient-general` arm of each cohort, count:
- Worlds with native child launches, number of native `GraphLaunched` events/child identities; launched child opcode sequences and source graph sizes (using final child code as an ancestry descriptor, **not** claiming the code is restored or inherited with fitness).
- Worlds with at least one **successful** child event (excluding `Trapped` and `Halted`); total successful native child events and distribution by `GraphCreated`, `CellWritten`, `CellRead`, `GraphTagRead`, `GraphNodeCopied`, `GraphEncoded`, `GraphLaunched`, generic `Executed`, other native event kinds.
- Worlds with child internal `CellWritten` events and children with at least one output to a shared-cell key previously written by a native parent process. This is descriptive output continuation only, **not functional heredity**, without necessary causal dependency and no resource-independence.
- Child native successful execution in >=2 separate epochs, and in >=2 W3 epochs with **no same-epoch external process starter**; all credits and ambient inputs remain externally labelled.
- Worlds with at least one original parent native write before launch, to avoid treating child's graph-allocation-only activity as inherited function.
- Child Trapped versus Halted native event totals, child-only activity under `ambient-only` versus `ambient-general`.

The auditor must handle registered ambient input, neutral host-starter intervention ledger and exact process ID ancestry; no optimization or pilot-seed selection. Include a minimal synthetic counterexample where a fake child event without a valid native GraphLaunched is rejected.

## Interpretation and follow-up

The retrospective audit may establish that the observed 12/128 and 17/128 'successfully executing child' worlds lack shared functional output, thereby limiting the prior reproduction interpretation. A new independent, **prospectively registered** experiment is necessary to test any true function-bearing offspring, code heredity, damaged-parent function recovery and source-linked organizational continuity. Absolutely no automatic declaration of cyberlife from `GraphNew` loops, child process count, or long-running host-fed CIL.

**CL-1 OPEN.**
