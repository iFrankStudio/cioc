# CL-1B B56-F1d1a — prospective native correlated-receipt price and information gate (v1)

**PRE-REGISTERED BEFORE native implementation. This is a direct-path engineering microcalibration, NOT a self-assembling organism, policy chooser, or B3/B4 success.** This experiment is deliberately narrower than the prospective B56-F1d1 full source-generated selection program, which remains pending.

## Fixed scientific question

F1d0 showed uncorrelated F1c EVERY paths strictly dominating FOURTH (12..24 vs 39..42 CIL monitor instructions per four real service writes). Does grafting the **real B53 source-semantic, paid nonce/echo/commit/bit receipt checker** onto the same two preassembled monitor routes produce a *conditional* compute/information tradeoff, and can it reject stale receipts without authenticating deliberately forged current-serial successes? A static F1d1 cost script predicts sensor source lengths 15 (fresh), 9 (echo mismatch), 15 (commit mismatch). Native VM behavior and strict resource legality are **not established** until CI records it.

## Frozen experimental matrix and resources

- Exactly **128 native PID1 isolated worlds = 2 routes (EVERY/FOURTH) × 4 response modes (FRESH, REPLAY_OLD, TORN_COMMIT, FORGED_CURRENT) × 16 possible truth-bit masks** across four service writes. One continuous world and one PID per cell; never reset its counter or nonce between the four services.
- Every world begins with **500** native compute credits, **16 maximum live data cells, 180 maximum CIL source nodes, 300 maximum total storage units**. All code is preassembled by the researcher, **no source edits or actor policy invention**, no free credit. Initial source vector size 168. Reuse idle PC slots rather than silently exceeding180.
- The **shared** paid B53 seven-operation request-serial generator runs once per service (28 CIL instructions across four), plus four common genuine service Write11 events. The service output's success/meaning is independently defined by the laboratory truth mask; the actor sees only packet cells.
- Both routes use the same CIL receipt checker topology structurally copied from B53 nodes106–123: paid Read of request serial26, echo27, commit28, Sub, Branch, and only on coherent packet Read20 then Branch. Reject stale echo or commit by a real paid Write29. **Nonce correlations are not cryptographic signatures**.
- The original F1c FOURTH counter uses key28, which conflicts with B53 commit28: relocate only this counter to an otherwise unused data key **40** identically on the paid periodic counter path. Counter rescoping is research-authored and must be disclosed. The remaining actual live keys must be counted and never exceed 16.
- The F1c common continuation PC110 collides with B53's Sub opcode at110: relocate the *neutral* common start to PC70. Keep the source-derived differential EVERY and FOURTH control-flow structure, and document every remapped terminal edge. Failure marker remains key25, rejection marker key29.
- The external packet intervention occurs only **after each real paid Write11**; perform three separate SetCell interventions bit20, echo27, commit28. They are free external inputs, not free computation or proof of trust:
  - FRESH: truthful bit, echo=nonce, commit=nonce.
  - REPLAY_OLD: success bit1, echo=nonce-1, commit=nonce-1.
  - TORN_COMMIT: success bit1, echo=nonce, commit=nonce-1.
  - FORGED_CURRENT: success bit1, echo=nonce, commit=nonce despite potentially false underlying truth.
- Run all masks to distinguish receipt validity from actual output truth; never treat forged current success as correct.
- Capture the native event-by-event CIL PC, operation result, process registers, credits, graph length, receipt/counter cells, memory/storage and all external interventions in source-pinned JSONL. Add a **separate JS observer** checking original events and path-level instruction/cell/credit invariants; state clearly if it is not a complete independent VM reexecution. Preserve exact-SHA original CI evidence, including failures.

## Predeclared path cost predictions (excluding 28 shared nonce operations and 4 Write11)

Per sampled *fresh or forged-current successful* receipt, the generic checker costs 15 instructions. For an old-echo rejection it costs9 instructions including paid marker write. For torn-commit rejection it costs15. The unchanged FOURTH periodic prefix costs8 on three unsampled services and12 on its fourth.

| Four-service packet condition | EVERY charged monitor instructions | FOURTH charged monitor instructions | Bits actually read |
|---|---:|---:|---|
| All FRESH, mask1111 | 60 | 51 | 4 vs 1 |
| All FRESH, arbitrary truth mask | 60+3 × number of failures | 51+3 × (fourth bit is failure) | 4 vs 1 |
| All REPLAY_OLD | 36 | 45 | 0 vs 0 |
| All TORN_COMMIT | 60 | 51 | 0 vs 0 |
| All FORGED_CURRENT | 60 | 51 | 4 vs 1, with false bits being accepted |

Evidence of a meaningful Pareto frontier is **conditional**, not universal: FRESH should make FOURTH cheaper but less informative; REPLAY_OLD may make EVERY both cheaper and more rejection-sensitive. Reject or qualify whenever measured native costs diverge. No throughput/benefit claim from sensor-only steps.

## Explicit falsifiers and exit criteria

The gate FAILS if strict capacities cannot be met, if any cloned receipt node path is skipped, if packet intervention happens before Write11, if paid nonce steps vanish, if source-correlation read is bypassed, if FRESH/REPLAY_OLD/TORN_COMMIT/FORGED_CURRENT are conflated, if any original native event or observer is missing, or if downstream work claims this direct-path calibration proves N56-A/B/C, B3/B4, or cyberlife.

**Next phase only if pass:** test **actor-paid assembly and source-identical policy-selection** under the original B56-F1d1 gate, with a feasible live-cell layout, isolated edits, and strong static baseline. This manifest makes **no assertion of such implementation**.
