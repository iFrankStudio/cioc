# CL-0 — Native Cyberlife Experimental World

**Status: CL-0 platform candidate, acceptance in [CL-0 validation record](docs/CL-0-acceptance.md).** This is a deterministic experimental world for investigating *possible* cyberlife, **not** a claim that cyberlife or open-ended evolution has emerged. The historical CIOC V0.1–V0.16 fixtures remain untouched.

## Included research infrastructure

| Milestone | Implemented scope |
| --- | --- |
| 0. Rules | Explicit epistemic contract, synthetic-control caveats and failure audit |
| 1. World | Rust deterministic round-robin VM, finite CIL registers, trace with external/internal origin |
| 2. Causality | SQLite persisted matched conditions, scheduled mid-run intervention, exact replay, persisted pre-action snapshots |
| 3. Information ecology | General program graphs, five-cell instruction descriptors, owner read access + revocation, bounded graph/data storage, strict allocation for process headers and grants, transferable compute pool |
| 4. Observation | Python standard-library read-only SQLite analysis and Vue 3 + TypeScript local-file observer |
| 5. Baseline | 24 seed-controlled initial programs, four matched conditions each, negative controls, recorded independent results and replay |

Generic CPU and graph operations do **not** encode `REPAIR`, `REPRODUCE`, `FITNESS`, or a host life/death verdict. An executable process, random opcode diversity, or a positive causal contrast is not evidence of life.

## Reproduce locally

Requires Rust stable, Python 3.10+, Node.js 22+ and npm. Cargo and npm **lockfiles are committed**.

```bash
cd cl0
cargo fmt --all -- --check
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
python3 -m unittest discover -s analysis/tests -v

# The 24-seed baseline registers and replay-checks 96 matched runs.
CL0_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run --locked -p cl0-lab --bin cl0-explore -- experiment.sqlite 24 54129
python3 analysis/analyze.py experiment.sqlite --json report.json --csv report.csv

cd console
npm ci
npm run dev
```

Import the generated `report.json` through the web observer's file selector. The viewer has no direct execution, host file or mutation privileges over the world. For historical miniature fixtures use `cargo run --locked -p cl0-world` or `cargo run --locked -p cl0-lab -- experiment.sqlite`.

## Documents

- [CL-0 acceptance and scientific boundary](docs/CL-0-acceptance.md)
- [Formal architectural baseline](docs/specification.md)
- [CIOC historical evidence audit](docs/cioc-transition.md)
- [Code/data allocation](docs/stage-3-graph-storage.md)
- [Executable graph objects](docs/stage-3-objects.md)
- [CIL instruction ABI](docs/stage-3.3-instruction-abi.md)
- [Causal scheduling and read permissions](docs/stage-3.4-scheduled-permissions.md)
- [Exploratory preregistered baseline](docs/stage-5-exploratory-baseline.md)
- [Analysis](analysis/README.md) / [Observer](console/README.md)

The unified storage ledger uses **abstract units**, not physically calibrated bytes or wall-clock energy. Any subsequent CL-1 research requires independently preregistered hypotheses and replicates; it must not retrospectively call these CL-0 synthetic fixtures organisms.
