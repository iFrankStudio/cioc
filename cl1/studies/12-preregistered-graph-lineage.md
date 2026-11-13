# CL-1 Study 12 — preregistered internal graph lineage provenance

**Registered:** 2026-11-13, before modifying VM provenance or executing any Study-12 run. Study 11's source/positive generic exchange outcomes are known. Fresh fixed independent seeds **1810000..1810095** inclusive; no re-use of earlier cohort seeds or cherry-picking.

## Falsifiable question

Can we attribute the origin of *each successfully created graph and directly copied or edited node* to a concrete charged CIL operation, and distinguish initial externally authored code from information transformed during internal execution? This tests **observer integrity and structural lineage**, not autonomous life, true invention or function.

## Frozen observer-only event extensions

The existing successful `GraphCopied` (graph-to-graph, with source owner) and `GraphTransferred` remain unchanged. For other successful native CIL operations, the single `Executed` event is replaced with:

1. `GraphCreated { graph, owner }` on successful `GraphNew` (zero-node object), with actor == owner.
2. `GraphNodeCopied { graph, owner, source_process, source_node, new_node }` on successful `GraphCopyNode`. The source is a **preexisting process-code node**; if the original process code was externally generated at initialization, this copy is explicitly an internal operation on external information, not autonomous novelty.
3. `GraphNodeEdited { graph, owner, node, edit }` for successful `GraphSetNext` (`edit="next"`) or `GraphSetConst` (`edit="const"`). A successful edit might change a value or write the same value; count an *executed edit*, not effective novelty. Other mutation operations remain generic observer events (not exhaustively attributed in Study 12).
   
Every event retains the existing `Origin::Internal(actor)`, original program counter, one credit per attempted instruction and deterministic tick. Denied and trapped instructions must **never** produce a success event. Events do not enter CIL registers and cannot change its behavior. Existing JSON serialization and deterministic replay must remain valid. This observer audit is **not** a complete data-lineage engine for shared cells, GraphEncode, GraphReplace, or launched descendant process nodes; it must explicitly report those uncovered channels as limitations.

## Fixed experiment

Reuse the **exact generator and interventions of Study 11** except for fresh seeds: three 28-node initial processes, 56 credits/process, 32 shared cells, max 40 nodes/process, strict capacity 192. The initial **nine-node symmetric code prefix is externally authored**, creates/publishes/grants three graphs. Nodes 9..26 are the externally randomized 12-choice continuation specified in Study 11 (LCG multiplier 6364136223846793005/addend 1442695040888963407), node27 Halt. Common prefix: exactly 27 attempted CIL instructions before matched fork. No external compute pool.

Per seed: five matched arms `control`, `sham-drop`, `target-drop`, `disable-owner`, `zero-credit`; **16 independent environment epochs × 12 CIL attempts**, identical initial hash at fork, all null/no-op/trapped worlds retained. **96 × 5 = 480 matched SQLite runs; 7,680 epoch snapshots**, every state replay-verifiable.

## Primary endpoints and prespecified validity

- `initial_graph_lineage`: exactly three pre-fork `GraphCreated` events, one per original process, and three pre-fork `GraphNodeCopied` events from pre-existing process code. All six events are **engineered scaffold calibration**, not emergence.
- `postfork_created_graphs`: count postfork successful `GraphCreated` events in untreated worlds and incidence/96.
- `postfork_original_code_copies`: count postfork `GraphNodeCopied` and record successful events attributed to external-seeded process code. Incidence/96.
- `postfork_graph_edits`: count postfork `GraphNodeEdited` (split by const/next), incidence/96.
- `postfork_intergraph_copies`: count existing `GraphCopied` events with pre-action owners and their source graph ID. This is *information transmission*, and it cannot by itself establish endogenous novelty.
- `structural_interaction_chain`: an untreated world with a successful postfork graph-to-graph copy **and** a successful postfork graph edit of any graph. This measures co-occurrence only, not that the copied code was itself edited, persisted or maintained.
- `nonowner_impact`: target-drop changes final non-target process states versus control (49/96 previously under Study11), recomputed from fresh matched worlds without excluding failures.
- `sham_equivalence`: identical simulated state control/sham; any deviation fails full study.
- `zero_credit`: 0 internal attempted steps after the fork and no successful postfork graph-creation/copy/edit events; violations fail study.
- `provenance_invariants`: successful object and node events carry valid internal actor, owner/graph/indices; postfork new handles cannot predate the common snapshot, and no graph-node event appears on a trapped attempt. Preserve exact event ordering and SHA256 by 480 replay checks.
- Describe counts, 96 independent-world denominators, two-sided 95% Wilson intervals for seed incidence. All epoch snapshots must have valid SHA256, strict capacity ≤192, no discarded worlds.

## Scientific stopping interpretation

Even if internally created objects, copies and edits occur, their **instruction types and initial source code were generated by the investigator**. The experiment does not detect true semantic novelty, independent joint function, reciprocal self-maintenance or internally initiated repair across two observation windows. All those remain **NOT EVALUABLE**, never zero estimates. Any direct proof of CL-1 still requires a separately preregistered multi-component functional maintenance study, including external-repair, sham and time-separated damage conditions. A failed baseline integrity check voids the entire cohort.
