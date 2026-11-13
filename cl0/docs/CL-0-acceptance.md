# CL-0 Stage Completion Contract — Infrastructure, Not Life

## Meaning of “CL-0 complete”
CL-0 is a **minimal reproducible, causal-intervention-ready native digital world**, not the discovery of cyberlife. Completion means the laboratory can express mutable programs as information, impose finite resource rules, run independently replayable experiments, and observe results without any life-oriented host oracle. Positive evidence for self-maintenance, organizational closure and open-ended evolution is **not** an acceptance criterion of the experimental infrastructure; those are the prospective research questions of CL-1 and later.

## Stage acceptance gates

| Gate | Mechanism | Objective verification |
| --- | --- | --- |
| G0: historical separation | CIOC V0.1–V0.16 synthetic repairs are archived; CL-0 world has no organism/fitness/repair host primitives | `cioc-transition.md`, core opcode audit |
| G1: deterministic execution | Rust scheduler, bounded CIL graph interpreter, complete event origin trace | same-seed world/trace equality; traps and zero-compute negative controls |
| G2: resource consistency | strict abstract storage for graph/data/process/grants, one computation credit per attempt, explicit transferable compute pool | capacity, refund, no-credit-mint and overflow conformance checks |
| G3: generic information dynamics | programs can read, write, create, replace, inspect, launch and delete executable graph structures with ownership checks | ABI round-trip, unauthorized read/write, atomic rejection |
| G4: causal interventions | intervention after a specified number of instruction attempts; byte-identical matched baselines; persisted snapshots and exact SQLite replay | pre-action snapshot hash/replay, missed-action control, tampering test |
| G5: independent observation | Python read-only SQLite metrics, hash validation and local Vue3/TS inspector | Python standard-library unit tests, frontend type-check/build |
| G6: exploratory protocol | researcher-generated seeded CIL baselines with no host fitness selection, four explicitly matched conditions | 24 seeds × 3 interacting processes × 4 matched conditions = 96 recorded runs, each replay verified |
| G7: reproducible build | committed `Cargo.lock` and `package-lock.json`, clean formatter and CI build | `cargo fmt --check`, `cargo test --locked`, strict Clippy, `npm ci && npm run build` |

## Hard scientific limits and provenance
- Initial random programs are **investigator supplied** and do not constitute spontaneous algorithm invention. CL-0 does not measure biological reproduction and labels no process an organism.
- Any experimenter-supplied compute pool refresh, cell modification, source-code preload or forced stop is logged as external. CIL program instructions are logged internal; world scheduling and budget decrements are deterministic kernel mechanics.
- “Novelty” from a seeded generator is externally introduced diversity, not internally generated evolutionary innovation. The baseline starts from researcher-generated distinct 12-node programs sharing a data space and deterministic scheduler; each carries eight credits. The Python dashboard computes descriptive differences without a self-maintenance score or life classifier.
- Storage units are discrete simulation units. Registers are covered by a constant per-process header charge in strict mode; byte-accurate host memory, real-time CPU scheduling and the analyst's SQLite log storage are not world resources.
- Read grants are limited to object IDs and fixed process IDs; no generalized object ownership transfer or multi-tier persistent/working storage is modeled. These are valid CL-1 extensions, not hidden claims of CL-0.
- Database SHA-256 and exact replay detect inconsistent stored state but are not an authenticated tamper-proof adversarial archive. Cross-version replay is explicitly unsupported: compare matching world format, source revision and pinned dependencies.
- No result from this baseline warrants claims of autonomy, self-maintenance, organizational closure, open-ended evolution or life. Report null results and unperformed interventions as faithfully as positive contrasts.

## Final acceptance — 2026-11-13

**CL-0 infrastructure acceptance: PASS.** The final executable-code checkpoint `625d2d6922ac72dc2089c2667cc52f1e747d20fb` was verified by [GitHub Actions run 43406674543](https://github.com/iFrankStudio/cioc/actions/runs/43406674543). Evidence from that run:

- Rust workspace tests: **48 passed, 0 failed** (32 world, 14 laboratory, 2 seeded-population runner).
- Independent Python analysis: **3 passed, 0 failed**.
- Rust `cargo fmt --all -- --check`: **passed**; strict `cargo clippy --locked --workspace --all-targets -- -D warnings`: **passed**.
- Vue 3 + TypeScript `npm ci` and production `npm run build`: **passed**.
- **24 independently seeded three-process worlds**, each branched into four matched conditions = **96 SQLite runs**; **96/96 exact replays passed**.
- SQLite, descriptive JSON and CSV output were archived in the named GitHub Actions artifact `cl0-acceptance` (artifact ID `14027218262` in the same run).
- The full CI checks committed Cargo/npm dependency lockfiles and an explicitly marked source revision. The acceptance artifact is retained by GitHub Actions for 30 days; reproducing it is documented in `cl0/README.md`.

**Acceptance applies to the CL-0 experimental platform, not to evidence of cyberlife.** Neither autonomous maintenance nor organizational closure nor open-ended evolution was established. Their absence as evidenced findings is not an infrastructure-test failure.

## Operational gate
The single end-to-end GitHub CI job verifies all components and archives SQLite, JSON and CSV acceptance data. The accepted executable checkpoint has the green source-code workflow cited above. Subsequent documentation-only commits do not alter tested execution semantics; verify their own CI run as a final source-tree consistency check. A later code change requires a fresh acceptance run. The full repository checkout is required to reproduce historical source revision labels; the Python observer never mutates the world.

For next-stage science, preregister independently falsifiable measures of maintained organizational dependencies, external-source tracing, nontrivial novelty accumulation and damage/restoration controls before exploring outcomes.
