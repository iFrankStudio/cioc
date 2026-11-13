# Stage-2 SQLite experiment registry and exact replay

Status: implemented as `cl0-lab` (Rust, SQLite via rusqlite). Infrastructure validation; **not evidence of life**.

```sh
cd cl0
CL0_SOURCE_REVISION="$(git rev-parse HEAD)" cargo run -p cl0-lab -- experiment.sqlite
cargo test --workspace
```

Each run records experiment condition, full serialized pre-intervention state, SHA-256 of that state, serialized external interventions, maximum execution attempts, actual attempts, full serialized final world (including events and origin labels), final SHA-256, schema version and source revision. The SQLite insert for matched conditions is one transaction; all experimental branches start from an identical `initial_json`.

`verify_replay(id)` hashes stored bytes, replays from the initial JSON and external interventions, then compares the complete resulting JSON. No unseeded RNG is present. A replay passes only under the **same CIL and world semantics**. The source revision is caller-supplied provenance, not a cryptographically attested running binary.

Negative controls: disable execution, zero computational credit, inert program (from world tests), rejected capacity write. These test platform behavior only. They do not establish any internally generated autonomy.

Limitations: the database has no signature or independent remote immutable checkpoint; an adversary with write access can rewrite all rows and hashes. SHA-256 here detects accidental/incomplete modifications and verifies replay, not authenticity. JSON encoding is stable under the current schema and BTreeMap ordering but not frozen as a multi-version canonical interchange standard. Schema migration and compatibility replay are deferred. The runner does not yet support a scheduled series of mid-execution interventions; interventions apply before executing a fixed number of attempts. Interventions are privileged **external** research operations and must never be interpreted as internally induced repair.

The SQLite log records external injections but a complete auditable accounting model for code graph memory remains future work. Avoid any claim of global energy conservation or open-ended evolution.
