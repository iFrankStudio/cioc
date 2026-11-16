# CL-1B B44 — independent source-parsed model forecast, native verification pending

**Frozen checkpoint before first B44 native Rust CI.** [32-world protocol](../studies/B44-observational-history-equivalence.md) · [native Rust study](../../cl0/crates/world/tests/cl1b_b44_causal_history_alias.rs) · [source-parsed paired interpreter](../analysis/b44_causal_history_alias_scan.mjs).

The independent CIL model parses the actual 38-node B42 Rust founder (no cloned or supplied program alternatives). It executes the researcher-authored, temporary external edge and shared-cell interventions, checks all model-accessible substrate state after restoring edges, and steps both actor branches forward under identical future interventions.

**Model-level prediction, not native results:**
- 8/8 pairs converge to identical code graph, registers, PC, cells, compute budget, tick and all remaining substrate state, while their experimenter-only event logs expose different histories.
- In 8/8 pairs, **117 subsequent paid native-style instruction observations are identical**. In 6/8 pairs the individual really wrote a different diagnostic value in the two worlds; for train=-1 both write zero but the last writer still differs (internal versus external).
- The legitimate **native diagnostic** success arm (its success induced by transient external routing) and the **externally forged diagnostic** success arm each emit **0 correct /104 wrong** on eight nonzero held-out worlds; both retain the original wrong code after convergence.
- Unmodified B42 natural source: **72 correct/26 wrong** holdout outputs; four-node STATIC_CORRECT: **128/0**.
- Each arm of 8 worlds spends 1,024 CIL instructions, all 32 worlds 4,096; none allocates an additional task actor or receives compute during the study.

The expected negative result is **epistemic**: current mutable shared state alone contains no native writer provenance after different histories converge. The observer-only `World::events()` does, but current CIL actors cannot read it. This does **not** prove robust provenance is impossible with changed generic substrate physics or that investigator-induced native success is organic autonomous adaptation.

**Native outcome UNVERIFIED until exact SHA-pinned Rust trace and full-source comparison appear in CI evidence.**
