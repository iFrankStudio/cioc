# CL-1B B22 — native single-founder executable relay assembly

**Outcome:** engineered organizational assembly **natively confirmed**, scientific endogenous-organization criterion **NOT MET**. [Prospective B22 protocol](../studies/B22-native-relay-graph-assembly.md), [native Rust differential](../../cl0/crates/world/tests/cl1b_b22_native_relay_assembly.rs), [independent source-parsed model](../analysis/b22_native_relay_assembly.mjs).

## Exact retrieved evidence

- **Native Rust B22 test:** [GitHub Actions run 43729982922](https://github.com/iFrankStudio/cioc/actions/runs/43729982922), source SHA `d028b7603751502416c7923ead56581db2ae225b`, workflow **success**.
- [B22 source-pinned test and observer summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/d028b7603751502416c7923ead56581db2ae225b-43729982922-1/summary.txt): **selected_native_targets=1**, B22 native **PASS**, all nine independent observer scans including B22 **PASS**, no failures.
- [Workflow and source identity](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/d028b7603751502416c7923ead56581db2ae225b-43729982922-1/result.txt).
- **Subsequent complete verification:** [run 43730266859](https://github.com/iFrankStudio/cioc/actions/runs/43730266859), SHA `7738dd944352f3f443235061eb8c11cc28fc49c0`: **all 23/23 native Rust targets and 9/9 independent scans PASS**, with [SHA-pinned archived full summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/7738dd944352f3f443235061eb8c11cc28fc49c0-43730266859-1/summary.txt). See [full B22-era checkpoint](CI-2026-11-15-full-B22-native-verification.md). The earlier targeted B22 pass remains a distinct first validation at its own source revision.

## Frozen experiments and causal comparison

The researcher supplies **one** initial 35-node CIL program with 29 initial compute credits. No other process is host-loaded. The program itself executes 25 charged graph-assembly / child-launch instructions, including five `GraphCopyNodeLocal` and four `GraphSetNext` operations, and pays for transferring the child's initial two compute credits. Both processes consume their remaining bootstrap credits; subsequent neutral pool packets (+3, then optionally +5) are **external** world resources.

| Precommitted condition | Initial externally loaded processes | Neutral pool inflow | Native attempts | Actual parent service writes at PC29 | Outcome |
| --- | ---: | --- | ---: | ---: | --- |
| BUILD_TRANSFER | 1 | 3 then 5 | **37** | **1** | CIL-built child draws4 and transfers1 credit to parent |
| BUILD_NO_TRANSFER | 1 | 3 then 5 | **37** | **0** | Same native construction/launch cost; child PC4 is a same-cost Const |
| BUILD_NO_SECOND_PACKET | 1 | 3 only | **32** | **0** | Actual child exists but cannot access sufficient neutral compute |

**Native evidence distinguishes internal construction from host setup:** graph creation, five local node copies, four explicit next-edge rewires and `GraphLaunched(parent=1,child=2,transferred_credits=2,moved_nodes=5)` all originate from process1. At the helper's real PC4 instruction the Rust test verifies that parent credits rise by exactly one only in BUILD_TRANSFER, rather than relying on generic `Executed` event counts.

**Energy conservation:** each charged native attempt decreases total process+pool compute by exactly1; external neutral pool packets increase total by the packet amount. BUILD_TRANSFER uses all 29 initially available credits plus 3+5 environmental credits = 37 native attempts and ends with zero process and pool inventory. There is no host `SetCredits` rescue and no life-specific VM opcode.

## Mechanistic significance and severe limitation

Compared with B21, the experiment removes **host-loaded helper process** from the *execution history*. A single process can **pay to construct a separate executable graph, create a second native process and establish a resource-transfer relation** using existing general-purpose CIL operations.

But the investigator **already embedded the helper's complete five-opcode source template, recipient PID, transfer quantity and 25-instruction construction procedure** in the founder. Thus the *information content of the helper algorithm did not originate during the experiment*. The experiment shows **native execution of predesigned organization**, not spontaneous organization, adaptive policy search, selection, persistent survival or cyberlife. Indeed, assembly has high one-time compute cost; B20's fixed properly sized single-actor Draw strategy is a simpler alternative under some energy schedules.

## Next decisive falsifier

A valid next experiment must remove the complete dormant helper template or the specific prewritten assembly recipe and show a continuing CIL actor synthesizing or choosing new relationships from ordinary source information, with **prospectively held-out** neutral resource schedules, paid trials and exact resource/provenance controls. A human-scripted graph compiler that simply writes the same template into memory would not satisfy that gate.

Do not promote B22 from an engineered mechanism precursor to CL-1B endogenous organization acceptance. Retain both the native pass and the negative conceptual interpretation.
