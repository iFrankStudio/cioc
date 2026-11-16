# CL-1B B46 — false attestation from a genuine same-PID write between two paid reads

**Prospective protocol frozen before any native B46 Rust GitHub Actions execution.** The 56-world outcome table below is a *Rust-source-parsed independent CIL model forecast*, not native evidence. No new VM opcode, world fitness rule, privileged repair mechanism, program-generated attacker or autonomous learning policy is introduced.

## Scientific hypothesis

B45's optional `ReadLastWriter` sensor truthfully identifies the **latest native writer of a cell at the instant of a charged sensor read**. But its investigator-authored guard reads **the data value first**, then branches and separately senses the **latest writer**. An intervening ordinary native CIL `Write` by the **same PID** can cause the guard to combine a previously read correct value with a newly observed internal writer tag, falsely attesting that the continuing defective program is good.

A second independent failure mechanism is **true own-PID provenance without ongoing program functionality**: one transient legitimate native CIL write of the correct diagnostic value can be followed by restoration of a permanently wrong service router. Provenance is a statement of *who last wrote*, not *whether future code works*. The one-time native interposer is researcher-authored and temporarily enabled; no individual spontaneously invented a deception or validation policy.

### Two distinct falsifiable claims

1. **Native wrong-value post-read write:** the actor's first charged PC11 `Read` sees the externally forged correct diagnostic value. Before its later paid provenance sense at PC38, an ordinary native `Write` by actual PID1 overwrites the cell with **zero**, *which is wrong for six of eight nonzero challenge worlds*. The last-writer sensor honestly returns **PID1**. The guard accepts the stale earlier value and retains defective service code. A time-of-check/time-of-use (TOCTOU) false attestation occurs even though no actor can falsify the writer metadata.
2. **Native correct-value transient write:** the same interposer instead writes the true diagnostic challenge by PID1. This makes current data and current source identity genuinely correct; the guard still retains the old defective service router PC10 because its one-time successful write says nothing about the later service source. **False generalization** must be distinguished from **the wrong-current-value race in claim1**.

Neither experiment falsifies the correctness of the `ReadLastWriter` instruction. It falsifies a particular **source-authenticated value + future-functional-code decision policy**.

## Frozen source and 56-world design

Four initial task inputs `[-1,0,1,17]` × two unseen nonzero task inputs `[-9,23]` × seven arms = **56 worlds**, one continuing actor PID1 and zero compute shared pool in each. Strict shared capacity192, original 128 local compute; execute up to64 charged training and64 charged heldout instructions; external task input7 is replaced only at phase boundary. Read and sense events retain true PC/origin. Score only **actual actor-origin service output writes after original input restoration**, never attacker writes, diagnostic writes or mere source labels. An unchanged four-op static correct source remains the strong comparator.

Every non-static arm uses B42's **exact original CIL founder** plus B45's **exact same four paid CIL validation steps** at nodes38–41. No adaptation guard is newly invented. For five arms the only new source difference is a **single paid opcode at node42**, reached after PC11's value read and before the earlier PC12 comparison. The lab then applies **one explicitly external `PatchEdge(node11,next12)`** after the first PC42 execution, so that temporary source change does not become an accidental perpetual corrected service algorithm. All five arms get exactly this one-time patch and one extra paid native CIL attempt. This patch is an **experimental intervention, not actor-authored self-repair or native adaptation**. Source initial sizes42 or43, clone new node at42 or43 accordingly.

Native producer PC10 always writes zero before the challenge feedback. For all spoof arms an **external** SetCell overwrites cell11 to the computed challenge `train+1` after PC10 and before PC11. An external event never counts as native task success.

Arms (eight worlds each):

| Arm | One-time native PC42 operation and controls | Origin expected at first PC38 sense |
| --- | --- | --- |
| **NativeWrong** | PID1 CIL **Write0** to output11 **after** PC11 Read, then lab source restoration | PID1; now-current value is wrong for six challenge worlds |
| **NativeEcho** | PID1 CIL Write challenge register0 to output11 after PC11 Read, then same lab restoration | PID1; correct once, but code PC10 remains defective |
| **NopSpoof** | one charged generic Const that does not change output11, then same lab restoration | external0; ought to reject spoof |
| **OutsideLateWrong** | identical Const arm, plus an external SetCell11=0 **after** PC11 Read but before the provenance sense | external0; ought to reject even after value changed |
| **NopGenuine** | identical Const and source restoration without any external output spoof | genuine1; zero-alias and retest policy remain imperfect |
| **B45GuardSpoof** | original B45 authored guarded CIL (without any extra paid PC42) and the same early external spoof | external0; direct previously verified B45 baseline |
| **StaticCorrect** | original four-op immutable correct copier, no spoof or extra node | not sensed; efficiency and truth baseline |

**Important time-credit confound:** the extra PC42 instruction shifts the timing of native task input/output service relative to the fixed 64-attempt phase boundary. This can *improve output counts* even if it is just a paid no-op; do not interpret that as an adaptation breakthrough. The B45 baseline lacking the extra instruction deliberately reveals the size of this scheduling/denominator artifact.

## Independently executed model forecasts (NOT native results)

Each arm contains exactly8 worlds, each of which completes128 charged native-style instruction attempts. The model independently parses the Rust B42 source, checks the B45 four-op guard, verifies B46's one-shot source interposer and simulates all CIL instructions, real data/last-writer tracking and external intervention timing. The following seven vectors are frozen in the native Rust test **before the first native CI**, so there is no post-hoc outcome adjustment.

| Eight-world arm | Training real correct/wrong | Heldout real correct/wrong | Paid compute | New native source nodes | Paid provenance reads | First authenticated source | First origin-check accepts |
|---|---:|---:|---:|---:|---:|---|---:|
| **NativeWrong** | 14/42 | **0/104** | 1,024 | 0 | 8 | PID1 ×8 | 8 |
| **NativeEcho** | 14/42 | **0/104** | 1,024 | 0 | 8 | PID1 ×8 | 8 |
| **NopSpoof** | 24/0 | **104/0** | 1,024 | 8 | 16 | external0 ×8 | 0 |
| **OutsideLateWrong** | 24/0 | **104/0** | 1,024 | 8 | 16 | external0 ×8 | 0 |
| **NopGenuine** | 24/14 | **72/26** | 1,024 | 6 | 8 | PID1 ×8 | 8 |
| **B45GuardSpoof** | 24/0 | **96/8** | 1,024 | 8 | 16 | external0 ×8 | 0 |
| **StaticCorrect** | **128/0** | **128/0** | 1,024 | 0 | 0 | not observed | 0 |

The NativeWrong arm predicts **six** genuinely incorrect *current* output values accepted after the actor's own one-instruction overwrite; two `train=-1` worlds correctly retain zero-challenge as a boundary negative control. NativeEcho's output is correct when sensed but its continuing original wrong code is nevertheless accepted in all eight worlds; this is a different epistemic failure. Both own-PID arms eventually perform **0 correct,104 wrong** unseen service writes. The single extra *NoOp* makes NopSpoof produce **104 correct / 0 wrong** on heldout vs B45 original **96 correct /8 wrong** under the same fixed 64-attempt evaluation; this change **must be attributed to execution scheduling and different instruction phase placement**, not to improved provenance or task competence.

## Acceptance and interpretation

Native passing requires all56 worlds and all seven exact source-model aggregate vectors, step-level paid compute invariants, actual event origin, first value-read register, optional post-read native output/write source0 versus1, native source restoration exactly once for matched arms, explicit unchanged task-service scoring, 192-unit storage cap, strong compact baseline, and CI acceptance of all existing B00–B46 studies with independent observer and B31/B34 ABI checks. The first native run may fail and must be retained rather than silently retried as if it passed.

**This is a negative boundary study about what provenance can establish, not a general security attack or autonomous deception demonstration.** The extra native CIL Write at42 was *installed by the researcher*. The outer patch is *external*. The task/objective and guard are authored, the fixed observer data/scores are not given to the actor as a fitness oracle. The source tag is genuinely accurate; the flaw is attributing an earlier value to that tag or treating one authentic output as proof of a future computation policy.

If native results agree, B47 should not merely pile on another guard. Ask whether **atomic value-and-origin sampling** is necessary in the generic substrate and what its compute/storage tradeoffs would be, versus pursuing within-actor identification of actual functional errors across unseen tasks. An atomic sample alone **still cannot establish that a same-PID correct write implies a correct steady-state program**.

**CL-1B B3 cumulative endogenous adaptation: NOT MET. B4 organizational closure: NOT MET. Cyberlife: NOT ESTABLISHED.**
