# CL-1B B52 — paid post-Write feedback, reopened search, and residual feedback-freshness alias

**Result:** H52a native mechanism verified and H52b narrow, bounded positive transfer within the *researcher-authored* affine grammar; **H52c freshness/causal protection NOT MET**. **CL-1B B3/B4 NOT MET; cyberlife not established**.

## Exact native source and independently verified CI

**B52 protocol registered before native implementation:** [B52 prospective hypotheses and controls](../studies/B52-paid-revalidation-reopenable-search-preregistration.md). It stays frozen regardless of the mixed outcome.

- **Source SHA** `380d0ce2fdaedaea4ccf835ea9bb87b7bf63f83f`, [GitHub Actions run44062660190](https://github.com/iFrankStudio/cioc/actions/runs/44062660190): **scope=changed-only, 6/6 selected real Rust native targets PASS; 40/40 independent observers PASS; B31/B34 ABI PASS; 0 native and 0 observer failures; evidence publisher PASS**. This is **not** a full 53/53 native regression.
- [Original native/independent run summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/380d0ce2fdaedaea4ccf835ea9bb87b7bf63f83f-44062660190-1/summary.txt) · [original source SHA and run conclusion](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/380d0ce2fdaedaea4ccf835ea9bb87b7bf63f83f-44062660190-1/result.txt) · [original independent B52 scientific output](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/380d0ce2fdaedaea4ccf835ea9bb87b7bf63f83f-44062660190-1/b52_native_paid_revalidation_replay.json) · [actual native trace](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/380d0ce2fdaedaea4ccf835ea9bb87b7bf63f83f-44062660190-1/b52_native_steps.jsonl) · [B52 Rust](../../cl0/crates/world/tests/cl1b_b52_native_paid_revalidation.rs) · [B52 independent source-parsed JavaScript VM](../analysis/b52_native_paid_revalidation_replay.mjs).
- Original source `73fee420` [run44062434714](https://github.com/iFrankStudio/cioc/actions/runs/44062434714) was **not** green: native B52 passed but independent observer failed its own source-preflight guard because it searched for `proc.pc=72` while the actual Rust source had `p.pc=72`. The corrected [run44062660190](https://github.com/iFrankStudio/cioc/actions/runs/44062660190) preserved unchanged native B52 code, fixed the observer guard, and demonstrated **zero actual per-event divergence**. Do not silently omit this failure or claim independent verification of the earlier failed run.

## What the single native actor actually does

B52 imports **all 72 unmodified original executable B51c CIL instruction nodes**, append-only adds 20 ordinary CIL instructions, and starts at node72. Its **paid native `PatchNext`** at node74 changes node49's next edge 50→76, so the *original* researcher-authored B51c candidate-selection program routes its future source-retention writer to a newly appended feedback-monitor sequence. The host **never** writes actor executable code or chooses the replacement source. All source-generated candidate cloning/patching, source-route retention, post-output feedback `Read`, failure `Branch`, actor-origin search-reopen marker `Write` at cell25, reinitialization of actor-local offset/sign cells, and subsequent re-selection are **paid native instructions executed within one PID1 without reset**.

A genuine output `Write` to cell11 is followed by the **laboratory** sending a 1-bit correctness value to ordinary cell20 using an `Origin::External` intervention. Only later may the actor pay `Read` from that data cell. A source-native `Write{key:25,value:1}` explicitly requests the next frozen [5,-3] training probes for a reopening search. Heldout service probes follow [11,-7,0]. At no time does the lab set the target task sign/offset in an actor-readable cell or inject the correct program. However, the **task objective, observable truth-bit evaluator, fixed input schedule, arithmetic grammar, watchdog trigger and search algorithm are all researcher-authored**. The experiment is a *closed-loop engineering mechanism*, not evolution of the mechanism.

Exactly **20 native PID1 worlds**: two hidden task modes A `x-2→-x+3→x-2` and B `-x-3→x+2→-x-3`, two paid initializer phases and five identical-source arms. Each arm contains 4 worlds and **12,000 actually paid CIL instruction events**, all with the original **3,000-credit local budget per world**, <=16 cells, <=160 nodes, <=256 storage, no credit top-up, no second actor.

## Actual source-pinned measurements, no favorable-world exclusions

Each stage allows up to 24 **service** outputs per world; training candidate outputs are separately charged and not counted as heldout service success.

| Arm | Service correct | Service wrong | Service outputs realized | Native source retentions | Native reopens | Paid CIL steps |
|---|---:|---:|---:|---:|---:|---:|
| TRUE | **280** | **8** | 288/288 | 12 | 8 | 12,000 |
| STALE_SUCCESS | 192 | **96** | 288/288 | 4 | 0 | 12,000 |
| FORCED_ZERO | 64 | 0 | **64/288** | 64 | 64 | 12,000 |
| STATIONARY_TRUE | 288 | 0 | 288/288 | 4 | 0 | 12,000 |
| STALE_DELAYED | 272 | 16 | 288/288 | 12 | 8 | 12,000 |

**TRUE:** all 4 worlds reopened paid search *twice*: the selected source states were `(+1,-2)→(-1,+3)→(+1,-2)` for A and `(-1,-3)→(+1,+2)→(-1,-3)` for B, **with no process reset**. Across four worlds, stage correctness was **[96,92,92]**; wrong submissions **[0,4,4]**. The first wrong service output after each hidden mode switch is causally required to discover the change using a post-Write correctness bit; subsequent 23 heldout service writes in that stage were correct in every world. In total TRUE had **842 paid service feedback Reads**, **136 post-initial-retention source edge modifications**, and **8 CIL-origin search-reopen marker events**. The matched budget/time-pipeline avoids treating a merely external bit assignment as agent-side sensing.

**STALE_SUCCESS:** held-out correct by stage **[96,0,96]**, with **96 incorrect fresh-task outputs falsely scored as success** by the replayed bit and **zero** reopens or code modifications. The return-stage correctness came solely from persistent old code, not adaptation. Crucially, this arm executed **1,104 paid service feedback Reads** yet never recovered: *having a paid Read is necessary but insufficient for trustworthy feedback*.

**FORCED_ZERO:** all 4 worlds paid exactly 3,000 steps but delivered only **64/288** requested stage outputs because recurring false failures kept triggering expensive new searches. Its 64 correct/0 wrong should **NOT** be mistaken for superiority without accounting for the **224 missed stage outputs**. Fake negative bits drove **64 unnecessary reopens**, demonstrating wasted computation and lack of source discrimination.

**STALE_DELAYED:** one prior-output bit was fed to the paid watchdog. 8 false-positive feedback reads coincided with newly wrong output after each stage transition. Delayed detection doubled wrong outputs, **16 rather than 8**; even eventual same-grammar task reacquisition cannot claim source-bound causal freshness.

**STATIONARY_TRUE:** 288/288 correct, 0 unnecessary reopens under unchanged tasks, but per-output watchdog costs extra paid instructions versus unmonitored B51d actor; compact per-task static oracles were not executed as a B52-native comparator, which is a **preregistration incompleteness to keep explicit** rather than inventing an oracle outcome. No claim that B52 is compute-optimal.

## Independent execution and nontrivial limits

The independent JavaScript evaluator parses B51c's **actual 72-node Rust founder** and B52's **actual appended 20-node Rust literal**, then reexecutes all **60,000 of 60,000 actual native CIL paid steps** in source order with **8,822 interleaved externally originated input/feedback interventions**, reconstructing **1,118 real native changed executable edges**, PC/register/credit/graph/memory state, source retention, task-stage outcomes, false-success staleness and exhaustive per-world summary. **First divergence is null**; missing same-run source SHA/native trace and any mismatching event fail CI closed. The observer does not use native output values as interpreter input.

There are major unresolved scientific barriers:

1. **Freshness identity:** a one-bit feedback value, even when read through charged `Read`, does not expose which submission it evaluates. An adversarial or delayed external bit can be observationally identical to a correctly fresh success; the previously committed [B52 abstract alias falsifiers](../analysis/b52_feedback_alias_falsifier.mjs) and [multi-probe stale alias control](../analysis/b52_multi_probe_stale_alias.mjs) make this limitation explicit. In the Rust VM the generic `ReadLastWriter` can distinguish *process vs environment writer identity* but cannot, by itself, distinguish **stale** and **fresh** external outcomes when both come from `Origin::External`.
2. **Policy authorship:** a watchdog that researchers prearranged is not endogenous discovery of monitoring, repair, motivation, or self-organization. The actor autonomously **executes** edits within bounded CIL, not invents the repair meta-policy; B3/B4 remain unmet.
3. **Task breadth/resources:** regimes only switch within the predeclared affine grammar with a researcher-provided output-correctness evaluator; finite, externally seeded 3,000 compute, no neutral-flux resource acquisition, and no unexpected software/network adaptation. Stable behavior does not establish indefinite self-maintenance.
4. **Comparison fairness:** equal **paid compute budgets** are verified across arms, but the FORCED_ZERO arm makes far fewer useful service attempts, and static correct program throughput is not natively benchmarked in this study. Report missing outputs and instruction costs, not normalized survival-only success rates.

**Narrow H52a:** ACCEPTED as native engineering mechanics. **Narrow H52b:** ACCEPTED as feedback-driven re-adaptation with one detectable error per switch; matched TRUE net correct-minus-wrong **+272** vs STALE_SUCCESS **+96**, FORCED_ZERO **+64**, and STALE_DELAYED **+256**; stage service output denominators disclosed. **H52c:** NOT ACCEPTED / information-provenance failure. **B3/B4 NOT MET; cyberlife NOT ESTABLISHED.**

## Next discriminating experiment

A **new, prospective B53** should test whether a **generic paid causal-binding affordance** (e.g. an actor-readable monotonic response ticket/nonce tied to the immediately preceding output, or a generic event provenance record, without disclosing correct task answer or target selection) can actually distinguish fresh and replayed external feedback. It must not promote a task-specific oracle or assume trust in a malicious evaluator, and must include source-pinned native CIL execution with independent step replay and matched tampered/delayed controls. Only if a freshness *assumption* is explicitly enforced can a source-bound bit be treated as evidence of the latest output. This would still leave endogenous monitoring-policy invention and durable organization untested.
