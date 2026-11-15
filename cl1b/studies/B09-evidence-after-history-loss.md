# CL-1B / B09 — reacquiring functional evidence after history loss

**Prospective registration: 2026-11-15, before writing the B09 Rust test or inspecting B09 output.** Human-authored diagnostic program. This is an **engineering calibration**, not spontaneous goal discovery, autonomous maintenance policy, generalized evolution or acceptance of B3/B4.

## Question and scientific target

B08 established a conditional information limit: a deterministic process cannot reconstruct an erased historical distinction from the same current state. But **new, world-local evidence** may support a new functional solution without reconstructing the exact past. Can one **single CIL process**, with no saved edge checkpoint, detect a bypassed service write by trial-running its own code, reading the resulting data and calculating a candidate code-edge change from its **current** edge? How do a missing-cell/valid-zero collision and stale matching output defeat that measurement?

**Critical caveat:** The service-output key, correctness comparison, failure detector, and “decrement edge by one” repair policy are *hardcoded human-authored CIL logic*. The VM supplies only generic `Read`, `Write`, `Sub`, `ReadNext`, `PatchNext`, `Branch`; no host repair or fitness oracle. The program does **not** decide its own objectives or discover the diagnostic algorithm.

## Frozen 19-node CIL source

One process id1, 200 local compute credits, strict storage limit64, no pool. Input cell7 is seeded externally; output cell9 is **absent** except in the explicitly labeled deceptive-stale arm. No actor copy, no child.

| PC | CIL action | next / branch |
| --- | --- | --- |
|0|Const r0=7 (editable code node)|1|
|1|Const r2=7 (input key)|2|
|2|Const r3=9 (output key)|3|
|3|Read r1 from cell7|4|
|4|Const r0=7|5|
|5|Const r2=7|6|
|6|Const r3=9|7|
|7|Read r1 from cell7; **next8 originally**|8|
|8|Write cell9=r1 (service)|9|
|9|Read r2 from cell9 (missing cell reads as zero under existing VM)|10|
|10|Sub r2=r2-r1 (observed minus intended result)|11|
|11|Branch r2 nonzero12, zero16|—|
|12|ReadNext r1 from node7 (read own executable edge)|13|
|13|Const r3=1|14|
|14|Sub r1=r1-r3 (candidate target = **current edge − 1**)|15|
|15|PatchNext node=r0, target=r1, next16|16|
|16|Const r2=7|17|
|17|Const r3=9|18|
|18|Const r0=7|7|

After the initial setup, the actor loops via PC7, and observations are read from an ordinary world cell. On an external lesion `PatchEdge(process1,node7,next9)`, PC8 service write is bypassed. On nonzero input and initially absent output cell9, reading zero allows the actor to detect the missing result, compute candidate8 from observed next9−1 and restore the edge internally. No frozen reference to the original8 is stored in data. The hardcoded subtraction-by-one is nonetheless a hidden *human hypothesis about neighbor code*, not general search.

## Frozen 5-arm matrix × 5 input labels = 25 native worlds

Inputs `[-13,0,7,23,101]`. Every world starts with an external `Process::new(1, source, 200)` and an external `SetCell(7,input)` before execution. Each arm is run for **exactly 72 charged CIL attempts** after these declared initialization interventions.

1. **INTACT:** original PC7.next8, no output preset. Valid service write must occur at PC8 and be read back; no effective internal patch.
2. **BYPASS:** external `PatchEdge(node7,next9)` and no output preset. For **nonzero** inputs, the first readback sees zero and differs from input, then native `ReadNext` + `Sub` + `PatchNext` must restore node7.next9→8. For **input0**, missing output=0 is indistinguishable from a valid zero, so **no repair**.
3. **BYPASS_PATCH_DISABLED:** same bypass and input, but opt-in `with_native_patchnext_disabled()` prior to run. For nonzero inputs, actor must try to patch but the native edit is charged and suppressed; zero input produces no patch attempt. No effective repair.
4. **BYPASS_STALE_MATCH:** external `SetCell(9,input)` before execution and same bypass, deliberately supplies a **stale output equal to expected input**. Because readback matches, the actor must not repair even on nonzero input; no actual PC8 service write may occur. This is an explicit external confound, not valid CIL evidence.
5. **BYPASS_STALE_WRONG:** external `SetCell(9,input+1)` and same bypass. First readback differs so actor must attempt correction for **all inputs**, including zero, since stale output != expected; with patch enabled the native self-edit must occur and later PC8 service writes resume.

The machine's past is not known to this candidate. Its selection objective and neighbor target are provided by the author, and the external preseed acts are externally traceable. None of these seeds demonstrates spontaneous “search”.

## Predeclared endpoints

- Explicit `Origin::Internal(1)` on any future `ExecutableEdgePatched`, and on all native `CellRead`/service `CellWritten` events. No `GraphLaunched`, no other process, no trap, no host action **after** the declared before-run interventions.
- At 72 attempts, final compute credits = **128** and total inventory128 in all arms (same opportunity; NOT equal correct outputs). Starting code graph unchanged between arms except explicitly externally patched lesion, never replaced by Rust in the test.
- Exact final code edge8 when repair occurs, edge9 when repair fails; no native effective edits in INTACT, zero-input BYPASS, STALE_MATCH or PATCH_DISABLED; exactly one effective native repair for nonzero BYPASS and all STALE_WRONG inputs. For PATCH_DISABLED, at least one charged suppress event when nonzero.
- Service writes originate only at PC8. INTact emits at least one correct service write, repaired BYPASS/STaleWrong eventually resumes correct service writes; all unrepaired bypass conditions emit **zero** correct native service writes.
- The first readback after bypass must come from CIL PC9 and return zero when output cell9 absent, or exactly the researcher-provided stale value. Record the **stale/zero blind spots** without excluding them from denominator.
- All 25 worlds must remain schedulable to 72 opportunities, with exact provenance and resource accounting. The lab is forbidden from using prior event logs as CIL-visible oracle.

## Scientific acceptance boundary

Even if all native tests pass, B09 is **engineered local functional feedback**, not individually originated evaluative criteria. It demonstrates only that a previously erased exact edge can sometimes be replaced by computing a functional candidate from **fresh native observations**, while readback ambiguity or stale observations can defeat that rule. A stronger follow-up must replace author-fixed rule `next-1` with multiple **agent-constructed** candidates, resource-accounted trial-and-rollback and a future/held-out fault protocol. No pure performance score or short-term output increase is evidence of cyberlife.
