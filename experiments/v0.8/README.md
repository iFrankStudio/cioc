# CIOC V0.8 — distributed fragments and role recovery

Research toy model. No holder has a complete copy of another role's two-instruction program.

Run from this directory with Python 3.10+:

    python -m unittest -v
    python run.py

The environment supplies deterministic instruction interpretation, a bounded candidate alphabet, exact-code role oracle, budget and atomic program installation. O detects failures and can enumerate a replacement for G; G proposes for other roles; V checks; M commits.

Expected results: G-only loss recovers internally; O, V and M losses do not; G+M loss, missing fragments and insufficient budget fail. External restoration is separately counted.

**Limits:** Exact-code matching is not independent behavioral verification; fragments and role protocols are predesigned; no emergent self-organization, autonomous evolution, or demonstration of life is claimed. The central hypothesis of full organizational closure remains unverified.
