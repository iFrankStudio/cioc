# CL-1 Study 25 — prospective source-graph-content causal contrast (2026-11-13)

**Technical experiment VERIFIED. Prespecified strict output-value causal contrast: 0 exposed worlds. Autonomous CL-1: NOT ACCEPTED.**

## Preregistration and source-pinned evidence

- [Frozen preregistration](../studies/25-preregistered-prospective-code-content-causality.md) was committed at `674ccbc3f275fb20c1ff9b0dc3e28b24eb3d579f`, before runner, audit implementation, fresh-seed simulation or outcome inspection.
- [Verified CL-1 CI run 43451819901](https://github.com/iFrankStudio/cioc/actions/runs/43451819901) at source revision `78ec5b70719fc1201083e8ab3ef388ad145b9ef2`. Both quick workspace/strict lint/Python gate and Study-25 targeted ecological job passed. The study job ran exact Rust SQLite epoch replay, read-only independent Python scientific audit and archived raw SQLite plus JSON for 30 days.
- **96 new independent seed-worlds (2,250,000..2,250,095)** × **9 matched arms** = **864 paired runs** and **20,736 separately hashed epoch snapshots**, all accepted; every world is kept including stopped and trapped worlds. These seed-worlds are disjoint from Studies 14, 22 and 24. Matched branches are **not independent statistical replicates**.

## Frozen manipulation

Without changing the CIL instruction vocabulary, the researcher externally deleted source graph1, graph2 or both, then reintroduced the same graph handles and owner/reader permissions, but with node0 replaced by `Halt` (tag 19). An additional control reintroduced graph1 with **byte-identical original content**, and an untreated/rejected sham contrast preserved identical simulated state. The other arms independently controlled externally granted compute and zero compute. All injected graph code, including the content-changed variant, is explicitly attributed to `Origin::External` and is not an organism's internal modification.

The narrow primary endpoint requires matching successful source `GraphTagRead` events (same actor, graph and epoch) in a seed's untreated and manipulated arms, reading **different tags**, followed by successful same-actor `CellWritten` operations to the **same cell** in that epoch but with **different numeric output values**. This is stricter than counting graph-code copies, read counts or same-epoch write totals.

## Observed independently audited outcomes

| Registered endpoint | Result |
| --- | ---: |
| Worlds with successful foreign-graph read in untreated W1 | **40/96** |
| Worlds with foreign-graph read followed by a same-actor same-epoch write in untreated W1 (temporal only) | **4/96** |
| Worlds with strict paired changed source tag + changed written numeric output, mutation of graph1 | **0/96** |
| Worlds with strict paired changed source tag + changed written numeric output, mutation of graph2 | **0/96** |
| Worlds where previously observed control source-read/output candidate was absent in graph2 mutation | **3/96 (descriptive, ambiguous)** |
| Untreated W3 worlds with any charged CIL execution | **0/96** |
| W3 worlds with charged execution under externally supplied compute | **55/96** |
| W3 worlds with successful foreign-graph reads under external compute | **7/96** |
| W3 worlds with same-actor graph-read-then-write under external compute | **0/96** |

The three graph2 candidate dropouts are **NOT** strict proven causal output loss: the alternate branch may have changed path, stopped, trapped or failed to read the modified source; the primary required paired read/changed value evidence is absent. Even a mutation-induced dropout would be an **externally applied code-content injury to externally seeded programs**, not autonomous functional reconstruction.

**Interpretation:** In the new seed distribution, foreign code reading and a handful of same-epoch read-then-write opportunities are real VM phenomena. Yet the prespecified causal output value criterion was not exposed under paired content mutation. This is a **narrow null exposure for the registered criterion**, not an estimate that causal functionality is mathematically impossible and **not evidence of life**. Endogenous design, multi-component organizational closure and source-sensitive internal repair across two later windows remain **NOT EVALUABLE**.

## Scientifically necessary next step

Distinguish actual control-source-read/altered-source-read pairs from path loss and trap-mediated output dropouts using an independently pre-registered source-content negative-output analysis; do not reinterpret missing writes or copy events as success. Future actual functional maintenance testing requires source-qualified internally initiated reconstruction, a *same-function* external restoration positive control, exact budget matching, sham/no-repair/zero-credit controls, and two separated windows after observed functional damage.

**Stage verdict: CL-1 OPEN.**
