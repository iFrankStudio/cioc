# CL-1B G3Q-C1-P0 — exact 180-node original PID1 paid rule deployment bridge

**Evidence level: FROZEN SOURCE + NEW RUST INTEGRATION TEST + CI REGISTERED; NATIVE SUCCESS NOT YET VERIFIED.** This is a limited precursor to C1, not completed task-service deployment or feedback learning.

## What was implemented

1. Prior to Rust test implementation, froze the exact [180-node founder](../studies/B56-F1d1k-G3Q-C1-P0-frozen-180-node-paid-bank-deployment-founder-v1.json) in commit `3b3ab2fae5b1bcba925d2d5c208601be9967e07f`.
2. The first **163** instructions are exactly the corresponding original C0 founder instructions; original C0 Halt at PC163 is replaced with 17 final instructions at PC163..179. All 180 nodes are reachable in the static control-flow graph. Original C0 rules and verdict semantics are preserved before final deployment.
3. Under the unchanged physical caps **16 distinct data addresses, 180 CIL nodes, 300 occupied units**, original PID1 is instructed to read saved bank cells 51,52,53, then write active descriptor cells 62,63,64. The bank is expected to contain `[1,3,2]`. It then reads active cell64 and emits a **diagnostic** Write11(2), before Halt. This final Write11 is *not* a hidden functional task service.
4. [Rust-native test](../../cl0/crates/world/tests/cl1b_b56_f1d1k_g3q_c1_p0_paid_bank_deployment.rs) committed at `580288490d90abfd3749263067ba21dfd84fae39`, with world instruction accounting, original PID1 provenance, immutable source, 64 structural certificates, 72 safe-bank retention writes, 3 original bank reads, 3 actor-paid writes into active slots, one final diagnostic emission, no PID2 and no post-load external rule write. The expected full run is **24,066 actor-charged instructions**, 32,000 initial credits, peak at most 197 occupied units.
5. Focused GitHub Actions workflow updated at `98f61fbc49ec52b795e453696b9a39c8b7e2765a` to register **73** named Rust tests, including P0. The pre-existing **63 independent replay observers** remain unchanged; **there is no independent original P0 step replay yet**.

## Precisely what P0 could establish upon a passing native run

The actor that originally computes rule certificates and retains its safe descriptor would, without source mutation or host copy-in, later perform native, separately charged reads from the bank and writes into active descriptor data. This would prove a restricted **same-PID rule-bank deployment primitive**, not that the active bytes subsequently direct a newly executed dynamic meta interpreter under a hidden task.

## What P0 cannot establish

P0 does **not** use paid original task-bytecode interpretation, input7, post-service feedback20, two probes and a heldout case, a hidden task change, feedback-conditioned selection, rule repair, or authenticated score. The output at PC178 is merely the bank's third word read back from active data. Certificate-based rescue and adaptive superiority over a privileged fixed safe rule are **not** supported. Full G3Q-C1, Hk and CL-1B final criteria remain pending.

## Next protocol gate

After obtaining exact-source native CI success, create a separate, prospectively frozen **C1-P1** controller that gives the bank-derived active descriptor real control over paid candidate task execution and source-origin task Write11. Include two seed orders, honest/blind/inverted/absent feedback, guarded/unguarded/monitor-only/static baselines, and an unreachable target. Strictly separate the new actual task services from certificate and P0 diagnostic output. Independently reexecute each original paid instruction and publish the source-pinned original trace. If a 180-node CIL founder cannot encode those steps, publish an infeasibility result rather than increasing physics or injecting host-side decisions.
