# C28-R3 — actor-paid adaptive sampling cadence and finite schedule frontier

**Retrospective and systematic non-native engineering model, not a new native result.** Parent 057a3a432e938859aea02920d37dfbd29e3341ec. Original C28 evidence unchanged. B3/B4 NOT_MET.

## Mechanism

Use the relocated duplicated k12 production block. Register0 is a countdown in skipped blocks; a new paid block-end gate chooses measured block completion or decrements countdown. When countdown reaches zero, charged SenseOwnCredits starts the next measured block. Skipped production blocks are not included in cost samples. Actual observed block expenditure is compared to the retained baseline. After equal samples, read/write ordinary cell48 and double skip count 1→2→4→8 (cap 2/4/8, fixed per controller), after mismatch run original paid candidate evaluation and reset baseline and backoff. First measured block establishes baseline, next sampled immediately. No free host sensing, direct change flag, future horizon or reward oracle. The rule is authored and adaptive only in its cadence, not a discovered new strategy. A changed regime inside the initial baseline remains a weakness. Same-source on/off suppresses physical PatchNext only. Original compact initial-only is a distinct-source economic comparator.

Fixed skip0/1/3 use the same paid gate and reset but no adaptive arithmetic; original C28 monitor provides a separate lower-overhead fixed reference. New allocated node counts: fixed 490, adaptive 500, original 481, initial-only 113. Adaptive hits original C28 max_nodes=500 exactly; with extra cell48 peak predicted occupancy 509 (not native verified), within total520. No scalarized storage fitness. Model charges every instruction and compares equal production cutoff as C28.

## Known C28 schedules

Eight formerly prospective schedules now revealed. All model policies evaluated both on/off (except once). Economic net = initial-only minus monitored, not suppression contrast. Aggregate:
```json
{
  "once": {
    "n": 8,
    "net": 0,
    "wins": 0,
    "ties": 8,
    "losses": 0,
    "worst": 0,
    "regret": 663,
    "physical": 0,
    "nodes": 113
  },
  "original": {
    "n": 8,
    "net": 80,
    "wins": 3,
    "ties": 0,
    "losses": 5,
    "worst": -201,
    "regret": 583,
    "physical": 2052,
    "nodes": 481
  },
  "gated0": {
    "n": 8,
    "net": -100,
    "wins": 3,
    "ties": 0,
    "losses": 5,
    "worst": -222,
    "regret": 763,
    "physical": 2052,
    "nodes": 490
  },
  "skip1": {
    "n": 8,
    "net": 114,
    "wins": 3,
    "ties": 0,
    "losses": 5,
    "worst": -144,
    "regret": 549,
    "physical": 1908,
    "nodes": 490
  },
  "skip3": {
    "n": 8,
    "net": -65,
    "wins": 3,
    "ties": 0,
    "losses": 5,
    "worst": -177,
    "regret": 728,
    "physical": 1620,
    "nodes": 490
  },
  "backoff2": {
    "n": 8,
    "net": 6,
    "wins": 3,
    "ties": 0,
    "losses": 5,
    "worst": -154,
    "regret": 657,
    "physical": 1908,
    "nodes": 500
  },
  "backoff4": {
    "n": 8,
    "net": 42,
    "wins": 3,
    "ties": 0,
    "losses": 5,
    "worst": -154,
    "regret": 621,
    "physical": 1908,
    "nodes": 500
  },
  "backoff8": {
    "n": 8,
    "net": 42,
    "wins": 3,
    "ties": 0,
    "losses": 5,
    "worst": -154,
    "regret": 621,
    "physical": 1908,
    "nodes": 500
  }
}
```

## Enumerated family (systematic engineering stress, not blind)

Before executing the family in this turn, specified N=144 for both starting modes, 2 stable cases, all 143 possible single reversal boundaries per mode, transient reversals at stages 6..126 step6 with durations 1/4/12, and double reversal stages 12..72 step12 with durations 24/48. Total 438 equally enumerated schedules, each evaluated with all policies and physical-effect arms; no selection by world. This is an artificial weighting (single changes dominate), not a real environment distribution. Regret uses per-case best of once/original/gated0/skip1/skip3 in hindsight, a diagnostic envelope, not an executable oracle; adaptive can have negative per-case regret. Aggregate:
```json
{
  "once": {
    "n": 438,
    "net": 0,
    "wins": 0,
    "ties": 438,
    "losses": 0,
    "worst": 0,
    "regret": 33462,
    "physical": 0,
    "nodes": 113
  },
  "original": {
    "n": 438,
    "net": -10592,
    "wins": 178,
    "ties": 0,
    "losses": 260,
    "worst": -296,
    "regret": 44054,
    "physical": 111326,
    "nodes": 481
  },
  "gated0": {
    "n": 438,
    "net": -22486,
    "wins": 166,
    "ties": 0,
    "losses": 272,
    "worst": -326,
    "regret": 55948,
    "physical": 111326,
    "nodes": 490
  },
  "skip1": {
    "n": 438,
    "net": -4850,
    "wins": 190,
    "ties": 0,
    "losses": 248,
    "worst": -294,
    "regret": 38312,
    "physical": 105430,
    "nodes": 490
  },
  "skip3": {
    "n": 438,
    "net": -2500,
    "wins": 142,
    "ties": 0,
    "losses": 296,
    "worst": -281,
    "regret": 35962,
    "physical": 95660,
    "nodes": 490
  },
  "backoff2": {
    "n": 438,
    "net": -14448,
    "wins": 166,
    "ties": 0,
    "losses": 272,
    "worst": -312,
    "regret": 47910,
    "physical": 102072,
    "nodes": 500
  },
  "backoff4": {
    "n": 438,
    "net": -12704,
    "wins": 166,
    "ties": 0,
    "losses": 272,
    "worst": -312,
    "regret": 46166,
    "physical": 95330,
    "nodes": 500
  },
  "backoff8": {
    "n": 438,
    "net": -12704,
    "wins": 166,
    "ties": 0,
    "losses": 272,
    "worst": -312,
    "regret": 46166,
    "physical": 95330,
    "nodes": 500
  }
}
```

Per-family summaries including negative cases are in the machine-readable aggregate. Fixed parameters are not tuned separately per world. Note that cap8 may not be reached before finite cutoff. The adaptive policy does not learn the value of evaluation, only stable observation duration. Comparison with fixed skip1 and initial-only must remain central; positive native physical edit contrast is not enough.

## Reproduction

`node cl1b/analysis/cl1b_c28_adaptive_sampling_frontier.mjs` emits all individual scenario/arm costs, probe stages, physical edits, actual observed sampling stages, and aggregate regret. The source body and enumeration ran in connector-side JS; no native CI claimed. [Machine-readable aggregate](CL1B-C28-R3-adaptive-sampling-frontier.json). No native promotion until performance across stable and changing cases justifies cost; next investigate a cheaper actor-paid outcome signal for *realized* evaluation return rather than using stability alone. Keep the original native evidence and no per-world tuning. B3/B4 NOT_MET.
