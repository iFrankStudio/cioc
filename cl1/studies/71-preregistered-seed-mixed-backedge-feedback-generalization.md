# CL-1 Study71 — preregistered independent seed-mixed CIL backedge feedback generalization

**STRICT PROSPECTIVE; seed generator change registered BEFORE any new cohort execution.** Stage68 (256 numerical seeds4,500,000..255) and Stage69 (256 new seed labels4,600,000..255) found respectively **79/256** and **81/256** broad-random-backedge native source-responsive W2+W3 information loops versus **0/256** short-backedge, with the same scripted three-instruction generic input/output register interface, finite credits and **no prewired cyclic tail**. Both registered results passed full native Rust and independent Python SHA256 replay. Stage70 post-result integrity audit found no *exact* genotype duplicates among the 512 seed-labelled original programs, but **higher exact opcode pattern similarity under matching seed offsets** than a fixed permutation comparator, so the two numeric cohorts must not be uncritically pooled as independent random samples. Stage71 tests the **same frozen CIL feedback affordance under a new, unambiguously registered one-time seed mixing layer**, not native genotype evolution.

## Exactly frozen novel cohort and RNG

- **256 fresh raw seed labels4,700,000..4,700,255**, all included, 8 native matched code/source/Read-ablation variants per seed: `short-low`,`short-high`,`short-ablated-low`,`short-ablated-high`,`broad-low`,`broad-high`,`broad-ablated-low`,`broad-ablated-high`. Total **2,048 new native runs and 49,152 source-pinned SHA256 epoch snapshots**, 24 epochs×12 charged instruction attempts, zero runtime researcher intervention.
- **The ONLY random-code sampler change from Study69:** initialize the original CIL LCG state as `state=SplitMix64(raw_seed)` rather than `state=raw_seed`, with **exact** wrapping 64-bit algorithm:
```
x = (raw_seed + 0x9E3779B97F4A7C15) mod 2^64
x = ((x xor (x >> 30)) * 0xBF58476D1CE4E5B9) mod 2^64
x = ((x xor (x >> 27)) * 0x94D049BB133111EB) mod 2^64
state = x xor (x >> 31)
```
Use it **once at program generation start**. Thereafter the original LCG `state=state×6364136223846793005+1442695040888963407 mod2^64`, six-way `random(state)%6` instruction choices, same extra draw for `Const(r0)` and one extra identical draw on every random `Branch` in **both** topologies. No accidental random draws or additional sampler differences.
- Exactly Study68/69 fixed CIL: original one-process three native I/O port constants at PC0..2 (`r1=1,r2=7,r3=9`), 24 sampled instructions PC3..26 among generic Read, Write, Add, Sub, Const and Branch, and **Halt at PC27 in BOTH** topologies (no researcher-authored loop). `short` Branch nonzero PC−1 (PC3→3); `broad` Branch nonzero `3+edge_choice%(PC−3)` (PC3→3). Same frozen 4-cell/32-node/64 total strict storage, exactly300 initial externally endowed actor credits, pool0, no native DrawCompute, code patch, child, host starter or other after-setup action. Researchers externally initialize sensor cell7 as3 or8; ablated Read→Const0 retains identical sampled PC/next.
- **No seed exclusion:** independently verify SHA256 canonical CIL genotypes and report duplicate or correlated source program counts *before interpreting primary*, but include all original256 seed labels and all eight matched modes even if any full program repeats. A genotype uniqueness check is an **integrity outcome**, NOT a way to select favorable programs.

## Frozen prospective primary and negative controls

Per original genome require in BOTH W2 epochs8..15 and W3 epochs16..23 a *new native* `CellRead(key7)` in the same window in both intact low/high versions, then successful original-actor `CellWritten(key9)` at the same writer PC with high−low exactly +5, source-tainted r0 until that Write. Native Const(r0) kills taint; Add/Sub r0±1 preserve; Branch doesn't create source. Count separately broad/short original genomes out of256, Wilson95 and exact paired broad-only/short-only/both/neither. Prospective directional generalization passes if broad-only>short-only, subject to the actual event/seed integrity checks. Report failures regardless of code traps/halt, absent read, W3 inactivity or code source differences.

**Negative:** all four `-ablated-` source arms have zero native Read, and low/high paired Write tick/PC/value trajectories must be identical. Entire source program plus branch topology must be reproduced independently from each seed by Python SplitMix64+LCG; same seed program choices identical except only broad/short Branch targets and Read-ablation, with no other hidden changes.

**Secondary:** native W1/W2/W3 attempted activity, source-tainted Read/Write counts, actual taken altered backedges before the first qualified W2 Write and before the W3 Write, full-gene SHA256 duplicate count, opcode-tag similarity for adjacent seed labels and fixed permutation checks (descriptive only). Prove exact one-credit native charge and no environmental actor-directed restart. Archive untouched original SQLite and full independent report. Only 2 focused calibration tests for the seed-mixer/paired-grammar and native source-taint gates, not a broad incidental suite.

## Interpretation and next decision

This is a **new random-population generalization**, NOT a mathematically perfect IID proof. SplitMix64 reduces predictable seed-neighbor correlation but remains a deterministic pseudorandom generator; avoid unjustified independence claims and historical pooled significance. The protocol does not add a fitness oracle, encoded survival routine or native self-organization. Even a robust positive establishes only **recurrent CIL information response under externally provided native I/O and finite credits**. Subsequent work must demonstrate **within-world changing stimulus response**, then genuinely native code/data organization and function repair, with source-pinned resource/actor provenance.

**CL-1 OPEN.**
