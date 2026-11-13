# CL-1 Prospective Feasibility Campaign — research result (2026-11-13)

**Scientific status: BOUNDED NEGATIVE / INCONCLUSIVE. CL-1's positive organizational-closure goal is NOT ACCEPTED.**

All ten **preregistered** studies have completed their fixed experiments, including external-environment timing, internal graph-object ablation, cross-owner information exchange and event-level provenance studies. All technical controls and deterministic replays passed. None produced sufficient evidence for an autonomously maintained, causally closed organization or cyberlife. A verified **experimental execution** is not proof of a positive **scientific hypothesis**.

## Source and evidence provenance

Every study protocol was committed **before** its corresponding run and before its outcomes were inspected. The first four preregistrations are listed here; Studies 05–08 have their own dated registration and archive links below:

| Study | Preregistration commit | Frozen study | Archived results |
| --- | --- | --- | --- |
| 01: random code ecology | [`2b4e2ea`](https://github.com/iFrankStudio/cioc/commit/2b4e2ea5f9eac5afd011bb00cedca906eecd7161) | [Protocol](studies/01-preregistered-random-ecology.md) | [SQLite and JSON](https://github.com/iFrankStudio/cioc/actions/runs/43411607915/artifacts/14029398627) |
| 02: reciprocal sensitivity | [`1de0311`](https://github.com/iFrankStudio/cioc/commit/1de0311414bd2063e71585b8b587fb238ff3f4d2) | [Protocol](studies/02-preregistered-reciprocal-causality.md) | [SQLite and JSON](https://github.com/iFrankStudio/cioc/actions/runs/43411607915/artifacts/14029558411) |
| 03: damage/restoration | [`f2e423a`](https://github.com/iFrankStudio/cioc/commit/f2e423a2384e2b64f57f7a4a307cc5e9394eb9d3) | [Protocol](studies/03-preregistered-code-recovery.md) | [SQLite and JSON](https://github.com/iFrankStudio/cioc/actions/runs/43411607915/artifacts/14028924253) |
| 04: externally funded ecology | [`c7fc4d8`](https://github.com/iFrankStudio/cioc/commit/c7fc4d81139921cfa0589ac98d3d24d1bb8b2f21) | [Protocol](studies/04-preregistered-open-compute.md) | [SQLite and JSON](https://github.com/iFrankStudio/cioc/actions/runs/43411607915/artifacts/14029298264) |

**Validation:** [GitHub Actions workflow 43411607915](https://github.com/iFrankStudio/cioc/actions/runs/43411607915) at executable source revision `aba4a2b4c8ddfab851039df085aa8c0d59fb2520`: all **four** independent study jobs passed, including Rust workspace tests, strict Clippy, committed Rust formatting, targeted Python unit tests, frozen study runner, independent read-only SQL metrics and archive upload. [CL-0 regression CI](https://github.com/iFrankStudio/cioc/actions/runs/43411608070) also passed at this revision. The archive retention is 30 days; reproduce with code rather than treating temporary downloads as permanent evidence.

The four disjoint seed cohorts contain **64 + 128 + 128 + 128 = 448 distinct initial worlds**, with **320 + 768 + 640 + 640 = 2,368 matched experiment records** and **2,368/2,368 complete exact SQLite replays**. Different generators and interventions mean these 448 worlds **cannot be pooled** as one binomial sample for any proposed life-property estimate. Conditions forked from the same seed are **paired**, not independent replicates.

## Frozen outcomes

| Study | Independent initial worlds | Verifiable outcome | Scientific interpretation |
| --- | ---: | --- | --- |
| 01 | 64 | 48/64 final graph-change proxies; 9/64 indirect lesion-vs-sham state differences; 0/64 other-process attempt contrasts | Graph edits and some public-data sensitivity, **not autonomous maintenance** |
| 02 | 128 | 1/128 seeds with at least one reciprocal process-state sensitivity pair; paired sham states identical | **Causal state responsiveness** only, not regenerative dependency |
| 03 | 128 | 128/128 edge lesions reached, 90/128 checkpoint-24 exposures, **0/128 internal edge restoration by 24 or final**, **0/128 persistent structural restoration** | No observed recovery of the registered damaged edge in this seeded distribution |
| 04 | 128 | Closed world >84 attempts: **0/128**; initial external compute pool >84: **3/128**; repeated external pool supply >384: **3/128**; zero-credit: 0 attempts | Long execution occurs in a few cases only with explicitly **external compute supply**; not self-maintenance |

Study 01 further recorded 15/64 control worlds with >48 attempts and 0/64 new processes; these activity and graph counts are descriptive. Study 02's reciprocal candidate was pair (1,2); directional sensitivities were rare (1–4/128 for each ordered pair). Study 03's disabled-process control did not show reactivation, and reached external-restore control offset 24 in 90/128 cases. Study 04 showed final code-node growth in 90/128 initially funded worlds and three new-process events by seed-world count; **node growth and process spawning are not reproductive success**.

### Prespecified uncertainty
- Study 02 reciprocal candidate proportion: 1/128, **95% Wilson interval [0.00138, 0.042926]**.
- Study 03 registered internal restoration: 0/128, **95% Wilson interval [0, 0.029137]**.
- Study 04 resource-supported execution >84 attempts with initial external pool: 3/128, **95% Wilson interval [0.008002, 0.066644]**.

These intervals describe variation across *the chosen deterministic seed generator family* under a hypothetical independent-seed interpretation. Adjacent LCG seeds and fixed VM conditions are **not** a random sample of all possible digital algorithms or worlds. The interval must not be interpreted as a bound on probability of life across the entire possibility space.

## Important nulls and failure audit

1. **No life assertion:** no population demonstrated long-lived, internally restored, bidirectionally maintained organization under a preregistered operational test.
2. Study 01 explicitly labeled code manipulation as a proxy and did **not** test sustained functional closure.
3. Study 02's lone reciprocal state difference might be caused by scheduling and shared data dependencies; it says nothing about mutual regeneration of essential components.
4. Study 03's **zero** recovered edges is distribution- and horizon-conditional. It does not establish that cyberlife is impossible or that generic CIL cannot express restoration. No favorable trials were excluded.
5. Study 04's replenishment is **an external environmental intervention**; cyclic code consuming that pool is not self-sufficient energy production. Only three seed-worlds reached the long-run thresholds; repeated supply opportunities themselves were contingent on continuing internal execution.
6. Negative controls, failed/trapped runs, null attempts, unreached scheduled actions and externally supplied initial code remain in the dataset. CI initially exposed rustfmt and Clippy errors in new runner files; fixes changed formatting and lint compliance **without post-hoc retuning seed sets, treatment groups or measurement rules**.
7. Snapshot hashes and replays establish consistency with the corresponding source revision, not authenticated tamper-proof archival evidence.

## Why CL-1 is not declared successful

The original [handoff](../cl0/docs/CL-1-handoff.md) requires evidence of an **internally maintained, causally interdependent organization**, not simply a running VM, graph mutation or ablation sensitivity. In these four studies:
- One-time or short-lived structural changes were common, but no self-maintenance of an organizational function was operationally established.
- A few bidirectional state effects did not show recovery or closure.
- The only sustained long runs were supported by **explicitly externally sourced computation**.
- No time-separated, internally generated *effective* novelty with reproduced organization was verified.

Consequently, the four-study feasibility campaign is **executed and auditable**, but **CL-1's positive scientific milestone remains open**. A scientifically valid null outcome is **not** an engineering failure, and it cannot be quietly converted into a positive stage acceptance.

## Reproduction and next experimental gate

```bash
cd cl0
cargo fmt --all -- --check
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings

CL1_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl1-study01 -- study01.sqlite
CL1_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl1-study02 -- study02.sqlite
CL1_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl1-study03 -- study03.sqlite
CL1_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl1-study04 -- study04.sqlite
python3 ../cl1/analyze.py study01.sqlite --json study01.json
python3 ../cl1/analyze_study02.py study02.sqlite --json study02.json
python3 ../cl1/analyze_study03.py study03.sqlite --json study03.json
python3 ../cl1/analyze_study04.py study04.sqlite --json study04.json
```

Start with empty, distinct SQLite databases; the runner **appends** records, so reusing a populated path will fail frozen cohort validation. Use source commit `aba4a2b` to match the cited artifact's source revision and output. The [next scientific gate](docs/next-experiment-contract.md) defines what would be required before a positive CL-1 acceptance.

---

## Study 05 — independent external epoch environment (verified 2026-11-13)

The protocol was frozen at [preregistration `9047f751`](https://github.com/iFrankStudio/cioc/commit/9047f751c8af0214f41abf5748bbdb65bfaf2860) before code or outcomes. [Study 05 protocol](studies/05-preregistered-exogenous-epochs.md) specifies 64 new seed-worlds, four matched conditions, 24 externally administered epochs and eight CIL attempts at most per epoch.

**Verified executable revision:** `0537d40964b5e68e75197b071b5d2b0be1549462`. [GitHub CI](https://github.com/iFrankStudio/cioc/actions/runs/43413383336) and [SQLite/JSON artifact](https://github.com/iFrankStudio/cioc/actions/runs/43413383336/artifacts/14030009254). All **256/256 runs** and **6,144/6,144 post-epoch snapshots** replayed and passed SHA-256 consistency checks.

| Condition | Total CIL attempts (64 worlds) | External pool injections | Active in epochs 12–23 | Restart after inert epoch |
| --- | ---: | ---: | ---: | ---: |
| Closed | 1,464 | 0 | 0/64 | 0/64 |
| Initial pool | 1,715 | 64 | 0/64 | 0/64 |
| Epoch pulses | 2,012 | 384 | 2/64 | 0/64 |
| Zero-credit + epoch pulses | **0** | **384** | **0/64** | **0/64** |

In **4/64** matched seed-worlds, the final process/data state differed between initial-pool and repeated-pulse conditions, excluding the directly injected external pool and event logs. All **384** planned external pulses were delivered in the zero-credit condition even while the world was inert, demonstrating the laboratory epoch clock is **not conditional on CIL execution**. Yet no CIL process autonomously regained the capacity to execute.

This strengthens environmental intervention methodology, not evidence of life. Resource supply remains **explicitly external**. No autonomous functional maintenance or organizational closure was established.


**Cumulative technical record (not one pooled statistical sample):** After adding Study 06, there are **640 different initial seed-worlds, 3,264 matched runs, 3,264 full exact replays and 18,944 post-epoch snapshots** from Studies 05 and 06. The 18,944 snapshot hashes are separately checked and their states reconstructed in exact Rust replays. Different generator distributions cannot be merged to estimate a life probability. **The CL-1 positive goal remains unaccepted.**

---

## Study 06 — internally constructed graph-object ablation (verified 2026-11-13)

The [protocol](studies/06-preregistered-internal-graph-ablation.md) was frozen in [preregistration commit `c513fc4`](https://github.com/iFrankStudio/cioc/commit/c513fc460f147059cd02dc29ceb3e846bc5865c8) before implementation. The causal ablation used new seeds **1160000–1160127** and a fixed pretreatment period. Selection was predeclared as the **smallest nonempty internally constructed graph object** existing at the identical prefork state, not a favorable object chosen after observing treatment outcomes.

At the verified implementation revision `a7ab6753ea004d6bd63be7db517af2d89c982377`, all **640/640 matched runs** and **12,800/12,800 epoch snapshots** passed exact replay and hash checks, with six CI jobs and the original CL-0 regression successful. [Workflow](https://github.com/iFrankStudio/cioc/actions/runs/43414360089) · [SQLite and JSON artifact](https://github.com/iFrankStudio/cioc/actions/runs/43414360089/artifacts/14030733004).

| Prespecified endpoint | Observed initial-world count |
| --- | ---: |
| Nonempty internally created object available at prefork | **16/128** |
| Corresponding external object deletion performed | **16/128** |
| Subsequent process-state difference excluding removed object | **2/128** |
| Difference propagated to nonowner processes | **0/128** |
| Post-lesion nonempty graph creation by the same owner | **0/128** |
| Sham intervention incorrectly changed simulated state | **0/128** |
| No eligible object (retained in denominator) | **112/128** |

95% Wilson intervals on the **128 independent seed-worlds**, not 640 paired conditions: eligible object 16/128 **[0.078426, 0.193426]**; indirect process effects 2/128 **[0.004295, 0.055181]**; nonowner effects 0/128 **[0, 0.029137]**.

**Interpretation:** The experiment demonstrates that at least some objects created by internal CIL instructions can become causally relevant to their creator's subsequent process state. This is stronger than simply observing new graph allocations but does **not** demonstrate reciprocal distributed maintenance. No cross-owner effect, nonempty reconstruction of a replacement graph, stable functional dependency or autonomous recovery was observed in this frozen distribution. The explicit host ablation is attributed `Origin::External` and is never counted as internal maintenance.

### Next scientific bottleneck

The first six studies are highly skewed toward short-lived, investigator-seeded 16-node programs. Only 16 of 128 prefork worlds in Study 06 had eligible nonempty graph objects. Repeating this exact generator with more seeds would mostly increase statistical precision on a narrow null, **not** solve the missing mechanism of organized interaction. A genuinely new study should preregister richer **generic** object/data interoperability and longer externally supported computation *without specifying preferred repair roles or fitting seed selection to the positive result*. It must independently test internal provenance, multi-component functional dependence and multi-epoch restoration, with a matched unmodified world, sham and externally repaired controls. Positive CL-1 stage acceptance remains **open**.

---

## Study 07 — graph-to-graph code exchange in a fresh ecology (verified 2026-11-13)

[Preregistered protocol](studies/07-preregistered-cross-owner-information.md) was committed independently in [`35865fde`](https://github.com/iFrankStudio/cioc/commit/35865fde358c2afa6309672d92c56c683a70004d) before this new run. [Verified CI](https://github.com/iFrankStudio/cioc/actions/runs/43417372928), [SQLite/JSON archive](https://github.com/iFrankStudio/cioc/actions/runs/43417372928/artifacts/14032745891); source `0ddbc8d44d9f1e414f060cbd9b49e14b1fd128f8`, new independent seeds 1270000..1270127.

New generic CIL primitives `GraphCopyFrom` (authorized source graph to owned target graph, one charged code node) and `GraphTransfer` (ownership transfer to an existing process with no duplication or compute minting) passed focused hand-authored mechanism tests. The **randomized** population of 128 new seed-worlds produced **640 matched runs**, each with exact replay and 20 hash-verified epoch snapshots (**12,800 snapshots**).

| Fixed outcome | Result |
| --- | ---: |
| Internally created eligible nonempty graph at prefork | **13/128** |
| Successful external ablation of eligible graph | **13/128** |
| Downstream process-state effect excluding object identity | **0/128** |
| Cross-owner process-state effect | **0/128** |
| New nonempty graph after object deletion | **0/128** |
| Sham/untreated mismatch | **0/128** |

**Scientific limitation:** Under Study 07's coarse `Executed` event granularity, the prespecified *successful cross-owner copy/transfer* endpoint was **NOT EVALUABLE**, not measured zero. This result must remain unchanged after adding new instrumentation. There was no evidence of reciprocal organizational maintenance or closure.

## Study 08 — event-level code exchange and pre-action owner provenance (verified 2026-11-13)

[Preregistered protocol](studies/08-preregistered-instruction-provenance.md) was frozen independently in [`f50694c6`](https://github.com/iFrankStudio/cioc/commit/f50694c6fe77a8332bc3875c2e137284ce63bc19) before adding event provenance or inspecting Study 08 results. [Verified eight-job CI](https://github.com/iFrankStudio/cioc/actions/runs/43418306702), [SQLite/JSON artifact](https://github.com/iFrankStudio/cioc/actions/runs/43418306702/artifacts/14033505218); source `b883be2a74828bd99337fda259f2418a74110bbc`. 128 fresh initial worlds (seeds 1380000..1380127), **640/640 final exact replays and 12,800 epoch state hashes**.

The VM now records only **successful** `GraphCopyFrom` and `GraphTransfer` as structured observer events (`GraphCopied`, `GraphTransferred`) with the actor's internal provenance and the objects' pre-action owners. An unsuccessful or unauthorized attempt remains a charged `Trapped` event. This annotation does not grant CIL any access to logs, resource privileges or survival goals.

| Fixed outcome | Result |
| --- | ---: |
| Eligible internally constructed graph at prefork | **11/128** |
| Indirect process-state effects from deletion | **1/128** |
| Other-owner process-state effect | **0/128** |
| Post-ablation nonempty graph construction | **0/128** |
| Successful cross-owner graph-node copies in untreated runs | **0 events** |
| Successful different-owner graph transfers in untreated runs | **0 events** |
| Worlds with at least one of either successful cross-owner exchange | **0/128** |
| Sham mismatches | **0/128** |

Study 08's observed exchange proxy 0/128 has a two-sided 95% Wilson interval **[0, 0.029137] for the preregistered generator family**. It is **not** a bound on arbitrary digital-world or life emergence probability. The event instrumentation passed positive calibration tests on an explicitly *hand-authored* cross-owner exchange, which is not counted among the randomized cohort. Study 07 remains *not evaluable* on that same metric, and neither is silently rerun to claim improved past data.

### Cumulative engineering verification and continuing negative scientific result

Across all **eight** distinct preregistered cohorts: **896** independently initialized seed-worlds, **4,544** matched experiment records, **4,544/4,544 exact final-state/event replays**, and **44,544** separately hash-verified post-epoch state snapshots from Studies 05–08. Cohorts use different generators, interventions and objectives. These are **not** a pooled sample for life probability estimation.

The latest [full study workflow](https://github.com/iFrankStudio/cioc/actions/runs/43418306702) and [CL-0 regression](https://github.com/iFrankStudio/cioc/actions/runs/43418306631) passed. Outcomes remain **bounded negative / inconclusive for the positive CL-1 goal**: no preregistered multi-component functional restoration, mutual maintenance or organizational closure has been verified.

The next useful scientific step is not merely adding more random seeds to a grammar that rarely creates shared objects. It is a **prospectively specified, broader but still non-teleological mechanism** for discoverable object references, provenance of internally constructed code and functionality tests across multiple causally connected parts. That protocol must be frozen before using any new data and must not insert a host-defined repair, fitness or reproduction oracle.

---

## Study 09 — permission-reachable general ecology, but no eligible internal objects (verified 2026-11-13)

[Study-09 protocol](studies/09-preregistered-permission-ecology.md) was frozen in [`8baeb530`](https://github.com/iFrankStudio/cioc/commit/8baeb530767bcc78df0e6069e4700649be8bf713) **before** running new seeds or inspecting Study-09 outcomes. This was a prospective correction to Study 08's *structurally unavailable read-grant pathway*. The Study-09 generator included `GraphGrantRead` and `GraphRevokeRead`, in addition to ordinary graph allocation, copying and transfer, and used three 32-node random programs per initial world rather than 16-node programs. No beneficial repair or cooperation instruction was introduced.

**Verified source:** `b5cdbb6122781560b41c7b07e2622b806c018b02`. [Nine-job CI](https://github.com/iFrankStudio/cioc/actions/runs/43419453968) · [SQLite/JSON evidence](https://github.com/iFrankStudio/cioc/actions/runs/43419453968/artifacts/14033347855). New independent seeds 1490000..1490127; **640/640 exact final replays and 20,480/20,480 epoch snapshots validated**.

| Preregistered outcome | Observed |
| --- | ---: |
| Internal nonempty graph eligible at the shared intervention fork | **0/128** |
| Successful graph deletion exposure | **0/128** |
| Cross-owner successful graph copy events in untreated arms | **0 events** |
| Successful different-owner transfers in untreated arms | **0 events** |
| Indirect process-state consequences of graph deletion | **Not causally exposed (0 eligible targets)** |
| Nonowner process-state impact from deletion | **Not causally exposed (0 eligible targets)** |
| Sham effect | **0/128 mismatches** |
| Every matched condition and snapshot deterministically replayable | **640/640 runs; 20,480/20,480 snapshots** |

**Interpretation:** Adding the missing permission opcodes made cross-owner exchange *possible in the VM grammar*, but did not ensure any randomly seeded population reached a state with a usable nonempty graph before treatment. Thus zero observed exchanges and zero ablation effects are **not** informative null findings about organizational closure: exposure was absent in all 128 worlds. This finding also rules out treating a larger opcode vocabulary or greater source code length as automatic progress toward life.

The next experimental bottleneck is to create an **investigator-disclosed, neutral opportunity scaffold** where generic CIL information/object operations actually become reachable without installing life-specific survival/repair goals. Such a scaffold is external initial engineering, never itself counted as emergent organization. See [Study-10 design proposal](docs/study-10-neutral-opportunity-design.md). No Study-10 protocol or outcome is preregistered yet.

### Updated project totals — engineering reproducibility only

Across **nine** different preregistered cohorts: **1,024 independent seed-worlds**, **5,184 matched runs**, **5,184/5,184 exact final-state/event replays** and **65,024** independently hash-verified, replay-consistent post-epoch snapshots from Studies 05–09. These values demonstrate reproducibility, **not** life or autonomous self-maintenance. Generator families and matched conditions are not statistically exchangeable. **CL-1's positive scientific target remains unaccepted.**

---

## Study 10 — reliable neutral graph interactions are reachable, but externally scripted (verified 2026-11-13)

[Precommitted Study-10 protocol](studies/10-preregistered-neutral-opportunity.md) at [`02092cdf`](https://github.com/iFrankStudio/cioc/commit/02092cdff530936b5e3fda9a744133f52d27050b), **committed before implementation and results**. Verified implementation source: `9bf20b088036479d28dcc9442c23097d478234e3`. [Full ten-job CI](https://github.com/iFrankStudio/cioc/actions/runs/43421195384) · [raw SQLite and JSON](https://github.com/iFrankStudio/cioc/actions/runs/43421195384/artifacts/14034830489).

Study 10 explicitly replaced the Study-09 trap-prone random starting programs with a **symmetric, externally programmed generic CIL opportunity prefix**, followed by an eight-node random continuation. For every seed, the first 18 instruction attempts produced three nonempty graph objects, published their handles to shared cells and granted read permissions to ring neighbors. These actions were *executed internally* but **were specified by the researcher in the initial code**, so they are not spontaneous construction, social interaction or emergence.

New independent seeds **1600000–1600063**; 64 matched baseline worlds × five conditions = **320 matched records**, 12 external epochs per treatment with eight CIL opportunities per epoch, **3,840 post-epoch snapshots**, **320/320 exact world+event replays**, all hashes and strict storage checks verified.

| Prespecified measure | Observed |
| --- | ---: |
| Three nonempty published graph objects with expected grants at fork | **64/64** |
| Target graph actually deleted in target-drop arm | **64/64** |
| Successful cross-owner instruction copies in untreated postfork CIL | **192 successful events (3/world)** |
| Successful different-owner graph transfers in untreated CIL | **192 successful events (3/world)** |
| Worlds with a successful cross-owner exchange | **64/64** |
| Indirect process-state change after object deletion | **64/64** |
| Nonowner process-state change after object deletion | **64/64** |
| Sham/untreated simulated-state mismatch | **0/64** |
| Post-ablation nonempty graph construction | **0/64** |
| Zero-credit activity | **0 CIL attempts** |

**Interpretation:** The engineering mechanism and causal exposure problem from Study 09 are corrected: accessible graph handles, authorized copying and owner transfer can all execute reliably within finite credits, and deletion affects otherwise unchanged programs. **This was expected from the prewritten scaffold**. The measured cross-owner transfer cycle is **investigator-designed**, not an evolved social structure; the forced disruption and its effects are not evidence of internally initiated maintenance. No prospective functional output independent of the scaffold was assessed, no successful repair or spontaneous closure was demonstrated, and the study cannot be used to claim life.

Across **ten** disjoint, nonexchangeable cohorts, there are now **1,088 initial seed-worlds**, **5,504 matched experimental records and exact final-state replays**, plus **68,864 hash-verified, replay-consistent post-epoch snapshots** from Studies 05–10. These are *engineering workload totals*, not an estimate of life probability. The positive CL-1 scientific goal remains **unaccepted**.

### Next scientific experiment boundary

Study 11 should isolate externally scripted opportunity behavior from subsequent internally generated functional change. A fresh registered cohort must compare identical scaffold resources with randomized versus disabled/perturbed continuations, instrument which *post-scaffold* code mutations create genuinely new effective dependencies, and use matched interventions on multiple internally produced parts. **A successful transfer or copy prewritten at a particular PC cannot count** toward functional recovery, organizational closure or “emergence.” Prospective stable function and multi-epoch recovery endpoints must be frozen **before** observing Study-11 outcomes. No host fitness, repair or reproduction oracle.
