# CL-1B B53 — generic paid response correlation: detect stale feedback without a task-specific oracle

**Originally preregistered 2026-11-17 before any B53 native implementation. Now native-executed; see post-registration result appended below.** [B52 native 60,000-step evidence](../results/B52-native-paid-feedback-revalidation-and-alias-audit.md) proved that a paid watchdog can reacquire correct source after a hidden task shift, but also measured **96 false-success service outputs from a replayed success bit** and **8 stale false successes from a one-output-delayed bit**. [B52 abstract multi-probe alias counterexample](../analysis/b52_multi_probe_stale_alias.mjs) establishes that repetition of x-values or more probes alone does not guarantee temporal provenance. No B3/B4 or cyberlife inference is permitted.

## Frozen scientific question

Can an otherwise task-agnostic native CIL process **generate and pay to retain its own request identity**, submit it with task output, and verify that the externally delivered feedback is correlated to **that exact output** before acting on the success bit?

Use the **existing generic** `Const`, `Add`, `Sub`, `Read`, `Write`, `Branch`, `PatchNext`, `CloneNode` primitives. A new task-specific `GOOD_FEEDBACK` or `IS_FRESH` opcode is disallowed. Adding a new general event-log provenance opcode, if ever justified, must be a clearly separate kernel-level change with a preregistered ABI audit; do not silently change B52 Rust VM behavior.

### Assumptions and impossibility boundary

A response nonce can detect **accidental or adversarial replay of an older complete response packet that still carries its old nonce**. It cannot, without an authenticated/trusted evaluator, detect **fabricated false feedback carrying a deliberately updated nonce**. The B53 outcome must distinguish:

- **A. Honest source with stale/delayed delivery**: feedback content and echoed nonce both refer to an older output; native CIL should reject it.
- **B. Adversarial feedback for current nonce**: an attacker or faulty evaluator can write the current nonce with an untrue bit; the actor's nonce check must not be described as verifying semantic correctness. Include this as a hard negative control.
- **C. No response / wrong order**: a PID1 can only read a response on paid instructions. Missing messages, partial bit+nonce writes, or responses arriving between two ordinary Reads must never be scored as verified causality without explicit correlation.
- **D. Source identity**: generic paid `ReadLastWriter` distinguishes environment versus actor but not current-versus-stale `Origin::External`; never equate provenance alone with temporal correlation.

## Native actor acceptance design

1. Take the exact [B52 authored paid-search individual](../../cl0/crates/world/tests/cl1b_b52_native_paid_revalidation.rs) and add one **documented appended generic CIL request/response sequence**. All actor-side request sequence increments, source edits, feedback and nonce reads, comparisons and decisions consume native finite credits. No host mutation of PID1 code, credit, counter or selected candidate.
2. For each genuine output `Write11`, have the PID1 **write/retain an incrementing request serial** to a normal cell before waiting on the next external outcome. The host may echo the serial and a one-bit correctness value to two ordinary externally-origin cells. The actor must pay `Read` for both, compare the received serial against its own saved local serial via `Sub`, then only branch on the success bit when the serials match and when the *entire packet* is complete. A rejected response must not silently become a success via a preexisting bit in the cell.
3. No read-accessible hidden affine mode, expected output, winner hint, code repair instruction, or task-specific fallback. Laboratory changes only externally frozen inputs, response packet contents and hidden task mode. Use old B52 task shifts A `x-2→-x+3→x-2` and B `-x-3→x+2→-x-3`, source-identical across conditions.
4. Freeze per-world budget **4,000 paid CIL instructions** and strict **16 cells, 180 nodes, 300 storage units**, with a transparent comparison against B52's original 3,000-credit cost; all B53 arms share identical initial conditions and no compute top-up. Revalidate that 16 cells are sufficient for extra serial, echo, packet-complete flag; otherwise declare a protocol amendment BEFORE the first native run, not after seeing output.
5. Use the same native 24 service-output-per-stage three-regime evaluation and nonzero train probes. All worlds (including zero-service/early-stop) remain in denominator. Require both service accuracy and actual output throughput under the equal paid budget.

## Frozen causal controls

The minimum full prospective comparison is **two transitions × two deterministic input phases × five arm types**:

| Arm | External packet contents | Expected interpretation |
|---|---|---|
| FRESH_CORRELATED | Current serial echo, true correctness bit, complete packet | Necessary positive control for online re-adaptation |
| DELAYED_WHOLE_PACKET | Echo/bit from previous submission, paired with old serial | Should be rejected, not interpreted as current outcome |
| REPLAY_SUCCESS_PACKET | Reuse previous successful packet including old serial | Should not falsely maintain wrong program |
| FORGED_CURRENT_SERIAL | Current serial echo but dishonest success bit | Must be recorded as a **cannot-detect-content-lie** negative control |
| TORN_PACKET | Update bit and serial at different externally controlled moments | Recheck commit marker or reject without a false success |

Add a compact static task-known oracle as a separately labeled privileged upper bound; do not compare it as a source-blind individual. A `NO_ID` B52 baseline may be included if the same instrumentation/budget makes exposure fair; otherwise compare historical source-pinned results explicitly as non-matched.

## Independent validation and stop conditions

Use complete source-pinned same-CI real Rust events with PID1, paid credits, input/output provenance, native `Read` of feedback and serial, serial creation, comparison result, source code edit, actual writes and graph hashes. Independently reexecute **every actual paid native CIL event** with an interpreter that source-parses the real founder and the appended program literal. Missing/wrong SHA or divergence must fail closed; scientific outcome acceptance must not force a green result.

- **H53a accepted** only if truthful current correlated responses are actor-paid observed and source repair/retention is still functional on both shifts and returns.
- **H53b accepted** only if old-serial stale/delayed/replayed packets are rejected by **native actor decisions**, with no wrong success caused solely by replay. If complete-flag or serial is host-trusted in a way that decides task truth, reject H53b.
- **H53c intentionally bounded:** current-serial fabricated success cannot be distinguished without authenticating truth; if B53 appears to distinguish such a lie, independently audit for leaked task mode, answer, or host oracle behavior instead of claiming magical inference.
- **B3/B4 stay NOT MET** until evolution of the adaptation/monitoring policy, autonomous resource maintenance, larger non-preauthored task families and stable self-organization are separately evidenced.

**Post-registration result — original hypotheses and protocol above left unchanged:** [scientific B53 native audit](../results/B53-native-paid-response-nonce-replay-and-forgery-audit.md), exact tested SHA `ad360c7a16a62be33dbcf9773bbe4db92dd841f1`, [full CI run44068903769](https://github.com/iFrankStudio/cioc/actions/runs/44068903769), **54/54 native Rust targets PASS, 41/41 independent observers PASS, B31/B34 ABI PASS**. All **80,000 native paid CIL steps**, **12,088 external interventions**, **466 code-edge patches** independently matched across 20 native PID1 worlds with zero divergence. H53a accepted narrowly for PID1-paid response-serial binding and CIL-origin source recovery after genuine current feedback (**280 correct/8 wrong**). H53b accepted narrowly for rejection of complete older-sequence packet replays/delays before feedback-bit Read; this **does not** confer task recovery on those arms (**192 correct/96 wrong**) or validation of source truth. H53c's current-serial forged-success case recorded **96 wrong submissions scored as successes** and is a hard confirmation of the intrinsic content-authenticity limit. The torn packet arm checked stale commit **before** use, not adversarial between-Read atomicity. The B52 training trial still lacks a corresponding nonce check. [B54 prospective TOCTOU test](B54-native-paid-response-time-of-check-time-of-use-preregistration.md) is registered but not executed. **B3/B4/Cyberlife NOT MET.**
