# B55a — frozen executable trial design (pre-native)

Status: **original preregistered complete protocol NOT yet executed. A separately labeled 5-arm×2-ecology×4-seed native feasibility pilot now exists and is audited below; the frozen full scientific acceptance conditions remain unchanged**. Parent scientific criteria: [B55 preregistration](B55-endogenous-monitor-meta-policy-variation-preregistration.md). Frozen on 2026-11-17 before B55a native execution. No claims of B3/B4.

## Falsifiable hypothesis

H55a (Tier 1 only): A source-identical continuous PID1 that pays to select and patch a monitoring route after prospective task-blind trials obtains strictly higher *predefined service utility* than BOTH (i) an equally funded fixed every-output monitor and (ii) an equally funded blind trial selector, in at least the truthful task-switch ecology. Its utility must not be lower than the blind selector in the stationary ecology. Failure of either condition rejects the joint H55a claim. A seeded oracle is an upper-bound comparator, not a success threshold.

## Experimental units and constraints

Use 8 precommitted world seeds per arm per ecology, one original PID1 per world, no PID replacement, no post-start host source injection, no compute refill. The same 8 seeds, input streams, regime-change points, graph caps and initial compute pool must be paired across all arms. **Before implementation**, derive and commit a single manifest specifying exact seed integers, task streams, native paid-step cap, graph cap, resource supply, and transitions; its hash must be printed by the Rust test and independent replay. Do not execute native B55a before this manifest exists. This document deliberately does not invent capacities that have not been checked against the existing Rust VM and B54 baseline.

## Prospective candidate contrasts

A: actor-paid selection between *never monitor*, *monitor every output*, and *monitor every fourth output*. These are explicitly investigator-designed policies, not discovered algorithms. Trial two consecutive service periods, collect actor-observable feedback and compute cost using existing CIL primitives, then use native paid PatchNext to retain one route. No host-side choice or edit.

B: fixed every-output monitor, equal original PID1 and resource opportunities. C: blind selector with identical trials and patch overhead but its feedback observation replaced by a constant, with no task label leak. D: fixed never-monitor. E: fixed every-fourth-output. F: task-aware static oracle, marked privileged and excluded from claims of endogenous selection. G: mutation-disabled control, matched for opportunities and no-op instruction costs.

## Frozen ecologies

E1: stationary truthful correct task; E2: hidden task switch then return, truthful current feedback; E3: current-nonce forged success and/or delayed response, with no hidden authenticity oracle. The actor sees only native-paid observations; the laboratory alone knows task truth. All modes must be evaluated on the same preregistered heldout service opportunities, not only on successful writes.

## Primary score and rejection criteria

For each world, let C = correct service writes, W = wrong writes, M = missing mandatory writes, P = actual paid CIL instructions. Define U = (C - 2 W - M) / max(1,P). Report the raw tuple (C,W,M,P) as well as U; terminated worlds count all remaining mandatory outputs as M. Compute paired world differences U(A)-U(B) and U(A)-U(C), with 8 seeds each. **Accept H55a only when every paired difference in E2 is positive and no paired difference U(A)-U(C) in E1 is negative.** This is intentionally stringent and falsifiable, not a statistical population-generalization claim. Also report instruction cost per correct output, paid monitoring reads, source edits, graph nodes, and first recovery paid-step latency. No tuning or exclusion after inspecting outcomes.

## Causal provenance and independent verification

Record actor-visible trial signals, paid instruction IDs, source-graph before/after hashes, exact patched edge, paid credit changes, external interventions, PID continuity, and all writes. A separate source-parsed JS interpreter must reproduce native event order and the first divergence must be null. Verify actual GitHub Actions tested SHA and original log; green Rust assertions alone are insufficient. Distinguish (1) source graph patch, (2) functional policy change, (3) improvement in U. Only (3) with controls supports narrow Tier 1.

## Negative predictions

In E1, an always-monitor policy wastes paid work compared with no monitor if monitoring has positive cost and the task is stationary. In E2, never-monitor is expected to miss task changes; this is a **prediction**, not a measured native result. In E3, a forged current-nonce success remains observationally indistinguishable from truthful success absent independent provenance; any claimed universal detection would falsify the stated observation model or indicate leakage. These predictions are frozen, not results.

## Gate before first native run

Implementation must first commit the complete manifest and Rust+CIL experiment together with independent JS replay; verify no B54 regression or implicit unlimited compute. If the source cannot support one PID1 performing prospective trials without a host-side policy choice, record B55a as blocked rather than substituting a laboratory-selected policy. Keep CL-1A archived.

**Subsequent B55a-PREFLIGHT FULL 56/43 source-pinned regression (2026-11-17):** [run44102110431](https://github.com/iFrankStudio/cioc/actions/runs/44102110431), tested source SHA `33f2b3a5992276de8c18327d116da17cbd38d8f7`, **scope=full, 56/56 named native Rust integration targets PASS, 43/43 source-parsed independent observer analyses PASS, B31/B34 ABI PASS, both failed counters zero; evidence publisher PASS**. [Original full CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/33f2b3a5992276de8c18327d116da17cbd38d8f7-44102110431-1/summary.txt) · [original same-source 160k-step independent replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/33f2b3a5992276de8c18327d116da17cbd38d8f7-44102110431-1/b55a_paid_monitor_choice_preflight_replay.json) · [exact SHA/run conclusion](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/33f2b3a5992276de8c18327d116da17cbd38d8f7-44102110431-1/result.txt). Original frozen B55a manifest bytes remain source-checked under SHA-256 `dff516c18ac087e885a8021b33d0a0b798a5b22fafd30ef94c32be4b2049c312`. Experimental outcome identical to the previously scoped green run: 40 worlds, 160,000 actually paid native CIL instructions,35,608 external interventions,892 edge changes, zero interpreter disagreement, **future-shift learner chooses NEVER and loses to fixed EVERY**. Full frozen7×3×8 B55a is **NOT** complete; H55a/B3/B4/cyberlife NOT MET.

## Post-registration partial feasibility evidence (NOT the full protocol)

[B55a native limited-pilot audit](../results/B55a-preflight-paid-monitor-choice-ecology-alias-audit.md), exact tested SHA `f442c3ebfb6fe25afea5980b8509ff46fda5b740`, [scoped9/9 native+43/43 observer run44100959934](https://github.com/iFrankStudio/cioc/actions/runs/44100959934): **40 genuine Rust PID1 worlds, 160,000 independently replicated paid CIL events, 35,608 external interventions, 892 executable-edge edits, first divergence null**. Source-origin paid monitor-route edits are mechanically demonstrated; all4 learning-arm worlds choose NEVER from successful pre-change evidence in both stationary and future-shifting ecologies. Under the latter they score **192 correct/96 wrong (U=0)** versus fixed every **280/8 (U=0.0165)**. **No H55a success claim**: only5 arms,2 ecologies,4/8 seeds and a reduced before-selection trial schedule were executed, not the original7×3×8 protocol; current-nonce forged feedback E3, privileged oracle F and disabled mutation G were not executed. The missing trial breadth must not be erased by changing this frozen document. The earlier source commit `e52cd4edf938900a2ee1e084a468f419a2a53151` failed due an initial-trial source reset vs retention classification bug and missing observer manifest URL; failures remain preserved. [B55b separate future online policy-revision protocol](B55b-online-monitor-policy-revision-under-hidden-change-preregistration.md) is NOT executed. B3/B4 remain NOT MET.
