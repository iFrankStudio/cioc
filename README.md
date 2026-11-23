# CIOC — Cyber Individuality & Organizational Closure

**赛博生命研究 / Native Cyberlife Research**

CIOC explores what self-maintenance, organizational individuality and potentially open-ended change could mean in a world composed of information, executable algorithms, storage and compute resources. Its governing distinction is **a working experimental platform is not evidence of life**.

## Current project status

| Track | Status | Entry point |
| --- | --- | --- |
| **CL-0** — native digital experimental world | **Infrastructure accepted · 2026-11-13** | [Project / instructions](cl0/README.md) · [acceptance evidence](cl0/docs/CL-0-acceptance.md) |
| **CL-1A** — population / generational evolution | **ARCHIVED / PAUSED · 2026-11-15**; historical original CL-1 studies retained | [CL-1A route](cl1a/README.md) · [original registry](cl1/README.md) |
| **CL-1B** — continuous individual evolution | **ACTIVE · C10 paid physical source-parameter rollback verified (focused native CI); B3/B4 NOT MET** | [CL-1B plan](cl1b/README.md) · [B00 protocol](cl1b/studies/B00-self-edit-feasibility.md) |
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

**Current scientific conclusion:** CL-0 is verified infrastructure, not life evidence; CL-1A remains archived. CL-1B has source-pinned original Rust evidence for C10: a single PID1 paid to checkpoint a previously executed physical source parameter, attempted 228 restorations, physically changed the active executable parameter 19 times, and subsequently re-executed and read back the restored value 10 times. An independent CIL interpreter replayed all 887,330 paid instructions across 274 worlds without divergence in [focused Actions #45041352673](https://github.com/iFrankStudio/cioc/actions/runs/45041352673) (tested SHA `353e8fdc914dd9f52f66d22802ebb3afcd4ebb1e`). On 45 frozen truthful tasks, actual rollback cost 153,000 instructions versus 154,840 for the same-initial-source, same-charged PC102-effect suppression; this +1,840 causal advantage is **not** a global advantage: the different-source C9 checkpoint baseline cost 134,510. The rollback trigger is a researcher-authored state machine, **not** learned return-on-investment estimation or invented code. This is a focused 1-Rust-test/1-independent-replayer verification, **not** a current full 104-Rust/63-observer regression. The original evidence remains in a [time-limited Actions artifact](https://github.com/iFrankStudio/cioc/actions/runs/45041352673/artifacts/14761872568), expiring 2027-02-20; the [evidence branch archive](https://github.com/iFrankStudio/cioc/tree/cl1b-ci-evidence/run-records/353e8fdc914dd9f52f66d22802ebb3afcd4ebb1e-45041352673-1) now contains `original-native-artifact.zip` (the original 274 JSONL traces within 278 ZIP entries), its SHA-256 manifest and a [permanent integrity index](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/353e8fdc914dd9f52f66d22802ebb3afcd4ebb1e-45041352673-1/archive-integrity.md). The ZIP was checked when downloaded from Actions; the Git-stored copy's path and blob were confirmed, **but an independent fresh bytewise SHA-256/CRC check of that Git copy has not yet been reported**. The Actions artifact remains time-limited; the Git archive does not add a new Rust execution or replay. **B3 cumulative endogenous adaptation, B4 organizational closure and cyberlife remain unproven.** See [CL-1A archive](cl1a/README.md), [historical findings](cl1/RESULTS.md) and [CL-1B](cl1b/README.md).
