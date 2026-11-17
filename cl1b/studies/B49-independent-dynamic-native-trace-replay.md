# CL-1B B49 — independent dynamic replay and measurement-integrity gate

**Preregistered before first B49 Rust CI, 2026-11-17.** This is a test of the credibility of the B47/B48 native measurement and its *source-level causal interpretation*, not a new organism, reward rule, or repaired learning strategy. CL-1A remains archived; B3 and B4 remain NOT MET.

## Why this experiment is the immediate priority

B47's observer checked only Rust **source structure**, not actual CIL execution. B48's native crossover produced preliminary feedback-vs-sham output differences, yet (1) one blind sham and one edit-disabled arm stopped early, violating B48 H48's equal-realized-exposure clause, and (2) the independent observer did not reproduce native events step-by-step. Treat B48 H48 as **not accepted**, even though its selected CI returned green. Do not post hoc relax it.

B49 explicitly tests the following H49 *methodological* hypothesis, not H48 or CL-1B B3:

> Across a frozen unseen B48 block and additional newly frozen task sequences, a separate CIL interpreter reading the original B42 Rust graph must exactly reproduce **every actual paid Rust native instruction**, its native event kind, event provenance, PCs, registers, real input/output cells, retry and checkpoint data, dynamic code-node allocation, patched edges, credit cost and storage usage, including early stops and preamble phase shifts. If even one event or final state differs, or a trace is missing, H49 fails.

The independent interpreter must generate its outputs **without using** expected Rust output values as its inputs. It may parse the original seed CIL source and verify that Rust arm modifications match the frozen protocol, as all source-defined models must. It must **not** reconstruct the Rust trace by copying it.

## Exact population of experimental worlds

Run **96 native worlds**: one PID1 continuously executes 144 locally paid CIL opportunities (three regimes ×48 attempts), finite storage cap192, 16 live data cells, max80 CIL nodes, no resource refill or experimenter-supplied native code repair.

Two input-sequence blocks, with **no post-hoc seed selection**:

- B48's eight previously frozen unseen inputs: `[23,0,-1]`, `[-9,17,23]`, `[1,-9,0]`, `[0,23,17]`, `[17,-9,-9]`, `[-1,1,23]`, `[23,23,0]`, `[-9,0,1]`.
- Four **additional B49 prospective** sequences, not part of B47/B48: `[2,0,-7]`, `[-4,31,0]`, `[0,7,-4]`, `[31,2,7]`.

Each sequence gets **2 deterministic phase offsets** (baseline and one charged, ordinary entry `Const` preamble) and **4 CIL policies**:

| Arm | Source policy, every instruction charged |
|---|---|
| TRUE | Exact original B42 ProbePlus graph, feedback-aware native edits |
| BLIND_SELF | Same source and opcode cost except node12 becomes `Sub(lhs0,rhs0)`; feedback erased |
| BLIND_ZERO | Same source/cost except node12 becomes `Sub(lhs0,rhs3)`; semantically different blind source with known risk of early halt |
| STATIC | Compact four-op immutable live input→output copier, strong resource/correctness comparator |

Source selection, initial code and program counter are fixed *before* execution. Both `TRUE` and `BLIND_SELF` are expected to execute all144 paid steps, as is compact STATIC; `BLIND_ZERO` may stop early, and such worlds must **remain** in the denominator. No successful-source or minimum-work filtering.

**Count every actual actor-origin** output `CellWritten(key11)` against **cell7's current value at that exact Write event**, including internally constructed diagnostic writes. This is B48's *raw* scoring; it differs deliberately from B47's post-restoration filtering. Do not credit external writes or static graph metadata. Also report edits, clone count, halted status, total paid attempts, remaining budget and storage.

## Native and independent oracle boundary

Native Rust test `cl0/crates/world/tests/cl1b_b49_dynamic_replay.rs` writes **a machine-readable JSONL event artifact** in the CI evidence directory, including:

- exact source SHA in a top-level metadata record;
- source, task sequence, phase offset, initial graph length and credits in each world header;
- one `input` record at each of the three external input transitions;
- one `step` record per **actual charged** CIL step (tick, phase, PC, next PC, normalised true event kind, all four registers, live cells7/11/22/30, native graph length, key patched successors, new clone successor, credits, stopped flag, actual strict storage);
- an independently checkable end record of charged total, raw true/wrong output writes, native code edits, halting and peak storage.

The source-parsed JavaScript interpreter `cl1b/analysis/b49_dynamic_native_trace_replay.mjs` must independently calculate every step and compare all those fields. It prints the first divergence (including case/line/expected/actual) and exits nonzero; absence or truncation of the native artifact must **never** produce a green result. The observer runs only **after** the Rust test has produced its native trace during that same exact-SHA CI job. The CI artifact and the immutable `cl1b-ci-evidence` branch must archive the raw JSONL; independent results are separate.

**No observer-only mock substitutions, no source-pattern pass as a substitute for replay, no shared CIL interpreter implementation with Rust.** The JS model reads only actual B42 founder CIL and prescribed source modifications, never a Rust output vector or a previously existing native trace as its state transition input.

## Frozen directional and reference expectations

B49's first eight sequences are an **existing B48 replication**, not novel independent predictions. Under B48's same CIL source, B49 should reproduce its previously observed raw holdout correct/wrong aggregates:

| Phase / arm | Holdout1 correct/wrong | Holdout2 correct/wrong |
|---|---:|---:|
| baseline TRUE | **64/15** | **64/16** |
| baseline BLIND_SELF | 18/54 | 20/60 |
| baseline BLIND_ZERO | 0/9 | 0/10 |
| baseline STATIC | **96/0** | **96/0** |
| +1 TRUE | **64/15** | **63/10** |
| +1 BLIND_SELF | 18/54 | 20/60 |
| +1 BLIND_ZERO | 0/9 | 0/10 |
| +1 STATIC | **89/7** | **89/7** |

These values are only a regression check against the B48 native output, **not** the independent dynamic replay itself. The newly frozen four B49 sequences have **no selected numerical answer**; the independent interpreter and native Rust must agree at every step, regardless of whether the resulting counts support or reject a mechanistic hypothesis.

The methodological H49 succeeds only if **all96 native worlds**, all3 external input transitions per world, every actual charged native event, all end-state counters, full snapshot/source SHA and B48 replication aggregates match the source-parsed independent model. Realized exposure must be reported explicitly for each arm. H49 does **not** imply H48, autonomous policy discovery, durable cumulative adaptation or open-ended digital life.

### Fail-closed and no B3 inflation

A native green test that only asserts Rust self-consistency without a passing independent observer is **insufficient**. A static-source parser is **insufficient**. A changed-only run is not full-regression verification; describe the exact scope. A native target failing its trace or an independent interpreter diverging is a recorded experimental **failure**, not a partial success. Unverified native outputs are forecasts only.

If H49 passes, a next project question is whether within-architecture true feedback improvement survives a **different, nonterminating, equal-exposure semantic sham**, and whether that success can ever reflect self-*generated* mutation/selection policy rather than a founder-authored program. These are new hypotheses and must be preregistered rather than retroactively appended to H48.

**B3 endogenous cumulative adaptation NOT MET; B4 organizational closure NOT MET; cyberlife not established.**
