# CL-1B B44 — causal-history aliasing in the existing world information physics

**Prospective protocol fixed 2026-11-16, after the B43 native findings and before the B44 Rust CI trial.** Independently source-parsed predictions below are **model forecasts**, not native evidence. B44 does not add any CIL instruction, fitness score, preinstalled verifier or host-administered actor repair.

## Why B44 rather than another heuristic

B43 showed that the B42 actor believes a forged value because shared cell11 contains what its next ordinary `Read` returns. A single fixed probe can move the blind spot (B42), and an external timely overwrite can create a false success (B43). The next question is **epistemic**: *is the information needed to distinguish genuine from forged output even present in the accessible world state?*

Construct two histories leading to **byte-for-byte equal serialization of every World state field except the private observer event history**. The lab must independently verify that one history genuinely executed a native CIL Write of the correct diagnostic value, while the other executed a native CIL Write of an incorrect value and had the apparent success externally forged. Both worlds then continue, at identical program counter, registers, code nodes, shared cells, compute inventory, storage use, scheduler, tick, and future environment inputs.

This is a paired counterfactual test of a **state-information impossibility**: given the same deterministic guest state, no CIL instruction sequence that reads only state-accessible substrate information can retroactively infer **which different historical event** produced the same current value. It is not a universal impossibility of life, nor proof about other substrates, untested nondeterminism or sensors with access to historical origin.

## Exact source and causal construction

Use **the same 38-node native B42 `ProbePlus` founder** imported into B44's Rust test. One PID1, no reproduction, no new VM rules, 128 local compute, zero shared pool, strict storage cap. Four training values `[-1,0,1,17]` × two held-out nonzero values `[-9,23]` yield **eight paired histories (16 worlds)**. Add **eight natural/unmodified B42** and **eight immutable four-op correct** reference worlds. **Total denominator: 32 native worlds**, 128 CIL charges each, 4,096 task-world charges overall.

After original environment input and the actor's first **nine charged instructions**, the same ancestor CIL program has PC9, register0=`challenge = train+1`, shared cell7=`challenge`, and saved original input at cell30. **Fork this single world into matched independent research trials**; this is an explicit researcher control, not native replication or actor self-reproduction.

At precisely PC9, use transient **EXTERNAL** `Intervention::PatchEdge` events to construct different task-output histories:

- **Real native output**: temporarily redirect B42's router at PC9 to its **existing generic PC7 Write** and set PC7's successor to PC11. The actor pays one native execution to write register0 (`challenge`) into cell11 (the router has set address register2=11). The external lab then restores both source edges (PC9→10, PC7→8). The last writer to cell11 remains the genuine **native CIL PC7**.
- **Externally forged output**: apply two equal-number no-op control interventions to the same edges, execute the ordinary router PC9 and B42's **native faulty PC10 Write0**, then explicitly overwrite cell11 using **EXTERNAL** `SetCell(key11, challenge)`. Restore the same edges to the same values.

At the corresponding post-output slot in the legitimate trial, apply **one equal-count no-op external SetCell to the already-matching task-input cell7** (not output11). Both histories therefore have exactly the same count of external interventions, native instruction charges and world ticks, although **their host-only event logs record different intervention targets and native writer PCs**. They converge at PC11.

**Important scientific boundary:** the real native output in the positive-control history was caused by **externally temporary source-edge routing**, not organically discovered by the actor. The experiment proves observational aliasing, not ability to invent authentic execution, naturally defend itself or learn.

### Explicit verified equality criterion

Serialize each native `World` using the project's existing `serde_json`. Remove **only** the private `events` member, leaving every other serialized world field intact (not just a selected set of task cells). Require exact JSON value equality, and separately require distinct event histories with native-PC Write provenance and external `SetCell` attribution. The CIL ISA contains no instruction that reads `World::events()`.

Then run the two worlds **in exact native step-by-step lockstep** for the remaining53 steps in the first64-step phase; externally set the same held-out task input in both; execute 64 further charged CIL instructions. Assert equality of **every subsequent full native Event record** and the event-excluded serialized world state **after every step**. Count the first CIL output Read, decision branch, retained source router, restored task input, correct/incorrect service writes, total charged compute and stopped conditions.

If the state projection is not identical, the histories unexpectedly converge only on a partial observation, if future CIL events diverge, or if the actual write provenance differs from the declared histories, the B44 hypothesis **fails**. No post-hoc exception for zero input.

## Frozen independent-model forecast

Source-parsed CIL model predicts **8/8 exact-state converged pairs**, **8/8 identical future internal event streams**, including input/output Read/Write, PC, resource charges and tick, and **6/8 genuinely different native output values** (for train0,1,17; training -1 makes true challenge0). All eight pairs have a different **last-writer provenance**, even the zero-value control.

| Eight worlds per arm | Training true/wrong task service | Holdout true/wrong | Charged compute |
|---|---:|---:|---:|
| Genuine native diagnostic output, **after external temporary routing** | 16/48 | **0/104** | 1,024 |
| Wrong native output then external diagnostic-value forgery | 16/48 | **0/104** | 1,024 |
| Unmodified B42 ProbePlus | 30/16 | **72/26** | 1,024 |
| Compact immutable correct source | 128/0 | **128/0** | 1,024 |

The genuine native output is **genuine only at that one diagnostic Write**. We restore the actor's original wrong steady-state source at the convergence point. Both branches therefore retain equally wrong future code: a true positive diagnostic output does **not** guarantee the continuing program remains correct. The two genuinely different causal histories have identical future task results by construction; this is the *point* of the equivalence experiment.

All host routing/forgery actions are external and not counted as CIL energy, source mutations by the individual, fitness scores, new computational life, or meaningful self-generation. The result says **this substrate is missing guest-visible provenance information**, not that no possible future version can have it. The actor's ability to detect a **future** output discrepancy (using real task feedback) is a separate question; historical write origin is specifically unobservable once the histories converge.

## Post-B44 decision gate

If the full-world equivalence holds natively, do **not** spend B45 on increasingly elaborate recheck heuristics: all mechanisms using the same information cannot restore deleted provenance. The next design decision would need to make a **generic identity-neutral causal record** accessible within world physics, with finite read cost, storage/accounting, tamper boundaries, and cross-domain compatibility tests, *without a special-purpose privileged fitness/repair oracle*. Benchmark overhead and an unchanged CIL-only negative control before asserting robust self-assessment, endogenous B3 or B4. If this architecture cannot be built without teleological hand-holding, record that as a research limitation.

**B3 cumulative endogenous adaptation NOT MET; B4 organizational closure NOT MET.**
