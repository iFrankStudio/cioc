# CL-1 Study101 — finite shared pool selects between two engineered native self-copying CIL variants

**Focused original CI verified:** [GitHub Actions run 43606097528](https://github.com/iFrankStudio/cioc/actions/runs/43606097528), completed **SUCCESS** with one native-standing-variant-competition job, original 10 human-scripted fixed worlds. Source pinned to commit `f00d09f68dfe538e879c24a8d4c2e5d022742698`, frozen protocol [Study101](../studies/101-preregistered-founder-variant-finite-pool-competition.md) was committed separately first at `fc0cb3e4b87764100dc2fa2f9869b999907025b6`. Original Rust executor/test is `cl0/crates/world/tests/study101_finite_pool_competition.rs`, committed before CI at `2911a911f7014c45f6fea6c926a4431053abf2e7`.

## Observed complete original ten-world output

Each original world starts with two hand-authored, complete self-copying CIL founder genotypes, which differ by *one* executable edge: fast F skips an otherwise dormant delay and slow S executes a 42-attempt no-op loop before each self-copy. Both descendants inherit the exact original 20-node executable code, and all children also execute native `Read(key7) -> Write(key9)`, `GraphCopyNode` of the entire current genome, native `DrawCompute(160)`, and `GraphLaunch(160)`. The input source3 or8 is an *external boundary condition*, not an organism-derived sensor. Each actor funds own CIL charges; an initial common public compute pool1600 is finite and shared. No process receives a host-side reward.

| Original source | World | F descendants | S descendants | Total native launched children | Charged attempts | Final shared pool |
|---|---|---:|---:|---:|---:|---:|
| 3 | mixed F founder1 / S founder2 | **6** | **4** | 10 | 1408 | 0 |
| 3 | mixed S founder1 / F founder2 | **6** | **4** | 10 | 1408 | 0 |
| 3 | F / F founders | 5 + 5 (both F) | 0 | 10 | 1240 | 0 |
| 3 | S / S founders | 0 | 5 + 5 (both S) | 10 | 1660 | 0 |
| 3 | mixed F/S, pool closed | 0 | 0 | 0 | 320 | 0 |
| 8 | mixed F founder1 / S founder2 | **6** | **4** | 10 | 1408 | 0 |
| 8 | mixed S founder1 / F founder2 | **6** | **4** | 10 | 1408 | 0 |
| 8 | F / F founders | 5 + 5 (both F) | 0 | 10 | 1240 | 0 |
| 8 | S / S founders | 0 | 5 + 5 (both S) | 10 | 1660 | 0 |
| 8 | mixed F/S, pool closed | 0 | 0 | 0 | 320 | 0 |

Every open-pool condition has **12 native source-value-writing actors** (two externally seeded founders + ten truly internally launched children), with correct native PC2 source Read and PC3 Write, and no process-specific external credits. Source3 and source8 worlds give identical child counts; reversing which founder ID receives fast or slow retains F6:S4, ruling out a simple fixed advantage from parent process ID. Both same-variant controls are symmetric 5:5. Closed pool produces 0 children despite surviving native deferred Draw attempts. All ten source-pinned original focused Rust tests passed, including cross-generation exact program-code identity, successful native actor-owned copies, event origin, native Draw/Launch chronology, and full 320+pool minus charged native instructions conservation.

The measurable result is a **20% share difference (6/10 vs4/10) in a finite resource competition between two predesigned coding strategies**, rather than a uniform shared resource allocation. More meaningfully, fast has 50% more observed descendants than slow (6 versus4) in each mixed world, and swapping founder IDs leaves the result invariant. No statistical confidence interval or p-value is warranted from pretending the four comparisons are independent random samples: source3/8 and ID swap are deterministic controls of one *hand-authored* genotype contrast.

## What this does and does NOT establish

**Establishes:** The unmodified native VM substrate can preserve whole-CIL heritable code identity across descendants, execute the same inherited native source function, and let code-execution cost affect realized descendants through ordinary charged instructions, finite shared compute and deterministic scheduling, with no external reproduction/fertility oracle. Resource scarcity plus heritable variation now has a demonstrable *engineering mechanism* for differential reproduction.

**Does not establish:** No mutation creating F or S occurred in these worlds; both original founders were human-programmed. Study100 separately shows intentionally hand-scripted neutral mutation is inherited, but does not combine fitness-relevant random mutation with this competition. Therefore Study101 is **NOT natural selection of new endogenous variation, spontaneous adaptation, self-originating life, reciprocal organizational maintenance, or open-ended evolution**. Original examples are fixed genotype and fixed finite pool, not a sample of a broad algorithmic population. The observed 6:4 is specific to this program, resource budget and scheduler. It was not fitted after outcome and must not be extrapolated to unknown seeds, other CIL algorithms or a probability of life.

**Important remaining reproducibility limit:** This focused Phase0 fixture prints all 10 original worlds and validates internal invariants at runtime; it does **not yet archive the complete original per-epoch SQLite states or independent Python audit**. Such state-level replication is mandatory before stronger scientific evolutionary conclusions, especially a prospective stochastic ecology.

**Next gate:** A separately preregistered ecosystem in which **native-executed code modification generates the competition-relevant heritable variant** (not two different researcher-loaded initial founder programs), followed by unselected mixed-population resource competition, copied-genome source ancestry, fixed genome distribution/seed policy, mutation-disabled negative, source-function survival, and real genotype-by-descendant counts. Prespecify and report denominator including all nonexposed/trapped genomes; no retrospective cherry-picking or tuning the winning edge. **CL-1 remains OPEN.**
