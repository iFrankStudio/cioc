# CL-1B B49 — 11,437 native CIL events independently reproduced, zero divergence

**B49 methodological H49: ACCEPTED within the specified deterministic CIL subset.** This is **measurement/implementation verification**, **not** evidence of autonomously originated algorithm discovery, cumulative adaptation, B3, B4 or cyberlife.

**Source-pinned evidence:** exact tested code source SHA **`111b407164191cc36c744c47b9da5d0cacff5513`**, [GitHub Actions run43993850940](https://github.com/iFrankStudio/cioc/actions/runs/43993850940).

- [Original focused CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/111b407164191cc36c744c47b9da5d0cacff5513-43993850940-1/summary.txt): `scope=changed-only`, **1/1 selected Rust B49 integration target PASS**, **35/35 independent observer programs PASS**, B31/B34 private ABI checks PASS, both failure counters zero.
- [Source SHA + final job conclusion](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/111b407164191cc36c744c47b9da5d0cacff5513-43993850940-1/result.txt): `native_job_result=success`.
- [Original **3,408,593-byte** machine-readable Rust native event log](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/111b407164191cc36c744c47b9da5d0cacff5513-43993850940-1/b49_native_steps.jsonl): exact-source SHA metadata; 96 case headers, 288 external input transitions, **11,437 actual paid native steps**, case end records. File content was independently retrieved by its Git blob SHA `a5b7b4c2e0a6a9893aed8e41183c77f48855616e` because the GitHub Contents API omits large-file text.
- [Independent dynamic JS model result](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/111b407164191cc36c744c47b9da5d0cacff5513-43993850940-1/b49_dynamic_native_trace_replay.json): independently parsed B42 Rust CIL founder, executed `Const/Add/Sub/Read/Write/Branch/CloneNode/PatchNext/ReadNext/Halt`, and compared every native event and full selected state projection. **First divergence = null; all 11,437 paid events matched.**
- [Real B49 Rust native test output](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/111b407164191cc36c744c47b9da5d0cacff5513-43993850940-1/cl1b_b49_dynamic_replay.log): native B48 unseen aggregate reproduction and all reported phase/storage/credit totals.

## Verified population and event-level fidelity

[Prospective fixed protocol](../studies/B49-independent-dynamic-native-trace-replay.md) · [pre-native checkpoint](B49-model-dynamic-replay-pre-native-checkpoint.md) · [native Rust test and JSONL emission](../../cl0/crates/world/tests/cl1b_b49_dynamic_replay.rs) · [actual independent JS CIL virtual machine](../analysis/b49_dynamic_native_trace_replay.mjs).

**96 actual worlds** = eight previously frozen B48 unseen three-input sequences plus **four genuinely newly frozen B49 sequences**, crossed with two paid initial program-phase offsets and four source policies (true paid-feedback mutable graph, source-matched blind-self feedback, blind-zero feedback, immutable compact copy). Each world starts with 144 CIL credits and a strict 192-unit total storage cap; no compute refill or reproduction. **288** input transitions are explicit external events, not native actor actions. Native scoring credits only the actor-origin output11 `Write`, compared with *the then-current cell7 value*, including diagnostics (no author-chosen restored-only cutoff).

A separately written JavaScript model parsed the original actual **38-node Rust B42 CIL source** and re-executed it, paying every instruction, handling dynamic node cloning, edge patching and halting. It independently reproduced, **line for line**:

- **11,437 / 11,437** actual paid CIL step rows; zero missing or mismatched process events;
- **288 / 288** external task-input transitions; all 96 world headers and actual end-state records;
- **138** distinct changed-edge code-patch events, including their node, old target and new target, plus native clone growth and selected executable edge values;
- all tracked event types, event tick, PC-before, PC-after, four registers, four important shared cells, executable node count, candidate successor, credits, stop state and strict storage usage;
- **22** previously non-excluded early-stopped blind-zero worlds, with correct actual charged-attempt totals.

The JavaScript output reports `first_divergence: null` and `native_source_sha` matching the Rust trace metadata and CI job SHA. **This is independent dynamic execution parity**, not the earlier B47 source-pattern audit. The interpreter is intentionally limited to **the actual B49 CIL opcode subset**, not an independently reimplemented general CL-0 kernel.

## Actual realized exposure; do not impute stopped-world steps

The same initial 144 credits per world do not imply every world executes144 steps:

| Arm / all 24 worlds (12 task sequences ×2 phase offsets) | Actual paid CIL attempts | Halted worlds |
|---|---:|---:|
| TRUE feedback source | **3,456** | 0 |
| BLIND_SELF matched-source-cost | **3,456** | 0 |
| BLIND_ZERO, non-equivalent feedback sham | **1,069** | **22** |
| STATIC compact correct source | **3,456** | 0 |

In the first eight B48 unseen sequences, paid exposure was 2,304 for TRUE, BLIND_SELF and STATIC, versus **785** for BLIND_ZERO (14/16 early stopped). In the new four B49 sequences, paid exposure was1,152 for the three continuing arms, versus **284** for BLIND_ZERO (8/8 stopped). All prematurely halted worlds were retained in the denominators, not renamed as zero-error successful behavior.

## Historical B48 replication and new prospective heldouts

Eight prior B48 unseen sequences exactly reproduced their original **raw** output-write counts:

| Phase / source arm | Holdout1 correct/wrong | Holdout2 correct/wrong |
|---|---:|---:|
| Baseline TRUE | **64/15** | **64/16** |
| Baseline BLIND_SELF | 18/54 | 20/60 |
| Baseline BLIND_ZERO | 0/9 | 0/10 |
| Baseline STATIC | **96/0** | **96/0** |
| Phase+1 TRUE | **64/15** | **63/10** |
| Phase+1 BLIND_SELF | 18/54 | 20/60 |
| Phase+1 BLIND_ZERO | 0/9 | 0/10 |
| Phase+1 STATIC | **89/7** | **89/7** |

**New B49 task sequences**, frozen before first B49 execution: `[2,0,-7]`, `[-4,31,0]`, `[0,7,-4]`, `[31,2,7]`:

| New block / source arm | Holdout1 correct/wrong | Holdout2 correct/wrong |
|---|---:|---:|
| Baseline TRUE | **36/4** | **36/4** |
| Baseline BLIND_SELF | 9/27 | 10/30 |
| Baseline BLIND_ZERO (stops all 4) | 0/0 | 0/0 |
| Baseline STATIC | **48/0** | **48/0** |
| Phase+1 TRUE | **36/4** | **36/0** |
| Phase+1 BLIND_SELF | 9/27 | 10/30 |
| Phase+1 BLIND_ZERO (stops all 4) | 0/0 | 0/0 |
| Phase+1 STATIC | **44/0** | **44/4** |

The continuing TRUE feedback arm outperforms the identical-architecture BLIND_SELF ablation on correct net outputs under equal realized instruction exposure, including new task sequences; but it **does not beat the simpler STATIC source** under the same initial 144-instruction budget. This confirms a **specific engineered feedback-semantic benefit** with stronger measurement evidence, not endogenous discovery or B3. Phase shifts alter the fixed comparator's apparent correctness too, and BLIND_ZERO cannot legitimately enter equal-exposure rankings while it halts early.

### H48 result is NOT retroactively rescued

B48's hypothesis was conjunctive: beating two meaningful semantically distinct shams with equal realized exposure under both phase shifts. B49 does **not** change the failed exposure condition (BLIND_ZERO stopped) and does not upgrade a register-alias-correlated sham into an independent semantic comparator. H48 stays **NOT ACCEPTED**. B49 H49 is a separate **measurement-validity** claim now fulfilled for the specified native opcode/task family.

## Failures preserved and coverage qualification

The B49 study had **three failed CI runs** before the independent dynamic parity passed:

1. [First full run43993215301](https://github.com/iFrankStudio/cioc/actions/runs/43993215301), SHA `839e17f6`: **47/48** old/native targets passed but B49 failed to compile due a fixed B48 reference-array shape typo. 34/35 observers passed, B49 observer correctly rejected missing trace. [Detailed failure](B49-first-native-array-shape-failure.md).
2. [Second changed-only run43993441770](https://github.com/iFrankStudio/cioc/actions/runs/43993441770), SHA `6886fb9b`: native B49 itself passed and reproduced B48, but JSONL was written under Cargo crate CWD rather than the observer's CI evidence directory; B49 observer refused missing file. [Detailed failure](B49-second-native-trace-location-failure.md).
3. [Third changed-only run43993641624](https://github.com/iFrankStudio/cioc/actions/runs/43993641624), SHA `187fe3fd`: path correction introduced an unrelated `PathBuf` diagnostic `Display` compile error; B49 observer again correctly failed closed. Third failure recorded in the same [path correction audit](B49-second-native-trace-location-failure.md).
4. **Fourth corrected changed-only run43993850940**, SHA `111b4071`: 1/1 B49 selected native PASS, 35/35 independent observers PASS, B31/B34 ABI PASS, **source-pinned 11,437-step independent replay PASS**.

**No single corrected-SHA full48/48 Rust regression run exists:** the first full run verified the preceding47 targets and their ABI checks, but B49 then failed to compile. Subsequent corrected runs touched only B49 Rust test/artifact plumbing and documentation; the workflow deliberately ran its source-coupled selected target plus all observers rather than re-running 47 unchanged historical studies. State these two evidence types separately. **Do not translate the targeted fourth run into 48/48 full regression success.**

## Research consequence

B49 removes a major *observer fidelity* weakness: within the tested subset, source-selected CIL, actual mutations, real task IO and paid resources are now independently reproduced at the event level across old and newly frozen task sequences. The next study should **stop improving only the verifier** and instead test a falsifiable increase in the actor's endogenous candidate-generation and retention ability, with prospective task-function changes, nontrivial baseline comparisons and an evaluator that does not smuggle in the winning program. Strictly distinguish a source-authored program choosing an authored branch from actor-created mutation rules that are effective without redesign for each task family.

**CL-1B B3 endogenous cumulative adaptation: NOT MET. B4 endogenous organization: NOT MET. Cyberlife: NOT ESTABLISHED.**
