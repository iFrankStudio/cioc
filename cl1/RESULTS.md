# CL-1 Prospective Feasibility Campaign — research result (2026-11-13)

**Scientific status: BOUNDED NEGATIVE / INCONCLUSIVE. CL-1's positive organizational-closure goal is NOT ACCEPTED.**

All thirteen **preregistered** studies have completed their fixed experiments, including external-environment timing, internal graph-object ablation, cross-owner information exchange and event-level provenance studies. All technical controls and deterministic replays passed. None produced sufficient evidence for an autonomously maintained, causally closed organization or cyberlife. A verified **experimental execution** is not proof of a positive **scientific hypothesis**.

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
| At least one successful random-tail instruction after scripted prefix | **63/64 seed-worlds** |
| Successful random-tail instruction attempts | **747 total across untreated worlds** |
| Trapped internal attempts after fork in untreated worlds | **148 total** |


**Preregistered post-scaffold activity audit:** The `late_random_tail` outcome had been declared in the frozen Study-10 protocol. An initial read-only analyzer reported the core scaffold events but omitted this declared diagnostic; the omission was corrected in analyzer-only commit [`de8d6ead`](https://github.com/iFrankStudio/cioc/commit/de8d6ead2cd0792069e1c16295f20f2616174b7e), then the entire ten-study suite was rerun successfully ([CI 43421758544](https://github.com/iFrankStudio/cioc/actions/runs/43421758544)). **63/64** untreated seed-worlds executed at least one successful CIL instruction in the randomized continuation (95% Wilson **[0.916659, 0.997236]**); there were 747 successful random-tail attempts and 148 charged postfork traps. The fixed scaffold remained exactly as preregistered. These are *opportunities to execute externally randomized initial code*, **not** self-organized functions or accumulated novelty. The original archived source revision remains reproducible and the subsequent analysis change is disclosed rather than retrospectively editing the preregistration.

**Interpretation:** The engineering mechanism and causal exposure problem from Study 09 are corrected: accessible graph handles, authorized copying and owner transfer can all execute reliably within finite credits, and deletion affects otherwise unchanged programs. **This was expected from the prewritten scaffold**. The measured cross-owner transfer cycle is **investigator-designed**, not an evolved social structure; the forced disruption and its effects are not evidence of internally initiated maintenance. No prospective functional output independent of the scaffold was assessed, no successful repair or spontaneous closure was demonstrated, and the study cannot be used to claim life.

Across **ten** disjoint, nonexchangeable cohorts, there are now **1,088 initial seed-worlds**, **5,504 matched experimental records and exact final-state replays**, plus **68,864 hash-verified, replay-consistent post-epoch snapshots** from Studies 05–10. These are *engineering workload totals*, not an estimate of life probability. The positive CL-1 scientific goal remains **unaccepted**.

### Next scientific experiment boundary

Study 11 should isolate externally scripted opportunity behavior from subsequent internally generated functional change. A fresh registered cohort must compare identical scaffold resources with randomized versus disabled/perturbed continuations, instrument which *post-scaffold* code mutations create genuinely new effective dependencies, and use matched interventions on multiple internally produced parts. **A successful transfer or copy prewritten at a particular PC cannot count** toward functional recovery, organizational closure or “emergence.” Prospective stable function and multi-epoch recovery endpoints must be frozen **before** observing Study-11 outcomes. No host fitness, repair or reproduction oracle.

---

## Study 11 — random CIL continuation produces cross-owner exchange without prewired exchange instructions (verified 2026-11-13)

The [Study-11 protocol](studies/11-preregistered-postscaffold-ecology.md) was frozen at [`cd859cf7`](https://github.com/iFrankStudio/cioc/commit/cd859cf7fbd844345e8969755b68eb6778dabc60) **before implementation or inspecting the new trials**. Study-11 source revision `833b6a609ebbf5a7d5d354588bb4d0770a3c595a` was validated by all **eleven** jobs in [CI 43422732727](https://github.com/iFrankStudio/cioc/actions/runs/43422732727). [Full SQLite, 7,680 epoch snapshots and JSON outcome artifact](https://github.com/iFrankStudio/cioc/actions/runs/43422732727/artifacts/14034463544). Every one of **480/480 matched runs** and **7,680/7,680 postepoch state snapshots** replayed with exact hashes.

This study removed `GraphCopyFrom` and `GraphTransfer` from the nine-node **reliable externally scripted opportunity prefix**. The prefix only created three generic executable graph objects, published their handles, gave each neighboring process read access and loaded an ordinary graph reference into a register. It was followed by 18 nodes of uniform 12-choice *externally randomly generated* CIL code, with no host-defined cooperation/repair objectives. **No deliberate inter-owner copy/transfer was prewritten in the fixed prefix.**

| Prospective outcome | Observed across 96 fresh initial worlds |
| --- | ---: |
| Eligible internally executed graph objects at the matched fork | **96/96** |
| Successful cross-owner graph-node copies from random continuation | **308 events, 73/96 seed-worlds** |
| Successful different-owner transfers from random continuation | **89 events, 65/96 seed-worlds** |
| At least one successful cross-owner exchange | **89/96 seed-worlds** |
| Original graph-node structures changed under control | **93/96 seed-worlds** |
| Targeted graph deletion affected downstream process state | **76/96 seed-worlds** |
| Deletion affected *nonowner* process state | **49/96 seed-worlds** |
| Late-epoch internal activity | **10/96 seed-worlds** |
| Nonempty graph created after target deletion by the original owner | **0/96 seed-worlds** |
| Sham/untreated simulated-state mismatch | **0/96** |
| Charged trapped attempts in untreated postfork execution | **212 events** |

On 96 independent seed-worlds the successful-exchange incidence **89/96** has a two-sided 95% Wilson interval **[0.857070, 0.964232]**. Nonowner process-state sensitivity after graph ablation **49/96** has interval **[0.411960, 0.608071]**. This describes only this preregistered generator/horizon, not any general probability of digital life. The five conditions within one seed-world are paired, not independent.

**Research interpretation:** The experiment improves substantially on Study 10's engineering calibration: with no prewired exchange instruction, general seeded CIL continuations reliably execute authorized cross-owner information transfers and their removal can alter other processes. However, the **random continuation source code was still generated by the researcher at initialization**, so these events cannot be counted as autonomous discovery, a preference for cooperation, or an evolved self-sustaining organization. The preauthored prefix creates the graph references and the mutual read rights, and therefore some causal dependencies are externally made possible. **No regenerated nonempty graph after damage, no restored joint function and no time-separated internal organizational closure were observed or even fully operationalized.** The positive CL-1 acceptance criterion remains **NOT MET**.

### Aggregate verification boundary

Across the **11** nonexchangeable registered cohorts: **1,184** initial seed-worlds, **5,984 matched records and exact final-state/event replays**, plus **76,544 hash-verified postepoch snapshots** from Studies 05–11. All experiment protocols were frozen in separate commits before their respective runners. These are reproducibility and research-scope totals, **not pooled data for estimating life probability**.

**Next methodological priority:** provenance of actual *internally created or changed executable nodes*, including the origin of new object nodes and copied operations, followed by prospective **joint information-function** ablations of two genuinely internal components and explicit internal restoration analysis across separate epochs. Copy events sourced from investigator-randomized *initial* code cannot be promoted to endogenous adaptive maintenance merely because they influence other processes.

---

## Study 12 — traceable origins of internally executed graph construction and code edits (verified 2026-11-13)

The independent [Study-12 preregistration](studies/12-preregistered-graph-lineage.md) was frozen in commit [`9dae1bb`](https://github.com/iFrankStudio/cioc/commit/9dae1bbfdc06a4988fe56137a63695a2efc79460) before implementation or inspection of outcomes. The verified executable source `f64f754be50fb1f5f0cfc993a6629836ce3bba7f` produced the study report in [GitHub CI 43425754065](https://github.com/iFrankStudio/cioc/actions/runs/43425754065) with [raw SQLite and JSON artifact](https://github.com/iFrankStudio/cioc/actions/runs/43425754065/artifacts/14037216449), and an independently committed [permanent JSON summary](results/study12-report.json).

New VM observer events distinguish `GraphCreated` (graph allocation), `GraphNodeCopied` (node copied from executable process code), and `GraphNodeEdited` (generic set-constant or edge-edit operation). Existing `GraphCopied` and `GraphTransferred` preserve graph-to-graph source/owner and transfer provenance. Each event replaces a successful ordinary `Executed` event; none changes resources, agent behavior, or CIL-visible observation rights. Traps generate no false success event. The new information does **not** fully trace the ancestry of shared cells, graph encoding/replacement or launched process code.

The fixed Study-11 grammar and scaffold were reused with fresh pre-registered seeds **1810000–1810095**: 96 independent initial worlds, 5 matched conditions each, **480 exact run replays and 7,680 epoch snapshot hashes**.

| Prespecified or source-tracing observation | Fresh Study-12 result |
| --- | ---: |
| Valid externally scripted prefix graph creation and node copy | **3 + 3 per seed** |
| Postfork successful graph allocations (untreated runs) | **433 events**, across **58/96 worlds** |
| Postfork copies from existing process code | **25 events**, across **22/96 worlds** |
| Postfork successful graph edge edits | **218 events** |
| Postfork successful graph constant edits | **0 events** |
| Worlds with any successful postfork graph edit | **58/96** |
| Worlds with both successful cross-graph copying and a successful edit | **33/96** |
| Cross-owner graph node copy events | **279** |
| Different-owner graph transfer events | **103** |
| Worlds with at least one successful cross-owner transition | **87/96** |
| Targeted deletion effects on nonowner process states | **44/96** |
| New nonempty graph by original owner after targeted damage | **0/96** |
| Invalid sham mismatch | **0/96** |

The **87/96** seed-world incidence of cross-owner interaction has 95% Wilson interval **[0.831347, 0.949892]**; nonowner lesion effects **44/96** have **[0.362188, 0.557684]**. These are within-generator intervals, not population-wide probabilities of life.

**Interpretation:** Study 12 supplies concrete, source-attributed evidence that CIL operations can create graph headers, copy existing executable instruction nodes and edit some graph edges. It also demonstrates that information exchange and editing co-occur in some worlds. **It does not prove the copies were newly invented, causally necessary for a maintained function, or restored after damage.** Initial process code, the scaffold and the random continuation are all generated by the investigator. The successful edits could even set an edge to its existing value. Semantic novelty, organizational closure and internal functional restoration remain **NOT EVALUABLE**, not demonstrated negative probabilities.

### Updated independent cohort accounting

Across **12** preregistered cohorts: **1,280** distinct seeded initial worlds, **6,464 matched experimental runs with 6,464 exact replays**, and **84,224 separately checked postepoch snapshots** from Studies 05–12. These totals describe reproducibility and experiment workload, not a statistically pooled sample of life emergence.

---

## Study 13 — paired graph ablations and two-window reciprocal information flow (verified 2026-11-13)

The [registered protocol](studies/13-preregistered-reciprocal-information-flow.md) was separately frozen in [`85bab403`](https://github.com/iFrankStudio/cioc/commit/85bab4034ddf0e42c17db4fddac2c6bd9744badd) before implementation or outcomes. The source revision `bc670fafd26ea7c93b018f92c6bda62749036882` passed the Study-13 CI job in [workflow 43426830986](https://github.com/iFrankStudio/cioc/actions/runs/43426830986); the full 576 matched trials and 9216 postepoch states were exact-replayed and SHA-256 checked. [Original SQLite/JSON archive](https://github.com/iFrankStudio/cioc/actions/runs/43426830986/artifacts/14037911787) (temporary retention) and [permanent version-pinned JSON report](results/study13-report.json).

**New independent seed-worlds:** 1930000 through 1930095 inclusive; **96** independently generated worlds, six arms per initial world, 16 external epochs per arm, 12 possible CIL attempts per epoch. The matched externally attributed arms are untreated control, invalid sham, deletion of graph1, deletion of graph2, simultaneous graph1+graph2 deletion, and zero-credit control. The common externally authored nine-node scaffold creates three shared-access graph objects; the remaining executable instructions are externally randomized. No host agent can trigger repair.

The **preregistered functional marker** is not graph-count growth or arbitrary register state difference. For a pair of distinct pre-action graph owners A,B, successful `GraphCopied` events must include both A→B and B→A within one observation window (W1 = epochs0–7; W2 = epochs8–15). This marker is defined by the observer and unavailable to CIL as a fitness signal.

| Independently measured endpoint | Observed |
| --- | ---: |
| Untreated worlds with bidirectional information flow in W1 | **4/96** |
| Untreated worlds with bidirectional information flow in W2 | **0/96** |
| Both separated windows contain a bidirectional candidate | **0/96** |
| W1-qualified worlds exposed to target-specific lesion contrast | **4/96** |
| Graph1 ablation removes the original W1 pair marker | **4/96** |
| Graph2 ablation removes the original W1 pair marker | **4/96** |
| Additional loss requiring joint, rather than either single, ablation | **0/96** |
| New populated graph under any of three lesion arms | **0/96** |
| Later-window reappearance + graph creation and node-copy evidence | **0/96** in every lesion arm |
| Reciprocal-copy event count in untreated W1 and W2 combined | **380 in W1; 26 in W2** (including one-way events) |
| Sham/untreated simulated-state mismatch | **0/96** |

The 4/96 W1 candidate incidence has a two-sided 95% Wilson interval **[0.016321, 0.102282]**. The 0/96 two-window persistence and candidate-reappearance proportions have intervals **[0, 0.038476]** under the frozen seed generator. **Only four worlds supplied the W1 function exposure**, so all four graph lesions removing the candidate is a very small exposed subcohort, not a sweeping estimate of closure.

**Result:** One-way graph copying is common, but preregistered two-way information relations are **rare and transient**. Both single-object ablations can disrupt these relations, confirming *limited causal dependence* on the preauthored graph substrate. There was **no** durable two-window reciprocal relationship, no internal reappearance candidate and no new populated graph after targeted damage in this cohort. The original runnable code and published access grants are investigator-generated; a graph-object signal is not itself a jointly maintained function. **Endogenous maintenance, semantic function restoration and organizational closure remain NOT EVALUABLE / NOT DEMONSTRATED**, never accepted on the basis of these counts.

**Method note:** After implementing the runner and before publishing this result, the independent analyzer clarified the prespecified new nonempty graph endpoint to include successfully created-and-populated objects subsequently dropped before the final snapshot. This was an analyzer-only event-provenance fix, checked with a targeted test; the frozen seeds, instruction generator, interventions, observation windows and functional marker were unchanged. The report and source revision above are pinned so that this adjustment is auditable.

### Updated cohort accounting

Across **13** separate preregistered cohort designs there are **1,376** independently seeded initial worlds, **7,040** paired experimental runs with full exact replays, and **93,440** separately hash-checked postepoch snapshots from Studies 05–13. Different cohorts and paired arms are not interchangeable samples; none of these totals estimates a probability of life. Positive CL-1 scientific acceptance remains **open**.

## Study 14 — extended-window randomized reciprocity (verified 2026-11-13)

This is the 96-world preregistered cohort in [Study 14](studies/14-preregistered-functional-restoration-gate.md), 576 matched runs, and 13,824 hashed epoch snapshots. Exact replay and independent audit passed. Early first-window reciprocal graph-copy candidates occurred in **4/96** untreated worlds; W2 and W3 each had **0/96**; all three-window persistence was **0/96**. No newly populated graph or late reappearance candidate was seen under the three damage arms. **Graph copying was only an exposure proxy**, and no attributable source-sensitive functional output could be established. The structural mechanism-positive arm was not yet implemented for the random cohort. Read the [frozen null checkpoint](results/study14-checkpoint.md). Functional maintenance and closure were **NOT EVALUABLE**, not zero.

## Studies 15–21 — source attribution and external function-control calibration (verified 2026-11-13)

These were *technical* calibrations, **not seven additional independent ecological cohorts**. Studies 15–17 added origin-attributed successful cell I/O, generic cell-descriptor-to-graph coding and graph-to-process launch metadata. Study 18 demonstrated a read-only source-chain auditor using predeclared synthetic positives and counterexamples; retrospectively the fixed Study-14 cohort yielded **0** fully traceable strict chains in **576 paired runs**, with **1,086** native trapped-instruction events ([checkpoint](results/study18-checkpoint.md)). This cannot be interpreted as an estimated maintenance rate.

Study 19 verified exact externally initiated graph-object reinjection without compute minting, preserving source provenance and replay ([checkpoint](results/study19-checkpoint.md)). Study 20 used a hand-authored two-graph reader: untreated/sham output **11**, either graph loss **no output**, same-object exogenous reinjection **11**, external code replacement **30**, zero-credit **no output** ([checkpoint](results/study20-checkpoint.md)). Study 21 improved the positive control: an actual failed readout in W0 followed by **externally initiated restoration**, with new, externally authored reader probes outputting **11 in both W1 and W2**; unrepaired damage remained inoperative, mutated code yielded **30**, and zero-credit did not execute. Exactly **11 paired fixture arms, 33 hashed snapshots and 0 newly sampled independent ecological worlds** ([source-verified checkpoint](results/study21-checkpoint.md)). These functions were *designed by the investigator*. **No endogenous regeneration was observed.**

## Study 22 — fresh-seed independent test of late compute limitation (verified 2026-11-13)

**Preregistration**: [Study 22](studies/22-preregistered-compute-limitation-ecology.md), frozen at `ce6f78bd` before implementation. **Evidence**: [verified source-pinned checkpoint](results/study22-checkpoint.md) and [successful full CL-1 CI](https://github.com/iFrankStudio/cioc/actions/runs/43445242022), source revision `7da289de`.

Exactly **96 independent new seed worlds**, **768 paired runs** and **18,432 hashed epoch snapshots** were preregistered and independently audited. All eight matched schedules and the untouched/sham state comparison passed; zero-credit never executed postfork. Recharges `SetCredits(1..3,47)` were externally scheduled at epochs 8 and 16 and do not restart trapped processes.

| Endpoint (worlds / 96) | Untreated W3 | External recharge W3 |
| --- | ---: | ---: |
| Any charged native CIL attempt | **0** | **57** |
| Successful cross-owner GraphCopied | **0** | **15** |
| Successful CellRead | **0** | **23** |
| Successful CellWritten | **0** | **0** |
| Both-direction graph copy in window | **0** | **0** |

The paired W3 comparison had **57 recharge-only active worlds versus zero untreated-only**, and **15 recharge-only successful cross-owner copy worlds versus zero untreated-only**. W3 recharge active-world frequency 57/96 had Wilson 95% interval **[0.493739, 0.686547]**; W3 copied-world frequency 15/96 had interval **[0.097037, 0.241915]**. This is a concrete demonstration that **externally subsidized compute is a confound for late activity and copying**, yet **not enough to restore sustained code-dependent cell-output function**. Exogenous credit is never spontaneous energy harvesting, and successful graph copying is not a maintenance mechanism.

### Updated stage ledger through Study 22

Across **15 separately preregistered ecological study cohorts (Studies 01–14 and 22)**: **1,568 initial independent seed worlds** and **8,384 total matched runs**. The separately scripted technical calibrations Studies 15–21 do not enter those denominators. Do not pool heterogeneous generators or matched arms to infer a universal life-emergence rate. No CL-1 positive scientific milestone has been accepted.
