# CL-1B C27-R2 — absolute lifetime economics and paid-monitor break-even

**Evidence tier: retrospective non-native source-cost model; zero new Rust worlds, zero new heldout cases.** Baseline main `7da2305d17fa1212088a1cf63306559261c31f0c`. Original native evidence [45206215513](https://github.com/iFrankStudio/cioc/actions/runs/45206215513) and [archive 45206590441](https://github.com/iFrankStudio/cioc/actions/runs/45206590441) remain unchanged. B3/B4 NOT_MET.

## Why the prior positive effect does not establish absolute profitability

C27's matched physical-effect suppression identifies the effect of allowing source edits *conditional on the same paid measurement policy*. Both arms pay for two candidate comparisons. An economic comparator which avoids evaluation entirely is distinct from that causal control and is necessary to assess whether the mechanism pays for itself.

The narrow deterministic model walks the exact frozen 170-node source with one debit per CIL and post-debit credit readings, preserves the ordinary stage-12 mode intervention and PatchNext suppression, and reproduces all eight historical costs (625/673, 627/673, 626/626, 624/720), production counts and edited edges. It is not a new independent comparison of original native traces; that was already done by C27. It is deliberately scoped to the encountered opcodes, and is not a substitute for the Rust runtime.

New retrospective comparators:
- oneShot: change only PC162.zero to 100 (no second measurement); preserve the initial paid measurement, physical on/off, stage accounting and environment.
- fixedA/fixedB: start at PC100 (no initial trials); PC162.zero=100; set initial PC100.next to 101/131 respectively. Retain the charged stage accounting, even the now redundant stage-12 check. These are different initial configurations, not a same-source single-factor causal claim. The best fixed arm is selected retrospectively; no agent receives that oracle.

| Regime | C27 on | oneShot on | fixed A | fixed B | C27 excess vs oneShot | C27 excess vs best fixed |
|---|---:|---:|---:|---:|---:|---:|
| 0→1 | 625 | 576 | 478 | 478 | 49 | 147 |
| 1→0 | 627 | 575 | 478 | 478 | 52 | 149 |
| 0 stable | 626 | 528 | 430 | 526 | 98 | 196 |
| 1 stable | 624 | 527 | 526 | 430 | 97 | 194 |

All model arms complete 24 production writes; C27 executes 4 trial writes, oneShot 2, fixed 0. C27's net advantage over physical-effect suppression remains valid and is not contradicted. The absolute economics are different: even in stable mode1 the initial paid comparison costs 97 and the 24 cheaper services save only 96 vs fixed A (527 vs 526). In changed regimes the second comparison and associated decision/readback/edit costs 97 or 100, inferred by adding back the 48 production savings to the paired excess vs oneShot. Stable overhead is 98/97. Hence the second comparison needs at least 25 or 26 *future* services at a 4-CIL advantage to earn a strictly positive return, but C27 has only 12 left. This is a cost decomposition of deterministic model counterfactuals; not a native proof.

Reproduce from root: `node cl1b/analysis/cl1b_c27_absolute_economics.mjs`. The eight historical totals and one-shot totals are asserted, not claimed as new native observations.

## Design consequence for the next experiment

Do **not** simply move the 97–100-CIL full probe to an event gate or repeatedly poll the environment. This cannot earn its cost in the current 24-service mid-reversal setup even with perfect detection. Separate cheap *production-cost observation* from expensive candidate evaluation:

1. At sparse actor-scheduled production boundaries, bracket actual production with charged SenseOwnCredits; retain the measured baseline in ordinary actor storage and compare against a fresh production interval. Account for every debit including cadence counter, storage, comparison, failed observations and scheduling. A raw mode/cell50 change detector is forbidden for the monitoring policy; the environmental input can still be read by the service itself.
2. Only trigger a paid alternative trial when the observed production cost departs from its stored baseline. Test candidate output and read live source before changing it; do not assume that any cost change means the alternative is cheaper. Retain a fail-closed path and count rollback, failed trials and no-op writes. Do not expose the future horizon/change time or expected reward as an actor oracle.
3. Compare fixed midpoint evaluation, initial-only evaluation, and cost-triggered evaluation under matched initial source/effect-on/off where applicable; separate distinct-source economic baselines. Freeze heldout change times, stable cases, transient spikes and reversals before executing; report every world, actual cost and completion. Use a longer lifecycle as a separate declared treatment rather than silently lengthening the old experiment. A fixed-policy comparison must not choose per-world winners as an agent-accessible baseline.

For an accounting bound, if monitoring uses M charged instructions, trial and switch cost E, and there are R useful post-switch services with actual saving delta, incremental net gain is at most delta*R-M-E, before other harms. To beat the initial-only strategy in C27's changed cases, the combined new monitoring and switch overhead must be strictly below 48; late detection reduces that ceiling by 4 per missed service. Stable controls have zero switching benefit, so paid monitoring must be explicitly counted as loss. This condition is a falsifier, not a policy that knows R.

**Next gate:** implement and freeze a low-cost production-metrology controller with a paid audit of the realized observation overhead before scaling to prospective native holdouts. No candidate invention, autonomous change detection, or B3/B4 success claimed here.
