# CL-0 — Native Cyberlife Experimental World

CL-0 is a new research platform, **not** a continuation of CIOC's pre-programmed maintenance roles. The historical experiments remain unchanged in `experiments/`.

**Evidence status:** the first milestone is infrastructure. Executing a graph or persisting a trace is not evidence of life, self-maintenance, self-organization or open-ended evolution.

## Scope and status

- Stage 0: specification, limitations and preregistered tests.
- Stage 1: Rust deterministic minimal kernel, finite execution/storage, CIL graph interpreter, event trace and matched negative controls.
- Stage 2: **implemented foundation**: SQLite matched-run registry, labeled external interventions, serializable world snapshots and exact same-version replay. Mid-run interventions and long-term compatibility remain deferred.
- Stage 3: **partial**: bounded executable graph objects, owner-controlled read grants and revocation, move-based launch with credit transfer, and five-cell descriptors generating existing CIL opcodes. Byte-accurate memory accounting and full instruction introspection remain incomplete.
- Stage 4: Python analysis and Vue 3 / TypeScript observational console.

The Stage-3.4 lab additionally supports deterministic **mid-run intervention schedules** and SQLite exact replay; see [Stage-3.4 specification](docs/stage-3.4-scheduled-permissions.md).
- Stage 5: exploratory populations and open-endedness measurements.

See [technical specification](docs/specification.md), [stage plan](docs/stage-1-plan.md), [CIOC evidence audit](docs/cioc-transition.md), [Stage-3.1 allocation](docs/stage-3-graph-storage.md), [Stage-3.2 graph objects](docs/stage-3-objects.md), and [Stage-3.3 data-to-instruction ABI](docs/stage-3.3-instruction-abi.md).

The experimental world does not label programs as organisms. An observer may later classify organization candidates from externally computed evidence. Initial programs are interventions supplied by the experimenter, not instances of discovered life.

## Local commands

Once a Rust toolchain is available:

```bash
cd cl0
cargo fmt --all
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p cl0-world
CL0_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run -p cl0-lab -- experiment.sqlite
```

No claims of test success should be made without actual execution. GitHub CI is a separate verification channel.
