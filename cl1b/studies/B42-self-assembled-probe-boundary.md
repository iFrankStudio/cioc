# CL-1B B42 — actor-generated diagnostic input and assembled service node

**Protocol fixed on 2026-11-16 after independent source-parsed model construction, before Rust native CI.** Model-visible forecasts are not claimed to have been preregistered before examining the model. This experiment follows verified B41 and retains CL-1A as archived. **No reproduction.**

## Narrow question

Can one continuing native-CIL individual *construct* a new executable node from an existing **generic internal Write instruction** and *pay for its own discriminating diagnostic input*, then test and retain the new service path based on actual output readback? Does such an internally generated probe improve **generalization**, or only move the boundary of a known overfitting case?

Neither the rule producing the diagnostic value nor the algorithmic assembly sequence was discovered by the actor. They are investigator-authored seed instructions. The source template is an already-present ordinary Write, **not** a novel opcode or autonomous algorithm invention.

## World and mechanism

One PID1, 38-opcode founder (four opcodes in immutable static control), initial **128 process compute**, zero pool credit, strict finite storage, no host repair or external candidate list. Environment seeds **only task input cell7**. The actor reads this value, pays for a checkpoint in cell30, and computes challenge `x+1` or `x-1` by ordinary CIL `Const` and `Add`. It actually writes the diagnostic value to shared cell7, runs its initially wrong constant-zero Write against cell11, reads back that true service output, computes the discrepancy and branches.

When discrepancy is nonzero, paid native `CloneNode` copies its **generic challenge-input Write PC7** to a genuinely new executable node38, then native `PatchNext` changes the clone's feedback successor and redirects service router PC9 to node38. The individual executes this newly minted candidate against the diagnostic input, verifies the actual output a second time, then edits its chosen service node's successor to preserve steady work. If both versions fail, it halts. On acceptance it restores the originally sampled cell7 from its own cell30 checkpoint, then repeatedly reads real task input and writes to real output11. The trial marker in cell22, two code-edge edits, one clone and restoration all cost real CIL instructions/storage.

**Important threat boundary:** the actor writes the diagnostic challenge into the same shared cell7 used by the task. Trial outputs before genuine input restoration are classified as **diagnostic**, not as successful external-task service; the observer checks native provenance and distinguishes these from service outputs after restoration. Writing the challenge to the environment is an authored affordance, not trusted world feedback.

## Prospective native falsifiers / frozen model forecast

Exactly **4 training inputs (-1,0,1,17) × 2 holdout inputs (-9,23) × 6 arms = 48 source-matched worlds**. Each starts with128 local compute and receives at most64 attempted charges in each task-input phase; stopped actors retain unspent compute. Inputs change exogenously after the first64 opportunities; do not cherry-pick successful worlds. No external candidate code or source patches are provided.

Arms: `ProbePlus` (challenge x+1), `ProbeMinus` (x-1), `NoProbe` (same-cost Add x+0), `IgnoreFeedback` (same-cost Sub x-x), `PatchDisabled` (charged PatchNext suppression while CloneNode remains paid), and `StaticCorrect` (four-node immutable live-input copier).

Independent source-interpreted forecast, **not Rust native CI**:

| Arm (8 worlds each) | Training correct/wrong **service** | Holdout correct/wrong **service** | Charged CIL | New nodes | Halted |
|---|---:|---:|---:|---:|---:|
| ProbePlus | 30 / 16 | **72 / 26** | 1,024 | 6 | 0 |
| ProbeMinus | 30 / 16 | **72 / 26** | 1,024 | 6 | 0 |
| NoProbe | 46 / 0 | **72 / 26** | 1,024 | 6 | 0 |
| IgnoreFeedback | 16 / 48 | 0 / 104 | 1,024 | 0 | 0 |
| PatchDisabled | 0 / 4 | 0 / 0 | 312 | 8 | 8 |
| StaticCorrect | **128 / 0** | **128 / 0** | 1,024 | 0 | 0 |

Trial output before environment restoration is separately logged; it is **excluded** from the task-service numerator. PatchDisabled halts and consumes only312 credits across its eight worlds, so its zero task output is not 1,024 paid attempts. Native source-node/edge traces and total-inventory conservation must match model forecasts for native verification.

### Exact boundary cases (not post-hoc exclusions)

- `NoProbe` training0 accepts false constant-zero; holdout nonzero is wrong.
- `ProbePlus` training0 generates challenge1 and can create a correct cloned Writer; **training -1 produces challenge0**, accepting the same bad constant-zero instead.
- `ProbeMinus` moves that failure to **training +1**.
- The two nonzero holdouts require a program that generalizes beyond the diagnostic value, while static correct remains significantly cheaper.
- Frozen input change may still occur between source Read and output Write; retain all stale incorrect outputs.

## Claim boundary

A native success proves **paid endogenous challenge construction and new-node assembly from an externally authored template under an externally authored policy**. It cannot prove autonomously discovered mutation rules, reliable self-generated tests, evaluator tamper resistance, improvement against a compact program, or CL-1B B3/B4. The key expected negative is that **the aggregate heldout performance of x+1, x-1 and no-probe is the same**, despite shifting the zero-alias failure.
