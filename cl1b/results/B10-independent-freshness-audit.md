# CL-1B B10 — independent instruction-level audit, not native CI

**Evidence category:** independent lightweight interpretation of the [frozen 20-node CIL program](../../cl0/crates/world/tests/cl1b_b10_freshness.rs), all five inputs `[-13,0,7,23,101]` under five conditions (25 simulated worlds). The [pre-registration](../studies/B10-native-freshness-and-spoof.md) was committed before the test or simulation. **This is not a Rust VM run, SQLite exact replay, or native acceptance.**

Under each condition exactly80 CIL instruction attempts were simulated. No candidate agent generated the repair algorithm: site6, edge-offset3, marker value1 and the very definition of success were human-programmed. External actions are kept distinct from CIL activity.

| Arm | Native-analogue service writes at PC9 | Native-analogue marker writes at PC7 | Real code edit | PC6.next after 80 | Key caveat |
| --- | ---: | ---: | --- | ---: | --- |
| INTACT | 5 | 5 | none | 7 | routine still uses a scripted diagnostic |
| DAMAGED | 4 | 4 | one `6:10→7` | 7 | first trial detects absent marker |
| STALE_BOTH | 4 | 4 | one `6:10→7` | 7 | actor's own `Delete` discards externally preseeded matching marker |
| DAMAGED_PATCH_DISABLED | 0 | 0 | none (4 suppressed attempts) | 10 | edit is causally required for restoration |
| POST_DELETE_MARKER_SPOOF | 3 | 3 | one `6:10→7` **later** | 7 | forged marker in first trial causes false acceptance and delays recovery |

The output counts and patch sequence are identical across all five input labels, **including input0**: a nonzero actor-produced marker distinguishes missing output from a legitimate zero-valued output *when there is no active spoof after reset*. In the timed-spoof arm, the host injects marker13=1 exactly after the actor's PC2 Delete and damaged PC6→PC10 route, but before PC11 reads it. That initially fools the scripted guard. At the next unspoofed iteration the guard detects missing marker and repairs. This proves a narrow threat-model distinction, not general authentication of evidence.

### Main scientific lesson

A self-repairing digital individual must manage not only resource-bounded code changes but also its **evidence provenance**. Clearing an observation channel before a trial is a meaningful CIL-expressible freshness action, yet it is not a cryptographic source proof or protection against a hostile writer during the test. More markers or copies alone cannot solve this indistinguishability. A future B11 test should test competing, agent-created probe sites, multi-round retention and held-out adversarial corruptions. Until a native Rust test succeeds with inspectable artifacts, all numerical outcomes here remain *path audit only*.
