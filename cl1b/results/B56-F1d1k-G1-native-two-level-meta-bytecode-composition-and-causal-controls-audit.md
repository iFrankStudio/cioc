# CL-1B B56-F1d1k-G1 — original native PID1 paid two-level meta bytecode, rule composition, negative transfer and adversarial control audit

**Verified outcome:** a strictly same original PID1 now *executes two separate data-encoded mutation-rule opcodes*, dynamically fetched by its native CIL program from its own paid data memory, checkpointing actual intermediate meta-state, to calculate and write the **next task-program bytecode**. It can pay to modify both meta instruction slots following real probe feedback and carry a validated rule program forward across a hidden task change. Nine different meta-code combinations were exercised but they implement **only three different task-selection functions**, all from a frozen researcher-authored finite ADD modulo3 grammar. This is **B56-F1d1k-G1 partial bounded composition**, NOT general algorithm invention or full Hk.

## Chronology: frozen before outcome-bearing native code

1. Initial [interpreted multi-instruction rule-program preregistration](../studies/B56-F1d1k-interpreted-mutation-rule-program-and-fail-closed-grammar-preregistration.md), commit `cdbdc683ba3eee82092a6fc90dfb871e903fcb0f`; [strong endogenous rule-synthesis and provenance falsifier](../studies/B56-F1d1k-endogenous-rule-synthesis-and-provenance-falsifier-preregistration.md) existed at previous main head `afad1e5d09ee9a9a80caa2877663314ca952b372`. Strong Hk requires more than a 2-word arithmetic grammar and yoked/invalid controls, so **Hk is NOT claimed passed**.
2. [Frozen exact 178 original source CIL nodes](../studies/B56-F1d1k-G1-frozen-exact-178-node-source-v1.json), SHA-256 `5eafe0c637a1318ccee407e071496d688e25ab6a9e0767fa43a00eb51f70d46f`, and [frozen G1 160-world control protocol](../studies/B56-F1d1k-G1-exact-two-level-meta-bytecode-and-160-world-controls-preregistration.md), SHA-256 `0e5e229664d942035cbe293a4cbdce9b04d19031f6af015ed2ab5bc8a72ff11a`, **were both committed BEFORE Rust implementation or actual native experimental outcomes** as `d9639149269750cda29a9333a953d5d4b123ad18`.
3. [Rust native 160-world original PID1 integration source](../../cl0/crates/world/tests/cl1b_b56_f1d1k_g1_two_level_meta_rule.rs) committed `bc7b9cb480353b0a85e4623486eae5e93f98bc2a`; [independent source-parsed JavaScript replay](../analysis/b56_f1d1k_g1_two_level_meta_rule_replay.mjs) committed `5a126a08d97ac60ce404568f4dc105d48c1c6d26`; workflow scope updated before final verification at `efaf1fd54b5ba26a7b3b46dc829d6a3511658a95`.
4. [FULL GitHub Actions run44312999669](https://github.com/iFrankStudio/cioc/actions/runs/44312999669) tested **exact SHA `efaf1fd54b5ba26a7b3b46dc829d6a3511658a95`**, **69/69 Rust original integration targets PASS,60/60 independent source/event analyses PASS, B31/B34 ABI PASS**, failed native targets0, failed observers0 and evidence branch publication success. The earlier unregistered source-only failed/cancelled runs are not positive evidence.
5. Source-pinned [complete raw original charged CIL/host events JSONL](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/efaf1fd54b5ba26a7b3b46dc829d6a3511658a95-44312999669-1/b56_f1d1k_g1_native_two_level_meta_rule.jsonl) · [all 160 independently replayed original case summaries](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/efaf1fd54b5ba26a7b3b46dc829d6a3511658a95-44312999669-1/b56_f1d1k_g1_two_level_meta_rule_replay.json) · [full test scope and failures ledger](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/efaf1fd54b5ba26a7b3b46dc829d6a3511658a95-44312999669-1/summary.txt) · [exact source-SHA pass record](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/efaf1fd54b5ba26a7b3b46dc829d6a3511658a95-44312999669-1/result.txt).

## Original strict world and live-cell physics

All 160 original worlds share identical researcher-authored **178 native CIL instruction source nodes** below cap180; **one original PID1**, finite4,000 personal credits per world, no compute pool, GraphLaunch/PID2, external code graph patch, any new native VM instruction or privileged credit replenishment. Physics `World::with_strict_capacity(16,180,300)` is unchanged. Allowed original data cells are exactly **14 potential shared keys** `7,11,20,21,31,32,33,50,52,53,56,61,62,63`, maximum observed total occupancy **193/300** (178 source + one process +14 simultaneous keys). Each original native paid step is independently checked for PID, PC, all4 registers, exact charged credit, every allocated cell including missing feedback, unchanged source fingerprint, event kind and occupied storage. CIL meta and task interpreters intentionally time-share cell56 intermediate accumulator: the unused other program's temporary value is dead before each paid reset.

The host writes **ONLY** `21` (immutable initial control once), `7` (actual next probe/heldout input) and `20` (ordinary post-service feedback bit when present). The host NEVER writes rule-program cells62/63, task bytecode52/53, task mode33, score61, meta PC50, accumulator56 or service11. No trust or freshness is inferred from the ordinary signal20.

## Two-level actor program execution: what is actually paid

**Meta-rule program level:** Original PID1 owns two independently CIL-writable data meta-opcode cells62 and63. Native CIL resets metaPC50=0 and accumulator56=0, then **for each meta instruction** pays `Read50→Add(62+metaPC)→Read corresponding stored opcode`, decodes the *researcher-authored* finite meta ISA `0=ADD0/NOP`, `1=ADD1`, `2=ADD2`, computes accumulated mode modulo3, pays **Write56 intermediate checkpoint**, and advances meta PC. After precisely two paid fetches and two real checkpoints, it pays to write the computed task mode into cell33. The graph has a source-visible `3` invalid-opcode Halt; **this was not injected as a separate malformed-code control** and does not establish rejection of every arbitrary integer outside the bounded ISA.

Examples of actual interpreted meta code, *not a researcher branch for each pair*:

| Two paid meta opcode values | Accumulated mode modulo3 | Selected task-bytecode function |
|---|---:|---|
| `[1,0]` | 1 | C: `−x+2` |
| `[1,1]` | 2 | B: `−x` |
| `[1,2]` | 0 | A: `x` |
| `[2,0]` | 2 | B: `−x` |
| `[0,0]` | 0 | A: `x` |

The independent replay observed **9 distinct meta-code pairs** `[0..2]×[0..2]` across all arms, but modulo3 produces only **3 semantic modes**, not 9 new capabilities. One original PID1 in S3 `IDENTITY` actually composes and executes `[1,0]→C`, `[1,1]→B`, `[1,2]→A` as feedback rejects the first two, and pays to retain `[0,0]` after A's real two-probe score2. This crosses the one-bit policy-selection limit from F1d1j **at the level of two-opcode data-program composition**; it still does not discover any new primitive operation.

**Task-program level:** The output of meta interpretation is realized by **two paid PID1 Write operations to task op slots52/53**, not by environmental code injection. Data task program `[1,0]` = KEEP_INPUT then ADD0 (A), `[0,0]` = NEGATE_INPUT then ADD0 (B), `[0,2]` = NEGATE_INPUT then ADD2 (C). PID1 pays to read both task data operations and checkpoint their intermediate accumulator in cell56 on every service before a real `Write11`. These are researcher-authored positional task operators; unlike F1d1g's variable-PC task interpreter, the task-program interpreter here is intentionally narrowed to a fixed two-op format to make room for dynamically fetched meta instructions under the unchanged 180-code-node cap.

**Validation and actor-side editing:** each candidate must produce two actual probe services, one on a negative and one on a positive input (probe order permuted by two frozen seeds). Only after paid Read20 feedback=1 on both does actor score61 reach2, allowing charged `Write62(mode)` and `Write63(0)` in enabled gate1. Failed candidates make actor pay to increment meta opcode cell63 `0→1→2` and rerun the meta interpreter, generating a genuinely different next task program. After three failed modes, PID1 pays a reset of opcode offset63=0 and reinterprets the base to execute one heldout *fallback*; an unsolved task remains unsolved. At hidden task boundary, PID1 retains base62, resets temporary offset63 and counters by paid operations, never receives the task identity.

Controls have **byte-identical original 178-CIL-node founders**: gate1 enables validated rule-base writes; gate0 redirects two successful-program meta writes to cost-bearing shadow key56 and freezes base62=C-first; gate2 uses actor-paid initial B-first meta opcode; gate3 actor-paid A-first baseline. **Gate0 does NOT suppress trial-time temporary meta-offset63 writes**; thus it is a persistent-base-learning ablation, **not** a completely frozen meta-bytecode ablation. Gate3 is not a genuine donor/yoked imported-history protocol, so strong preregistered Hk is not met.

## 160 native worlds and heldout/transfer falsifiers

The precommitted complete design is **2 seeds** `[-7,5]→heldout11` and `[5,-7]→heldout−11` × **4 two-phase hidden task schedules** S0 NEGATE→NEGATE, S1 COMPOSED→NEGATE, S2 NEGATE→COMPOSED, S3 IDENTITY→UNREACHABLE `x+4` × **5 signal regimes** `TRUTH,BLIND_SUCCESS,INVERTED,ABSENT,PHASE0_C_DECOY` × **4 gates** 1/0/2/3 = **160 isolated source-identical PID1 worlds**, no result-dependent selected samples. All task A/B/C outputs on both probes and heldout are distinct, and the unlearnable target `x+4` is outside A/B/C at both heldout values.

**Truthful heldout correctness under actor-paid enabled rule learning:** S0=2/2, S1=2/2, S2=2/2, S3=1/2 for each of two seeds ⇒ **14/16 heldout correct** and two explicitly documented failures on the unavailable x+4 task. This 14/16 is a narrow experimentally selected task family, not a generalization accuracy or open-ended evolution result.

Measured actual **second-phase CIL instruction costs to real heldout service**, per seed:

| Hidden two-phase task | PID1 rule learning enabled | Source-identical frozen C-first | Interpretation |
|---|---:|---:|---|
| S0 NEGATE→NEGATE | **209** | 371 | **162 fewer paid steps (43.7%)** by retaining learned B-first rule |
| S2 NEGATE→COMPOSED | 525 | **209** | **316 extra steps** due to harmful transfer of previously learned B-first meta program |

In S0 the source-identical privileged B-first gate2 has even less **total** original compute (436) than the enabled agent that must pay first to learn. This is a mandatory negative cost/control finding: **preinstalled correct bias can outperform paid learning in short horizons**. No claim of universal superiority or PID1 directly observing its own private credit balance.

In S2 the learned B-first meta code requires testing B, then A, then C to solve the shifted COMPOSED task, while frozen C-first begins with C. This **negative transfer** is 316 paid instructions in both preregistered seeds even though the final heldout is correct.

**Targeted deception:** S0 truthful and `PHASE0_C_DECOY` begin with byte-identical source, gate, input and actual first C services, but only C's *phase0* ordinary feedback bits differ. Decoy falsely declares C successful on NEGATE, causing PID1 to retain meta `[1,0]` instead of truthful `[2,0]`; decoy's actual phase0 heldout output−9 is incorrect vs NEGATE target−11. On next phase **both receive truthful feedback**, yet PID1 executes different first meta programs and task-slot edits (truth B vs decoy C), with 209 vs371 paid second-phase steps. This links earlier feedback to persistent code-data state and later future candidate execution; it does **not** establish authentication of the bits.

**Unreachable function:** S3 phase0 IDENTITY really causes PID1 to execute meta compositions C,B,A and retain A when feedback is truthful; phase1 target `x+4` is outside the authored task operator grammar. All three candidate trials fail, actor pays source-level fallback and emits its base function A, **incorrect** at both heldout seeds. Neither two-stage meta composition nor rollback solves missing semantics.

**ABSENT is literal missing feedback key20**: existing Rust Read20 gives0, and PID1 cannot tell the difference from a valid failure bit. `BLIND_SUCCESS` is a PRESENT uninformative1; `INVERTED` is deliberately wrong; phase0 decoy is partly wrong. All 160 are archived including harmful/error conditions. There is no source-attested truth, freshness oracle, authenticated receipt or B53 continuity claim.

## Original source/event-by-event independent evidence ledger

The separately authored JS observer extracts every native `Op::...` node from the Rust source, checks exact equality against the **precommitted** 178-op JSON, independently executes every native paid instruction and every external intervention, matches 4 registers, PC, process credits/PID, live-cell presence, event kind, unchanged original source FNV and exact total storage, and rejects first divergence. This is a genuine independently reexecuted original world history, not analysis of authored expected outputs alone.

| Actual strict original-world quantity | Independent replay |
|---|---:|
| Full original same-source PID1 worlds | **160** |
| **Actual paid original CIL instructions independently reexecuted** | **117,648** |
| External actual input/feedback/gate interventions | 2,776 |
| **Paid dynamic meta-opcode reads** | **1,340** |
| Paid meta intermediate accumulator checkpoints | 1,340 |
| **Completed two-opcode composed meta executions** | **670** |
| PID1-paid validated-candidate meta code-slot Writes62/63 | 120 |
| **Actual changes of persisted meta base opcode62** | **20** |
| Paid failed-trial meta offset/fallback mutations | 430 |
| Paid meta-edit suppression shadow Writes | 360 |
| Task-data bytecode slot Writes52/53 | 1,340 |
| Paid task-program operator reads | 3,000 |
| Paid task intermediate accumulator checkpoint Writes | 3,000 |
| Paid feedback Read20 events | 1,180 |
| PID1-paid candidate score Writes | 480 |
| **Real native service Write11 outputs** | **1,500** |
| Actor-paid fallback branch edits | 80 |
| Per-world actual paid instruction range | 420–1,208 |
| **Measured peak total occupied units** | **193/300** |
| Original PID1 spawned child / mutated source | **0 / 0** |
| **Independent replay first divergence** | **null** |

The `120` meta descriptor Writes count **paid writes**, not 120 distinct rule inventions; 20 actually changed previous base opcode value. The 9 observed meta word-pairs are only 3 semantic program choices, with some equivalence aliases. Real process private credits remain inaccessible to the agent via a CIL introspection primitive; cost measurements are by independent researchers. The real task/feedback grammar and finite two-word meta ISA were authored by researchers before experiment.

## Interpretation and next discriminating falsifier

**Supported:** real same-PID1 execution of two dynamically fetched actor-owned meta instruction words, actual intermediate meta computation, paid assembly/updates of multi-word meta programs, full native causal influence on subsequent task-program bytes and services, truthful repeated-task efficiency alongside provable negative transfer, and two unreached-target failures without magic repair.

**NOT supported:** full original F1d1k Hk—arbitrary new rule syntax, more than three authored meta semantic outcomes, more-than-two-length rule programs, true source-program reachability of arbitrary malformed values, a fully suppressed temporary+permanent meta writer control, authentic imported/yoked learning, provenance-authenticated task fitness, original F1c founder identity, N56-A/B/C, B3/B4 or cyberlife.

**Next F1d1k-G2 gate should attack these exact shortcuts**, especially the modulo3 equivalence of all nine descriptors and the inability to reject every malformed opcode in the 178-node interpreter. Under unchanged 16/180/300 physics it must separately freeze an opcode-dispatched program grammar with variable rule length/cost and source-visible validation/recovery, or honestly publish a bounded negative-capacity result. Do not downgrade to a static rule index or raise substrate limits.
