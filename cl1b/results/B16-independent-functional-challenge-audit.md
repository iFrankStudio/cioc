# CL-1B B16 — challenge/payload evidence audit (simplified, not native Rust)

**Evidence tier:** source-parsed independent instruction-level interpreter; **not** the original CL-0 Rust VM, SQLite replay, or confirmed GitHub Actions. B16 [protocol](../studies/B16-functional-challenge-and-forgery.md) was preregistered before [Rust test implementation](../../cl0/crates/world/tests/cl1b_b16_functional_challenge.rs) and before these outcomes were inspected. The [repeatable independent script](../analysis/b16_functional_challenge_scan.mjs) reads the same frozen 32-node Rust founder directly. Five labels `[-13,0,7,23,101]` × six conditions = **30 simulated worlds**, 120 charged instruction opportunities each; no drops or traps.

## Original functional writes, not proxy markers

The CIL actor saves source input in key15, injects a **nonzero challenge17** into source key7, deletes output key11 before trying its real payload-producing PC11, reads the resulting key11 at PC13, then restores the original input and performs ordinary PC31 output. No host oracle or special repair primitive is added. The expected value17 and repair heuristic next−1 are **human-authored**. The initial input might be0 without making the challenge ambiguous with a missing read0.

| Arm | Actual CIL challenge writes at PC11 | Correct ordinary service writes at PC31 | First PC13 read (key, value) | Effective CIL code edits |
| --- | ---: | ---: | --- | --- |
| INTACT | **5** | 4 | (11,17) | none |
| SERVICE_SKIP | **4** | 4 | **(11,0)** | node10:12→11 |
| SERVICE_SKIP_STALE_PRESEED | **4** | 4 | **(11,0)** | node10:12→11 |
| SERVICE_SKIP_PATCH_DISABLED | **0** | 4 | (11,0) | none; 4 charged suppressed attempts |
| CHECKER_BYPASS | **0** | 5 | **(7,17)** | none; **false acceptance** |
| TIMED_FORGED_RESPONSE | **3** | 4 | **(11,17)** | node10:12→11 **only later** |

All input labels had the same pathway counts in this **frozen fault matrix**, but they are deterministic input variations, not independent structural mutation samples. Every simulated arm completed120 opportunities and ended with120 modeled compute credits. The Rust integration test checks native event origin, exact writes, restoring original input at PC31, first readback, edits, compute storage, and externally attributed forgery; **actual Rust execution remains unverified**.

### New mechanism established only in a simplified execution model

A CIL-owned **fresh nonzero functional challenge** is more informative than a separately produced success marker for the **specifically tested skipped-payload edge**: after deleting the old output, a missing write reads0 and cannot impersonate the expected17, even if the user's real input is0 and even if output was externally preseeded to the expected challenge value. The actor's CIL policy then patches its source executable and repeats the trial.

**The boundary survives:** CHECKER_BYPASS mutates the PC10 control edge to skip both the actual output write and the instruction that loads the output address. The actor's PC13 `Read` now fetches *its own challenge input key7*, reads17, and falsely accepts. Unlike B10's external marker spoof, this error arises from **internal code-corruption of the measuring path** with no hostile memory injection.

The timed-forgery arm additionally shows an external writer can put17 into the output slot *between* the actor's Delete and Read. The first trial is falsely accepted; the next unspoofed trial discovers the fault and repairs. No guarantee of non-forgeable evidence exists in this shared-cell substrate.

### Scientific significance and next gate

B16 advances an **engineer-authored evidence mechanism** from a proxy status marker to a directly exercised, actor-generated nonzero payload challenge. However, **neither direct readback nor an internally generated challenge is a universally trusted truth source when the evaluator path itself can mutate**. The experiment does not prove autonomous policy formation, ongoing evolution, natural selection, agency or cyberlife.

Before CL-1B gate promotion require a source-SHA-pinned Rust native run, then a preregistered *held-out* corruption campaign of both task and checker graph, and a continuing actor that itself generates candidate diagnoses and chooses which evidence channels to trust under finite compute and storage. The host must not select a successful actor or inject winning code.
