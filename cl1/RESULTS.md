# CL-1 Prospective Feasibility Campaign — research result (2026-11-13)

**Scientific status: BOUNDED NEGATIVE / INCONCLUSIVE. CL-1's positive organizational-closure goal is NOT ACCEPTED.**

The first four **preregistered** studies have completed their fixed experiments. All technical controls and deterministic replays passed. None produced sufficient evidence for an autonomously maintained, causally closed organization or cyberlife. A verified **experimental execution** is not proof of a positive **scientific hypothesis**.

## Source and evidence provenance

The four preregistrations were committed **before** their respective runners generated results:

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
