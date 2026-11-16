# CL-1B B43 — a truthful CIL read can report forged task-output data

**FULL RUST-NATIVE VERIFICATION PASS.** [GitHub Actions run 43857848182](https://github.com/iFrankStudio/cioc/actions/runs/43857848182), source SHA **`396df796d47a44b11274e3f833ac72be66b26d2d`**. [Original SHA/run-pinned summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/396df796d47a44b11274e3f833ac72be66b26d2d-43857848182-1/summary.txt) · [full 40-world B43 Rust event log](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/396df796d47a44b11274e3f833ac72be66b26d2d-43857848182-1/cl1b_b43_feedback_provenance.log) · [independent source-parsed CIL output](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/396df796d47a44b11274e3f833ac72be66b26d2d-43857848182-1/b43_feedback_provenance_scan.json).

The exact run completed **43/43 CL-1B Rust integration targets**, **30/30 independent source-parsed CIL observers**, B31/B34 private instruction ABI tests, and evidence publication, with zero failed test targets. B43's integration target additionally executes the reused B42 module's original test. Non-fatal `dead_code` warnings arose from observer-only fields, so this is **not** a separate formatting/lint certification.

## Experimental intervention and provenance

[Pre-native protocol](../studies/B43-forged-feedback-provenance.md) · [Rust native test](../../cl0/crates/world/tests/cl1b_b43_feedback_provenance.rs) · [independent exact-Rust-source CIL interpreter](../analysis/b43_feedback_provenance_scan.mjs) · [pre-native model forecast](B43-model-feedback-forgery-forecast.md).

The same B42 38-opcode `ProbePlus` founder is imported unchanged for all B42-based arms (only the Rust test helper's visibility was expanded; **no CIL opcode or code changes**). It computes `challenge = trainingInput + 1`, writes the challenge to shared task cell7, then genuinely executes defective `PC10 Write0` to cell11. The actor next executes a paid `PC11 Read(cell11)` and compares the read value to the challenge before deciding whether to construct a corrected Writer via native `CloneNode` + `PatchNext`. The laboratory injects an **EXTERNAL** `SetCell(key11,value)` event at a registered boundary; this is not an actor action or an added VM capability.

The true source event in each attacked world is **actor-origin PC10 Write0**. In six of the eight early-forged worlds, the environment overwrites cell11 with the computed challenge (nonzero), so **PC11 genuinely reads the now-present, forged value**. The CIL event is a real `CellRead` with internal origin, but that fact does **not** establish that the *read value was previously produced by native task execution*. The actor has no authenticated last-writer provenance at its CIL level. It branches to success and retains the wrong constant-zero Writer rather than constructing a new candidate.

Three causal controls isolate the mechanism:
1. No extra intervention: the native read sees the true output zero; nonzero challenge drives actual self-code construction.
2. Externally write **the same actual value zero** between PC10 Write and PC11 Read: no change in function decision or subsequent task behavior.
3. Externally write the **forged challenge only after PC11 Read**: the actor's register is already bound to true output zero; decision and later behavior match the untouched control.

A four-opcode immutable correct task copier remains a resource-matched strong baseline, not an alternative self-assessment policy. All external data writes are traced as external and **excluded** from native work metrics; earlier challenge-related outputs before input restoration are separately classified diagnostic, not task service.

## Fully verified 40-world matrix

Four original inputs `[-1,0,1,17]` × two unseen nonzero inputs `[-9,23]` × five arms = **40 actual Rust-native worlds**. Every world begins with128 local credits and runs **exactly128** charged CIL steps, 64 opportunities before and64 after the externally updated task input, with no host compute refill. All 40 worlds' native observations match the independently executed source-parsed model, not just an aggregate summary.

| Arm / eight worlds | Training true / wrong task writes | Heldout true / wrong | Actual CIL charges | Native cloned candidates | External output overwrites | Newly false successes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| **GENUINE** | **30 / 16** | **72 / 26** | 1,024 | 6 | 0 | 0 |
| **FORGED_BEFORE_READ** | **16 / 48** | **0 / 104** | 1,024 | 0 | 8 | **6** |
| SAME_VALUE_BEFORE_READ | 30 / 16 | 72 / 26 | 1,024 | 6 | 8 | 0 |
| FORGED_AFTER_READ | 30 / 16 | 72 / 26 | 1,024 | 6 | 8 | 0 |
| STATIC_CORRECT | **128 / 0** | **128 / 0** | 1,024 | 0 | 0 | 0 |

The two `train=-1` worlds are retained, rather than omitted: there, the computed challenge is zero and the faulty Write0 genuinely matches it, so an external zero overwrite adds no *new* false success. For training0,1,17, the nonzero challenge would otherwise expose the faulty Writer; an external forged cell contents produces six causal new false-acceptance outcomes. In every early-forged world, **no new native node38 is built**, router node9 retains node10, and output remains incorrect for both unseen nonzero inputs. This is verified only over the bounded 128-step observation; it does **not** prove that the defective program is literally permanent.

## Scientific interpretation and limitations

**Result: a negative robustness finding, not autonomous adaptation.** B43 conclusively demonstrates that a *specific B42 authored* task-output checker can be made to accept incorrect program behavior by an **externally forged data value supplied at the vulnerable read window**. The CIL `Read` is honest about current memory contents, but memory content is not an authenticated causal record of the last native producer. External events provide provenance to the laboratory observer, not automatically to the actor.

This experiment did **not** demonstrate an actor learning to deceive itself, inventing a failure-resistant evaluator, learning a goal, restoring function after a spoof, or achieving ecological fitness. The intervention is **host-controlled**, narrowly timed and not resource-priced to a separate attacker. This negative does not establish that *all* evaluator architectures are equally vulnerable. The source continues with a fixed authored task and known correct comparator, and the compact correct program remains better under total-cost controls. **CL-1B B3 and B4 remain NOT MET; cyberlife not established.**

## Implication for the next discriminating study

Avoid another fixed checksum/recheck loop. The next scientifically useful comparison is between (a) output-value readback alone, (b) an independently observable causal function witness that the actor can access only through *ordinary paid world primitives*, and (c) explicit corruption of whatever evidence the actor can itself modify. Test whether authenticity arises from the environment's ordinary causal structure instead of being supplied as a privileged instruction or host life oracle. If authenticity is infeasible under the current CIL observer model, record that as a necessary limitation and redesign the world-visible information physics before claiming B3.
