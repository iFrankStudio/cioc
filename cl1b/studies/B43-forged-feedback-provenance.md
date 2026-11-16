# CL-1B B43 — feedback-data forgery at a native self-evaluation boundary

**Frozen protocol, 2026-11-16, before any native B43 CI result.** The native test and the independent Rust-source-parsed CIL model were authored first; aggregate expectations below are **source-level model forecasts**, not confirmed Rust findings. This is a **causal falsification of B42's evaluator**, not a new or improved guard algorithm. CL-1A remains archived.

## Hypothesis and scientific distinction

B42 paid for a computed diagnostic value, wrote a real trial result and used real task-output readback to decide whether to retain a newly cloned executable instruction. Its observer currently treated `CellRead(key11,value)` as evidence of actual service execution. But cell11 is shared writable data: its observed value may have an **external provenance that the native B42 CIL evaluator cannot distinguish from its own previous Write**.

B43 asks: **Can a forged value, delivered only between the actor's first genuine diagnostic Write0 and the next CIL Read, make the continuing actor falsely certify its defective constant-zero output, skip constructing the useful candidate and retain wrong code into unseen tasks?** The null is no causal effect of this injection after controlling for external intervention count/value and relative timing.

The tested B42 seed, CIL decision policy, graph instructions, task definition, credits, clone mechanism and initial data are identical in every matched B42-source arm. Only the **researcher-controlled environment intervention** differs. Such an intervention is NOT actor self-deception, not self-generated false feedback, not a naturally occurring attacker and not a property of the VM itself.

## Frozen experiment

Exactly **4 training inputs `[-1,0,1,17]` × 2 held-out values `[-9,23]` × 5 arms = 40 worlds**. One PID1 starts with **128 local compute**, zero neutral pool, strict storage limits. First64 native attempts occur under original task input, then the environment updates only cell7 to held-out input and executes the remaining64 attempts. No reloading a process, host rollback, free actor code repairs or external compute refill.

The original B42 `ProbePlus` CIL founder (38 nodes) is imported verbatim; its helper's visibility is increased solely to avoid duplicating the founder in the test. No dedicated validation opcode or new program source is installed. The compact immutable four-opcode `StaticCorrect` remains a strong comparator.

Each intervention targets the first actual CIL `CellWritten(key11,value0)` at B42 PC10, when the task challenge was internally computed as `train+1`. Native PC11 immediately follows as a charged `Read`.

| Arm | External action | Causal expectation |
|---|---|---|
| **GENUINE** | No extra output intervention | Truthful native output0 seen by the actor |
| **FORGED_BEFORE_READ** | After PC10 native output0, externally set cell11 to the challenge `train+1` **before** PC11 Read | Actor reads forged success, may retain defective PC10 |
| **SAME_VALUE_BEFORE_READ** | Same intervention timing, but write the true output0 instead | Injection itself does not explain false success |
| **FORGED_AFTER_READ** | Externally write `train+1` only **after** PC11 Read has already filled actor register | Same data but too late to affect the decision |
| **STATIC_CORRECT** | Four-node correct immutable copier, same credit budget, no attack | Independent efficacy and overhead baseline |

No external write counts as a native output, diagnostic success or improvement. Record event **origin**, PC, original native Write value, actual CIL Read value, first Branch successor, true CIL patch, clone count, restored original input, correct/wrong internal outputs and exact attempted compute. Count only actor-origin `CellWritten(key11,value)` *after task input restoration* as real external task service. All diagnostic output before restoration is excluded.

## Frozen model-level expectations, before Rust CI

Across eight matched worlds **per arm**:

| Arm | Training true/wrong real service | Held-out true/wrong | Paid compute | New executable nodes | False success verdicts |
|---|---:|---:|---:|---:|---:|
| GENUINE | **30 / 16** | **72 / 26** | 1,024 | 6 | 0 |
| FORGED_BEFORE_READ | **16 / 48** | **0 / 104** | 1,024 | 0 | **6** |
| SAME_VALUE_BEFORE_READ | **30 / 16** | **72 / 26** | 1,024 | 6 | 0 |
| FORGED_AFTER_READ | **30 / 16** | **72 / 26** | 1,024 | 6 | 0 |
| STATIC_CORRECT | **128 / 0** | **128 / 0** | 1,024 | 0 | 0 |

Two `train=-1` worlds form a necessary negative control: native diagnostic `train+1=0` already truly matches constant-zero output0, so an external write of zero cannot cause an *additional* false verdict. In the **six** worlds with nonzero diagnostic values, the forged value matches the challenge even though the **last genuine native task Write was zero**. This is a *false attestation of the executed program*, not false input data. The actor's decision routes to PC26, never clones node38, retains PC10's wrong source, and emits zero correct task writes on the nonzero held-out inputs. Late forgery and same-value intrusion should preserve genuine results.

## Falsification thresholds

A native success for the **negative hypothesis** requires all 40 worlds, 40 explicit provenance/timing records, exact inventory conservation, six wrong first verdicts limited to early forgery, and first-branch/clone/retention outcomes matching independent forecasts. Failure, unexpected traps, missed intervention window, mismatched provenance, wrong-world exclusions or zero-test CI are not scientific success.

**Acceptance boundary:** detecting or inducing a feedback trust failure is not creating an autonomous error detector. B43 neither remedies the flaw nor demonstrates task-value authenticity, evaluator hardening, emergent objectives or cyberlife. It is evidence about a specific weak readback architecture. CL-1B B3/B4 remain NOT MET regardless of the outcome.
