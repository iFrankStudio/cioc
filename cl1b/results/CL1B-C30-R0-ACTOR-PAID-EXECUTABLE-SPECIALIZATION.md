# C30-R0 — actor-paid executable loop specialization

Parent f1defcca428efb130e1079d22055a61d27f1d782. **Non-native engineering model.** No new Rust run or B3/B4 attainment. C29 fixed-feature learning path closed pending a materially new representation. C22 and earlier studies already demonstrate paid editing of installed parameters/edges; C30 specifically tests production execution of newly allocated instruction nodes, not a claim of first ever graph construction in CL history.

## Mechanism

Single PID, four registers, input x,y in1..16, coherent for initial construction and both trials and each subsequent service. Initial 89-node source contains only one parent multiplication loop (repeated Add). Read x through paid Read; keep it in cell40. Build a new body backwards from shared output: each iteration CloneNode copies actual parent Add, returns fresh index; paid PatchNext connects clone to accumulated tail; ordinary Add/Sub/Branch update tail and remaining count. Link special_entry to newly allocated head. No preinstalled complete unrolled candidate, no host code generation during actor execution. Generated body has exactly x cloned Add nodes, no loop-count Sub/Branch per addition. Guard literal initially0; positive inputs use parent. Same shared entry/body/output used for parent and candidate trials, bracketed by paid SenseOwnCredits, with temporary guard PatchConst enabling specialized candidate for observed x. Save parent output/span, compare candidate output exactly, reset guard to0 before deciding, compare measured signed span difference via charged subtract/branch loop (bounded by actual cost difference in declared domain). Only strict output equality and cheaper span allow final activation PatchConst (distinct PC). Paid PatchNext at finish routes common output to production path; no per-service trial metrology thereafter. Every production reads live input and guard falls back to parent when x differs. Returning x reuses generated chain. This is runtime specialization with authored transformation and guard, not free-form algorithm invention, no arbitrary-equivalence proof, no active source reclamation or adaptation to different x. Fall back via branch, not physical source rollback after deployment. Incorrect candidate negative control deliberately generates x-1 nodes (x>=2); checks reject it and production stays parent. External observer checks every production multiplication independently. No host truth provided to actor. Fault test does not prove arbitrary code safe.

## Controls

Same-source activation on/off: construction, candidate trial temporary activation, reset and output patch all actually executed; only final activation PatchConst physical effect suppressed in off, still charged. Kernel already exposes PC-local with_native_patchconst_suppression; do not suppress PatchNext globally, which would break generated chain construction. Model checks identical full preactivation state snapshots (PC/registers/cells/source), actual clone execution during trial in both, and actual execution during production on only. Plain baseline no generation or guard; guard-only baseline no generation, retains inactive guard, separate source. Complete lifetime from start through external equal number of output11 writes, no actor horizon knowledge. All model instructions cost1; strict 16 cells,200 nodes,220 total,20k private credits. Fault controls separate source, not same-source causal effect. Model capacity accounting only; native replay required.

## Results

7 initial x values1/2/3/5/8/12/16, y3; horizons12/48/96. Stable x with y changed at service4; shift x permanently at4; return x at24 for longer horizons. 56 selected schedules×4 arms plus3 deliberately faulty generators=227 model worlds. No blind or population claim. Compare same output service. Positive net denotes baseline paid minus on paid. All negative cases kept.

| family | cases | physical on/off saving | net vs plain | net vs guard-only | wins vs plain | max nodes | peak |
|---|---:|---:|---:|---:|---:|---:|---:|
|all|56|25568|3320|12896|17|105|116|
|stable|21|14664|6636|9912|9|105|116|
|shift|21|1128|-6900|-3624|0|105|116|
|return|14|9776|3584|6608|8|105|116|

{"worlds":227,"paid":516456,"services":12840,"newNodes":772,"productionCloneSteps":12784,"matchedPrefixes":56}

Stable8/96 costs2173 vs plain3169, off3709. Stable16/96 costs3101 vs plain5473, off6173. Stable1/12 costs269 vs plain145, despite local trial cheaper. Shift8/96 costs5853 vs plain5377, off5917. Thus initial local advantage does not guarantee lifetime gain; 17/56 win vs plain. Conditional total positive for selected mix, not robust economy. Every on activates, fault arms all reject because output differs. Physical activation savings 25568 do not establish generation's net value by themselves because off pays all generation and guard costs. Generated code remains allocated on rejection/inactive guard (no node removal instruction). Peak116; no capacity change. Complete paid model sources and machine-readable costs/edits provided. Full output and final source captured by reproducible script.

## Next gate

Freeze exact source arrays, fresh diverse input schedules (including x/y variation, short/long residency and faulty generation) and original calibration cases. Native Rust single-PID run using actual CloneNode, PatchNext, PatchConst, SenseOwnCredits and PC-local final activation suppression. Independent per-instruction replay reconstruct appended source nodes, verify actual generated PC execution and all output products, actual capacity/credit conservation, original artifact preservation. Keep no-generation baseline, same-source activation off, fault controls. Distinguish successful fixed-rule specialization from endogenous search of transformation rules, dynamic admission, physical rollback/reclamation and organizational closure. B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c30_executable_specialization.mjs`. Executed JS model in connector; native not executed.
