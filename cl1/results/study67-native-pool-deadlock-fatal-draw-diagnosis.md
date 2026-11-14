# CL-1 Study67 — exact historical cause of no W3 work despite native resource uptake

**Status:** source-pinned CI-PASSED RETROSPECTIVE diagnosis, **ZERO new independent genomes**. Not native self-maintenance or life.

## Frozen historical source and independent verification

- Registered AFTER Study66 prospective zero result and BEFORE the new resource-failure classifier, in `cl1/studies/67-registered-retrospective-resource-termination-cause.md`, commit `821bc3b76e526d84fe78a585e708de838b7bdb34`.
- Original immutable Study66 source revision **`156fd52bea389296b0c9a3b72e4bf8c737e30cf2`**, from [CI43568283297](https://github.com/iFrankStudio/cioc/actions/runs/43568283297), reused **256 original independent genomes**, eight arms each, **2,048 original runs and 49,152 original epoch hashes**, not new random trials.
- [CI43568790315](https://github.com/iFrankStudio/cioc/actions/runs/43568790315) **SUCCESS**; artifact `cl1-study67-original-resource-termination-audit`, ID **14103289439**, full read-only retrospective JSON. The auditor first reran the complete original Study66 independent source-generator, CIL event/credit ledger, intervention schedule, and SHA256 audit, then independently reclassified every original W3-entry actor and shared pool state.

## The two mutually exclusive observed failure mechanisms

| Exact original W3 entry condition, out of each 256 source worlds | Ambient input3 | Ambient input8 |
|---|---:|---:|
| Unstopped original actor with **0 local credits**, positive common compute pool | **207/256** | **207/256** |
| Original actor **permanently stopped after native DrawCompute insufficient-pool trap**, but shared pool positive later | **49/256** | **49/256** |
| Runnable actor at W3 start | **0/256** | **0/256** |
| Any native activity W2 | **1/256** | **1/256** |
| Any native activity W3 | **0/256** | **0/256** |
| Worlds with at least one successful native DrawCompute at any earlier point | **152/256** | **155/256** |

Across both intact ambient arms, original first fatal traps were **exactly 49/256** `DrawCompute` attempted during transient insufficiency of *then-current* shared pool (later ambient inflow is exogenous and cannot revive a permanently stopped actor). Remaining **207/256** actors never trapped/halted; they consumed their credits and became unrunnable despite **positive** shared pool. At W3 entry the inventory was not exhausted. Repeated external +8 input at epochs1..23 continued to inflate the dead pool even after process inactivity, but never provided actor-specific wakeups.

## Controls

- `closed-low`: **152/256** fatal insufficient-pool Draw traps and **104/256** nonstopped zero-credit with empty pool; `closed-high`: **155/256** fatal native Draw trap and **101/256** zero-credit empty-pool actors. No successful native Draw, no W2/W3 activity in closed arms.
- `ambient-draw-ablated-low/high`: all **256/256** actors remained unstopped zero-credit with positive pool, none native Draw, none W2 activity. This identifies access rather than total energy as the limiting substrate.
- `ambient-read-ablated-low/high`: **57/256** stopped with positive pool, **199/256** unstopped zero-credit and positive pool; 162/256 had successful native Draw and 6/256 briefly ran W2 but W3 was 0.
- **Source-dependent functionality already existed early**: intact ambient paired input3/input8 native source-tainted output occurred in **69/256** worlds in W1 but only **1/256** in W2, zero in W3; this is retrospective functional exposure, not the prospective W2+W3 Study66 primary. The paired W1 sensor-dependent output is based on original native `CellRead(key7)`→r0 register provenance→`CellWritten(key9)` across actual low/high worlds, not external sensor initialization alone.

## Scientific inference and decision

The Study66 all-W3 null has a **complete observed two-state mechanical decomposition**, not a mystery about initial sensor functionality or need for vastly more random seeds. There are two distinct, independently necessary mechanisms:
1. **Ignition deadlock with accessible shared energy** (207/256), because normal scheduling requires positive process credits before any native `DrawCompute` can execute.
2. **Irrevocable loss of the process on transient pool shortage** (49/256), even if subsequent external supply replenishes the shared pool.

Merely increasing initial/random process seed counts, increasing environmental +8 inflow, or judging W3 charged attempts as an organism outcome does not address the two mechanisms. The appropriate next research is to **prospectively** compare *generic, externally specified world resource-exchange laws* with explicit balance conservation and matching no-credit/no-ambient controls, WITHOUT target-specific host `SetCredits` and WITHOUT claiming VM-guaranteed runnable access as discovered autonomy. Test native source-tainted W2/W3 **function** separately from mere activity and record any kernel-provided resource advance as an engineered substrate opportunity.

The next clean sequence is (i) preregister a small explicitly human-scripted VM physics calibration for resource shortage and zero-credit native Draw access, then (ii) run a NEW, unchanged Study66 random grammar under matched generic resource-law alternatives with independent seeds; (iii) only if eligible native functional closure emerges, move to self-created graph/data organization and eventual damage/repair.

**CL-1 remains OPEN.**
