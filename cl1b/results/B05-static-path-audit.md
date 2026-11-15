# CL-1B B05 — independent simplified path audit (NOT native Rust CI)

**Date:** 2026-11-15. **Evidence level:** manual independent JavaScript instruction-level path simulation of the *committed test's* 28-node CIL source, after [prospective protocol](../studies/B05-reciprocal-monitors-throughput-paradox.md) and [test implementation](../../cl0/crates/world/tests/cl1b_b05_reciprocal_monitors.rs) commits. This is **not a CL-0 Rust VM execution**, SQLite archival replay, randomized study or independently verified CI artifact. Do not classify as experimental acceptance.

Conditions: single simulated input23, one founder, bootstrap8 interpreted instructions then100 postfork simulated instruction attempts; four manually specified lesion arms. Simulated operations were `Const,ReadNext,Read,Write,Sub,Branch,PatchNext`; the independent simplified interpreter did not enforce all native Rust VM validation, storage limits, detailed event hashes or credit accounting. Native integration tests in the linked Rust file are the required authority.

| Arm | Postfork service writes (all correct, key9) | Native-like repaired edge traces (simulated) | Final monitored edges | Final next PC |
| --- | ---: | --- | --- | ---: |
| SHAM | 5 | none | 9→10,17→18 | 19 |
| A_ONLY | 5 | 9:15→10 | 9→10,17→18 | 23 |
| B_ONLY | 5 | 17:23→18 | 9→10,17→18 | 18 |
| DOUBLE | **10** | none | 9→15,17→23 | 8 |

All arms: two simulated initial checkpoint writes (cell12=10 from PC3 and cell13=18 from PC7), fork PC8. The DOUBLE arm executes fewer monitor instructions per service cycle, which raises apparent output throughput despite losing **both** error-detection/repair pathways. Observed output is not evidence that its *capacity to maintain itself* survives. One lesion at a time leads to an effective source edge restoration by the other monitor under this original scripted architecture.

**Critical caveats:** 5 inputs are in the preregistered test but this simple path audit used one; the Rust test loops all five. Every checkpoint, monitoring rule and lesion is researcher-authored; neither spontaneous self-maintenance nor continuous adaptive evolution is shown. CI status and Rust verification remain **unconfirmed**.

**Methodological implication:** CL-1B should not use raw outputs, uptime, fast instruction cycles or process count as organismal fitness. Add separate (a) immediately observable task performance, (b) latent **repair capacity** under withheld lesions, (c) recovery after new damage and (d) net compute/storage expenditure. The high-output DOUBLE condition is a useful *negative control*, not a positive evolutionary endpoint.
