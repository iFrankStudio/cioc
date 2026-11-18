# CL-1B B56-F1d1i — strict one-PID feedback-conditioned mutation, three-phase task-shift revalidation and paid previous-best rollback

**Verified narrow result (not a life claim):** a *new researcher-authored* 157-instruction native CIL founder genuinely **reads feedback and scores** its own currently interpreted data-bytecode on two real probes, **branches to different paid bytecode mutations depending on the outcomes**, and on a prespecified unlearnable task **pays to retrieve and reinstall its previously validated program before executing a real held-out service**. Same PID1 remains present, code source immutable and storage/credits finite. This adds a feedback-dependent *choice to mutate*, beyond F1d1h's fixed unconditional monotonic A→B→C enumeration. It does **not** create a new virtual primitive or algorithm-selection rule.

## Strict timing: prospective protocol before code and evidence

- [Original F1d1i research hypothesis, adversarial controls and no-physics-change limit](../studies/B56-F1d1i-feedback-conditioned-variable-bytecode-mutation-and-regime-shift-preregistration.md), committed before new code as `dd5283260d86862efc0055fb31ffb0c3c26e7e21`.
- [Exact source-frozen 157-node, 16-world phase/regime protocol](../studies/B56-F1d1i-frozen-157-node-phase-shift-and-fallback-manifest-v1.md) hash SHA-256 `3d247cc0ddc06db40018965d038ea4d9ff78127019f6d462ded1ac31ab96f1e2` plus [all 157 native opcode edges/register fields JSON](../studies/B56-F1d1i-frozen-exact-source-v1.json) hash SHA-256 `d31cadec1a2db794235c0c4fff5ff5c7ea6ba1a9606b191183d1a49ef5986d1d`, committed **before** Rust integration at `aee315c43ab0e924836bb4cb29a9f7bccf8ceefd`.
- [Actual 16-world Rust native test](../../cl0/crates/world/tests/cl1b_b56_f1d1i_conditioned_mutation_rollback.rs) and [separately authored exhaustive JS CIL/feedback replay](../analysis/b56_f1d1i_conditioned_mutation_rollback_replay.mjs). No kernel source, physics, capacities or resource recharge changed for the experiment.
- [FULL GitHub Actions run44301654974](https://github.com/iFrankStudio/cioc/actions/runs/44301654974) at actual tested SHA **`1aae3ce701715cbbe64d118afa8c63f8ab3d0cbd`**: **67/67 Rust integration targets PASS,58/58 independent analyses PASS**, B31/B34 Rust ABI PASS, failed targets0, failed observers0; successful isolated evidence publication. This is **full-scope** coverage, not changed-only.
- Exact original [native event and CIL instruction JSONL](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/1aae3ce701715cbbe64d118afa8c63f8ab3d0cbd-44301654974-1/b56_f1d1i_native_mutation_regime_shift.jsonl), [independent 16-world per-instruction replay with first divergence null](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/1aae3ce701715cbbe64d118afa8c63f8ab3d0cbd-44301654974-1/b56_f1d1i_conditioned_mutation_rollback_replay.json), [original scope/failures](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/1aae3ce701715cbbe64d118afa8c63f8ab3d0cbd-44301654974-1/summary.txt) and [source-pinned test conclusion](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/1aae3ce701715cbbe64d118afa8c63f8ab3d0cbd-44301654974-1/result.txt).

## The real paid mechanism, not a host-computed winning candidate

**World:** `World::with_strict_capacity(16,180,300)`, one original PID1 per native world, exactly **4,000 initial finite compute credits**, no pool draw, no PID2, no graph objects, no researcher patches to source/bytecode/score/program. The native CIL founder uses **157/180 source nodes** and at most the 16-key set `{7,11,20,21,31,32,33,50,51,52,53,54,56,61,62,63}`; measured peak **174/300** total occupied. Interventions only set input7, one initial edit control21 or post-probe signal20; hidden task identity and target outputs are **never** available to PID1 as an oracle value or score table.

**Data-program interpreter:** actor initializes its program `[0,3,3,3]` (A: IDENTITY), then dynamically fetches `cell[51+virtualPC]` via actual CIL Read/Add/Read and executes primitives `0 LOAD_INPUT`, `1 NEGATE_ACC`, `2 ADD_LITERAL_2`, `3 RETURN` using paid native computation. An actual Write56 stores intermediate accumulator after each non-return instruction; an actual Write11 is the service. Both virtual primitives and the whole initial source are investigator-authored.

**Feedback-conditional selection:** each candidate is executed on actual probe inputs **−7** and **5**, each followed by a truly executed feedback Read20 (bit1 increments actor's own score61 using paid Write61, bit0 does not). A candidate score2 after two probes is retained as **validated mode62**; otherwise actor increments paid tries63 and executes a **new program mutation**, cycling mode **A(0)→C(1)→B(2)→A(0)**. The actual two-slot writes to program keys52/53 are paid by PID1; source-identical gate0 control redirects both writes to temporary accumulator key56 and therefore does **not** create the three programs. This is a deterministic authored mutation cycle *gated by feedback*, not source-authored fixed unconditional enumeration.

**Rollback:** after three failed two-probe candidate tests, PID1 **executes the preregistered rollback branch**, pays an actual **Read62** to retrieve its last validated mode, reconstructs that mode via actual paid Write52/53 and proceeds to held-out service. It does not assume rollback produces a task-correct function. Phase31 and probe32 stage are actor maintained; after each real held-out Write11, the actor transitions to the next epoch while keeping the program/last validated mode, and the environment changes only its hidden target used to issue subsequent feedback bits. The actor receives no task label or host choice of target program.

**Explicit intervention timing:** after real probe0 and probe1 Write11, environment sends feedback20 (except ABSENT) and next input7. After the actor itself pays the distinct `heldoutStageWrite` instruction (native PC141), environment supplies independent input11 and does not supply post-heldout feedback. At phase boundary the environment supplies probe input−7; all intervened values/timestamps are reexecuted independently. The observer independently source-parses the exact Rust CIL graph against the frozen preimplementation JSON, executes and compares **every** native instruction, state/register/PC/credit, output, cell absence/write, process and event, failing on first divergence.

## Truthful held-out outcomes, including the necessary failed result

Two source-frozen three-epoch hidden-task schedules, crossed with `TRUTH,BLIND_SUCCESS,INVERTED,ABSENT` and source-identical edit gate1/0: 2×4×2=**16** isolated Rust worlds.

| Hidden task schedule | Feedback regime | Edit enabled paid PID1 heldout result | Source-identical edit suppressed result |
|---|---|---|---|
| **S1: IDENTITY→COMPOSED→IDENTITY** | TRUTH | **3/3**, retained modes A→C→A | 2/3, original A remains |
| **S2: COMPOSED→NEGATE→UNREACHABLE (x+4)** | TRUTH | **2/3**, retained modes C→B→rollback B | 0/3, original A remains |
| S1 | INVERTED | 0/3, retained modes C→B→B (wrong) | 2/3 (A only) |
| S2 | INVERTED | 0/3 (A, wrong) | 0/3 (A, wrong) |

In S1 original real feedback selects A when initial IDENTITY probe succeeds; after task changes, A fails and PID1 pays program mutation to C and obtains correct `−9` on heldout11; at the third IDENTITY task C fails, then B fails, then A is reinstalled and succeeds. **This return to A is by the authored mutation cycle, NOT the prior-best rollback branch**; the report never confuses the two.

In S2 COMPOSED first selects C with heldout−9, then hidden NEGATE forces paid selection of B with heldout−11. The third hidden UNREACHABLE task `x+4` expects **15** on heldout11, but *none* of the authored A/B/C programs can produce it. Actor pays to test three failing candidates, and then **actually executes a source-level Read62 of previously successful mode2 and reinstates B**. Final heldout output remains **−11, incorrect**. This positive rollback event and negative task result must be interpreted together. Under gate0 PID1 does not alter code program slots, despite running the feedback logic and paying comparable redirected write operations; no claim of program ranking by that control.

Inverted feedback reverses whether PID1 proceeds to mutate after matched initial real outputs; the first-source, first-input conditions match, but the true-feedback and inverted-feedback cohorts choose different **next actual paid bytecode edits**, not merely different final labels. Blind-success is a PRESENT bit1 for every probe, not absence; ABSENT never creates cell20 and CIL Read20 gives default0, not a protected presence flag. Neither CIL feedback truth nor the environment oracle is authenticated. Three hidden task periods, two probes, and a fixed mutation cycle remain narrowly scoped.

## Complete source-pinned original paid instruction ledger

| Quantity | Native original, independently event-reexecuted |
|---|---:|
| Distinct strict same-source PID1 worlds | **16** |
| **Actual paid CIL instructions replayed** | **16,510** |
| External input, feedback and gate interventions | 412 |
| Real actor-paid feedback Reads | 186 |
| Actual actor-paid candidate score Writes | 58 |
| Actor-paid actual virtual-code-slot edits | 56 |
| Charged edit-suppression shadow Writes | 72 |
| **Prior-best rollback branch entries** | **19** |
| Dynamically fetched virtual instructions | 552 |
| Real accumulator checkpoint Writes | 318 |
| Real paid service Write11 outputs | 234 |
| Charged steps per world | 517–1,756 |
| Measured peak total storage | **174/300** |
| Native source mutations / spawned PID2 | **0 / 0** |
| **Independent first divergence** | **null** |

The full source Sha256 `16fd43ae3b980371da0e621f3f1aca309dcba4b76b8ff444ce612cd5f3209a28` is retained in the raw JS result; baseline source fingerprints are emitted per state. No free program mutations, free scoring, free rollback, hidden host-selected winning code or resource refill have been credited to PID1.

## Significance, limitations and falsifier for the next stage

B56-F1d1h had **paid scoring and retained selection** over a fixed externally authored A→B→C order. F1d1i now adds **feedback-controlled decision to try a different data program** and **real paid reconstruction of a previous validated program** on a failed new task while preserving one PID. It demonstrably does **not** solve an unlearnable hidden task. The primitive ISA, fixed A→C→B→A mutation grammar, 2/2 acceptance threshold, and deterministic 3-attempt fallback policy are *all researcher-programmed*. This is still a bounded, finite strategy, not evolution of mutation algorithms, arbitrary instruction innovation or cognitive task understanding. More complex tasks, long-run selection under depleted resources, cross-task generality and honest novelty generation are not proven.

**Next decisive falsifier:** test whether a program-encoded *mutation-rule description itself* can be modified by PID1 based on credit-normalized held-out value under adversarial feedback, rather than merely choosing an authored next program in a fixed cycle; pre-register a genuinely new grammar boundary, max-node16/180 caps, double-holdout overfitting controls and paid rollback. If the resulting candidate algorithms are still hardcoded by the researcher and only their IDs are switched, classify as policy selection, not open-ended autonomous algorithm invention.

**Original F1c founder continuity, authenticated B53 monitoring, strict N56-A/B/C, B3/B4, autonomous open-ended evolution and cyberlife NOT MET.**
