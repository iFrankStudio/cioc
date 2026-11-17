# B48 — First native semantic × phase crossover audit (2026-11-17 UTC)

**Evidence class:** native CI observation, not independent dynamic CIL replication. **H48 not accepted. B3/B4 NOT MET.**

## Source-pinned verification
- Source: `5d631ffd59319c3bb9f72a4be4efe87ae1029e18`
- [Focused Actions run 43983126223](https://github.com/iFrankStudio/cioc/actions/runs/43983126223), completed success 2026-11-16 23:54 UTC.
- Native job 133704378943: B48 `b48_feedback_semantics_phase_cross_over_prospective_screen ... ok`, 3 tests passed in selected target; `failed_target_count=0`. 34 named independent *existing* observers passed; evidence publishing job 133704554618 passed.
- This is a **focused**, not 48/48 full-regression run. Existing 34 observers do not provide B48 independent dynamic CIL event replay.

## Native aggregate (from B48_AGGREGATE; raw correct/wrong, eight seeds per phase)
Columns below are holdout windows 1 and 2, not the training window.

| Phase | Arm | Holdout 1 correct/wrong | Holdout 2 correct/wrong |
|---|---|---|---|
| baseline | truthful | 65/14 | 73/7 |
| baseline | blind-self | 18/54 | 20/60 |
| baseline | blind-zero | 0/9 | 10/0 |
| baseline | edit-disabled | 0/0 | 0/0 |
| baseline | compact immutable | 96/0 | 96/0 |
| +1 paid preamble | truthful | 65/14 | 73/0 |
| +1 paid preamble | blind-self | 18/54 | 20/60 |
| +1 paid preamble | blind-zero | 0/9 | 10/0 |
| +1 paid preamble | edit-disabled | 0/0 | 0/0 |
| +1 paid preamble | compact immutable | 90/6 | 88/8 |
| baseline, unseen | truthful | 64/15 | 64/16 |
| baseline, unseen | blind-self | 18/54 | 20/60 |
| baseline, unseen | blind-zero | 0/9 | 0/10 |
| baseline, unseen | edit-disabled | 0/0 | 0/0 |
| baseline, unseen | compact immutable | 96/0 | 96/0 |
| +1, unseen | truthful | 64/15 | 63/10 |
| +1, unseen | blind-self | 18/54 | 20/60 |
| +1, unseen | blind-zero | 0/9 | 0/10 |
| +1, unseen | edit-disabled | 0/0 | 0/0 |
| +1, unseen | compact immutable | 89/7 | 89/7 |

Native paired A-vs-B and A-vs-C win/loss/tie arrays (each 16 pairs per phase/holdout): baseline replication 12/2/2 and 14/0/2; unseen 10/4/2 and 14/0/2, identical across both blind comparator definitions in the printed paired summary. These are **native test output**, not an independently verified statistical reproduction.

## Causal validity and negative findings
1. The truthful arm shows a large *within-architecture* advantage over blind-self on both holdout families and phase offsets, but still falls below the compact immutable copier in correct service writes per common budget.
2. The **full preregistered H48 conjunctive hypothesis cannot be accepted**: blind-zero and edit-disabled arms frequently terminate after ~35–52 paid attempts (versus 144 for truthful and blind-self). Equal *initial* compute budget is not equal *realized* exposure; this was explicitly required by H48. Stopping must remain a reported outcome, not be imputed to 144 steps.
3. Phase offset changes even the immutable copier's outcomes, supporting the need to treat phase as an actual confounder.
4. The native output provides aggregate row data and some event classifications, **not** a separately source-parsed dynamic interpreter with event-by-event comparison. Consequently causal attribution is **provisional** and independent acceptance is withheld.
5. No endogenous objective invention, self-generated selection policy, B3/B4 attainment, or cyberlife discovery is inferred.

## Next discriminating experiment
Before adding B49, implement an independent dynamic B42 CIL subset replay (including CloneNode, PatchNext, writes, phase preamble) with per-event first-divergence checks against source-SHA-matched native traces. Also introduce a nonterminating semantic sham or explicitly redefine a *new preregistered* estimand separating intention-to-treat throughput and survivor-conditional performance. Never retroactively weaken H48 or silently exclude stopped worlds.
