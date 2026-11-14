# CL-1 Study74 — prospective random native feedback × zero-credit pool-paid Draw causal ecology

**Status: PROSPECTIVE, NOT EXECUTED at registration.** Freeze this exact scientific protocol before implementing the new runner, inspecting any new seed outcomes, or changing the CIL/VM policy. CL-1 remains OPEN. This is **not** a design for a repair/reproduction opcode or preloaded organism.

## Known prior results

Studies68,69,71 establish source-sensitive Read→Write in W2+W3 for random broad-backedge CIL under 300 initially endowed actor credits (Study71 **63/256**, short0/256). Stage72 establishes actual within-world response to external sensor cell7 change3→8 in **78/256** broad-backedge, short0/256, while still endowed with300 fixed actor credits. Study73's separately preregistered random native feedback/Draw ecology on128 *other* seed genomes, 64 initial actor credits and ambient+8 shared credits/epoch, produced primary W2+W3 source feedback+native Draw **0/128** broad and short; **82/128** broad actors did execute native Draw early but 127/128 broad-low worlds still ended with positive pool/zero runnable actor. Existing Stage73 **human-authored six-mode calibration** independently proved a generic, disabled-by-default VM `pool_paid_draw` world law lets a zero-credit actor with an eligible pending native `DrawCompute` pay exactly one native attempt charge from the world pool, draw credit, then produce native Write; this is **researcher-provided physics**, not native energy creation or organism repair. [Calibration CI](https://github.com/iFrankStudio/cioc/actions/runs/43580314222).

## Frozen fresh independent genome sampling

- Exactly **128 fresh original seed genomes 5,000,000..5,000,127**, every one retained; one-time SplitMix64 mixing and unchanged Study73 native 64-bit LCG seven-way24-node random CIL grammar on PCs3..26, fixed generic register/port setup PCs0..2 (r1=8, r2=7 sensor, r3=9 output), `Halt` PC27. Broad vs short nonzero `Branch` backedge is exactly Study73's alternative; `Read` / `Write` / `Add` / `Sub` / `Const` / `Branch` / `DrawCompute` weights and additional RNG draws unchanged. **No prewired cyclic tail**, no selected functional genome, no hand-authored read→write loop, no existing positive-seed reuse.
- Every original process starts with exactly **64 researcher-endowed local credits**. Researcher-supplied sensor cell7 is3/8. Ambient environment has **96 initially endowed world-pool credits plus +8 EXTERNAL to WORLD POOL each epoch1..23**, no after-initial `SetCredits`, `PatchEdge`, process-specific environment rescue or conditional fitness/refill.
- `pool_paid_draw = false` old scheduler or `true` new generic policy. The entire CIL opcode graph and initial RNG state must be byte-identical between on and off for each seed/topology/source. Enabled policy allows **only** a zero-credit actor whose next executable native CIL instruction is `DrawCompute` to pay that one attempted instruction from existing world-pool credit; the subsequent CIL Draw of amount r1 must be successful or trap normally; native attempts are all one-credit charges and no attempt creates compute. Explicit `event.pool_paid_attempt` proves environmental entry fee provenance; no claim of unassisted spontaneous energy creation.

## Exactly 16 matched arms per original genome

Eight intact ambient primary arms: `{broad,short}-ambient-{on,off}-intact-{low,high}` (2×2×2=8).
Four broad-code ambient Draw-disabled controls: `broad-ambient-{on,off}-draw-ablated-{low,high}` (2×2=4), where a random Draw choice becomes `Const(r1,8)` with identical next/LCG, no native Draw; source input and rest unchanged.
Four closed-pool negative controls: `{broad,short}-closed-on-intact-{low,high}` (2×2=4), same native code, no world-pool endowment or ambient input; any Draw with insufficient pool traps ordinarily.

All 16 arms branch from exactly the declared common original genome/port, differing only in registered topology, policy, source value, Draw ablation and environmental pool. **24 epochs × 12 native attempts = 2,048 complete matched runs and 49,152 original SHA256 snapshots.** No extra exploratory arms or filtered seeds. `World::with_strict_capacity(4,32,64)`, kernel and numeric CIL ABI unchanged.

## Registered prospective endpoints

**Primary A — late information-feedback + native resource acquisition beyond initial actor endowment.** Under `broad-ambient-on-intact-low/high` matched source pairs, count original genomes/128 whose native process executes source-sensitive `CellRead(key7)`→tainted `CellWritten(key9)` in **both W2 epochs8..15 and W3 epochs16..23**, with a newly executed native Read within each respective window and matched high−low written value exactly +5 at the same writer PC, and a **successful internally attributed DrawCompute in W3 earlier than the W3 output**, and an actual W3 Write after cumulative actor attempts exceed its original 64-credit endowment. Report the same for off-policy and broad Draw-ablated, plus short on/off. This is an **intermediate coupled native behavior**, not a living organism. If no trial meets all conditions, report zero and do not relax W3 or resource-gate definition.

**Primary B — native scheduler accessibility/continuation under shared pool** (mechanistic physics endpoint): worlds/128 in broad-ambient-low with **any charged native attempt in W3**, on vs off, paired discordance and Wilson95. The strict native PC/current-credit event ledger must distinguish pool-paid Draw from ordinary positive-credit instructions and explicit host actions. Directional evidence requires `on_only > off_only` AND at least one valid pool-paid native Draw attempt before the additional W3 activity; otherwise a null/reversal is valid.

**Secondary**: successful native Draw and pool-paid Draw by epoch, W2/W3 genuine source feedback without Draw gate, first qualifying resource-dependent output, native Write above original 64 endowment, halted/trapped/pool-positive zero actor deadlock at W3 start, unique source-genome fingerprints, resource conservation and exact ablation controls. All per-seed original outcomes retained in JSON.

## Mandatory controls and independent audit

1. Full source-pinned Rust `Laboratory::run_epochs` deterministic replay and Python reconstruction of exact 128 SplitMix64+LCG programs, *including extra choice4/5 random draws*, identical within-genome policy-paired code and source-high/low code.
2. Exactly original 16 labels and inputs and 24 snapshots per run, original SHA256 payloads, source revision, native PC/tick/process/events and immutable prefix, actual no-host-start actions.
3. `pool_paid_attempt` only when policy true, native actor had **zero local credits** before that event, pending actual opcode Draw, pool before event≥1, pool attempt fee1, then Draw transfers r1=8 if available after fee; failed Draw remains trapped, never resurrected. Other attempts pay from local actor credits.
4. Exact compute ledger: `pool + actor credits = initial64 + initial pool96 (if ambient) + 8 × number of external ambient epochs − native attempted instructions`. No double-counting pool-paid fees and no implicit energy creation.
5. Native Read taint/Const kill/Add/Sub propagation to CellWritten(key9) in BOTH low/high paired source variants; earlier true same-window Read and same writer PC; Draw-disabled cannot have native Draw. Closed-pool no successful Draw.
6. Two targeted synthetic calibrations at most: correct pool-paid native Draw succeeds and false pool-paid marker on non-Draw/credit-positive event must fail. Do not write incidental tests for every helper.

Report both registered primary metrics on the complete 128-world denominators, separately from feasibility calibration. Archive untouched original SQLite and independent JSON. Changing the policy after observing a negative result is **a new separately preregistered study**, not retroactive success.

**Scientific interpretation:** Successful native resource draw under a VM-supplied pool-paid entry law is physically environment-assisted. Even coupled W3 source output does NOT establish internally constructed feedback code, multi-component functional closure, endogenous repair, heredity or digital life. **CL-1 OPEN.**
