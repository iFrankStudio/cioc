# CL-1B B56-F1d1k-G3P — source-frozen original PID1 paid first-candidate duplicate detection and meta-program repair

**Verified scientific result:** under the same strictly capacity-limited original CL0 deterministic world, one and only one original PID1 interprets three dynamically fetched authored meta bytecode instructions, generates and retains its own feedback-derived mutation-rule program, and **before executing a duplicate candidate on a changed task** pays to compare its resulting task mode with the first candidate and edit the **middle meta-program instruction**, then reinterprets the altered program to emit a new, correct task program A. The source-identical control that disables this guard (but **still learns meta programs**) misses A. This is **a bounded online first-mode duplication repair mechanism**—not general G3 coverage-safety, spontaneous primitive invention, independent truth attestation or cyberlife.

## Preimplementation chronology and exact-source primary evidence

1. Broader prospective [G3 paid coverage monitor/repair preregistration](../studies/B56-F1d1k-G3-actor-paid-mutation-rule-coverage-preservation-and-feedback-poisoning-preregistration.md), main earlier `cb08674d0fd2e4e31ef273c3b2be468c17d02100`, explicitly **requires more than G3P's first-candidate check** for full G3; do NOT upgrade this study to full G3.
2. **BEFORE native G3P code**, [readable exact 180-node original CIL source](../studies/B56-F1d1k-G3P-frozen-readable-180-CIL-source-v1.txt), [machine-address-resolved full 180-node CIL JSON source](../studies/B56-F1d1k-G3P-frozen-exact-180-node-source-v1.json), graph SHA256 **`0360c59aeb9f11e8bf44587da627a6ed0ca6be7dad836d976ae873f5f177a0ee`**, and [exact 120-world controls and predictions](../studies/B56-F1d1k-G3P-120-world-first-mode-duplicate-repair-preregistration.md), SHA256 **`d206b793c7404d88cf10faf8022b335076c51ae85935a4ac97e052b685ae94e7`**, were prospectively committed together at **`78730d136cbb1bcff65f6cbdfb2656f80aacdc3b`**. The source-repaired control passed a preimplementation abstract model; **this was not native evidence**. The actual 180-node source is a NEW researcher-authored experiment founder, not spontaneous editing of the earlier G2S/F1c CIL executable.
3. Actual original [Rust native G3P 120-world test and paid-event recorder](../../cl0/crates/world/tests/cl1b_b56_f1d1k_g3p_paid_duplicate_repair.rs) committed `9c5815f37008dac317eb9db2b8d1c84dc254e82c`; [separate all-instruction source-parsed JS replay](../analysis/b56_f1d1k_g3p_paid_duplicate_repair_replay.mjs) committed `f521aeb924fc20b9473c05fe256a4a121bad002c`; full focused CI scope commit **`a3e04e6e64cc9d1051af3d08514b32bc7ef63644`**. The earlier partial source-only GitHub run44340918846 failed due the old whitelist and the interim observer-only run44341793000 was cancelled, **neither counts as G3P positive native verification**.
4. **FULL [original-source GitHub Actions run44341838257](https://github.com/iFrankStudio/cioc/actions/runs/44341838257)** executed precisely commit `a3e04e6e64cc9d1051af3d08514b32bc7ef63644`, with **71/71 original native Rust CL-1B integration targets PASS,62/62 independent observers PASS, B31/B34 native API ABI PASS and zero failures**. Separate evidence publication job SUCCESS. [Full native every-step JSONL](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/a3e04e6e64cc9d1051af3d08514b32bc7ef63644-44341838257-1/b56_f1d1k_g3p_native_paid_duplicate_repair.jsonl), [original independent full 120-case replay JSON](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/a3e04e6e64cc9d1051af3d08514b32bc7ef63644-44341838257-1/b56_f1d1k_g3p_paid_duplicate_repair_replay.json), [test scope and observer success ledger](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/a3e04e6e64cc9d1051af3d08514b32bc7ef63644-44341838257-1/summary.txt), [exact source result](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/a3e04e6e64cc9d1051af3d08514b32bc7ef63644-44341838257-1/result.txt).

## Substrate, identity and information boundaries

All **120** cases use byte-for-byte identical original **180 CIL source nodes** (zero spare within max180), at most **16 different shared data-cell keys** (all16 roles used) and **197/300 measured maximum total occupied units**. The original world is `World::with_strict_capacity(16,180,300)` with one initial PID1 and **4,000 private paid credits/world**. No PID2, no native source edit, no graph/process launch, no free host-generated program or privileged credit refills. PID1 cannot directly introspect exact private credit balance. Cell63 initial meta opcode is **genuinely absent**, returned as default0 by original VM paid Read; only later actual source-origin PID1 Writes allocate it. The external environment writes ONLY control21 once, next input7 and feedback20 after actual paid probe service Write11; no externally written meta code62/63/64, task bytecode52/53, score61, computed mode33 or output11. The hidden task target function never appears in PID1 memory as a privileged oracle.

Task bytecode uses researcher-authored two-position program `[1,0] → A(x)=x`, `[0,2] → C(x)=−x+2`, `[0,0] → B(x)=−x`; original PID1 pays Write/Read of both slots, one task accumulator Write56 checkpoint and actual service Write11. The **separate three-word meta program** is dynamically fetched at cells62,63,64 via original CIL paid meta-PC50 indirection. Researcher-authored meta atoms `ADD0,ADD1,ADD2` modulo3 plus **RESET0** create noncommutative rule behavior; three fetches, three intermediate paid Write56 checkpoints, and paid Write33 resulting task mode precede real candidate execution. Every normal candidate pays two distinct probes `[-7,5]` or reversed `[5,-7]` and two actual Read20 bits before considering acceptance.

## Source paid duplicate guard and why it is NOT a full coverage certificate

On two positive probe-feedback bits, *both gates* pay to learn and persist an authored-form meta program `[mode,RESET,mode]` via **Write62(mode),Write64(mode),Write63(3)**. The prospective source-admitted shapes therefore are the initial `[1,missing=0,0]`, and accepted `[0,3,0]`, `[1,3,1]`, `[2,3,2]`. Under these **four shapes**, the first candidate mode equals the base opcode62, so checking `mode33==base62` after at least one failed real trial is a valid test of duplication of the first. It is **not a general pairwise/triple rule-coverage proof**: other possible arbitrary 3-word descriptors may violate that assumption. Opcode validation for arbitrary malformed instructions was **removed** from this fresh source to fit strict180 nodes; G2S's specific bad4 test does not transfer to this different founder.

When enabled (experiment gate21=1), PID1 pays Read65(attempt count) after the meta interpreter. At count0, it proceeds normally. At count>0, PID1 pays Read33(computed candidate),Read62(base),Sub and Branch. On equal modes, it pays Read63(middle op), increments mod4, pays **Write63(updated code)** and reexecutes all three meta opcode Reads and checkpoints before writing task bytes or real service. This repair **does not count as a failed real probe**. At failed-search fallback, the PID1 pays a distinct Write65(0) before reinterpreting its retained baseline; otherwise the guard could unintentionally mutate the fallback. This fallback-reset origin and cost is included in every-step evidence.

The **source-identical** gate21=0 arm pays one `Write56(0)` shadow in place of the guard, **still accepts, writes and retains exactly the same class of three-word learned meta programs**. This arm is **NOT compute-cost-matched** to the variable-length checking/repair work. Consequently differences in costs mix checking costs, number of actual probe calls, and that single shadow; the outcome contrast is evidence for *enabling the functional guard*, not a precise net overhead estimate for an otherwise magically identical unit-cost implementation.

Within the admitted rule forms, a finite algebraic scope derivation (not independent native evidence for arbitrary grammar):

| Source-admitted original meta form | Candidate modes after actual failed probes with guard | Full candidate set in ≤3 real trials? |
|---|---|---|
| Initial `[1,0,0]` (middle initially absent/default0) | C,B,A = 1,2,0 | Yes |
| Learned `[0,3,0]` | A; duplicate A detected/repair; C,B = 0,1,2 | Yes |
| Learned `[1,3,1]` | C,B,A = 1,2,0 | Yes |
| Learned `[2,3,2]` | B,C; duplicate B detected/repair; A = 2,1,0 | Yes |

This checks **four source-admitted shapes only**, not all **64** possible three-word combinations or arbitrary erroneous external data programs. The native tests execute source-owned variants and their output consequences; the algebraic table clarifies *why* the observed repair works under this particular preauthored grammar.

## All 120 original worlds and causal outcome/cost results

Prospectively locked **two probe-order seeds × six two-phase hidden-task schedules × five signal regimes × two source-identical guard arms** =120 original original-PID1 worlds. Feedback regimes `TRUTH`, `BLIND_SUCCESS` (present constant1), `INVERTED`, `ABSENT` (cell20 never created, Read defaults0), `PHASE0_C_DECOY` (false success bits only on phase0 C; phase1 truthful). The source never sees hidden task labels or target values. All intervention times follow original paid outputs/actor-heldout-request state, no host candidate generation or rule edit.

**Truthful heldout correctness:** gate1 guarded `[2,2,2,1,2,2]` per seed = **11/12**, total **22/24** across two seed input orders. Gate0 guard-bypass `[2,2,2,1,1,2]` per seed = **10/12**, total **20/24**. The only additional solved task is S4 NEGATE→IDENTITY, one in each seed. These are paired authored toy tasks, **not** out-of-distribution/general statistical learning accuracy. All truthful S3 `IDENTITY→UNREACHABLE(x+4)` heldouts remain wrong in both arms: the input/output grammar lacks `x+4` and repeat rejection cannot create it.

| Original-PID1 true-feedback second-phase test (each seed) | Guard ON: heldout result, real paid CIL instructions | Guard OFF: heldout result, real paid CIL instructions | Interpretation |
|---|---|---|---|
| S0 NEGATE→NEGATE | **correct**, **222** | **correct**, **221** | Guard incurs **1** extra CIL instruction when no duplicate requires repair |
| S2 NEGATE→COMPOSED | **correct**, **426** | **correct**, **418** | Guard costs extra charged checks without changing the correct result |
| **S4 NEGATE→IDENTITY** | **correct**, **729** | **incorrect**, **715** | **14 extra paid steps to recover a previously accessible valid A** |
| **S5 IDENTITY→COMPOSED** | **correct**, **531** | **correct**, **598** | **67 fewer paid steps**, because duplicated A is rejected before spending a pair of real probes |

**Exact original source-event causal chain, S4 seed0:** both arms pay to validate B for NEGATE and retain `[2,3,2]`, then on shifted IDENTITY first test B and C. Gate1's native meta interpreter generates a duplicate B, but PID1 itself pays `Read65, Read33, Read62, Sub, Branch, Read63, ADD1, modular arithmetic and actual Write63(2)`, then reruns the same three-word interpreter, fetches meta opcode state `[2,2,2]` and produces A. The original PID1 pays Write52(1), Write53(0), real probes and real heldout output11 for target11. A source-identical gate0 world instead executes real B,C,B then falls back B, emitting **−11 rather than11**. Both source histories and feedback interventions are present, and the exact first-divergence-free independent observer checks paid event causation. This is not a free Rust patch or an actor-side memory alias.

**S5 seed0:** truthful IDENTITY stage learns `[0,3,0]`. Under shifted COMPOSED, guard's duplicate A is filtered before service with paid Write63(1), so only A and C are genuinely probed and C succeeds. Bypass really probes A twice before reaching C; its cost598 exceeds the guard's531 despite added guard overhead.

Unreached x+4 control stays incorrect, while inverted and deceptive feedback cases are kept in the full raw 120-world record (no claimed autonomous feedback-authentication). No process-private-credits sensor exists; all instruction costs above come from independent researchers' replay, not an actor magically observing its own credits.

## Fully reexecuted native-original accounting

Source-parsed JS observer independently checks the frozen 180-opcode Rust founder, each paid original PC, all4 registers, actor identity, private remaining credits, exact cell presence and value including absent63, every output, opcode fetch, metadata Write, control bypass, paid repair, fallback attempt reset and all allowed external interventions. It aborts on first divergent original native event or missing trailing events.

| Original PID1 world quantity | Exact independent replay |
|---|---:|
| All source-identical originally loaded PID1 worlds | **120** |
| All charged original native CIL instructions | **99,768** |
| Original external input/feedback/control interventions | 2,048 |
| Dynamic native meta-opcode fetches | 1,530 |
| Paid meta intermediate checkpoint writes | 1,530 |
| Completed real three-word meta-program executions | 510 |
| Paid feedback-validated meta descriptor writes | 546 |
| Effective base62 / tail64 changes | **62 / 126** (different from raw paid writes) |
| Paid trial/repair meta writes | 328 |
| **Actual native PID1 Write63 duplicate repairs** | **18** |
| Disabled-guard source-paid shadow Writes56 | 248 |
| Paid fallback attempt65 resets | 58 |
| Paid failed candidate counter Writes65 | 252 |
| Real task-bytecode writes52/53 | 984 |
| Paid task-opcode fetches | 2,216 |
| Task intermediate accumulator writes56 | 1,108 |
| Paid feedback reads20 | 868 |
| Paid score writes61 | 364 |
| Actual real PID1 service Write11 | 1,108 |
| Paid fallback entries | 58 |
| Actual charged steps/world | 450–1,549 |
| Peak occupied units | **197/300** |
| Spawned PID2 / original native source edits | **0 / 0** |
| **Independent first divergence** | **null** |

**Important counting:** 18 is the total actual paid duplicate repair Write63 events over all enabled-feedback/hidden-task worlds; this does **not** mean 18 novel distinct algorithms, and much less 18 de-novo invented primitive operations. Similarly 546 is the total *paid* validated descriptor writes, not 546 novel distinct rules. The observer reports some syntactic meta states containing a missing middle data address `"1,,0"`, which is an initial implicitly zero read, not a fully allocated three-slot program.

## Remaining limits and next falsifier

**Proven:** bounded same-individual paid first-candidate de-duplication on source-admitted meta code forms, full charged meta-program reexecution after an actual PID1-paid rule instruction edit, matched same-source causal recovery of a known solution, and costs including false-positive-case overhead, useful probe savings and truly unlearnable function failure.

**Not proven:** complete all-64-descriptor meta code verification, general paid 3-mode coverage bitmap/certificate, any arbitrary-bad-opcode safety in this new source, authenticated fitness provenance, donor/yoked learned-rule transplant, genuine organism-original-code ancestry from F1c or G2S, endogenous invention of new native/meta primitive meanings, N56-A/B/C,B3/B4,Hk or cyberlife.

**Next exact obstruction:** the verified G3P source uses **all180 native instruction slots and all16 shared keys**. A source-native complete coverage monitor must refactor real duplicate guard, task evaluator and meta interpreter or reuse explicitly dead cells while preserving original exact state/cost; simply appending a bitmap/validator is impossible under the same frozen source. Preregister a G3Q generalized three-mode coverage verifier, with independent malformed-opcode adversarial arm and real rule-history transfer control. If a particular attempted graph fails capacities, publish the bounded failure; never enlarge CL0 substrate silently. 
