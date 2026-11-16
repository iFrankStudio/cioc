# CL-1B B45 — native-verified finite provenance sensor, a limited repair of B44's missing information

**FULL NATIVE CI PASS after one documented compatibility-test failure and correction.** Final code and test source SHA `1cc2ba607b554416ba85d043c41082c03ce57a96`, [GitHub Actions run 43905125781](https://github.com/iFrankStudio/cioc/actions/runs/43905125781).

Source-pinned full evidence: [45/45 Rust and 32/32 source-parsed observers summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/1cc2ba607b554416ba85d043c41082c03ce57a96-43905125781-1/summary.txt) · [B45 56-world original Rust trace and test log](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/1cc2ba607b554416ba85d043c41082c03ce57a96-43905125781-1/cl1b_b45_paid_last_writer.log) · [independently executed 56-world Rust-source-parsed CIL model](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/1cc2ba607b554416ba85d043c41082c03ce57a96-43905125781-1/b45_paid_last_writer_scan.json) · [success result/source SHA](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/1cc2ba607b554416ba85d043c41082c03ce57a96-43905125781-1/result.txt).

**Complete scope:** 45/45 CL-1B registered native integration targets PASS; 32/32 independent CIL observers PASS; B31 pool-sensor private ABI and B34 nonblocking-draw private ABI both PASS; no failed native targets or observers, with full CI triggered by actual world-kernel changes. This verifies the specific code source, **not subsequent documentation-only commits**. Rust warnings about unused diagnostic fields do not equal test failures; no broad clippy/format certification is claimed.

## What has actually been added to the world

[Pre-native frozen study](../studies/B45-paid-last-writer-provenance.md) · [real Rust world implementation](../../cl0/crates/world/src/lib.rs) · [B45 native test](../../cl0/crates/world/tests/cl1b_b45_paid_last_writer.rs) · [pre-native source-model forecast](B45-model-paid-provenance-forecast.md).

An opt-in generic **last-writer** record is now part of a native world, separate from the ordinary mutable cell value and separate from the lab-only full event history. Every live tracked data cell costs **two finite storage units** under the existing shared capacity law: one for its value, one for its most recent writer. `Intervention::SetCell` always marks writer **0 (external)**; ordinary native CIL `Write` marks its actual PID; `Delete` frees both units. Ordinary data writes cannot set arbitrary writer values without changing actual writer identity. Neither task success nor correctness enters this record.

Native CIL numerical **opcode33 `ReadLastWriter{dst,address,next}`** consumes a standard instruction credit to return one numerical source tag into a register, for any address permitted by ordinary shared-cell reads. It returns **-1 unknown** where tracking is absent or cell never allocated, **0 external**, or the true positive writer PID. A new observer event `CellWriterSensed` traces actual charged access. World tracking must be enabled before cells are allocated, not retroactively fabricated after data history was erased. Disabled legacy worlds retain their existing value and execution semantics, and serialize without the new opt-in fields.

Native focused controls passed: opcode33 5-word decoder; true **two-unit** cell capacity check and rejected external allocation; charged native Write failure when there is no room for metadata (no partial allocation); Delete frees both units; world serialization and JSON legacy default field absence; unrelated actor PID2's native writer tag **2** read by PID1 with a real charged CIL sensor. The one-actor experiments themselves do not introduce PID2 or reproduction.

## Real native results: all 56 worlds equal the frozen independent model

**Eight worlds per arm** from training `[-1,0,1,17]` × heldout `[-9,23]`. Every world begins with128 internal credits and zero shared pool. 64 training and64 heldout execution opportunities; actual native outputs at cell11 are scored only after the original environment task input has been restored. External forged writes never count as correct native task service.

The original 38-op B42 `ProbePlus` founder remains unchanged in the unguarded controls. In the other arms, an explicitly **investigator-authored** four-opcode guard senses the last writer, compares its raw numeric tag against actor PID1 via ordinary `Const` and `Sub`, then branches to existing B42 native candidate-creation logic or accepts the current path. All CIL instructions and constructed clone nodes are paid. This is authored program control, **not endogenous invention of validation rules**.

| Eight worlds / arm | Training correct / wrong | Unseen correct / wrong | Actual paid CIL | New native nodes | Halted | Paid writer reads | New false success verdicts |
|---|---:|---:|---:|---:|---:|---:|---:|
| **GuardTrackedSpoof** | **24/0** | **96/8** | **1,024** | 8 | 0 | 16 | 0 |
| GuardTrackedGenuine | 24/14 | **78/26** | 1,024 | 6 | 0 | 8 | 0 |
| GuardTrackedEcho | 30/0 | **102/2** | 1,024 | 8 | 0 | 10 | 0 |
| **GuardUntrackedSpoof** | **0/0** | **0/0** | **344** | 8 | **8** | 16 | 0 |
| OriginalTrackedSpoof | 16/48 | **0/104** | 1,024 | 0 | 0 | 0 | **6** |
| OriginalUntrackedSpoof | 16/48 | **0/104** | 1,024 | 0 | 0 | 0 | **6** |
| **StaticTracked** | **128/0** | **128/0** | 1,024 | 0 | 0 | 0 | 0 |

The original B42 program accepts forged before-read diagnostic feedback in six cases irrespective of whether provenance data exists in the substrate **because it never reads that information**. With tracking **and** authored guard, the actor sees the external tag0, executes its previously existing candidate assembly path, clones eight new executable nodes over eight worlds, and improves heldout correct outputs from **0 to96**. But phase-boundary stale reads still yield **8 wrong writes**, and a four-op static copier is more effective (**128 correct /0 wrong**) with much less code/storage/monitoring overhead.

**Do not misclassify `GuardUntrackedSpoof` as robust**: its missing provenance sensor reports -1, its fail-closed authored policy tries both paths and all eight worlds eventually halt after only**344 total real charged CIL attempts**, leaving no unseen task service. The apparently perfect `0 wrong` denominator is actually **0 total task writes**. In `GuardTrackedEcho`, an external write of the *same output0 value* changes last-writer to environment0, leading the guard to repair even though the observed numerical output has not changed. This is an **identity-signal false positive**; the improvement in the original zero-alias corner is not proof that the guard knows what code is correct. In the genuine no-forgery arm, `train=-1` still permits a constant-zero defective code to pass after a genuinely internal native Write0: native own-PID provenance **does not certify functional generalization**.

### B44 full-state epistemic counterexample: precisely what changed

B44 constructed two laboratory-controlled histories with the same complete event-excluded serialized native World state but different last-writer histories. B45's new test reconstructs the same kind of paired history in a provenance-enabled world using ordinary founder code. The two value cells agree, while the new **priced world provenance metadata** reports **cell11 writer1 for genuine native Write**, versus **writer0 for external forgery**. When the lab removes only the private `events` history from each serialized world, the states remain **different**. If it also removes only the newly added `cell_last_writer` map, the serialized states are again equal. Two actual paid CIL `ReadLastWriter` attempts into register1 yield the distinct values **1** and **0** respectively. This is not simulated oracle access to `World::events`: those private logs are still outside the instruction set.

These two histories were deliberately constructed by a researcher, including temporary external source-edge rewiring. They show a specific old-state indistinguishability class **has been separated by newly accessible information**, not an individual that independently discovered authenticity proofs.

## First-run failure, attribution, and correction

The initial full test of B45's implemented native mechanism **failed its complete CI gate**, source SHA `c8d8bb50fcd3bd15e272dd3391b25edee513526e`, [run 43904608723](https://github.com/iFrankStudio/cioc/actions/runs/43904608723). Actual B45 native Rust passed all four tests and all32 independent observers passed, but B31 and B33 native targets plus two B31/B34 private ABI library selections failed, because older tests had assumed `[33,...]` *must* be an unallocated instruction tag.

This is archived separately in [the unmodified first-failure research record](B45-first-native-abi-sentinel-failure.md) and its source-pinned CI artifacts. The fix changed **three ABI test-only future-tag sentinel numbers `33`→`34`**; it did **not** revise numerical tags0–32, native B45 science predictions, world physics or source behavior. The **second complete full CI run succeeded** for all 45 native and32 independent tests; retroactively declaring the first run passed would be invalid.

## Scientific claim boundary

**B45 demonstrates that a narrow part of the B44 information-loss barrier can be overcome by changing generic world physics, paying in storage and instruction count, then executing an authored ordinary CIL guard.** It does NOT demonstrate a generally trustworthy functional evaluator: only last-writer identity is supplied, not correctness or full history; an actor can write its own misleading result and retain its own valid PID; values may change between independent `Read` and `ReadLastWriter` attempts; external history remains beyond access; static correct code still outperforms self-assessment. Neither sensor nor guard was organically discovered by the individual.

**CL-1B B3 cumulative autonomous adaptation: NOT MET. B4 endogenous organization: NOT MET. Cyberlife: NOT ESTABLISHED.** This is a meaningful world-information-engineering improvement, not the scientific end state.

### B46 discriminating next question

Probe **same-PID source self-forgery, two-read interleaving and historical overwrites**, with no per-task privileged correctness check. Distinguish whether any practical paid native validation protocol can discriminate source authenticity from true causal-functional validity when the *origin of an incorrect output is genuinely the same actor*. Compare all added costs and irrecoverable failures against the small immutable correct copier. Avoid recursively adding more authored checks as if their existence were proof of autonomous policy evolution.
