# CL-1B B43 — model-only source-provenance forecast (native run pending)

**This checkpoint precedes native B43 execution.** The 40-world independent CIL interpreter parses B42's actual Rust CIL code and schedules the identical B43 external `SetCell` interventions at explicit native PC boundaries. It is an executable prediction, **not** Rust native evidence.

[Full frozen protocol](../studies/B43-forged-feedback-provenance.md) · [native causal intervention test](../../cl0/crates/world/tests/cl1b_b43_feedback_provenance.rs) · [independent model](../analysis/b43_feedback_provenance_scan.mjs).

The single actor's real diagnostic output is **always 0** at its initial PC10. In six of eight worlds, a paid CIL `Read` of cell11 after the hostile intervention returns the **externally forged** diagnostic value `train+1` instead. The actor's first comparator therefore accepts a code path that did not actually produce the claimed value. It retains the wrong code, never calls native `CloneNode`, and loses all unseen nonzero task service.

The original zero-challenge worlds (`train=-1`) are counted but do **not** become new false verdicts. The source-authentic natural/echo/late controls yield the original B42 eight-world totals `72 correct / 26 wrong` holdout writes; the early-forged group predicts **0 correct / 104 wrong**, while STATIC_CORRECT yields `128/0`. Each group spends exactly 1,024 CIL instructions (eight worlds × 128). Attacker interventions are explicitly **external**, neither locally paid nor output counted.

This is an attack-limited result. The test does not show that the actor invented adversarial deception or possesses a general trust verifier. A task evaluator that relies on shared cells without native event provenance/authoritative origin may be deceived; this does not automatically imply that every possible self-evaluation process is unsafe. B3/B4 are unchanged: **NOT MET**.
