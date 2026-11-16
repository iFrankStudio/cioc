# B30b — post-registration correction: process scheduler can select different sibling parents

**THIS RECORD IS POST HOC.** The original [B30 preregistration](../studies/B30-heldout-sibling-advantage-threshold.md) was committed before implementing the 8-world study. In [first full native CI run 43802804592](https://github.com/iFrankStudio/cioc/actions/runs/43802804592), the **B30 Rust test failed** and the **independent B30 observer failed**. The failure is retained as a real falsification of preregistered **third-parent identity** assumptions. Original B29 positive study remains separately native-verified.

## Incorrect preregistered assumption

B30 correctly registered that the mutation-rule FLIP arm gains an *extra birth* over same-cost NO_FLIP only at third-packet size156 of the fixed neighborhood `[155,156,157,158]`. However the preregistration hypothesized that at both157 and158 the third birth would be `1→4` (original founder reproducing again) because founder request156 was affordable.

This was incorrect. Deterministic scheduler state is affected by each process's last paid PC and the round-robin pointer, and a newly generated sibling can win even when a lower-numbered ancestor is equally eligible. The native failure explicitly logged, at third-packet157, **actual `3→4` versus predicted `1→4`**. The separate source-parsed CIL scheduler reproduces the same third-parent assignments and reveals a different parent at158.

## Complete corrected four-neighborhood source-parsed predictions

All controls use **one original host-loaded founder, 2 initial compute, packets `[157,157,x]`**, fixed 23-node B29 program and same-cost mutation-disabled comparison. Births are native-like internal `GraphLaunched(parent,child)` event sequences; parent IDs are not derived from PID order.

| Third neutral pool packet x | FLIP third birth / total births | NO_FLIP third birth / total births | Residual pool FLIP / NO_FLIP | Source-model result |
| ---: | --- | --- | --- | --- |
| **155** | none / 2 | none / 2 | 155 / 155 | both variants unaffordable |
| **156** | **3→4 / 3** | none / 2 | **0 / 156** | FLIP gains one birth in this threshold |
| **157** | **3→4 / 3** | **1→4 / 3** | **1 / 0** | no birth-count edge, but genotypes/parents diverge |
| **158** | **2→4 / 3** | **2→4 / 3** | **0 / 0** | high-request sibling2 can now reproduce; no count advantage |

**Interpretation:** The original hypothesis was not wrong merely due to a wrong count; it misattributed **which genotype actually used the newly available energy**. This error is precisely why event-native ancestry, scheduler pointer and process-local credit accounting are part of the CL-1B verification rules.

## Correction and acceptance policy

Correct the Rust B30 asserts and independent B30 observer to demand **the complete four-row table**, *including* the unexpected `3→4` at x157 and `2→4` at x158. Preserve the failed CI run link in the final audit. A later green run verifies the **post-hoc reconciled mechanism** and falsifies the specific preregistered parent-priority prediction; it must **not** be relabeled a clean preregistered forecast.

This is finite deterministic ecology among **investigator-authored** mutation-rule programs, not spontaneous self-improvement, general fitness, sentient agency or evidence that CL-1B requires reproduction. Its scientific value is a caution that genetically diverse siblings and more environmental resource do not imply stable selection for a desired mutation: scheduler event order can change which one receives the next reproduction opportunity.
