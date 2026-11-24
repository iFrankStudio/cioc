# C27-R3 — bounded signed comparison and compact paid metrology

**Evidence tier: retrospective narrow CIL model engineering preflight. No new native execution, independent native replay, prospective holdout, or B3/B4 advance.** Base main `8703127184c941ea2ea0e1e9b4e4ce055785a80c`, original frozen C27 source and eight native cases unchanged. See [C27-R2](CL1B-C27-R2-ABSOLUTE-LIFETIME-ECONOMICS.md).

## Source-level mechanisms

The original C27 comparison loop PC73–78 decrements both complete measured costs; it spends 55–56 CIL per evaluation at these measured costs. A bounded signed-difference comparator first constructs A−B and B−A, then tests successive positive distances 1..8 using ordinary Sub and Branch. At equal or out-of-range differences it returns to PC100 without edits (preserve actual current edge). This is a researcher-authored bounded decision policy, not general numerical ordering or a learned comparator. The range is selected retrospectively for engineering and includes the known gap of four. No new kernel opcode, free host comparator, or host cost oracle. The prototype appends 32 nodes (170→202); original node/storage capacity cannot be reused unchanged. Both code capacity and allocated storage must be accounted for in any native follow-up.

Compact metrology saves only the first measured interval in ordinary cell41, rather than storing three full credit readings. PC0/30 bracket trial A; PC34/60 bracket trial B. Each trial span begins after a charged post-debit sense and ends at the next charged sense; both paths have identical measurement framing. Measured spans become 7/11 instead of 9/13 because bookkeeping is moved outside the measurement spans. Four charged senses replace three per round, but fewer intermediate writes/reads and reduced comparison cost improve total expense. The production paths and actual PatchNext/ReadNext code are unchanged, including choice bookkeeping cell44 for the known regimes. On abstention cell44 may retain an old value; physical live edge, not cell44, is authoritative.

## All original known-model cases

| Regime | C27 original on/off | bounded comparator on/off | compact on/off | compact initial-only on/off |
|---|---:|---:|---:|---:|
| 0→1 | 625/673 | 552/600 | 534/582 | 532/532 |
| 1→0 | 627/673 | 554/600 | 536/582 | 528/528 |
| 0 stable | 626/626 | 556/556 | 538/538 | 484/484 |
| 1 stable | 624/720 | 548/644 | 530/626 | 480/576 |

The evaluated narrow model reproduces C27's original eight total charges, preserves 24 production writes in every case and the same physical edit sequence (including redundant B→B write). The compact policy reduces aggregate on cost 2502→2138 (364, 14.55%). Strict same-initial-source suppression effect remains 48/46/0/96 respectively. However, relative to its own compact one-shot comparator, two-round on still loses 2/8/54/50 CIL. In the changed cases second-evaluation overhead inferred from model totals is 50 and 56 CIL, versus 48 available service savings. Under the fixed 4-CIL gap, 13 and 15 useful services respectively are needed for strictly positive net return; C27 has 12.

## Reproduction and next falsifier

`node cl1b/analysis/cl1b_c27_compact_metrology.mjs` builds the explicit 202-node graph and produces the full case table; it asserts expected known-model results and includes an exhaustive 32×32 positive-input comparator dispatch check (1,024 cases, including ties and out-of-range abstentions). The table above was executed in a narrow JavaScript interpreter during this research turn; the repository script adds the comparator assertions for reproducibility. This is not an original native artifact and does not replace native world or independent trace verification.

The remaining work is not to polish away two instructions on these already-seen scenarios. The next causal gate should preserve the compact measurement principle but implement a cheap sparse *production* observation that compares actual charged intervals, without reading cell50 as a change oracle. Only an observed cost change may trigger paid alternative evaluation. Separate monitoring loss in stable cases from adaptation payoff after real change; include the compact initial-only policy as an economic comparator. Freeze novel durations, multiple change points, no-change, transient and insufficient-payback controls before first native execution, account for the extra source capacity, report failures and abstentions, and use matched source-effect suppression independently of economic comparisons. B3/B4 NOT_MET.
