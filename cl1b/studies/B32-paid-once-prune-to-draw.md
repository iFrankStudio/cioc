# CL-1B / B32 — paid one-time native control-flow pruning preserves access to compute

**Prospective study, 2026-11-16, before native or independent outcome inspection.** This is a *single continuing CIL individual* study, not population reproduction. Building on [B31](../results/B31-native-paid-pool-sensing-audit.md), it targets the B24 zero-credit deadlock mechanism: a self-edit can permanently remove an individual's own path to the generic pool-funded `DrawCompute` if too much CIL bookkeeping must be paid at the wrong PC.

## Source and hypotheses (all behavior authored at t0)

One externally loaded PID1, **2 initial local compute credits**, zero common pool; after bootstrap it is at `DrawCompute` PC1, zero local credits. **No child process, graph launch, targeted SetCredits, fitness oracle or host code repair.** Existing generic VM laws: `with_pool_paid_draw()`, `with_recoverable_draw_shortage()`, bounded capacity (16 cells,64 program nodes,128 total occupied storage).

The test uses a nine-step *source-editing founder* (10 actual PC nodes0..9):

| PC | Native CIL instruction | next |
| ---: | --- | ---: |
| 0 | `Const r0=9` | 1 |
| 1 | `DrawCompute amount=r0` | 2 |
| 2 | `Const r2=9` output cell | 3 |
| 3 | `Const r3=1` output value | 4 |
| 4 | `Write cell9=1` | **5**, before self-edit |
| 5 | `Const r0=3` new request | 6 |
| 6 | `Const r1=4` target executable node index | 7 |
| 7 | `Const r2=1` target new edge | 8 |
| 8 | `PatchNext(node=r1,target=r2)` | 9 |
| 9 | `Const r2=9` restore output cell register | 1 |

A native `PatchNext` at PC8 edits **PC4 Write.next from5 to1**, permanently bypassing PC5–9 on future services. This is not an artificial "repair" opcode: a generic CIL source graph edge is patched and the **actual execution path** changes. It is researcher-authored structural simplification, not a learned strategy.

The initial one-time Draw9 requires a **10-unit neutral packet** to pay Draw's instruction fee1+9 transferred compute. From these9 credits the founder pays8 attempts PC2–9 and transfers to a cheap repeat loop `Draw3 → Const output address → Const output value → real Write → Draw3`. There is one local credit left at the Draw after the patch, so subsequent resource packets can be acquired safely without an unpayable normal-opcode barrier.

## Controls and fixed resource episodes

Three source/physics variants:
1. **PRUNE:** real native PC8 PatchNext, changes PC4.next5→1; subsequent Draw3 work loop is paid and continuous.
2. **PATCH_SUPPRESSED:** bit-identical original code, same PC8 attempt cost and validated next edge; opt-in `with_native_patchnext_disabled()` prevents the actual edge edit. The full bookkeeping tail repeats, potentially stranding the process at normal PC5 after a smaller Draw3.
3. **STATIC3:** independently authored simple five-opcode always-request3 loop from B20/B23, same **2 initial credits**, neutral world pool, no self-editing. This control has less source complexity by design; it is an efficiency baseline, not an otherwise identical single-opcode ablation.

Four preselected external neutral supply schedules, packet-based (not wall-clock):
- Development `[10,4,4,4]`.
- Held-out H1 `[10,3,3,3]`.
- Held-out H2 `[10,5,5,5]`.
- Held-out H3 `[9,4,4,4]` (first packet insufficient for the editing founder's Draw9; unspent pool may later accumulate).

**Twelve deterministic worlds**, plus explicit zero-flux sanity check. Before injecting any packet, drain the initial two charges; after each neutral pool packet run the native scheduler until it returns false. Record *every* charged attempt, service Write at correct PC, residual pool, local credits, PC, actual native edge in program code, and edited/suppressed event provenance. Never discard a stuck or zero-output individual.

## Registered causal tests

- In PRUNE, once the first output and source edit complete, PC4.next becomes1 via real `Origin::Internal(1)` `ExecutableEdgePatched {node:4,before:5,after:1}`. It is not an observer-side edge or host-supplied new code.
- In PATCH_SUPPRESSED, the actor spends exactly the same PC8 attempt but PC4.next remains5; a native `NativeCodeEditSuppressed {opcode_tag:6}` event must attest to a real charged suppression, not a free repair.
- Compare actual service outputs across **all four** schedules, and whether PRUNE remains at pool-payable `DrawCompute` PC1 with zero credit when exhausted, while PATCH_SUPPRESSED eventually becomes stranded at **non-Draw PC5** when its remaining compute is too small to pay the bookkeeping tail.
- The constant/static request3 founder may beat PRUNE because it does not pay the first expensive Draw9 and code-pruning costs. Any such negative result must remain in the final comparison. **PRUNE output > PATCH_SUPPRESSED** is only evidence that authored one-time source pruning works, not a demonstration of autonomous policy discovery.
- Native compute conservation: every successful step decreases local+pool inventory exactly1; external packets increase it only by the injected amount; `PatchNext` is fully charged and never creates energy.
- Changes to external supply timing / packet windows must not be made after observing outcomes. If native CI disproves a registered expectation, preserve the original failing CI and correct predictions transparently.

## Science acceptance

B32 can show that **one continuing program can restructure its own executable code to amortize maintenance costs and avoid a resource-access trap**, a meaningful engineering prerequisite for ongoing individual adaptation. It still cannot establish learning: the target edge, new resource request and simplifying operation are explicitly written by the researcher, and the edit does not condition on trial-based usefulness. The next hard gate is endogenous choice **whether** to keep a particular mutation from paid evidence under held-out environments while respecting the compute-visibility and self-certification limitations; don't call a prewired one-time refactor a spontaneous cyberlife breakthrough.
