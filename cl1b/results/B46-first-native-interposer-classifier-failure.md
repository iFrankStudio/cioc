# CL-1B B46 — first native CI failed: PC42 observer misclassified native clone as interposer

**Immutable first-failure evidence, 2026-11-17**, exact tested study source SHA `37584bc5213e9cfb2776af41459ca41e0e73bc46`, [GitHub Actions run43909526141](https://github.com/iFrankStudio/cioc/actions/runs/43909526141). [Source-pinned run summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/37584bc5213e9cfb2776af41459ca41e0e73bc46-43909526141-1/summary.txt) · [B46 original native failure log](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/37584bc5213e9cfb2776af41459ca41e0e73bc46-43909526141-1/cl1b_b46_same_writer_race.log) · [original independent source-parsed 56-world model](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/37584bc5213e9cfb2776af41459ca41e0e73bc46-43909526141-1/b46_same_writer_race_scan.json).

**Full native job FAIL**: **45/46** registered native integration targets PASS, **33/33** independently executed B42-source-parsed CIL observer scans PASS, B31 and B34 private ABI checks PASS. The sole failed native target was `cl1b_b46_same_writer_race`. Its imported B42 source test passed; the new B46 test panicked at `cl1b_b46_same_writer_race.rs:160` on the assertion that every `CellWritten(key11)` at **PC42** must originate from the two **researcher-inserted post-read Write** arms.

### What failed (test observation classification only)

The experiment included the original B45 guard as an essential comparator: B45 starts with **42** source nodes and uses native `CloneNode` to append its first newly assembled service writer at **node42**, which emits a *genuine PID1 `CellWritten(key11)`* during its repair. B46's interposer arms instead start with **43** source nodes and use the node42 as the researcher-preinstalled one-time post-read operation. The native B46 logger incorrectly matched on `event.pc == 42` **without also checking `has_interposer(arm)`**, treating the genuinely cloned native B45 comparator writer as a B46 probe interposer.

This is **not** evidence that native `ReadLastWriter` was bypassed improperly, not evidence of spontaneous attacker behavior, and not a reason to alter the frozen seven-arm outcomes. The failing assertion was about **event classification**, not the VM physics or policy algorithm.

### Minimal proposed correction and evidence threshold

Change the native logger's event-match guard from `event.pc == 42` to `event.pc == 42 && has_interposer(arm)`. It already records the arm label and now requires both the address and the preregistered source regime. Do **not** change the CIL programs, environmental interventions, exact 56-world denominators, seven frozen 13-integer aggregate rows, independent model or B45/B46 result claims. A **new exact SHA-pinned full native CI must pass** before promoting B46 from model forecast to native result. This failed run is preserved as primary evidence, not rewritten into a pass.
