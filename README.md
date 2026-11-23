# CIOC — Cyber Individuality & Organizational Closure

**赛博生命研究 / Native Cyberlife Research**

CIOC explores what self-maintenance, organizational individuality and potentially open-ended change could mean in a world composed of information, executable algorithms, storage and compute resources. Its governing distinction is **a working experimental platform is not evidence of life**.

## Current project status

| Track | Status | Entry point |
| --- | --- | --- |
| **CL-0** — native digital experimental world | **Infrastructure accepted · 2026-11-13** | [Project / instructions](cl0/README.md) · [acceptance evidence](cl0/docs/CL-0-acceptance.md) |
| **CL-1A** — population / generational evolution | **ARCHIVED / PAUSED · 2026-11-15**; historical original CL-1 studies retained | [CL-1A route](cl1a/README.md) · [original registry](cl1/README.md) |
| **CL-1B** — continuous individual evolution | **ACTIVE · C21 new-heldout 186 original native worlds + full paid-CIL replay PASS; PC202 cost gain did not generalize; B3/B4 NOT MET** | [CL-1B plan](cl1b/README.md) · [B00 protocol](cl1b/studies/B00-self-edit-feasibility.md) |
| **CIOC V0.1–V0.16** — historical synthetic models | **Archived, retained as independent evidence and negative lessons** | [Experiment history](experiments/) · [evidence transition](cl0/docs/cioc-transition.md) |

**CL-0 verification:** [latest verified end-to-end workflow](https://github.com/iFrankStudio/cioc/actions/runs/43406928175) · [archived SQLite/JSON/CSV artifacts](https://github.com/iFrankStudio/cioc/actions/runs/43406928175/artifacts/14027303037).

The CL-0 acceptance run passed **48 Rust tests**, **3 Python tests**, Rust formatting and lint checks, the Vue 3/TypeScript build, and **96 of 96 exact SQLite replays** from **24 seeds × 4 matched conditions**. These are tests of engineering determinism and reproducibility, **not** observations of spontaneous life.

## What CL-0 contains

- **Rust world kernel / CIL VM:** deterministic scheduling, mutable executable graph instructions, explicit internal/external event provenance and bounded instructions.
- **Information and resources:** generic information-to-instruction construction, graph ownership and read permissions, finite abstract storage charges, process compute credits and externally traceable compute-pool allocation.
- **Experimental laboratory:** SQLite-recorded initial and intermediate snapshots, matched interventions at CIL-attempt offsets, reproducible replay and negative controls.
- **Independent observation:** read-only Python analysis and browser-local Vue 3 + TypeScript report inspection.
- **Seeded reference baseline:** externally generated programs with no host life oracle or predefined repair/reproduction objective.

See the **[current system specification](cl0/docs/current-system.md)** and **[acceptance contract](cl0/docs/CL-0-acceptance.md)**. The original [Stage-1 specification](cl0/docs/specification.md) is retained as an historical architecture baseline rather than a current feature list.

## Reproduce the accepted baseline

Requirements: Rust stable, Python 3.10+, Node.js 22+ and npm.

```bash
cd cl0
cargo fmt --all -- --check
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
python3 -m unittest discover -s analysis/tests -v

CL0_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl0-explore -- experiment.sqlite 24 54129
python3 analysis/analyze.py experiment.sqlite --json report.json --csv report.csv

cd console
npm ci
npm run build
npm run dev
```

Open the local observer and import `report.json`. For detailed arguments, research constraints and file layout, use [cl0/README.md](cl0/README.md). The example shell substitution is for POSIX-compatible shells; on other shells, set `CL0_SOURCE_REVISION` to the checked-out commit hash through the appropriate syntax.

## Historical evidence — not proofs of cyberlife

- **V0.1–V0.3:** causal maintenance, stored templates and bounded reconstruction; relied on fixed repair assumptions.
- **V0.4–V0.10:** mutually dependent regulators, delegated verification and damage to protocols; retained specialized host support and/or structural cutsets.
- **V0.11–V0.13:** diagnosis and conflict-aware votes; host oracles and correlated failures remained critical limitations.
- **V0.14:** finite-resource variation and mutation; not open-ended evolution.
- **V0.15:** explicit maintenance costs and trade-offs under damage.
- **V0.16:** two damageable controllers under resource limits; common-mode failures and preinstalled roles remained.

Read the [V0.16 research checkpoint](docs/research-checkpoint-v0.16.md) for individual results and the [CIOC → CL-0 transition audit](cl0/docs/cioc-transition.md) for the distinction between constructed success and endogenous organization.

## Research commitments

We separate **externally authored initial conditions**, **internal world activity** and **kernel mechanics**. We require predeclared measurement rules, negative controls, finite resource accounting, explicit failed interventions and replay evidence. Passing VM tests, increasing activity, code mutation or arbitrary structural complexity do not justify claims of autonomy, self-maintenance, organizational closure, open-ended evolution or life.

**Current scientific conclusion (2026-11-23 UTC):** CL-0 remains verified infrastructure, not life evidence; CL-1A is archived. CL-1B's verified C19A/C19B native experiments ran **548 original Rust worlds and 2,329,613 paid CIL instructions**, with independent instruction-by-instruction replay reporting `first_divergence=null`. The individual performed **47/47 charged executions of its current live PC173 source parameter** after temporarily patching and subsequently restoring its own control edge; no temporary edge was left behind. This is genuine paid self-inspection, **not** autonomous algorithm invention or profitable learning. On strict same-initial-source PC225-effect controls, the known calibration cohort saved 1,812 CIL, while the genuinely new heldout cohort **cost 2,755 more**, with no stage gain. The different-source C9 static no-edit program completed all 90 truthful tasks more cheaply, but is **not** a single-factor causal control. A [retrospective original-ZIP audit](https://github.com/iFrankStudio/cioc/actions/runs/45130246564) verified that even a *free, clairvoyant per-task selection between these two fixed PC225 gate policies* costs 396,148 CIL versus C9's 309,125 (87,023 excess); this **does not bound within-task switching or other programs**. A [source-pinned retrospective C19-R2 paid-PC audit](https://github.com/iFrankStudio/cioc/actions/runs/45140948911) independently rechecked the original 548 native worlds and counted every paid PC in all 270 truthful trajectories (90 tasks × 3 arms), with no new worlds or holdout exposure: **71/90 main-arm tasks never performed a live source read**, yet those tasks account for **81,610/90,127 (90.55%)** of the main-versus-different-source C9 excess cost. The new C19 PC229–240 nodes were executed only **564** times; inherited C14 PC180–188 nodes accounted for **24,456** paid executions. These are **descriptive PC counts, not same-source deletion ablations or causal attribution across different programs**. See [C19-R2 report](cl1b/results/CL1B-C19R2-ORIGINAL-NATIVE-PAID-PC-COST-ANATOMY.md). Subsequently, [C20 184-world native/Rust plus fully independent Python replay #45146154128](https://github.com/iFrankStudio/cioc/actions/runs/45146154128) rigorously compared exact same 241-node source PC202 paid ticket-consumption write-effect suppression on all 45 previously **known** C19B tasks. Both main and effect-suppressed arms fully completed 45/45 true tasks; suppressing PC202 reduced spending by 322 CIL (209,189 to 208,867), with only 2/45 subsequent behavior changes despite 11/45 physically exposed cases. This is not new heldout validation or evidence that removing the safety mechanism is generally beneficial. [Permanent original native ZIP archive #45147070690](https://github.com/iFrankStudio/cioc/actions/runs/45147070690) independently checked all 184 raw JSONL worlds and read back its Git-stored SHA256-verified ZIP; see the [C20 native causal report](cl1b/results/CL1B-C20-KNOWN-C19B-PC202-PAID-TICKET-EFFECT-184-native-audit.md). Most recently, [C21 186 genuinely new frozen-task native worlds with independent full paid-CIL replay #45151909596](https://github.com/iFrankStudio/cioc/actions/runs/45151909596) completed all 45×48 truthful stages on every one of four arms. At identical initial 241-node CIL, suppressing the charged PC202 ticket-write physical effect yielded **exactly the same 202,903 CIL as normal**, with only **1/45** exposed tasks and **0/45** downstream changes, while previously known C20 tasks had apparently saved 322 CIL. The C21 same-source PC225 marker suppression spent 202,630, versus normal 202,903; the different-source C9 economic comparator spent 146,966. [The original C21 artifact was SHA256-verified after permanent Git archive readback](https://github.com/iFrankStudio/cioc/actions/runs/45152339235); [C21 full report](cl1b/results/CL1B-C21-PROSPECTIVE-NEW-45-GOALS-PC202-TICKET-CAUSAL-186-NATIVE-REPORT.md) explicitly discloses a preregistration v1 missing-array validation failure before any native execution and the pre-execution v2 repair. These results show neither cumulative agent adaptation nor safe elimination of the ticket mechanism. Both C19 original ZIPs were permanently archived and independently checked; see the [CL-1B evidence and limitations](cl1b/README.md), [validation](cl1b/VALIDATION.md), and [C19-R1 report](cl1b/results/CL1B-C19R1-RETROSPECTIVE-PAID-PC225-GATE-ORACLE-CEILING.md). Focused native/replay CI is not proof of the latest HEAD's full historical regression. **B3 endogenous cumulative adaptation, B4 organizational closure and cyberlife remain unproven.**