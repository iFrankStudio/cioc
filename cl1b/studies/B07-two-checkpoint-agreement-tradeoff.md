# CL-1B / Study B07 — two-copy checkpoint agreement versus correlated memory faults

**Pre-registration: 2026-11-15, before B07 test authoring, independent B07 path simulation or native trial execution.** This is a **human-designed fault-tolerance calibration**, not a discovery of emergent trust, biological selection, or cyberlife. All failures remain visible. The protocol is deliberately agnostic about the exact service-output count until a native trace is available; no post-hoc performance success criterion is introduced.

## Question

B06 showed that a single poisoned checkpoint can make a code-maintenance routine damage *healthy executable code*. Does requiring **two separately written CIL checkpoint cells to agree** protect a single process from an independently corrupted checkpoint? What does this cost when the executable itself is damaged, and when both checkpoints are corrupted identically?

The actor has **no infallible copy or host repair oracle**. Both checkpoint cells are writable, can be corrupted and cost finite storage. Two equal wrong values can fool the policy. A mismatch is not magically resolved by an omniscient Rust checker: the human-authored native CIL simply declines to edit when the references disagree.

## Frozen single-actor CIL program (24 nodes)

One founder process id1, 250 initial compute credits, strict world storage64, no pool, no child launch. The researcher externally writes only the service input to cell7. CIL generates checkpoint cells12 and13 by reading the *same active own-code edge* at node8, originally next9.

| PC | Opcode / meaning | next or branch |
| --- | --- | --- |
| 0 | Const r0=8 (monitored instruction) | 1 |
| 1 | ReadNext r1,node=r0 (original 9) | 2 |
| 2 | Const r2=12 | 3 |
| 3 | Write checkpoint12=r1 | 4 |
| 4 | Const r2=13 | 5 |
| 5 | Write checkpoint13=r1 | 6 |
| 6 | Const r2=7 (service source) | 7 |
| 7 | Const r3=9 (service output) | 8 |
| 8 | Read r1,cell7 (target: next9 normally) | 9 |
| 9 | Write cell9=r1 (service) | 10 |
| 10 | ReadNext r1,node=r0 | 11 |
| 11 | Const r2=12 | 12 |
| 12 | Read r2,cell12 | 13 |
| 13 | Const r3=13 | 14 |
| 14 | Read r3,cell13 | 15 |
| 15 | Sub r2=r2-r3 (compare saved copies) | 16 |
| 16 | Branch r2 nonzero22,zero17 | — |
| 17 | Sub r2=r1-r3 (live edge vs agreed saved edge) | 18 |
| 18 | Branch r2 nonzero19,zero22 | — |
| 19 | PatchNext node=r0,target=r3,next22 (native restore or harmful patch) | 22 |
| 20 | Halt (unreachable sentinel) | — |
| 21 | Halt (unreachable sentinel) | — |
| 22 | Const r2=7 | 23 |
| 23 | Const r3=9 | 8 |

Pre-fork state is produced by exactly 8 CIL attempts: next PC8, **two internal** checkpoint writes (PC3: cell12=9, PC5: cell13=9), 242 local credits. Clone this full state before the experimental lesions. Any host `SetCell` and `PatchEdge` applied after this fork are **external intervention provenance**, not a native CIL recovery or mutation.

## Frozen 35-world factorial subset

Five independent *input labels* `[-13,0,7,23,101]`, seven arms per input (35 deterministic worlds), **100 further CIL attempts**, 150 ending credits, 0 minted compute, one and only one process. Equal attempts are a *resource exposure control*, not necessarily equal completed cycles or age.

1. **CLEAN:** no damage. Expect both saved values9, live next9, no native edits, correct service output.
2. **CODE_DAMAGE:** external PatchEdge node8.next9→10 (skips service write). Saved copies9/9. Expect exactly one **internal** native repair (node8:10→9), eventually restored service output.
3. **SINGLE_POISON_12:** only external SetCell12=10. Code remains healthy next9; references disagree (10 vs9). Expect **zero** internal code patches and continued correct service output.
4. **SINGLE_POISON_13:** only external SetCell13=10; same as arm3 symmetrically: zero code patches, healthy service.
5. **CORRELATED_DOUBLE_POISON:** external SetCell12=10 and SetCell13=10; code initially healthy next9, but two agreeing wrong references10/10. Expect **exactly one harmful internal edit**, node8:9→10, and later failure to produce service despite initially healthy code.
6. **CODE_DAMAGE_AND_SINGLE_POISON:** external code next9→10 and SetCell12=10; conflicting references10/9. Expect **zero** native repair and no correct service output (fail closed sacrifices availability).
7. **CODE_DAMAGE_PATCH_DISABLED:** external code next9→10, original saved copies9/9, plus explicit opt-in world law `with_native_patchnext_disabled()`. Expect **zero effective code repairs**, at least one charged suppressed native patch attempt, no correct service output.

No host restores a code edge after initial lesion. The input cell is not a host life score. Service success is trace-observed `CellWritten(key9,value=input)` with `Origin::Internal(1)`. For the correlated-poison arm, record exactly when the first service output stops; do not discard initial successful service as noise.

## Primary endpoints / falsification

- Both checkpoints are created only via **internal CIL** `ReadNext` + `Write`, with paid native instructions and finite storage; no checkpoints are researcher initialized.
- All seven arms start with the **same single founder code**; founder itself runs and owns the decisions. Compare total process count, no spawning, absence/presence of traps, origin of every mutation, number of attempted edits, effective edits and overall compute inventory.
- Confirm both opposed outcomes rather than cherry-picking: a single poisoned copy prevents unwanted edits **but also blocks legitimate recovery** from combined code damage; two correlated poisoned copies can still cause self-damage. The suppression arm demonstrates that a CIL patch — not merely the poison — changes the executable.
- Record all 35 worlds including failures. Do not claim a life detection statistic, open-ended self-evolution, spontaneous trust or adaptive improvement from this **researcher-authored** equality guard.
- A mismatch in native-world counts or branches is **experiment failure**, not permission to rewrite the preregistered prediction as success.

## Next scientific issue

These experiments expose a **safety–availability–correlated-failure tradeoff**. Future research should use alternative checkpoint generation and verification policies, withheld multi-site corruptions, time/schedule symmetry controls, and resource budgets that make redundant copies costly. In particular an agent-proposed checkpoint-aggregation policy must be tested against false repairs from correlated damage, without a hardcoded trusted monitor or externally assigned fitness reward.
