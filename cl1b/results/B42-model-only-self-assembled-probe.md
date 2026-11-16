# CL-1B B42 — independent model forecast, NOT native verification

**Model-only checkpoint, recorded before the first B42 Rust-native CI result.** [Frozen native protocol](../studies/B42-self-assembled-probe-boundary.md) · [Rust source](../../cl0/crates/world/tests/cl1b_b42_self_assembled_probe.rs) · [Rust-source-parsed CIL interpreter](../analysis/b42_self_assembled_probe_scan.mjs).

The independent JS interpreter parses the literal native Rust CIL graph and applies the same proposed `Const`, `Add`, `Read`, `Write`, `Sub`, `Branch`, `CloneNode`, `ReadNext` and `PatchNext` semantics with explicit charges. It covers **48 preselected worlds**, no post-selection of good runs. **Do not call this a native result.**

Main forecast:
- `ProbePlus` and `ProbeMinus` each attain **72 correct / 26 wrong holdout service writes** across8 matched worlds.
- `NoProbe` also attains **72 correct / 26 wrong holdout service writes**: adding a simple endogenous challenge did **not** improve aggregate holdout performance.
- `IgnoreFeedback` produces **0 correct /104 wrong** holdout writes, identifying an observable causal role for real task feedback within this authored mechanism.
- `PatchDisabled` halts all eight worlds after consuming **312 total** (versus1,024 in continuing groups); it creates8 uncommitted clones, without successful router edits.
- Strong four-node `StaticCorrect` produces **128 correct /0 wrong** holdout writes per1,024 charged native instructions.

The x+1 probe rescues the original B41-style zero-only alias case yet creates an **x=-1** alias; the x-1 probe shifts the alias to **x=1**. There is no general uncertainty management, reliable cross-context verification, internally discovered objective or performance advantage.

Native mechanism evidence, resource accounting and any scientific conclusion remain **UNVERIFIED until a source-SHA-pinned Rust run is inspected**. Preserve discrepancies between the forecast and native traces as failures, not as revised pre-registered predictions.
