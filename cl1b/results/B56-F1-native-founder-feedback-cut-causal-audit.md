# B56-F1 founder feedback-cut native causal audit (2026-11-17)

## Exact-source evidence
- Source SHA: `6a1e839aabd3ab198f88bb4c8efb87b2d55a93c3`.
- [Push workflow 44185647242](https://github.com/iFrankStudio/cioc/actions/runs/44185647242), completed **success**; full 59/59 named Rust integration targets, 45/45 independent observers, B31/B34 ABI, zero failed counters.
- [Immutable result](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/6a1e839aabd3ab198f88bb4c8efb87b2d55a93c3-44185647242-1/result.txt), [full summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/6a1e839aabd3ab198f88bb4c8efb87b2d55a93c3-44185647242-1/summary.txt), [original B56-F1 Rust log](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/6a1e839aabd3ab198f88bb4c8efb87b2d55a93c3-44185647242-1/cl1b_b56_f1_founder_feedback_cut.log).
- This audit records the already-executed, frozen three-arm founder-cut experiment; it does not claim the prospective two-monitor F1 experiment was executed.

## Native causal contrast (one 1,300-paid-step run per arm)
| Arm | Correct | Wrong | Writes | Feedback reads | Accepted candidate | Final nodes | End storage |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| intact | 173 | 2 | 175 | 4 | 1 | 74 | 83 |
| cut_output_to_feedback | 0 | 182 | 182 | 0 | 0 | 74 | 82 |
| cut_feedback_read_to_search | 0 | 141 | 141 | 141 | 0 | 74 | 82 |

All arms consumed 1,300 paid CIL instructions. The intact source produced one accepted candidate; both cut arms produced none. The second cut **still read feedback 141 times**, but no longer passed it to the candidate-search control path. Therefore the causal difference is not just monitoring cost: disconnecting the existing feedback-dependent search pathway destroys successful adaptation in this paired task. This is consistent with the preregistered direction of effect, but the intervention changes control flow and hence output counts; the study does not identify a pure informational effect independent of routing and schedule.

## Interpretation and boundaries
**Negative founder-origin finding:** the inherited B51c feedback chain is functionally consequential, not an inert decoration. Simply removing it to meet the stronger B56 strict de-novo monitor-origin requirement eliminates the original searcher's ability to retain a correct candidate under this test. A B56-F1 founder cannot simultaneously be described as inheriting the unchanged B51c searcher and lacking that existing feedback capability without a more carefully designed distinction between task-feedback and service-monitoring mechanisms.

The 45/45 independent observers in this full CI are the existing B00–B56-F0 observer suite. **No dedicated B56-F1 full dynamic trace replay is registered**; the F1 test prints only a short trace sample and aggregate counters. Therefore the three-arm F1 native observations are real, but do **not** meet F1's stronger independent per-instruction replay requirement. There is only one run per cut arm; no statistical generalization claim is justified.

## Next discriminating experiment (not yet executed)
Keep the proven task-search feedback path intact as a declared preexisting *training* circuit, but define a stricter, source-manifested novelty claim specifically for a **post-retention service monitoring policy**. Freeze the founder graph and prove the service-monitor output-to-check-to-routing cycle is absent at load; compare (1) actor-paid generated service monitor, (2) identical task search with monitoring-specific edits disabled *after task retention*, (3) feedback-blind monitor generation, and (4) static EVERY/FOURTH. Only then attempt two structurally distinct service monitors and paid within-PID selection, with a new dedicated independent trace replay. Do not upgrade the weaker service-path novelty claim to strict de-novo all-monitor origin.

**N56-A strict origin, N56-B/C, B3/B4 and cyberlife remain NOT MET. CL-1A stays archived.**
