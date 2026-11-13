# CL-1 Study 19 — preregistered external graph-injection calibration

**Registration: 2026-11-13, before implementation and before inspecting Study-19 calibration results.** Studies 01–18 are known. This is **host-side intervention engineering**, not an endogenous ecology, organism, adaptive repair program or new life evidence.

## Falsifiable technical question

Can a researcher restore the **same removed graph object** in a matched experimental branch using an explicitly external, non-CIL mechanism, while preserving deterministic replay and strict resource accounting? Without a working positive control for *the same registered component*, later failure to observe internal functional recovery is scientifically ambiguous.

## Frozen intervention contract

Introduce only a **host intervention** `PutGraph { graph: u64, object: GraphObject }`, inaccessible from CIL. The object contains the exact owner, executable node list and reader permissions. It is an external data injection operation, not a specialized algorithmic `REPAIR`, `FITNESS`, `REPRODUCE` instruction and not an automatic response to lesions.

Preconditions: `graph > 0`; the handle was previously allocated (strictly `graph < next_graph_id`); it is currently absent; owner and every authorized reader process exist; node list is nonempty and does not exceed `max_nodes_per_process`; strict total storage accounting can accommodate graph header, nodes and granted readers. Reject malformed or oversized inputs atomically. A valid injection neither charges nor mints compute credits; it consumes explicitly external storage space under the existing cap. Its provenance is `Origin::External`, fully serialized in intervention schedule/logs and excluded from native restoration claims. It does **not** restart stopped processes or manufacture output.

## Predetermined calibration tests

1. A hand-authored native CIL process constructs and populates a graph; snapshot the object and handle; externally `DropGraph`; externally `PutGraph` the same object and ID. Verify exact structural equality, identical permissions, resource occupancy before lesion and after injection, and unchanged total compute inventory.
2. Reject zero/never-allocated handle, duplicate existing handle, nonexistent owner, nonexistent reader, empty/oversized graph and storage exhaustion. Verify byte-equal simulated state except external rejection event/tick for all rejected operations.
3. Apply the same two-step external drop/injection schedule under `Laboratory::run_epochs`; confirm exact SQLite replay of event, epoch snapshots and final state, with explicit external origin. This is only a **mechanism positive control**; the host-created operation itself never counts as endogenous recovery.

## Scientific interpretation and next dependency

Even if all mechanism controls pass, Study 19 proves *only* that a precise graph-restoration positive-control intervention is available. It does not prove restoring a particular **functional input-output relationship**, because that still needs a separate observer-defined functional assay and an independently preregistered prospective ecology with fresh seeds, sham, zero-credit, randomized-code, single/double component ablations, external restoration and two nonoverlapping postdamage windows. If later functional restoration fails under the external positive control, mark the corresponding internal-maintenance outcome **NOT EVALUABLE** rather than zero. No biological metaphor, survival oracle, hidden fitness calculation, host-driven optimization or positive CL-1 acceptance is authorized.
