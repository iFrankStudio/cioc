# CL-1B B46 — native-verified same-PID output race and the limits of truthful provenance

**B46 scientific experiment VERIFIED in native Rust; corrected-source run uses changed-only CI scope.** The first complete CI run of code SHA `37584bc5213e9cfb2776af41459ca41e0e73bc46` ([run43909526141](https://github.com/iFrankStudio/cioc/actions/runs/43909526141)) failed **1/46** selected native targets (the new B46 target), while **45 old native targets, all33 independent observers and the B31/B34 ABI tests passed**. The B46 failure was an arm-unscoped PC42 event-classifier assertion, not a native source/VM result. [First original failing-run audit](B46-first-native-interposer-classifier-failure.md).

The correction changed only the B46 Rust *observer/test classifier* to require `has_interposer(arm)` when recognizing a node42 post-read output Write; it left **all CIL code, environment physics, intervention schedule, frozen numerical expectations and JS observer code unchanged**. The corrected SHA **`e0b530cda45fea4c7d1df24e6c94f093aeb0959f`** produced [successful targeted native CI run43909879549](https://github.com/iFrankStudio/cioc/actions/runs/43909879549): the new **B46 Rust integration target PASS** (its two native tests: B46 + the imported unchanged B42 source study), **33/33 independent source-parsed observers PASS**, **B31/B34 private ABI checks PASS**. [Source-pinned changed-only CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/e0b530cda45fea4c7d1df24e6c94f093aeb0959f-43909879549-1/summary.txt) · [B46 exact original native log](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/e0b530cda45fea4c7d1df24e6c94f093aeb0959f-43909879549-1/cl1b_b46_same_writer_race.log) · [independent model output](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/e0b530cda45fea4c7d1df24e6c94f093aeb0959f-43909879549-1/b46_same_writer_race_scan.json) · [exact-source success result](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/e0b530cda45fea4c7d1df24e6c94f093aeb0959f-43909879549-1/result.txt).

**Important coverage qualification:** the corrected SHA was verified with the workflow's **changed-only scope: 1/1 registered B46 native target, plus all33 independent observers and B31/B34 ABI checks**. The **45 older native integration targets** passed at the preceding full-source SHA, before this test-logger-only correction. These two source-pinned runs jointly establish regression coverage **with a small known source diff**, but there is **not a single corrected-SHA full 46/46 native run**. Do not mislabel the second run as full-scope 46/46.

## What was actually tested

[Pre-native frozen protocol](../studies/B46-same-pid-writer-race.md) · [B46 real Rust CIL](../../cl0/crates/world/tests/cl1b_b46_same_writer_race.rs) · [independent Rust-source-parsed CIL observer](../analysis/b46_same_writer_race_scan.mjs) · [unmodified model-only forecast archived before native run](B46-model-same-pid-writer-forecast.md).

Seven task/observation arms × four training values `[-1,0,1,17]` × two unseen nonzero task inputs `[-9,23]` = **56 fully native Rust task worlds**. Every world has **one continuously executing PID1**, zero shared compute pool, finite 192-unit storage cap, exactly128 actual charged CIL instructions (64 before and64 after the original-input phase boundary). External researcher interventions have explicit `Origin::External` and never count as actor service or actor-paid work. Task scoring distinguishes diagnostic writes from actor-origin output11 task service *after the original input was restored*. A compact, immutable correct four-op copier is the resource/correctness strong baseline.

The same **B42 38-node source** and **B45 four-op author-defined provenance guard** are reused. B46 adds only one ordinary charged CIL opcode to five arms by routing the initial PC11 `Read` to node42, then externally restoring PC11.next=12 in *every* matched arm after the first execution. **No new CIL instruction, source-authenticity oracle, task fitness oracle or repair mechanism** was introduced. The transient write, its source, and the external one-time edge restoration are **researcher-authored experimental manipulations, not actor-invented causal attacks**.

Two disjoint negative cases were confirmed in the full native event history:

**NativeWrong, six clear false-current-value approvals:** after the actual initial native B42 PC10 Write0, the lab intentionally forges a correct challenge in cell11. The continuing actor's **charged PC11 Read** takes that correct value into register1. Its next paid **native PC42 Write** *under the genuine PID1* overwrites cell11 with **0**. The subsequent charged `ReadLastWriter` at PC38 truthfully senses **writer PID1**. The authored guard at PC41 accepts the earlier, now-stale correct value even though **the currently stored value differs in the six nonzero-challenge worlds**. The actor retains the defective PC10 service router and emits no correct unseen service writes.

**NativeEcho, valid source/value once but defective future:** same matched interposer timing, but the genuine PID1 PC42 Write copies the correct challenge once. Both data and writer source are genuinely correct at the instant of sensing (no false-*current*-value accusation), yet after one researcher-controlled source restoration the continuing router remains the bad constant-zero PC10. The actor still retains the wrong program and fails on all nonzero unseen service tasks. This shows identity and one correct historic output are not equivalent to verified *future* functionality.

The source0 no-op and outside-after-read wrong-value controls still detect an external overwrite and take the native CIL construction path. Neither a paid no-op nor an external post-read SetCell can forge PID1 last-writer metadata. The unspoofed source control retains the original zero-challenge blind spot. Source tags remain **truthful throughout**.

## Exactly matched native result and frozen independent CIL forecast

Each of the seven arms contains8 fully counted Rust worlds. All **56 native `B46_NATIVE` records** were present in the corrected run's test log, and the resulting seven 13-integer aggregate vectors exactly matched the independent model predictions frozen before any native B46 CI. The original four training inputs and both holdouts remain included, including the negative `train=-1` zero-challenge worlds.

| Eight-world arm | Train correct/wrong | Unseen correct/wrong | Charged CIL | New native code nodes | Paid provenance reads | First writer tag | False current-value approvals |
|---|---:|---:|---:|---:|---:|---:|---:|
| **NativeWrong** | **14/42** | **0/104** | 1,024 | 0 | 8 | PID1 ×8 | **6** |
| **NativeEcho** | 14/42 | **0/104** | 1,024 | 0 | 8 | PID1 ×8 | 0 |
| **NopSpoof** | 24/0 | **104/0** | 1,024 | 8 | 16 | external0 ×8 | 0 |
| OutsideLateWrong | 24/0 | **104/0** | 1,024 | 8 | 16 | external0 ×8 | 0 |
| NopGenuine | 24/14 | **72/26** | 1,024 | 6 | 8 | PID1 ×8 | 0 |
| **B45GuardSpoof** | 24/0 | **96/8** | 1,024 | 8 | 16 | external0 ×8 | 0 |
| **STATIC_CORRECT** | **128/0** | **128/0** | 1,024 | 0 | 0 | not sensed | 0 |

**Crucial timing-confound finding:** simply adding **one charged `Const` no-op** to the old source changes the number/placement of outputs inside the fixed64-step task windows and alters the eight-world unseen count from B45's `96/8` to the matched no-op's `104/0`. This must **not** be reported as learned task competence, robustness to a broader adversary or emergent self-organization. The no-op alters schedule and output accounting; it adds no causal reliability of writer-origin information. Strong STATIC_CORRECT performs `128/0` with far fewer CIL nodes, meaning the ornate authored guard loses under normalized resource cost.

## Scientific conclusion and next experimental gate

The strongest validated negative is that a **correctly implemented, paid last-writer sensor can return genuinely accurate provenance and the decision built from it can still be functionally wrong**. Separate value and identity reads are not an atomic observation. More generally, even an atomic source/value pair would merely establish identity and value *at one instant*, not that the actor's retained future code will generalize across independently selected tasks.

B46 **does not** demonstrate an endogenous attack strategy, autonomously discovered deception, organism-created provenance guard, or active self-maintenance. The founder and all higher-level validation/control policies are researcher-authored. Fixed phase boundaries also cause false performance impressions independent of problem-solving progress.

The next scientifically useful action should **not** be another hand-authored security check with a narrow adversarial edge case. It should directly address **B3 autonomous cumulative adaptation**: prospective unpredictable task families, source-edit proposals genuinely generated from the individual’s currently accessible data and paid instruction set, identity-neutral task service feedback, meaningful comparison to compact unmodified programs, and retention that survives unseen regime changes. If that experiment cannot pass with the current fixed founder rule set, record the limit explicitly and treat any enlarged observer as a substrate affordance, not an endogenous evolutionary breakthrough.

**B3 cumulative endogenous adaptation: NOT MET. B4 endogenous organization: NOT MET. Cyberlife: NOT ESTABLISHED.**
