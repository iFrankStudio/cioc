# CL-1B B34 — resource-paid CIL-visible nonblocking acquisition result and native fallback

**Full native verification PASS:** [GitHub Actions run 43814088352](https://github.com/iFrankStudio/cioc/actions/runs/43814088352), exact tested source SHA `bf26cca97c825d528856baa34cd000ca9395dc8e`. [Machine-readable source-pinned full summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/bf26cca97c825d528856baa34cd000ca9395dc8e-43814088352-1/summary.txt) was fetched from independent CI evidence branch: **34/34 Rust native integration targets PASS, 21/21 source-parsed independent analyses PASS**, the **B31 sensor and B34 TryDraw numeric codec/serde/graph relocation unit tests PASS**, zero failed native/observer targets. Separate evidence publishing job succeeded. Earlier failed native integration runs were retained in the GitHub Actions history as normal implementation errors: an omitted exhaustive opcode match in the first kernel implementation, followed by a legacy unknown-opcode assertion still expecting numeric tag32 to be unknown, and an integration test attempting to call a private numeric encoder. The failing runs are **not** called successful validation.

[Prospective B34 protocol](../studies/B34-nonblocking-resource-feedback.md) · [real native CIL integration test](../../cl0/crates/world/tests/cl1b_b34_nonblocking_resource_feedback.rs) · [independent source-parsed 12-world interpreter](../analysis/b34_nonblocking_feedback_scan.mjs) · [world kernel](../../cl0/crates/world/src/lib.rs).

## What changed in the actual digital substrate

The Rust CL-0 CIL VM has a new **generic append-only opcode32**, `TryDrawCompute {amount,dst,next}`, without renumbering opcode tags0–31. Unlike the previously existing **blocking/deferred** `DrawCompute`, the new opcode **always advances** after a paid attempt when valid, and writes **1 for success or 0 for failure** to an ordinary CIL register. It transfers the requested amount of shared compute to the same process only if sufficient neutral shared pool compute exists; otherwise transfers **zero**. An observer-only event `ComputeDrawTried{requested,available_before,success}` documents the real resource outcome. Every attempt is charged from a **positive local compute balance**; at zero local credits, even a world pool with100 credits cannot pay for `TryDrawCompute`. Only the old ordinary `DrawCompute` has a generic opt-in pool-paid gateway.

This closes one **specific information bottleneck**: B20/B33 showed how the actor could be blocked at `DrawCompute` with no CIL-visible feedback about *why* a previous candidate request did not complete. B34 now provides an **optional, separately paid, nonblocking** attempt whose actual success/failure can be consumed by CIL's branch logic. This is *not* a privileged reward, truth about task success, environmental foresight, a learning algorithm or a zero-credit rescue.

## Frozen finite native experiment (12 deterministic worlds)

Every world loads exactly one process1 with **six initial local compute credits**; pool0 initially, then exactly one externally sourced neutral common-pool packet of **0,4,5 or6** before native execution. The researcher authors three CIL program arms:

- **TRY:** ten instructions. Initial request5 at PC0. At PC2 the native paid `TryDrawCompute(amount5)` reports a true result in register r2; CIL `Branch` at PC3 takes the authored fallback request3 after failure, performs a **genuine charged `PatchConst` on executable node0 (5→3)**, then enters standard pool-payable `Draw3` at PC6. When the first Try succeeds at pool5/6, it proceeds to the native true service loop without editing node0.
- **FORCED_FAIL:** same ten-opcode geometry and local instruction cost through setup; at PC2 a charged `Const r2=0` replaces the actual attempt and mechanically forces the same hardcoded request3 fallback without ever measuring the pool. This exposes whether the added environmental feedback *itself* beats a fixed authored behavior.
- **STATIC3:** compact, preauthored five-opcode service loop requesting3, starting with the same six local credits. This is the important total-cost efficiency baseline.

The service endpoint is an actual native `Origin::Internal(1)` `CellWritten(key9,value1)` at the correct program PC (9 or4). No host mutation, parent-child source code injection, process2 or targeted compute replenishment. Each native charged step reduces total local+pool compute by exactly1; the packet raises that inventory by exactly the amount injected.

| Common pool packet | TRY true resource attempt | TRY true service writes | FORCED_FAIL writes | STATIC3 writes | Unused pool TRY / forced / static |
| ---: | --- | ---: | ---: | ---: | --- |
| 0 | request5 **fails**; native source node0→3 | 0 | 0 | 0 | 0 / 0 / 0 |
| 4 | request5 **fails**; native source node0→3 | **1** | **1** | **1** | 0 / 0 / 1 |
| 5 | request5 **succeeds**; source node0 remains5 | 1 | 1 | 1 | 0 / 1 / 2 |
| 6 | request5 **succeeds**; source node0 remains5 | **1** | **1** | **2** | 1 / 2 / 0 |

For pool4, the **native failure flag** is a genuine causal input to the paid branch and real executable self-edit. It allows the actor to continue useful work even though its first attempted request5 did not fit the actual resource pool. A conventional blocked `Draw5` would be ineligible for a pool-paid attempt when only4 units are available, absent a different local retry design.

**Strong negative result:** The resource-feedback TRY actor has **no service advantage over the same-cost FORCED_FAIL actor in any registered packet world**, and the much smaller STATIC3 program **strictly beats TRY at packet6**. TRY receives true information and uses it to select a preauthored fallback, but that does not make the policy generally efficient or an autonomous adaptation. No candidate fitness estimate from genuine useful work is ever returned to CIL; the only feedback signal is *resource acquisition success*. The source already contains candidate5, fallback3 and the exact edit target.

## CI and VM safety checks

- All old opcode tags preserve numeric semantics; tag31 `SenseComputePool` is still valid and the unknown-tag compatibility test has been explicitly advanced to33. This is a correct **append-only VM schema revision**, not retroactive reinterpretation of existing serialized code.
- Native B34 numeric encoding, decoding, serde roundtrip and edge relocation are checked in required **private kernel unit tests**, not by falsely exposing a private encoder method as public.
- With **zero local credits** at an executable `TryDrawCompute`, a pool of100 cannot pay a step, produce a result or restart the process.
- Invalid destination register at the native step produces a **charged trap**, not a free success flag. Negative encoded operands are rejected.
- The first full source-change CI was red because the new opcode was not yet handled by all exhaustive Rust match sites; the second full run was red because the earlier B31 unknown-tag test had not been updated. Both implementation failures were corrected before the final full 34/21 source-pinned green run. Independent observer-only success was not promoted into a native success.

## Scientific decision

B34 establishes a **new generic substrate affordance** needed for possible adaptive individuals: an actor can obtain **real, CIL-visible outcome feedback at a known compute cost**, *without* making the scheduler declare survival or awarding artificial energy. It is the start of an evidence channel, **not** an organism learning or a functional self-certification solution.

The next hard gate is **repeated, resource-paid candidate evaluation and retention by the same continuing actor**, tied to real functional work, versus strong immutable alternatives on prospectively held-out resource sequences. It must retain negative cases, protect the pool-funded Draw path, and not simply rebrand an investigator-written if/else fallback as evolutionary discovery. **CL-1B B3/B4 and open-ended cyberlife remain NOT MET.**
