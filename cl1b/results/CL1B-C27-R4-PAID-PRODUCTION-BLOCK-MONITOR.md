# C27-R4 — paid production-block metrology and monitoring economics

**Retrospective engineering model only. Zero new native worlds, no prospective holdout, no B3/B4 advance.** Parent c4480b820e3d7d0fd16435e67fe0e4e441a403aa. C27 original frozen evidence is untouched.

## Mechanism

Use C27-R3 compact candidate metrology and bounded comparison. Extend the 202-node source to 219 nodes. After each evaluation reset the block baseline (cell47) and set a countdown in r0. Each block begins at PC206 SenseOwnCredits (post-debit), stores the observation in cell46, and ends at PC209 SenseOwnCredits. Ordinary paid arithmetic/read/write computes and compares actual block expenditure. The first block establishes a baseline; a subsequent nonzero difference triggers PC0 candidate re-evaluation; equality restarts the sample without evaluating alternatives. PC202–218 does not read cell50 or any host change flag. Candidate and production paths still legitimately read their normal environment cell50. Thus the timing of full re-evaluation is conditional on actual expenditure, not a preselected change point, but the monitoring interval and rule are researcher-authored. Changes prior to the initial baseline or between observation windows can alias; a transient may trigger wasteful trials. No candidate invention.

The first version (monitor3) adds three charged operations per production service for countdown and branching. The revised monitor2 reuses the stage-counter constant one at PC153, adds one Sub at PC156 and one Branch at PC157 (two operations per service). No new opcode. Each comparison block has paid sense, data persistence, arithmetic, branch and baseline reset overhead. Both physical-effect arms start with identical source for each interval.

All policies run until the external evaluation horizon stops them immediately after the Nth paid stage-counter Write at PC155, not native Halt. The horizon is not an actor input; no future task/change time is available. Equal completion and all charges up to this cutoff are compared. The fixed12 arm re-evaluates exactly once after stage12 (not every 12). The initial-only comparator has no subsequent monitoring and no redundant stage gate. Initial-only and fixed12 are different source policies, not single-effect causal ablations. All cases are deliberately selected engineering tests and may not be labeled holdout. No native capacity/storage allocation or scheduler cost claims: this model charges one per CIL and counts occupied cells only implicitly; a native study must freeze feasible budgets.

## All 10 selected scenarios (cost through equal service cutoff)

| Scenario | N | Once | Fixed stage12 | monitor3 k6 | monitor2 k6 | monitor2 k12 | monitor2 k24 |
|---|---:|---:|---:|---:|---:|---:|---:|
| A→B after 2 |48|814|861|960|914|910|928|
| A→B after 17 |48|754|949|924|878|850|868|
| B→A after 17 |48|750|941|926|880|852|864|
| A→B after 43 |48|650|845|888|842|790|764|
| A stable |48|630|825|868|822|770|744|
| B stable |48|626|817|864|818|766|740|
| A→B after 19→A after 21 |48|638|833|932|886|834|752|
| B→A after 19→B after 55 |96|1346|1681|1838|1744|1640|1776|
| A→B after 17 |96|1522|1861|1748|1654|1574|1618|
| A→B after 17 |240|3826|4597|4220|3982|3746|3712|

All complete N production writes. All 30 monitor2 on/off pairs, exact probe stages, paid block costs and physical edit sequences were evaluated in the narrow model and are reproduced by the script. The 240-service case is an explicitly exploratory extension added after the short cases, not prospective validation. It yields 80 and 114 net paid CIL savings versus initial-only for k12/k24; k6 loses 156. No other listed scenario beats initial-only, though several beat fixed12. Stable k12 costs an extra 140 in 48-service cases. Transient k12 spends on a needless re-evaluation (no edit). For k12 in 48-service reverse, actual source is patched B at stage0 then restored A at stage24. For k24 in the twice-reversal scenario only stage48 evaluation occurs, missing the second reversal before cutoff. Suppression can change downstream measurements, so on/off estimates bundled physical-edit effects, not an isolated rollback effect.

## Implication

The current monitor has structural recurring cost; merely speeding the full evaluation does not fix it. The asymptotic monitoring overhead of monitor2 in stable mode is roughly two paid instructions per production plus the per-block sample overhead; increasing k amortizes overhead while delaying or missing detection. A sustained 4-CIL advantage can overcome it at longer horizons, whereas stable or transient conditions cannot. The apparent positive long-case result is conditioned on a deliberately extended persistent regime; it is not evidence of general economic self-evolution.

Next priority: eliminate always-on cadence work where possible by using existing control-flow checkpoints or actor-generated sparse schedules; use a separate native feasibility gate with explicitly accounted source and storage capacity and independently replayed original events. Before native heldout execution freeze an untouched distribution including short/long lifetimes, stable modes, transient and multiple changes, and specify whether stop occurs by world cutoff or actor Halt. Keep initial-only and fixed policy economic references and same-source on/off. Report negative cases without selecting the best interval per world. B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c27_paid_block_monitor.mjs`. Source generation, deterministic charge interpreter, full rows and expected on totals are included. The script was transcribed from the evaluated JS model; no CI/native run is claimed.
