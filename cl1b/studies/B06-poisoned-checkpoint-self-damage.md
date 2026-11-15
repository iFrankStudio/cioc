# CL-1B / Study B06 — checkpoint poisoning turns maintenance into self-damage

**PRE-REGISTRATION 2026-11-15: authored before any B06 code, independent B06 outcome computation, or experiment run.** This is a causal calibration of a researcher-written two-monitor CIL organization. It is **not** spontaneous cyberlife, learning, generalization, or accepted endogenous repair.

## Hypothesis

The B05 program uses ordinary CIL-written shared data cells 12 and 13 as reference information for its two mutually monitoring code regions. If those cells are corrupted **without changing executable code**, the CIL maintenance machinery may **internally modify initially healthy executable instructions into faulty ones**. A repair policy that trusts unauthenticated data can therefore be a source of self-damage. An otherwise identical control suppressing native `PatchNext` should prevent actual code damage even if the actor continues to detect mismatches.

This addresses the difference between a present-tense service output and recoverable organizational integrity; output throughput alone must not count as fitness.

## Frozen source and matched fork

Reuse exactly `founder()` from [B05's committed Rust test](../../cl0/crates/world/tests/cl1b_b05_reciprocal_monitors.rs), **not** a newly tuned founder. One process id1 with identical source and 300 credits, finite strict capacity64, no compute pool. Externally seed source input at cell7. Run exactly 8 attempts, so CIL creates checkpoint cell12=10 and cell13=18; process next PC8, credits292. This state is forked before any poisoning; human lab then changes ordinary shared cells using explicitly external `SetCell`.

Both monitors remain capable of reading their original healthy code in the pre-fork state:
- A monitor at PC9 reads code node17.next, compares it with checkpoint13, and if unequal uses native `PatchNext` at PC14 to overwrite node17.next.
- B monitor at PC17 reads code node9.next, compares it with checkpoint12, and if unequal uses native `PatchNext` at PC22 to overwrite node9.next.

## Frozen arms and contrasts

Inputs `[-13,0,7,23,101]`, **five arms** per input, **25 deterministic worlds**, 100 postfork CIL attempts per arm; all results included.

1. **CLEAN**: no checkpoint edits. Expect zero real native code patches; code edges remain 9.next10 and17.next18.
2. **POISON_A_REFERENCE**: researcher changes cell13 from18 to23. Expect A's *native repair opcode* to convert the previously **healthy** node17 next18→23, removing monitor B's checking path.
3. **POISON_B_REFERENCE**: researcher changes cell12 from10 to15. Expect B's *native repair opcode* to convert previously **healthy** node9 next10→15, removing monitor A's checking path.
4. **DOUBLE_POISON**: corrupt both cells13=23 and12=15. Expect two harmful native patches to healthy code nodes 17 and9 respectively, disabling both monitoring paths while leaving the service path live.
5. **DOUBLE_POISON_PATCH_DISABLED**: identical double checkpoint corruption and source as arm4, with `with_native_patchnext_disabled()` external control world law enabled before postfork attempts. Expect no actual code patches and both original healthy next edges to remain10 and18. The native edit attempts are still charged; count suppressed edit events.

Externally poisoned checkpoints are **environmental damage**, not CIL-created novelty. Endpoints intentionally do **not** require a fixed number of service writes or fixed final PC: preserve and report all outputs, traps, attempted work, and actual repair events after exactly100 opportunities. Input is not a hidden reward; service success is observable `CellWritten(key9,value=input)`.

## Primary predeclared endpoints

- Original CIL-written checkpoints must be proven from native event origins at PC3/PC7 (not host writes). No external code `PatchEdge` in any B06 arm.
- A single or double poison must produce an internal `ExecutableEdgePatched` that **damages previously healthy code**, rather than repairing damaged code; no source-code divergence before poisoning. The clean arm must retain both healthy edges.
- Double poisoning must leave both monitoring paths bypassed, even if service continues or accelerates. For patch-disabled arm, code must remain at the original 9.next10 /17.next18; this isolates the CIL edit's causal role, not the effect of checkpoint corruption on its own.
- Event provenance, every charged attempt, exact external event counts (CLEAN2, one-poison3, double-poison4), zero births, bounded total compute inventory and full denominator including failures must be recorded.
- Outputs are *descriptive*, not an organism fitness score. A fast service loop with disabled code maintenance is an explicit **negative control**.

## Scientific interpretation

Passing native Rust tests would establish only: **human-authored native self-maintenance can be causally misled by corrupted internally stored information**. No individual developed its own monitor or repair policy. It shows why rollback/checkpoint algorithms cannot be treated as an infallible primitive of digital life, and why any attempt at continuous self-evolution needs to account for damage to the mechanism that decides what counts as an improvement.

Future: test CIL-owned provenance/integrity of checks and error-tolerant reference structures, without hardcoding unmodifiable trusted guardians. Any new monitor/checkpoint policy must face held-out corruption patterns and cost-matched controls. Do not count service throughput as adaptation.
