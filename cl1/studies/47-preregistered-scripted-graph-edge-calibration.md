# CL-1 Study47 — preregistered **scripted VM mechanism calibration**, not an ecology

**FROZEN BEFORE IMPLEMENTATION. ZERO new independent ecological worlds.** Studies41–46 used randomized native CIL and did not produce a child with successful shared-cell Write, while Study45 documented copied program edges pointing outside their launched graph. This Study47 creates a deliberately human-authored **graph builder** to determine whether the *existing* Rust CIL VM can natively copy a Write, close its local control edge and launch a working child. **This fixture is NOT autonomous digital life, spontaneous reproduction, a repaired organism, or evidence of evolution.**

## Predefined deterministic single-parent fixture

Load exactly one `Process::new(id=1, credits=30)` with the following nine executable nodes into an otherwise empty `World::with_strict_capacity(8,16,64)`. Registers initially zero, no world pool and no external post-load interventions. Native instruction attempts remain charged one credit each. Node list:

0. `GraphNew {dst:3,next:1}` — create a generic empty graph and save its handle in register3
1. `Const {dst:2,value:6,next:2}` — code-node index6 is source
2. `GraphCopyNode {graph:3,source:2,dst:2,next:3}` — copy the existing parent Write at node6 into the new graph; register2 becomes the created graph node index0
3. `GraphSetNext {graph:3,node:2,target:0,next:4}` — use register0 (initially zero) to make the copied node's next edge point to local graph node0
4. `Const {dst:1,value:3,next:5}` — literal child credit transfer amount3, researcher-authored fixture, **not earned**
5. `GraphLaunch {graph:3,credits:1,dst:2,next:8}` — launch generic graph into child; child writes must have actual `Internal(child)` provenance
6. `Write {address:0,value:3,next:7}` — copied source instruction, NOT executed by original parent
7. `Halt`
8. `Halt`.

This explicit builder is a **calibration only**. It intentionally specifies a working child function and therefore cannot satisfy natural digital-life discovery criteria.

## Frozen three fixtures, 8 epochs×8 native attempts each

1. `native-local-edge`: exact program above. Expected: native GraphLaunch parent1 creates child2 whose executable node0 is `Write(next0)`; child performs at least one successful native `CellWritten(key0,value0)` with `origin=Internal(child2)`.
2. `copied-stale-edge`: only replace parent instruction3 with `Const {dst:2,value:0,next:4}` (no native GraphSetNext). Expected: native launch may create child2 with single `Write(next7)`; child cannot execute its copied Write and traps with `invalid edge` without any native child CellWritten.
3. `zero-child-credit`: exact working edge-closure code, but instruction4 is `Const(dst:1,value:0,next5)`. The child must not produce native shared-cell writes; a zero-credit actor cannot execute even a valid Write.

All three inputs are **different human-authored initial programs**, so they are not independent randomized worlds or a statistically matched population. Store every initial state, input program, native event, 8 per-epoch SHA256 snapshots, exact Rust `Laboratory::verify_epoch_replay` result, and source revision in SQLite; run independent Python event/PC/source/credit accounting. Native parent graph/code operations must not be misclassified as externally repaired, even though the **initial builder logic** is human-authored.

## Acceptance of only the mechanism, not life

Native VM mechanism calibration passes iff positive executes a valid source-provenance child shared-cell Write and both negative controls have no child native Write; all credits conserved, no post-load host actions, all 24 snapshots hashed, and child program graph edge closure/trap reason validated. A fixture mismatch means the assumed mechanics are wrong and must be debugged; no success should be invented. Preserve outputs and experimental limits.

**Scientific CL-1 acceptance remains OPEN.** An actual offspring/repair result requires new *randomized, separately prospectively preregistered* CIL ecologies demonstrating unprogrammed functional construction, native maintenance under damage, and absence of targeted host repairs/starters.
