# CL-1B B12 — independent marker/service divergence audit

**Evidence level:** deterministic lightweight interpretation of the frozen B10 20-op CIL program under five preregistered single-node-lesion arms, one tested input23 for path counts. *Not* the CL-0 Rust VM, not source-pinned native CI, not a spontaneous policy, not B3/B4 acceptance. The [B12 protocol](../studies/B12-marker-function-divergence.md) and [B12 Rust integration test](../../cl0/crates/world/tests/cl1b_b12_marker_function_divergence.rs) are committed separately. The native test loops all input labels `[-13,0,7,23,101]`.

Exactly80 modeled CIL attempts, with no after-start host updates to output or marker. Marker and service reports below refer **only to internal CIL writes**; no value was seeded at output11 or marker13. Before starting, the investigator externally patched exactly one executable edge where indicated.

| Arm | CIL marker writes PC7 | Real service writes PC9 | Effective CIL repairs | Final node6.next | Diagnosis |
| --- | ---: | ---: | --- | ---: | --- |
| INTACT | 5 | 5 | none | 7 | both indicators match |
| MARKER_EXIT (node7→10) | **6** | **0** | none | 7 | false positive from native marker producer |
| OUTPUT_EXIT (node8→10) | **5** | **0** | none | 7 | false positive from native marker producer |
| MARKER_SKIP (node6→8) | **0** | **1** | **node6:8→5** | **5** | correct service misdiagnosed; repair introduces harmful loop between nodes5/6 |
| MARKER_SKIP_PATCH_DISABLED | **0** | **4** | no effective edit; 4 paid suppressions | 8 | disabling "repair" preserves service |

### Critical finding

**The endogenously produced marker is not evidence of the desired work unless the semantics connecting marker production to that work also remain correct.** In both false-positive arms the actor writes a brand new success marker from CIL and immediately tests it, yet never completes the promised service. This is *not* an external injection or staleness effect as in B10; the marker is **genuine but semantically disconnected** from function.

Equally, a false negative can be **worse when patching is enabled**. In MARKER_SKIP, the graph still legitimately performs one service, but guard misreads missing marker as failure and computes the hardcoded fix `8−3=5`; this creates an infinite low-value loop on nodes5 and6. Under the patch-disabled control, the actor retains its damaged-but-serviceable path and provides four correct outputs. Strictly: source-level simplified calculation predicts this, while the real Rust VM must verify trap/credits/storage and event/provenance invariants.

### Implications for research design

B10's `fresh marker` fix and B11's candidate selection are **not independent function evaluators under executable mutation**. They may select states that make the marker true while the task is false. The original guard is a researcher-authored criterion, and none of these traces demonstrates purposeful exploitation or emergent cyberlife.

Future protocols must use **causally independent held-out functional checks**, test faults both in *service and checker*, and retain false positive/false negative costs in the aggregate. A checker's own code is mutable and can be damaged: no particular output cell, marker, reward, or host observer may be quietly assumed to be the individual's perfect native fitness oracle.
