# CL-1B / B14 — native mutation of the candidate-generation policy itself

**Prospectively registered on 2026-11-15 before B14 implementation or outcome inspection.** This is an **authored mechanism-calibration** extending B11, not emergent cyberlife, naturally selected fitness, or accepted B3/B4. It uses *generic native CIL* `PatchConst` and `PatchNext`; the Rust world is not given a repair, scorer, or rollback primitive.

## Question

Can one continuing CIL actor, after observing that its first code-edit candidate failed a *fresh local trial*, modify **the literal parameter in the code of its own future candidate generator**, and then search again? Does disabling the *policy-edit* action causally change later function? Does the policy change help all external lesions, or only certain handpicked ones?

Unlike B11's constant decrement-one rule, B14's mutable search step begins at2 in its own `Const` instruction; after the second failed trial, the actor's *own* `PatchConst` changes that literal to1. The host never chooses a repair candidate after initialization. However the feedback criterion, retry schedule, threshold (second failure), target node and constant values **are investigator-authored**.

## Fixed native founder: 35 nodes, 4 registers

One process id1, initial 240 credits, no replenishment or spawn, `World::with_strict_capacity(16,64,128)`. Source input key7, freshness marker key13, service output key11, actor-written failure flag key12. Unseeded marker/output/flag are absent (normal read of absent cell returns0).

| PC | Fixed opcode | Next |
| --- | --- | --- |
| 0 | Const r0=6 (service entry to edit) | 1 |
| 1 | Const r2=13 | 2 |
| 2 | Delete marker13 | 3 |
| 3 | Const r2=7 | 4 |
| 4 | Read r1 from source key7 | 5 |
| 5 | Const r3=1 | 6 |
| 6 | Const r2=13 (editable next7) | 7 |
| 7 | Write marker key13=1 | 8 |
| 8 | Const r3=11 | 9 |
| 9 | Write output key11=r1 | 10 |
| 10 | Const r2=13 | 11 |
| 11 | Read r2 from key13 | 12 |
| 12 | Branch if marker nonzero→32 else→13 | — |
| 13 | Const r2=12 (failed trial flag key) | 14 |
| 14 | Read r2 from flag key12 | 15 |
| 15 | Branch if flag nonzero→20 else→16 | — |
| 16 | Const r3=1 | 17 |
| 17 | Const r2=12 | 18 |
| 18 | Write flag12=1 | 27 |
| 19 | Halt (unreachable) | — |
| 20 | Const r2=29 (target policy node) | 21 |
| 21 | Const r3=1 (new step) | 22 |
| 22 | PatchConst node29,value=r3 (policy 2→1) | 23 |
| 23 | Const r2=12 | 24 |
| 24 | Const r3=2 | 25 |
| 25 | Write flag12=2 | 27 |
| 26 | Halt (unreachable) | — |
| 27 | Const r0=6 | 28 |
| 28 | ReadNext r1 from source node6 | 29 |
| 29 | Const r3=2 (mutable proposed step size) | 30 |
| 30 | Sub r1=r1-r3 (target edge = current edge − current step) | 31 |
| 31 | PatchNext node6,target=r1 | 32 |
| 32 | Const r2=13 | 33 |
| 33 | Const r3=11 | 34 |
| 34 | Const r0=6 | 0 |

The actor's `PatchConst` at PC22 has no special success event in the current VM. Therefore verify **observed CIL execution at PC22** and direct before/after `Process.code.nodes[29]` snapshots under the matching source and controls. The permanent new literal1 is a changed *algorithm parameter in executable code*, not just a temporary register choice. Effective code-edge changes at PC31 are observer-traced via `ExecutableEdgePatched`.

## Precommitted 30-world matrix

Five inputs `[-13,0,7,23,101]`, six experiment arms, fixed **120 attempted native instructions** from start per world; no after-start host edits. Every arm shares identical original 35-node code; initial external lesions and the one world-law option are explicitly recorded.

1. **SHAM7:** no damaged edge, marker and service work; expected no native code/policy edits, policy literal remains2.
2. **LESION10:** external node6.next10. First failed trial proposes10→8 (step2). The next trial writes service output but not marker, so actor must **self-mutate the policy literal 2→1** and then propose8→7. Markers and service resume. This is the primary positive mechanism.
3. **LESION10_POLICY_LOCKED:** same node6.next10 **plus** host-authored edge intervention node21.next22→23, which skips the `PatchConst` instruction while keeping the other CIL guard logic runnable. First propose10→8, next candidate8→6 (still step2), causing a low-value self-loop; no fresh marker is recovered. This is an **unequal-cost negative control**, not an opt-in generic VM law. Disclose both external lesions.
4. **LESION10_PATCHNEXT_DISABLED:** same node6.next10 and identical founder, but opt-in native patch-next-disabled world law. The actor can still change its policy constant to1, but no effective code-edge correction occurs; no genuine service output is produced.
5. **LESION11:** external node6.next11. The actor tests 11→9 (wrong-register incidental write to key1), then, after the second failed trial, changes policy 2→1 and tries9→8→7. Expect extra failing candidates, accounting for the cell1 side effect.
6. **LESION11_POLICY_LOCKED:** same node6.next11 plus node21.next23 (policy mutation disabled by executable bypass). The original step2 proposes11→9→7, potentially recovering in fewer trials **without** the adaptive step change. This deliberately tests for **policy overfitting**: greater capacity to change the policy does not imply universal superiority.

## Falsification, accounting and interpretation

- Each initial `PatchEdge` is external; every native `PatchNext` and `PatchConst` attempt is internally charged; no host selection of candidates after initial lesions.
- All worlds must reach120 attempted instructions without traps or process birth. Final credits120, total compute inventory120; finite storage (key1 side effect counts). If any case traps or stalls after fewer than120 attempts, do not silently repair/rewrite its graph.
- For LESION10, observe ordered edges10→8→7, **one internal effective policy literal change** 2→1, future candidate decrements using the new policy. LESION10_POLICY_LOCKED must retain literal2 and fail to reach marker/service despite an initial possible service output; process becomes trapped in a low-value loop at edge6. LESION10_PATCHNEXT_DISABLED must retain damaged node6.next10.
- For LESION11 compare adaptive-vs-locked trials and do not interpret an advantage observed only on LESION10 as general adaptive competence. Record actual PC7 marker writes, PC9 service writes, wrong writes to key1, policy edit counts and any code-edit suppression events.
- All six arms, including negative and embarrassing cases, remain in the denominator. 120 equal instruction opportunities do **not** imply equal diagnostic effort or equal task output, and external damages are researcher-designed.
- The entire search/feedback *meta-algorithm* is human-written; changing a hardcoded step in that algorithm is evidence of **native self-modifying policy machinery**, not proof the agent invented its own adaptation policy, trust criterion, individuality, or open-ended evolution.

No later choice to change the initial step, the 2→1 rule, the marker criterion or the tested lesions may be incorporated under this B14 label after seeing results. If any prediction fails, preserve it as a negative result.
