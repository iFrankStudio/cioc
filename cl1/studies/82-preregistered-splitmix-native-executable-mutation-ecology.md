# CL-1 Study82 — independent prospective SplitMix64 CIL mutation ecology after invalid Study81 generator

**Status: PREREGISTRATION BEFORE Study82 stochastic worlds. ZERO Study82 ecological outcomes observed.**
Study80 Phase-0 actual CI success [43591338076](https://github.com/iFrankStudio/cioc/actions/runs/43591338076) established scripted native executable-code edit and strict mutation-disabled controls, but not self-maintenance. Study81 CI [43592644179](https://github.com/iFrankStudio/cioc/actions/runs/43592644179) **FAILED** its mandatory uniqueness gate at seed 5,400,026, with `duplicate seeded code genome`: the low bits of its LCG modulo 8 produced pathological correlations. Study81 is an aborted population, *not* a negative phenotype finding. This Study82 changes the sampler openly, in a NEW nonoverlapping prospective population, without selecting or excluding genomes based on phenotypes.

## Exact frozen original generator and population

Use exactly **128 new untouched labels** `5_500_000..5_500_128` (Rust exclusive high bound). The 64-bit random state starts with:
```
mix_seed(seed) {
  x = seed.wrapping_add(0x9e3779b97f4a7c15);
  x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
  x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
  x ^ (x >> 31)
}
```
Each *successive random draw*, including the opcode and parameter draws, uses actual SplitMix64 advance-and-scramble:
```
state = state.wrapping_add(0x9e3779b97f4a7c15);
z = state;
z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
return z ^ (z >> 31);
```
Use `draw % 8` for opcode choice after **this full scrambling**, and the same full-mixed method for `Const %8` or `Branch backedge` parameters. No retries on duplicates; require **all 128 unique full serialized initial CIL code genomes**, abort and record the failed sampler if not. Deterministic is not equivalent to statistical IID.

The CIL genome shape is fixed: 28 nodes, original setup PCs0..2 with `R1=8, R2=7, R3=9`; 24 random body nodes PCs3..26; PC27 `Halt`. All eight bins are exactly:
0 `Read(R0,cell[R2])`,
1 `Write(cell[R3],R0)`,
2 `Add(R0,R0,R1)`,
3 `Sub(R0,R0,R1)`,
4 `Const(R0,draw%8)`,
5 `Branch(test=R0, nonzero=3 + draw%(pc-3) for pc>3 else 3, zero=pc+1)`,
6 `DrawCompute(amount=R1)`,
7 `PatchNext(node=R1,target=R0,next=pc+1)`.
The edit opcode targets executable PC8 but its replacement edge is computed in actor register R0 (which may be an externally read value or internally modified value). Invalid edits trap ordinarily; this is generic VM code editing, not intentional autonomous repair.

## Frozen worlds and original ledger

For each seed run **exactly 4 arms**: low source3/high source8 × edit enabled/disabled, all with **byte-identical original code**. The only differences are external source-port value `cell[7]` and serialized opt-in `with_native_code_edits_disabled` law. Both laws must preserve ordinary charged execution and error semantics. Source port 7 and sink port 9 are researcher-configured environmental boundaries, not organisms.

Use `World::with_strict_capacity(4,32,64)`, one process id1 loaded with 64 local credits, pool96, generic `pool_paid_draw`, `recoverable_draw_shortage`, opt-in observer-only `trace_native_executable_edits`; all arms 24 epochs with 12 attempts/epoch, add 8 external compute-pool credits at each epoch1..23 using the state-predicted pool ledger (not a process-specific refill). Exact original native 512-world SQLite corpus, 12,288 whole-world SHA256 epoch records, deterministic Rust replay and independent read-only Python analysis must be archived at the exact source revision. A Rust duplicate-genome rejection is a **study failure**, not an invitation to substitute seeds.

## Frozen primary and secondary outcomes (identical meanings to Study81, no selective exclusions)

- Primary **effective actor-native executable code editing** per original seed/source: at least one successful `ExecutableEdgePatched {node:8,before,after}` with unequal edges, native `Origin::Internal(1)` at a real `PatchNext` writer PC and before/after edge lineage matching all original events and final real code. Denominator always **128 genomes** per source arm. Report attempted PatchNext, trapped, no-op, successful, effective and edit-disabled suppressed separately, plus paired low/high discordant seed counts and code diversity.
- Secondary **strict source-dependent functional output W2**: actor-native `Read(cell7)` taint surviving to native `Write(cell9)`, in both low/high matched worlds at identical source-def-use coefficient, offset and writer PC, and high-low output difference exactly5. Exact actor event/PC provenance, not mere pre-seeded source values.
- Exploratory joint code-usage/energy: above strict W2 with a preceding real effective executable edit, successful native DrawCompute and native charged attempts >64 in both paired worlds. Count separately, **never treat co-occurrence as proof the edit was useful or caused restoration**. Compare edit-enabled and disabled paired worlds with frozen baseline initial code; report no-exposure, traps and all source arms.
- **No lesion/no repair**: Study82 intentionally damages no code and has no recovery controls. It cannot establish code preservation, autonomous damage recovery, multi-component causal closure, heredity or life. A positive edit is a necessary intermediate capability only.

## Verification, attribution and stopping

The independent Python auditor must rebuild every initial genotype directly from the frozen full SplitMix64 algorithm, independently match all 128 original genotype hashes and all four matched arms, verify exact serialized event-prefix monotonicity, scheduled external pool+8 actions, 12,288 real snapshot SHA256 hashes, exact native event-PC/Read→register→Write taint ancestry, native before/after executable edge chain, native charged attempt ledger and resource conservation. No substitution of a human-scripted scaffold positive for a stochastic genotype positive. Run only dedicated focused CI, post signed-by-GitHub source-commit success/failure status and archive the raw SQLite+JSON. If any run or audit fails, mark this result unverified and diagnose; never re-label failed worlds as scientific positives. **CL-1 remains OPEN**.
