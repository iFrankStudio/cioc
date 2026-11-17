# CL-1B B54 — native paid response verification time-of-check/time-of-use falsifier

**Narrow H54-TOCTOU CONFIRMED by actual native step-boundary events.** The B53 source-authored monitor can verify current serial/commit values, then accept a later altered success bit for a genuinely wrong output. This is a failure of external packet truth/stability, **NOT** spontaneous individual adaptation, B3/B4 or cyberlife.

## Prior preregistration, source and exact native records

The [B54 prospective protocol](../studies/B54-native-paid-response-time-of-check-time-of-use-preregistration.md) was committed **before B54 native implementation**. The native program uses the [original B53 124-node founder](../../cl0/crates/world/tests/cl1b_b53_native_response_correlation.rs) entirely unchanged in actual CIL op semantics; only its Rust function visibility is exported for shared reuse. B54 changes **laboratory intervention timing at genuine paid native VM instruction boundaries**, never edits PID1's program, chooses the new candidate or gives PID1 a hidden task answer.

- **Tested source SHA** `ac234b9796409beb65eef69435cdca1a5d8b7d75`, [successful GitHub Actions run 44070067648](https://github.com/iFrankStudio/cioc/actions/runs/44070067648): **scope=changed-only, eight selected actual native Rust targets PASS (B49/B50/B51b/B51c/B51d/B52/B53/B54), all42 independent observers PASS, mandatory B31/B34 ABI PASS, both failed counters zero and source-pinned evidence published. This is not a full55/55 integration regression.**
- [Original CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/ac234b9796409beb65eef69435cdca1a5d8b7d75-44070067648-1/summary.txt) · [original exact-source run conclusion](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/ac234b9796409beb65eef69435cdca1a5d8b7d75-44070067648-1/result.txt) · [original B54 native paid JSONL](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/ac234b9796409beb65eef69435cdca1a5d8b7d75-44070067648-1/b54_native_steps.jsonl) · [native Rust log](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/ac234b9796409beb65eef69435cdca1a5d8b7d75-44070067648-1/cl1b_b54_native_interleaving.log) · [independent native-event replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/ac234b9796409beb65eef69435cdca1a5d8b7d75-44070067648-1/b54_native_interleaving_replay.json).
- [B54 Rust interleaving harness](../../cl0/crates/world/tests/cl1b_b54_native_interleaving.rs) · [source-parsed independent CIL interpreter](../analysis/b54_native_interleaving_replay.mjs). A separately contributed [abstract source-order model](../analysis/b54_toctou_source_model.mjs) is **not native evidence** and is not counted as an additional independent observer target; the native 80,000-step replay is the actual causal evidence.
- First B54 source revision `151e8a36be6adb2f2e7acb031db28e9e008f6eb8` [run44070038596](https://github.com/iFrankStudio/cioc/actions/runs/44070038596) was **canceled** when a separate abstract-analysis-only commit `ac234b9796409beb65eef69435cdca1a5d8b7d75` caused GitHub's documented cancel-in-progress concurrency. Use the succeeding exact-SHA run rather than concealing cancellation or claiming that canceled run was green.

## Source-identical finite task worlds and actual interleavings

**20 continuous PID1 worlds**, two hidden task transitions A `x−2 → −x+3 → x−2` and B `−x−3 → x+2 → −x−3`, two charged input initialization phases, five external-response scheduling arms. A single actor starts at the exact B53 founder node92, **4,000 paid native CIL instructions**, 16-cell/180-code-node/300-unit strict limits, no process reset or credit/graph injection; each arm paid16,000 native CIL steps across4 worlds.

1. `FRESH_ATOMIC`: after a genuine output11 the lab writes true bit20 and current serial echo27 /commit28, with no intervention during the actor's subsequent paid reads.
2. **`LATE_BIT_FLIP`**: the lab first writes an **honest** packet with correct current serial and a truthful **failure bit0 for a genuinely wrong service output**; **only after actor CIL actually evaluates both serial/commit comparisons and reaches PC119 immediately before the paid `Read(cell20)`**, the external lab performs `Intervention::SetCell{key:20,value:1}` and the actor then pays to read that value. The lab never changes actor's code/nonce/PC or output. This precisely demonstrates time-of-check/time-of-use.
3. `LATE_COMMIT_FLIP`: after PID1 actually pays to read the correct `cell28` commit at PC113, the lab changes that external cell back to `current_nonce−1` before the next paid instruction. The actor retains the correct old value in a register and proceeds, illustrating a **non-atomic snapshot** even though no false-success output was induced in this arm.
4. `STABLE_FORGED_CURRENT`: the external lab supplies a complete, consistently current nonce/commit with an untrue bit1; matches B53's honest-evaluator-trust impossibility contrast.
5. `OLD_NONCE_REPLAY`: repeats a previous successful packet under its old serial; native CIL rejects without reading the bit as current.

The attacker in the LATE_BIT_FLIP schedule uses laboratory hidden-task truth **to select a truly wrong submission for intervention**; this is an explicit evaluation adversary, not evidence the actor observed task truth. All interventions carry **Origin::External** and preserve compute inventory. The schedule depends on native process PC and next paid opcode, not host resupply or a task-specific optimizer.

## Exact measured native outcome

Each arm had four worlds × three stages ×24 heldout scored writes = **288/288 realized service opportunities**. Every arm had the **same actual paid16,000 instruction opportunity**, even after completing the first72 scored outputs per world.

| Arm | Correct / wrong service writes | Correct by stages | Native reopened searches | External late bit edits | External late commit edits | Wrong successes accepted |
|---|---:|---|---:|---:|---:|---:|
| FRESH_ATOMIC | **280/8** | 96,92,92 | 8 | 0 | 0 | 0 |
| **LATE_BIT_FLIP** | **192/96** | 96,0,96 | **0** | **96** | 0 | **96** |
| LATE_COMMIT_FLIP | 280/8 | 96,92,92 | 8 | 0 | **388** | 0 |
| STABLE_FORGED_CURRENT | 192/96 | 96,0,96 | 0 | 0 | 0 | **96** |
| OLD_NONCE_REPLAY | 192/96 | 96,0,96 | 0 | 0 | 0 | 0 |

**Main falsification:** Under initially honest matched packets, late-bit intervention at the exact **paid PC119 boundary** caused **96 genuine failed service outputs to be accepted as success**, preventing all8 otherwise observed true-feedback task adaptation reopens, and collapsing the changed-task window to0/96 correct. This replicates the stable-content-lie arm outcome, but crucially the external bit was still **honestly written until after the actor had already completed its serial/commit checks**.

**Non-atomic snapshot:** 388 post-read external edits to commit cell28 did **not** prevent PID1 from continuing its paid success/error logic with the cached register value. The mode-switch performance was still280/8 with8 useful reopens; this finding is about **stale verified metadata**, not an observed false-success in this arm. It cannot be conflated with the stronger LATE_BIT_FLIP false-positive result.

**Old serial replay:** 638 PID1-origin packet rejection markers, 0 erroneous success acceptances, and **0** source reopens in the shifted regime. These rejection counts extend beyond 288 scored service writes because the actor continues to pay instructions and submit outputs after stage3; do not misinterpret them as count of wrong scored outputs. A rejected response neither authenticates truth nor repairs a newly broken task by itself.

The source-blind B54 actor still uses the same pre-authored B51c/B52/B53 arithmetic grammar, B52 watchdog and B53 serial protocol, so this negative experiment is a controlled **information/time-of-use counterexample**, not a scientific proof against all possible digital organisms.

## Independent replay and scientific acceptance

The independent JS CIL interpreter reconstructs the full executable 124-node original B53 source from the actual B51c founder, B52 append and B53 append literals; independently replays **80,000/80,000 genuine charged native CIL instructions**, **12,052/12,052 external cell interventions** including all timed post-serial-check and post-commit-read modifications, **40 laboratory-only task-stage records**, and **602 native executable-edge edits**. PC, registers, data cells, storage, source graph, credits and actor responses are all compared event-by-event. **First divergence: null**. The observation cannot be produced by simply evaluating native outputs with a second scoring script.

**H54 original TOCTOU falsifier: ACCEPTED.** The unmodified B53 CIL source cannot infer external semantic truth or enforce an atomic multi-cell packet while the environment can modify the packet between independent paid Reads. The `LATE_COMMIT_FLIP` arm shows that rereading a stored value can leave a changing external cell unobserved, though it does not itself generate wrong-success acceptance.

**B3/B4: NOT MET. Cyberlife: NOT ESTABLISHED.** This study remains researcher-authored adaptation code; a future primary study should evaluate **actor-origin self-revision of its own monitoring/search policy, explicit resource-cost accounting under neutral scarcity, and tasks outside the author-supplied affine grammar**. Merely adding a fourth or fifth fixed guard cannot overcome an evaluator permitted to lie about both current nonce and task success, absent additional trusted information or assumptions.
