# CL-1B / Study B02 — CIL-native damage detection and self-patching calibration

**Preregistered 2026-11-15 before implementing/observing B02 results.** B02 is an *explicitly human-authored repair-script substrate control*, not evidence that an individual spontaneously invented recovery or that CL-1B's endogenous B2 criterion has passed.

## Fixed question

After an **external** corruption of one active executable graph edge, can a **single still-running CIL actor** detect its own changed instruction edge using `ReadNext`, perform a chargeable **internal** `PatchNext`, and resume its predeclared data-service function? Do the same tests fail if the identical CIL program cannot apply its internal patch?

## Frozen program and environment

Exactly one 12-node native CIL process id1, initially 100 credits, strict world storage budget 64, shared cell address key7 holding input (source), key9 (output). Program is a hand-designed 8-attempt service loop that reads key7 and writes the same value to key9. A guard uses `ReadNext` to inspect code node3's successor, subtracts the expected successor4, branches, and uses `PatchNext` when the observed edge differs. Thus the guard is **external researcher design encoded in the founder CIL program**. A loop iteration with a missing output is still charged as an iteration; logs cannot be used by CIL.

Nodes and flow:
- PC0: set r0=3 (index of protected active read node); 1: r2=7 (input key); 2: r3=9 (output key).
- PC3: `Read r1,[r2]; next4` (damage changes *only* next4→5), PC4: `Write [r3],r1; next5`.
- PC5: `ReadNext r1,node=r0; next6`; PC6: set r3=4; PC7: `Sub r1=r1-r3; next8`; PC8: `Branch r1, nonzero9,zero10`; PC9: `PatchNext(node=r0,target=r3,next10)`.
- PC10: set r3=9; PC11: set r2=7; loop to PC3.

After initial cell initialization, advance the world until exactly one successful service write at PC4, then **six more attempts** (PCs 5,6,7,8,10,11) until process PC3 is next. This pre-fork world is the baseline; it has spent exactly 11 attempts and contains one service output.

## Frozen 40-attempt matched arms

1. **Sham**: externally PatchEdge(node3,next4) no-op. Next 40 attempts → five successful outputs, no native edge repair.
2. **Damage**: externally PatchEdge(node3,next5) bypasses PC4. Next 40 attempts → first output window missing, CIL detects code edge and internally restores 3.next5→4, four successful subsequent outputs, exactly one native patch event.
3. **Damage + patch-disabled**: **same pre-fork program/code and same external patch**, but opt-in VM `with_native_patchnext_disabled()` after the fork (an explicit external world-law intervention). Next 40 charged attempts → zero outputs, five suppressed patch attempts, no recovery.

Input values fixed in advance: `[-13,0,7,23,101]`; exactly **15 deterministic worlds**, 5 inputs × 3 arms. All arms have equal post-fork opportunity (40 attempts), common 100 initial credits, no pool support or birth and no hidden host repair. Founder, external damage and experimental suppression must be separately attributed.

## Fixed endpoints

- Every arm: one process and no child process; exactly 40 post-fork charged attempts; no trap, no unexpected external actions, no resource minting; final local credits and aggregate inventory `100-11-40=49`. Process remains schedulable with next PC3 (this bounded assay is intentionally not naturally terminal).
- `CellWritten(key9,value=input)` with internal origin, PCs and ordering provide service evidence. Pre-fork exactly one valid output; post-fork sham5, damage4, suppressed0.
- Damage: exactly one native internal edge-patch event `node3:5→4`; post-fork read/write service returns after repair. Suppressed: zero edge patches, exactly five charged suppressed PatchNext events.
- Corrupted next edge is never patched by the host after the initial damage event. An observer-side replay is not scored as CIL recovery.

## Interpretation and next science

Even if every assertion passes, this establishes only that **a researcher-written internal safeguard** is expressible in generic CIL and can recover from one **researcher-chosen** damage. It does not demonstrate endogenous repair discovery, rollback of arbitrary corruption, adaptive generalization or cyberlife.

For B2 stage acceptance, later studies must make checkpoint/diagnostic behavior emerge or be reconfigured by native operations, show recovery across held-out lesion loci and resource contexts, preserve net compute/storage costs, use destructive/sham/suppression controls and report all failed cases.
