# CL-1B / B13b — post-hoc decision-edge audit of marker-proxy statistics

**Status:** post-hoc methodological reanalysis, performed on 2026-11-15 *after* B13 outcomes were observed. **Not a newly preregistered confirmatory study.** A subsequent actual Rust CI run supplied matching raw/strict counts but the test initially failed because it asserted the wrong strict event total. The B13 original classifications are retained unchanged.

## Issue

B13 originally classifies PC11 `Read(cell13)` itself as an "acceptance" or "rejection". But a legal single-edge mutation can change **PC11.next** to bypass PC12 `Branch`, in which case a marker read was never acted on. It is scientifically misleading to treat every read as an actual branch decision.

## Stricter criterion

Keep B13's full mutation/input universe and fixed 80-attempt windows. Count a decision **only if** an internal PC11 read of cell13 is immediately followed by execution of PC12 `Branch`, with the relevant value still in register r2. A false positive requires the branch to select success PC17 despite **zero** valid output writes since the last actor PC2 Delete. A false negative requires branch to select failure PC13 despite at least one valid output write. Cases where a marker is read but PC12 is bypassed must be reported separately and **not** counted as decisions.

## Post-hoc audit result, simplified interpreter only

Across 5 input labels, **19 mutants per input (95/1,805 mutant worlds)** contain at least one marker read whose control-flow bypasses PC12 directly. Yet in this particular frozen mutation universe **none of those bypassed reads contributed to B13's originally reported false-positive/false-negative counts**. The stricter decision metric leaves the **per-world categories unchanged** — **66 false-positive worlds and 52 false-negative worlds** — but **changes the event-level count**:

- **381** misleading marker **reads** before a potentially accepting branch;
- **355** actual false-positive **branch decisions** (26 readbacks did not reach PC12 before the fixed 80-attempt observation window ended);
- **82** false-negative readbacks and **82** actual false-negative branch decisions.

The 95 marker-read-bypass mutant worlds are a **different phenomenon**: this finite mutation sample contains no extra false-positive event counts from those bypassed paths. The **26 event discrepancy is horizon censoring**, not an extra set of underlying damaged edge types.

The native [focused workflow run 43723283061](https://github.com/iFrankStudio/cioc/actions/runs/43723283061) printed the same **381 raw / 355 strict** totals; that run failed only because the Rust test had erroneously asserted 381 for the **strict** value. The expected strict assertion was subsequently corrected, and a new source-SHA-pinned passing run remains necessary before formally accepting this Rust test.

Input-level strict false-positive worlds: `[15,6,15,15,15]`; strict false-negative worlds: `[8,20,8,8,8]`; traps `[74,66,52,70,74]`, aligned with the original simplified scan. The existing [B13 source-parsed scanner](../analysis/b13_edge_mutation_scan.mjs) reproduces both totals, and the native log provides an independent runtime comparison. Updated native CI acceptance remains pending.

## Scientific interpretation

The refined metric prevents a future false claim from non-consumed observations; B13's **per-world primary categories** survived this challenge, but its **event-level false-acceptance count had to be corrected**. The marker/function mismatch remains a defect in the *human-written* evaluator. No actor-generated objective, evolutionary novelty or autonomous self-maintenance was observed. Do not convert this post-hoc concordance into prospective evidence.
