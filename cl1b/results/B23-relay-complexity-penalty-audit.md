# CL-1B B23 — engineering control: an elaborate self-built relay can lose to a five-node solo policy

**Scientific status:** preregistered negative cost/fitness control; not endogenous adaptation or cyberlife. [Prospective B23 protocol](../studies/B23-assembly-cost-vs-solo-policy.md) preceded the [Rust integration differential](../../cl0/crates/world/tests/cl1b_b23_assembly_cost_vs_solo.rs) and its outcome inspection.

## Primary native evidence and denominator

The [B23 native Rust result in full campaign run 43730787980](https://github.com/iFrankStudio/cioc/actions/runs/43730787980) was **PASS**. That source SHA is `bfe83566316520faf45a33102f3a7a718ea45d01` and its campaign selected 24 targets; an independent B23 source-parsed observer was added **after** that specific checkpoint. The [independent B23 observer model](../analysis/b23_assembly_cost_solo_scan.mjs) evaluates two externally fixed resource schedules, 2 B22 source variants and 7 solo policies, **18 deterministic worlds**. A later CI checkpoint must be read by exact source SHA before independently promoting the observer conclusion.

All worlds start with precisely **one externally loaded process**, **29 initial local compute credits**, world pool0 and the same opt-in generic native Draw laws. All run to blocked state before neutral packets; each gets **8 total extra world-pool compute**, supplied in either `[3,5]` or `[4,4]` order. The only service counted is actual native `Origin::Internal(1)` `CellWritten(key9,value1)`. Both programs and request amounts are investigator-authored. The prewritten B22 builder has35 founder nodes; the simple solo program has5 founder nodes.

## Exact finite native comparison

| Native program (all author-designed) | `[3,5]`: true service writes | `[4,4]`: true service writes | Source nodes |
| --- | ---: | ---: | ---: |
| **Assemble five-node child relay and transfer** (B22) | **1** | **1** | **35** |
| Assemble child with same-cost no-transfer instruction | 0 | 0 | 35 |
| Solo native Draw request1 | 0 | 0 | 5 |
| Solo native Draw request2 | 0 | 0 | 5 |
| **Solo native Draw request3** | **2** | **2** | **5** |
| Solo native Draw request4 | 1 | 1 | 5 |
| Solo native Draw request5 | 1 | 1 | 5 |
| Solo native Draw request6 | 1 | 1 | 5 |
| Solo native Draw request7 | 1 | 1 | 5 |

The source-pinned Rust `--nocapture` log reports **37 charged native attempts** for the assembled-transfer organization and the request3 solo control: 29 from the initial founder endowment plus all 8 environmental credits. Their *difference is actual useful work*, not access to more energy. The assembled organization takes **43 occupied storage units** at the end, compared with **7** for the request3 solo actor (two processes plus graph/information overhead and service data vs one small program).

The request3 policy's outcome is not offered as something the individual learned or found. **The investigator selected the policy for the control.** It is a lower-complexity alternative that proves more native processes, compiled graphs and internal relays are not automatically an adaptive improvement.

## Why the result matters

B22 demonstrated *native expressibility* of constructing and launching a helper. B23 refutes the stronger, tempting interpretation that this ability by itself improves fitness/functional service in the demonstrated environments. Under both frozen equal-resource schedules, the simpler preauthored solo candidate yields **twice as much native useful output**.

An organization may still be valuable under other task distributions, scarce storage models, parallel workloads or differential damage, but those are **new hypotheses**, not evidence supplied by this two-schedule comparison. The code lengths are intentionally different; this is a cost/complexity efficiency comparison, **not** a mutation-only causal ablation that isolates the biological significance of cooperation.

The next CL-1B scientific hurdle is therefore **internal source-and-evidence-driven selection or composition of effective compute acquisition policies**, charged for exploration and robust to false internal checkers. Merely building an elaborate helper, copying five dormant nodes or selecting a known better request in the host will not qualify as adaptive autonomy. Held-out supply patterns and the entire candidate/failure denominator are required.

## Verification discipline

Do not conflate green CI with autonomous evolution. The native B23 and existing B22 code-path tests must be source pinned; independent observer analysis should run from the *same* Rust source revision. Any future edits to the test or source require a new native run. The original [full B22 predecessor checkpoint](CI-2026-11-15-full-B22-native-verification.md) remains valid for the older tested source state.
