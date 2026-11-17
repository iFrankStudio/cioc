# B55a — frozen executable trial design (pre-native)

Status: **protocol freeze only; no B55 native result**. Parent scientific criteria: [B55 preregistration](B55-endogenous-monitor-meta-policy-variation-preregistration.md). Frozen on 2026-11-17 before B55a native execution. No claims of B3/B4.

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
