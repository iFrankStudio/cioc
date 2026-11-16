# CL-1B B44 — NATIVE full-state aliasing: historical writer provenance is absent from the current guest state

**FULL RUST-NATIVE CI PASS** — [GitHub Actions run 43898413499](https://github.com/iFrankStudio/cioc/actions/runs/43898413499), exact tested source SHA **`348e812379f93fb586ee8494ad777a4abd791ee2`**. [Archived original 44-target/31-observer full CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/348e812379f93fb586ee8494ad777a4abd791ee2-43898413499-1/summary.txt) · [eight native pair records and the original B42 test log](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/348e812379f93fb586ee8494ad777a4abd791ee2-43898413499-1/cl1b_b44_causal_history_alias.log) · [independently executed Rust-source-parsed CIL model](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/348e812379f93fb586ee8494ad777a4abd791ee2-43898413499-1/b44_causal_history_alias_scan.json).

**Verification scope:** **44/44 registered CL-1B Rust native integration targets PASS**, **31/31 independent observer analyses PASS**, B31/B34 private ABI tests PASS, `failed_target_count=0`, `failed_observer_count=0`, archived `native_job_result=success`. Rust B44 integration reuses and also executes the B42 source module's existing test. This is not an additional formatter/clippy certification. The tested Git SHA is the study code+protocol commit, not later documentation-only commits.

[Prospective protocol](../studies/B44-observational-history-equivalence.md) · [actual native Rust B44 test](../../cl0/crates/world/tests/cl1b_b44_causal_history_alias.rs) · [source-parsed independent CIL interpreter](../analysis/b44_causal_history_alias_scan.mjs) · [pre-native forecast archived before CI](B44-model-history-equivalence-forecast.md).

## Strictly observed native result

This study required a stronger outcome than a coincidental equality of two observed output values: **the entire `serde_json`-serialized native `World` state had to be equal after removing only the `events` history field**. No other state field was omitted, including process code graph, PC, registers, shared cells, compute credits, world tick, graph handles, capacity and scheduler. A full serialized native state includes the current resource and process state, not just a researcher-selected few CIL registers.

A **single original B42 38-opcode `ProbePlus` founder** executes its first nine native instructions (paid) and computes `challenge = train+1`. At the resulting PC9, the lab clones the entire world into two experiment worlds. It then applies **labeled external temporary `PatchEdge` and `SetCell` interventions**:

- **Authentic native data**: a temporarily redirected generic B42 PC7 native `Write` executes and really writes `challenge` to the service output cell11; the lab restores the original code edges, touches only task input7 using a no-op SetCell, and leaves output11's last writer **native**.
- **Forged data**: the ordinary faulty native PC10 `Write` executes and really writes **0**; an explicitly **external** SetCell then overwrites output11 with `challenge`. Source edges and task cells are returned to identical values.

The lab uses the **same number of external interventions and the same two charged native instructions** for each side at the fork, to avoid a trivial difference in event length, tick, PC or credits. These are controlled researcher manipulations; the authentic native output was **not autonomously invented** by the actor.

**Native findings from eight exact pairs:**

- **8/8** pairs had equal **entire event-excluded serialized native World state** at the actual CIL Read boundary, but different full observer-only event histories.
- **8/8** pairs produced **identical full native internal Event records at every one of 117 subsequent paid CIL steps**, not merely the same output sequence. Event timestamps, PCs, origin, opcode effects, credits and observable data match; the post-step event-excluded full world state was also asserted equal **at each individual step**.
- **6/8** pairs had different authentic native output **values** at the checkpoint: correct diagnostic challenge versus wrongly emitted zero; the two `train=-1` controls had true challenge0, so both genuine native outputs had value0 but the **last writer origin still differed**.
- The first paid native `Read(cell11)` in both histories returned the same `challenge`, selected the same **success route PC26**, and retained the old faulty source PC10. **Neither history constructs a novel node38 after convergence**. No code/data differences remain for a CIL-only checker to inspect.

## Independent model and native numeric differential

Four training values `[-1,0,1,17]` × two nonzero unseen values `[-9,23]` = **eight pairs** (16 native worlds). Plus eight unmodified B42 and eight compact immutable correct controls = **32 actual Rust-native worlds**. Every world spends exactly128 CIL credits (4,096 total); external lab interventions are counted in event history but **not** mistakenly reported as paid actor instructions.

| Arm / eight worlds | True/wrong task training service | True/wrong unseen task service | Total CIL instructions |
|---|---:|---:|---:|
| Genuine one-time native diagnostic write, **lab-forced temporary routing** | **16 / 48** | **0 / 104** | 1,024 |
| Wrong native diagnostic Write0, then **external output forgery** | **16 / 48** | **0 / 104** | 1,024 |
| Unmodified B42 `ProbePlus` founder | **30 / 16** | **72 / 26** | 1,024 |
| **STATIC_CORRECT** four-op program | **128 / 0** | **128 / 0** | 1,024 |

The two paired histories are *both* poor subsequent task programs, despite one having genuinely written the correct diagnostic value at its one lab-rerouted task step: the **source was returned to the same faulty state**. A successful native output observed once does not prove a continuing program computes that output correctly. The negative result concerns accessible information about **past events**, not a claim that all future correct service is impossible.

The independent source-parsed model reproduced all eight convergence cases, 117-step equality, six distinct true output values, 4,096 total CIL charges and all four output aggregates. Native CI matched the independent prospective forecast. Failure cases (zero-challenge control, wrong retained programs) were retained, not discarded.

## Proven information boundary and scope

Within the **existing deterministic CL-0 world and the CIL ISA that cannot read the private `World::events()` history**, two distinct host-verifiable causal histories can lead to the same entire currently accessible world state. Therefore any further CIL-only computation with the **same future inputs** will have the same behavior in the two worlds: it cannot recover an absent historical last-writer fact by rereading the same state, no matter how many extra charged instructions are added. In these matched worlds, even the native **world tick** and event count are equal; it is not just an apparent equality at one arbitrary chosen address.

This is a **conditional impossibility** based on current substrate state observability, not a theorem about all digital worlds, all CIL extensions, cryptographic signatures, external observers or future information channels. The experiment's correct native output is itself caused by *temporarily externally rewiring* existing instructions; it is a **laboratory positive control**, never evidence of self-originated innovation. Copying/forking the World to form a matched experiment is also **an external experimental operation, not biological or digital reproduction**.

**CL-1B B3 (cumulative autonomous adaptation): NOT MET. B4 (organizational closure): NOT MET. Cyberlife: NOT ESTABLISHED.** B44 proves an *epistemic barrier* for the existing observation interface; it does not show the actor has acquired its own causal provenance, a self-maintenance drive, novelty generation or effective improvement against the strong static source.

## Consequence for B45 experimental design

Do not keep piling extra same-cell reads and task-specific checks on top of B42. Under this world representation the erased origin information is unavailable; a correct solution requires either **changing the substrate's generic causal information physics** or explicitly treating identity-specific historical causation as unobservable.

A defensible B45 research hypothesis would introduce an *identity-neutral, resource-bounded, read-only historical event observation interface* usable through ordinary paid CIL execution, with **no fitness/repair oracle**. The actor would have to derive any functional decision itself, and a matching no-history, free-history and provenance-corruption control would show whether real task service improves after charging additional compute and storage. It should be treated as a new world-law experiment requiring ABI and compatibility auditing, not as the discovery of life.
