# CL-1B / B12 — native success marker can diverge from task function

**Prospective registration 2026-11-15**, before B12 Rust code authoring or B12 simulation. Negative mechanistic probe; no spontaneous life, and no B3/B4 scientific acceptance. This is **a newly identified failure of the hand-designed marker criterion** used by B10/B11. Its causal consequences must not be conflated with true task completion.

## Question and new falsifier

B10/B11 trust a nonzero marker written at PC7 as proof that service output PC9 was produced in the same trial. This is only valid under an unspoken invariant about the coupling of service/marker code: **the marker producer and service consumer's control-flow edges remain intact**. Can a single native mutation of the same actor's graph violate that invariant, making feedback **false-positive** (marker produced, output absent), or **false-negative** (output produced, marker absent)? Does the scripted repair rule then cause additional harm rather than recover?

This is a stronger failure mode than *externally spoofing* a cell: both the misleading evidence and real service omission arise **inside the same CIL actor**, without any host injection into marker/output cells.

## Exact frozen founder

Reuse **byte-identical B10** 20-node `founder()` graph and same CIL instruction rules. One process id1, 200 initial credits; strict world capacity64, no pool, input cell7 seeded by researcher. Program uses its own `Delete(cell13)` at PC2, native marker `Write(cell13=1)` at PC7, actual service `Write(cell11=input)` at PC9, and guard `Read(cell13)` at PC11. On missing marker, its researcher-authored algorithm computes current node6.next minus **three**, and applies `PatchNext` to node6. A marker **does not attest PC9 unless native control-flow preserves PC7→8→9**.

## Frozen arms (five input labels × five arms = 25 worlds)

Input labels `[-13,0,7,23,101]`. Every world starts from exactly the same unmodified source and initial credits. Initial host `PatchEdge` only; no host updates to output/marker cells. Run a fixed 80 CIL instruction attempts per world, retain every world, all edits and side effects.

1. **INTACT**: no lesion; marker and service output are generated together; no native edit.
2. **FALSE_POSITIVE_FROM_MARKER_EXIT**: external `PatchEdge(process1,node7,next10)` bypasses PC8/PC9 *after* a legitimate native PC7 marker write. Expected: repeated marker=1 from PC7 with **zero native service writes PC9**, no self-repair, graph node7 stays corrupted.
3. **FALSE_POSITIVE_FROM_OUTPUT_EXIT**: external `PatchEdge(process1,node8,next10)` bypasses PC9 after marker write; repeated marker=1, **zero native service output**, no native edit, node8 stays corrupted.
4. **FALSE_NEGATIVE_FROM_MARKER_SKIP**: external `PatchEdge(process1,node6,next8)` skips PC7 marker writer but still runs PC8/PC9 real output. The B10 guard sees absent marker and attempts to 'repair' node6 to **5** (8−3), though it has not recovered the service coupling. Predict at least one native code edit `node6:8→5` with potential control-flow or resource side effects; log whether later service survives. Do not silently redefine "successful repair" by mere edge movement.
5. **FALSE_NEGATIVE_PATCH_DISABLED**: same node6.next8 lesion with `with_native_patchnext_disabled()`; guard still misclassifies real output, but actual code cannot be altered; **all native service writes remain correct**, exposing that enabling the repair policy may be worse than disabling it for this fault.

## Primary endpoints

- In false-positive arms, CIL PC7 produces legitimate internal `CellWritten(key13,value1)` and later PC11 reads1, yet no CIL PC9 service writes occur. **No host-provided or stale markers.** This conclusively demonstrates that `fresh` is not equivalent to `correct provenance of desired downstream work`.
- In false-negative arms, native PC9 does write correct `CellWritten(key11,value=input)` before the guard reads0 at PC11; in enabled arm the repair opcode must actually change node6.next; in disabled arm, edit attempts are charged but suppressed.
- Resource inventory must match initial200 minus 80 attempts; no pool/minting, no child, no traps. Account for any unintended intermediate loop and report missing outputs rather than filtering failed worlds.
- Exactly count native marker writes, service writes, `CellRead(key13)`, native code edits and suppression events, with `Origin::Internal(1)`, and distinguish the initial researcher `PatchEdge` as `Origin::External`. No after-start external restoration.
- An internal marker is not a protected oracle; both sides of its implication must be tested. Investigators must not relabel a successful marker as a successful service.

## Interpretation and next gate

If the two false-positive arms pass native testing, the **B10/B11 marker definition is insufficient as an independent evaluator of an evolving executable**. A candidate with freedom to edit the evaluator's dependencies can satisfy the evaluator while disabling the real work — a form of evaluator exploitation without autonomous intent. If false-negative allowed edits destroy output whereas mutation-disabled conditions retain it, this is a causal harmful-repair result.

The next strong experiment must test an internally held **independent witness** of the *actual functional transformation*, and then lesion the witness. No hardcoded invulnerable monitor, trusted host-provided reward or output-count fitness may be assumed. Changing the rules from marker-based to a hand-scripted output checksum is a narrow engineering control, **not** B4 emergence. All native CI outcomes remain unverified until recorded with source-pinned artifacts.
