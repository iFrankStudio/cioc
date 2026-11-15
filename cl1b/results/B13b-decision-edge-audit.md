# CL-1B / B13b — post-hoc decision-edge audit of marker-proxy statistics

**Status:** post-hoc methodological reanalysis, performed on 2026-11-15 *after* B13 outcomes were observed. **Not a newly preregistered confirmatory study and not Rust CI.** The B13 original classifications are retained unchanged.

## Issue

B13 originally classifies PC11 `Read(cell13)` itself as an "acceptance" or "rejection". But a legal single-edge mutation can change **PC11.next** to bypass PC12 `Branch`, in which case a marker read was never acted on. It is scientifically misleading to treat every read as an actual branch decision.

## Stricter criterion

Keep B13's full mutation/input universe and fixed 80-attempt windows. Count a decision **only if** an internal PC11 read of cell13 is immediately followed by execution of PC12 `Branch`, with the relevant value still in register r2. A false positive requires the branch to select success PC17 despite **zero** valid output writes since the last actor PC2 Delete. A false negative requires branch to select failure PC13 despite at least one valid output write. Cases where a marker is read but PC12 is bypassed must be reported separately and **not** counted as decisions.

## Post-hoc audit result, simplified interpreter only

Across 5 input labels, **19 mutants per input (95/1,805 mutant worlds)** contain at least one marker read whose control-flow bypasses PC12 directly. Yet in this particular frozen mutation universe **none of those bypassed reads contributed to B13's originally reported false-positive/false-negative counts**. The stricter "actual immediate branch decision" metric therefore leaves the aggregate **66 false-positive worlds, 52 false-negative worlds, 381 false-positive decisions and 82 false-negative decisions unchanged**. This is an important *surviving robustness check*, not proof that the old metric definition was sound in general.

Input-level strict false-positive worlds: `[15,6,15,15,15]`; strict false-negative worlds: `[8,20,8,8,8]`; traps `[74,66,52,70,74]`, aligned with the original simplified scan. A standalone audit script and source-pinned native comparison are required before presenting the revised criterion as VM-verified.

## Scientific interpretation

The refined metric prevents a future false claim from non-consumed observations; B13's **specific** numerical primary result survived this challenge in the independent interpreter. The marker/function mismatch remains a defect in the *human-written* evaluator. No actor-generated objective, evolutionary novelty or autonomous self-maintenance was observed. Do not convert this post-hoc concordance into prospective evidence.
