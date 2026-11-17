# CL-1B B55a — native paid monitor-choice feasibility pilot and unseen-regime information failure

**Status:** Actual native Rust/CIL **pilot verified**; this is **NOT** the frozen complete B55a protocol, **NOT** successful H55a, **NOT** Tier 2/B3/B4/cyberlife. It tested a meaningful subset in advance of the complete 7-arm ×3-ecology ×8-seed protocol and must not silently be promoted to a preregistered positive result.

## Original frozen design versus this explicitly narrower executable pilot

Frozen before any pilot native code:

- [B55a full executable protocol](../studies/B55a-monitor-choice-executable-protocol-freeze.md), requiring **seven** matched arms A–G, **three** ecologies E1–E3, **eight** specified seeds and strong causal acceptance criteria.
- [B55a v1 resource manifest](../studies/B55a-executable-resource-manifest-v1.md), SHA-256 **`dff516c18ac087e885a8021b33d0a0b798a5b22fafd30ef94c32be4b2049c312`**, freezing one original PID, 4,000 native credits, 16 cells, max180 nodes, max300 storage, two tasks and two trial blocks, 24 service opportunities per stage, source-identical arms.
- **This feasibility pilot deliberately implements only 40 native worlds** = 5 arms × E1/E2 × first4 seeds **101,211,307,401**. It **does not run** E3 forged *current-nonce* response, control F task-aware oracle, G mutation-disabled, last four seeds, or all three candidate policies as separately scored pre-selection trials. E3 must not be mislabeled as a simple uncorrelated bit flip. Thus all original complete H55a acceptance gates remain **UNFULFILLED**.

## Exact source-pinned execution evidence

- Tested SHA **`f442c3ebfb6fe25afea5980b8509ff46fda5b740`**, [scoped successful CI run 44100959934](https://github.com/iFrankStudio/cioc/actions/runs/44100959934): **9/9 selected actual Rust native target tests PASS; 43/43 independent observer programs PASS; B31/B34 ABI PASS; failed target and observer counts both 0; native evidence publication PASS**. This is **scoped**, not the full 56/56 Rust suite at this SHA.
- [Original same-run CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f442c3ebfb6fe25afea5980b8509ff46fda5b740-44100959934-1/summary.txt) · [original exact-SHA run conclusion](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f442c3ebfb6fe25afea5980b8509ff46fda5b740-44100959934-1/result.txt) · [original actual Rust native steps](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f442c3ebfb6fe25afea5980b8509ff46fda5b740-44100959934-1/b55a_preflight_native_steps.jsonl) · [independent source-parsed replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f442c3ebfb6fe25afea5980b8509ff46fda5b740-44100959934-1/b55a_paid_monitor_choice_preflight_replay.json).
- [Original Rust pilot](../../cl0/crates/world/tests/cl1b_b55a_paid_monitor_choice_preflight.rs) · [separate JavaScript CIL VM replay](../analysis/b55a_paid_monitor_choice_preflight_replay.mjs) · [exact original B52 founder reused unchanged](../../cl0/crates/world/tests/cl1b_b52_native_paid_revalidation.rs).
- The original implementation at SHA `e52cd4edf938900a2ee1e084a468f419a2a53151` [run44100408628](https://github.com/iFrankStudio/cioc/actions/runs/44100408628) **FAILED**: its harness misclassified the original B51c `node2:42→35` **failed candidate reset** as `node2:42→selected` **successful retention**, and its independent JS observer used a wrong manifest-relative path. Both are explicitly preserved in the failed original evidence. The fixed Rust code and independent observer in passing [run44100959934](https://github.com/iFrankStudio/cioc/actions/runs/44100959934) distinguish failed search retry from real retained monitoring source, and use the manifest's actual code-relative URL. No claims from the failed run were promoted.

The separate JS interpreter source-parses the original B51c 72-node CIL founder, B52's original 20-node append, plus B55a's 51 explicitly authored CIL operations placed in the bounded **178-node initial graph**, with **two native-paid generated arithmetic clones** filling the **180-node** maximum. Under strict `World::with_strict_capacity(16,180,300)` and no credit replenishment, the interpreter independently reexecuted **160,000/160,000 native paid CIL steps**, **35,608/35,608 external input/feedback interventions**, and **892/892 observed native executable-edge changes** across all40 worlds, including PC, registers, code length, output edge, node76's `PatchConst` literal, storage, credit, observed data cells, actor state and 40 final objective scores. **First divergence = null**. The initial source cost is high because many researcher-authored dormant code nodes occupy storage.

## What the actor actually did

This is a human-written CIL trial-and-choice policy. PID1 pays to modify the existing B52 retention bridge (a `PatchConst` on node76 to route into the trial program), executes exactly four genuine trial outputs (2×2 probes under the original task), pays to Read success feedback during its first two trials, and uses ordinary `Read`, `Add`, `Branch`, `PatchConst` and `PatchNext` to set the retained writer2 edge to a monitoring path:

- **NEVER**: `node2.next→30`, the original B51c no-feedback service path.
- **EVERY**: `node2.next→77`, the B52 charged post-output watchdog.
- **FOURTH**: `node2.next→160`, a separate 10-instruction paid counter path that checks feedback every fourth output and routes to B52's original monitor for potential reopening.

At source selection, PID1 also pays `PatchConst node76` so an *eventual* B52 search retention restores the same policy. In controls B/D/E, the experimenter preloads a clearly labeled fixed policy-route hint in ordinary cell27 **before startup**. That is an **externally assigned fixed control**, not actor-discovered policy. The trial and route mutation are actor-origin; the policy menu, trial plan, candidate operator grammar, task truth evaluator, reset, and threshold rule are all investigator-authored. No claim of a self-invented monitoring meta-policy is supported.

## Native results and original fixed-budget utility

Each arm×ecology includes four paired worlds, each with **P=4,000** realized paid CIL instruction opportunities and 72/72 service outputs (288/288 scored service opportunities per arm+ecology). Utility **U=(C−2W−M)/P** is the original frozen primary score; `M=0` for all this pilot's scored worlds.

| Ecology | Arm | PID1 monitor route after two trial blocks | C | W | M | P | U |
|---|---|---:|---:|---:|---:|---:|---:|
| **E1 stationary** | A actor choice | NEVER (4/4) | 288 | 0 | 0 | 16,000 | **0.018** |
| E1 | B fixed EVERY | EVERY | 288 | 0 | 0 | 16,000 | 0.018 |
| E1 | C blind choice | NEVER | 288 | 0 | 0 | 16,000 | 0.018 |
| E1 | D fixed NEVER | NEVER | 288 | 0 | 0 | 16,000 | 0.018 |
| E1 | E fixed FOURTH | FOURTH | 288 | 0 | 0 | 16,000 | 0.018 |
| **E2 hidden shift and return** | **A actor choice** | **NEVER (4/4)** | **192** | **96** | 0 | 16,000 | **0.000** |
| E2 | B fixed EVERY | EVERY | **280** | **8** | 0 | 16,000 | **0.0165** |
| E2 | C blind choice | NEVER | 192 | 96 | 0 | 16,000 | 0.000 |
| E2 | D fixed NEVER | NEVER | 192 | 96 | 0 | 16,000 | 0.000 |
| E2 | E fixed FOURTH | FOURTH | 256 | 32 | 0 | 16,000 | 0.012 |

**Crucial negative:** on each of the four paired pre-shift trials, the externally hidden future shift makes *no difference* to the original task's trial feedback. Therefore the actor chooses NEVER in both E1 and E2: `stage_policy_recommendation_same_before_hidden_shift=true`. When E2 later switches, it receives **no more feedback Reads**, and all96 shifted-task service writes are wrong. The blind C baseline makes the **same choice and has identical utility**. Native source choice is real but **not useful generalization to unknown change**. By contrast, B fixed EVERY actually reopens and retains after each change (8 reopens, 280/8); E fixed FOURTH pays less frequent reads but incurs 32 wrong outputs (256/32) before detecting changes. The 40-world pilot fails the prospectively desired positive E2 paired superiority.

**Metric limitation discovered WITHOUT changing frozen score:** because all worlds fully complete the 72 *capped* scored service submissions but continue spending the remaining 4,000 instructions, U cannot distinguish the stationary overhead of **0 E1 service feedback Reads** in A/D from **approximately 380/354 per world** in fixed EVERY B; E1 utility is the same0.018 for every arm. This is a *property of the frozen score/observation window*, not evidence that monitoring costs nothing. Prospective later studies should report service throughput under a fixed paid budget and/or completion latency in native steps, in addition to preserving original U, before calling the resource-efficiency question resolved.

The full 7×3×8 B55a hypothesis remains **NOT YET TESTED**; this narrower pilot's negative observations are useful for experimental design but may **not** be cited as a full preregistered H55a failure or success. At most this is paid selection of an **authored finite menu** (Tier 1 *mechanism feasibility*), not a successful Tier 1 utility claim, and not Tier 2.

## Research interpretation

A native actor that has seen only stationary successful trials **cannot distinguish** two deterministic worlds that share identical task input/output/feedback histories before a hidden future shift. Selecting NEVER for short-term efficiency cannot be justified as robust to unknown future change without risk preference, environmental priors, minimum sampling, resource pressure, or a new signal. This is not solved by a third fixed watchdog.

**Next discriminating step:** preserve the frozen full B55a manifest as incomplete; prioritize a new prospective experimental variant that (a) makes policy exploration represent the actual future ecology without leaked regime labels, (b) includes generic paid nonzero intermittent sampling so a selected low-cost monitor can later revise itself on evidence, (c) measures first-correct-recovery latency and work per useful output without the capped-window score alias, and (d) again distinguishes seed-authored menu choice from genuinely source-generated and later self-revised policy topology. Do not add a task-specific `AutoAdapt` primitive or claim a future-shift oracle. **B3/B4/CYBERLIFE NOT MET.**
