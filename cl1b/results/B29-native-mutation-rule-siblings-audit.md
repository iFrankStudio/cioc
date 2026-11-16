# CL-1B B29 — paid self-edit of mutation-rule source generates distinct full-genome siblings

**Engineering status: source-pinned native PASS.** [GitHub Actions full run 43802302999](https://github.com/iFrankStudio/cioc/actions/runs/43802302999) at SHA `8f893316fcfa4450f7270631d7c50fcd8b37d116`: **30/30 Rust integration targets and 16/16 independent source-parsed analyses PASS**, evidence publication PASS. [Read the original 30+16 PASS summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/8f893316fcfa4450f7270631d7c50fcd8b37d116-43802302999-1/summary.txt). [Prospectively frozen protocol](../studies/B29-inherited-mutation-rule-siblings.md), [native CIL test](../../cl0/crates/world/tests/cl1b_b29_mutation_rule_siblings.rs), [independent deterministic CIL interpreter](../analysis/b29_mutation_rule_siblings_scan.mjs).

## New native affordance versus B28

B28 generated new executable request values but used a **fixed authored mutation delta** throughout a parent's lifetime; two sibling descendants of one parent inherited the same delta. B29's **same CIL parent** pays for a native `PatchConst` that alters its own executable mutation-delta instruction **after** making a child: `+1→−1→+1`. The source mutation is recorded by observing real before/after native `Process.code.nodes[12]` at charged instruction PC22. Each child inherits the **entire executable 23-node replicator** copied from the parent at the moment of birth, and its request nodes PC0/PC11 are amended by CIL `GraphSetConst` before launch.

One founder process with2 initial compute is externally loaded. All post-start supply enters an **identity-neutral world pool**, only via external packets157,157,156; no second founder, host-coded mutation, process-targeted credits, or privileged fitness reward is used.

## Frozen four-arm outcomes

| Same initial source budget; supply packets | Genuine native birth edges | Effective parent-source mutations | Last common pool |
| --- | --- | --- | ---: |
| **FLIP**: charged PC22 `PatchConst` | **1→2, 1→3, 3→4** | **P1: +1→−1→+1; P3: −1→+1** | 0 |
| **NO_FLIP**: PC22 same-cost ordinary `Const` | **1→2, 1→3** | No source mutation | 156 |
| **NO_FLUX**: FLIP code, zero packets | None | None | 0 |
| **COPY_DISABLED**: same FLIP code, neutral packets | None; graph copy suppressed and native launch traps | None | 313 (additional local stranded compute) |

The actual FLIP final requests/mutation rules by process ID are **(P1 156,+1), (P2 157,+1), (P3 155,+1 after its own post-birth rule inversion), (P4 154,−1)**. Importantly, P2 and P3 were **both native offspring of P1** with request157 versus request155; they were not separately loaded by the experimenter. Their divergent genotype originated when P1 paid to modify its own mutation operator between births. P3 then produced P4 under a shared packet156 that could not fund P2's expensive request157.

Each neutral packet in FLIP is exhausted in exactly the expected paid attempt count `157,157,156`, after the initial two charged startup instructions. In NO_FLIP the first two windows each consume157 native attempts, the final packet156 yields **zero** internal attempts and stays unused because all awaiting Draw operations require at least157. Both full-program copying and alternating signed delta mutation were performed by native CIL; no new life-specific VM opcode was added.

## Interpretation and limitations

**Confirmed:** the native substrate permits a continuing process to **modify its own inherited mutation-rule source** and generate genetically distinct *full executable descendants*, which can then differ in ability to access a common resource pool. This goes beyond a single fixed signed mutation step along a lineage, and beyond a helper process whose entire code was embedded as a separate template.

**Not confirmed:** that the actor discovered or evaluated its mutation rule; the inversion algorithm `δ←−δ` was **prewritten by investigators**, as were the source-copy program, trait dimension, starting request and packet sequence. The sibling difference and resource outcome are deterministic consequences of authored code and physics, not open-ended life or spontaneous adaptive strategy selection. More descendants in one hand-picked environment is neither a general fitness increase nor evidence of intelligence.

**CL-1B individual-only reminder:** replication remains an *optional* experimental avenue and is not required for acceptance of the continuous-individual route. CL-1A's population route remains archived. No B3/B4 endogenous cyberlife acceptance is claimed.

The next falsification should test whether the FLIP benefit survives alternate **prospectively held-out third-packet supply sizes** and changes in scheduler order. It must attribute all births and consume real compute, and should include a no-flip sibling control. A lower-request genotype only wins if it is affordable and scheduled before competitors; do not report mere birth order as independent fitness.
