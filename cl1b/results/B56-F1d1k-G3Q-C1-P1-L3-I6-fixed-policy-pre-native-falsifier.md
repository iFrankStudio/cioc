# I6 fixed-policy falsifier — pre-native semantic counterexample

Status: **PRE_NATIVE_ONLY**, 2026-11-19. This note is a new independent calculation over the frozen I5 initial-safe semantic table, not a new Rust run. Parent verified I5 SHA `c02b6c67fb81167de5d89ff99b1dd2397737b9cf`, [Actions #44563928096](https://github.com/iFrankStudio/cioc/actions/runs/44563928096).

## Falsifiable result and consequence

The existing I6 graph proof established that all eight I5 one-edit-impossible seeds admit a two-edit path. A more stringent question is whether **choosing the second edit from newly observed evidence is necessary**. The answer for the frozen I5 first-edit policy is **no**: in **all eight** hard cases, the actual I5-predicted first patch is PC6, and the fixed second patch **PC3 (middle word) → 0** yields an initially safe three-word rule. This baseline does not inspect the new coverage witnesses to choose the second edit. It is a direct counterexample to inferring evidence-conditioned second-edit *selection* merely from 8/8 repair success.

| Initial genotype | I5 predicted first patch result | Fixed second edit result |
|---|---|---|
| [0,2,3] | [0,2,0] | [0,0,0] |
| [0,3,3] | [0,3,0] | [0,0,0] |
| [1,1,3] | [1,1,1] | [1,0,1] |
| [1,2,3] | [1,2,1] | [1,0,1] |
| [2,1,3] | [2,1,2] | [2,0,2] |
| [2,3,3] | [2,3,2] | [2,0,2] |
| [3,2,3] | [3,2,0] | [3,0,0] |
| [3,3,3] | [3,3,0] | [3,0,0] |

Reproduce: `python3 cl1b/analysis/g3q_c1_p1_l3_i6_fixed_policy_falsifier.py`. The script asserts all eight actual frozen first-patch results, distinct second edits, and membership of the final states in the frozen I5 `initial_safe` set. This is **not** an independent rederivation of CIL semantics.

## Revised native experimental discrimination

Implement a native PID1 that pays two patches, resets all three coverage flags and recertifies after each patch, and compares **evidence-selected second patch** against **fixed PC3→0**, one-edit cap, patch disabled, stale-witness and inverted-feedback controls. Because the fixed baseline already reaches 8/8 semantic repair, a claimed evidence-selection benefit must instead be shown by a prospectively blinded distribution shift where the fixed edit is not sufficient, or by a separately precommitted cost/robustness outcome. Do **not** post hoc select such cases or claim evidence-based selection was necessary for the eight frozen genotypes.

No Rust/CIL I6 native experiment or GitHub Actions success is claimed here. B3/B4 and G3Q/Hk remain unproven.
