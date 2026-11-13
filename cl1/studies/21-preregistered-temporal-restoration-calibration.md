# CL-1 Study 21 — preregistered temporal loss-and-restoration functional calibration

**Registered 2026-11-13 before implementing the Study-21 runner or inspecting its results.** This is a **calibration fixture**, not an independent stochastic ecology or evidence of autonomous life. Prior Studies 01–20 remain separate, unchanged and non-pooled.

## Falsifiable research-instrument question

Does a prespecified two-graph input/output function first **fail in an observed execution window after actual component removal**, and only then return in two subsequent nonoverlapping windows when the researcher explicitly restores the exact lost graph objects? Can this be verified without ever restarting a trapped process, creating a native repair opcode, or conflating scheduled external probes with internal self-maintenance?

## Frozen common prefork

Three externally loaded **independent but researcher-authored reader programs** (process IDs 3,4,5) serve as one-shot probes for epochs 0,1,2, respectively; each program initially has zero compute credits, and writes to a distinct shared cell 90,91,92. Each uses the 9-instruction `Const 1; Const 2; Const 0; GraphReadTag(graph1,node0); GraphReadTag(graph2,node0); Add; Const output_address; Write; Halt` pattern. They are **measurement apparatus**, NOT replicas, regeneration or descendants.

Two other externally authored CIL programs (owners 1 and 2) are loaded with nine compute credits each. Both create one nonempty graph object with one copied native CIL instruction, and grant read access to *all three* probe processes. Owner1 copies its node0 `GraphNew` (tag 11) and owner2 copies its node1 `Const` (tag 0), using only normal native CIL. A common 18-instruction prefork generates graph handles 1 and 2, each owned by its creator, one node long and readable by IDs 3–5. The two owner processes then have zero credit. All initial source code and code choice are investigator supplied; no novelty claim.

World constraints: strict world, shared-cell cap 4, max process nodes 16, **total storage cap 96**, total compute inventory checked. All subsequent arms fork from the **identical serialized prefork world**, and apply only explicit external `Intervention` operations. Three epochs of 12 maximum charged CIL attempts, with exactly **three independent probe opportunities**; when a probe fails it traps and is never restarted. Each epoch's readout is whether **that epoch's designated probe** successfully emits `CellWritten(key=90+epoch,value=expected)` and stores the same value.

## Fixed eleven paired arms (no random seeds)

The experiment uses exactly **11 matched conditions**, **33 independently hashed post-epoch SQLite snapshots**. Each arm receives `SetCredits(probe3,9)` at epoch0, `SetCredits(probe4,9)` at epoch1, `SetCredits(probe5,9)` at epoch2, except `zero-credit` which gets none. There is no external process restart or script modification after prefork.

1. `control`: no lesion, outputs [11,11,11].
2. `sham`: rejected `DropGraph(0)` at epoch0, outputs [11,11,11], same simulated state as control.
3. `drop1`: externally remove graph1 before epoch0, outputs [absent,absent,absent].
4. `drop2`: remove graph2, outputs [absent,absent,absent].
5. `drop-both`: remove both, outputs [absent,absent,absent].
6. `restore1`: remove graph1 before epoch0, **observe failed probe3**, then externally `PutGraph(1, exact_original1)` before epoch1; outputs [absent,11,11].
7. `restore2`: analogous graph2 restoration; outputs [absent,11,11].
8. `restore-both`: remove both before epoch0, externally inject *both exact original* graph objects before epoch1; outputs [absent,11,11].
9. `mutate2`: remove graph2 before epoch0, then externally inject graph2 with node0 replaced by Halt (tag 19) before epoch1; outputs [absent,30,30], showing content-dependent return, not mere handle presence.
10. `no-repair`: remove graph1 before epoch0 and never restore; outputs [absent,absent,absent]. Distinct delayed-negative-control label.
11. `zero-credit`: no lesion, no funded probes; outputs [absent,absent,absent], no internal CIL after prefork.

Researcher must not retune expected outputs, node tags, credits, intervention schedule, epoch length, or number of arms after inspecting results. A failed external injection, missing functional loss in epoch0, missing restored output in either epoch1 or epoch2, sham difference, unexpected internal write, any extra internal execution on a zero-credit branch, or unequal prefork hashes fails the **mechanism calibration**. Rust full epoch and event exact replay, independent Python read-only audit of SQLite initial/actions/snapshots/provenance and strict CI are mandatory.

## Scientific boundary

This test only demonstrates that **external restoration after an observed loss of an *externally prescribed* graph-dependent function** is measurable. It cannot demonstrate autonomous recovery: the researcher programmed separate readers and chooses when to grant probe credits, and all reconstructed graph nodes are injected by an external host intervention. `PutGraph` success is never a `CellWritten` by CIL or self-generated code. No homeostatic target, hidden optimizer, native `REPAIR`/`FITNESS`/`REPRODUCE` opcode, host-selected adaptive mutation, or artificial process restart is introduced.

Any future genuine CL-1 claim requires independently preregistered *new* stochastic seed-worlds, source-verified endogenous multi-component assembly and dependence, paired ablations with intact/sham/zero-credit and external positive controls, and two late functional windows with **internally executed and causally attributable restoration**. One mechanism fixture with 11 arms provides exactly **0 independent ecological worlds**, regardless of outcome.
