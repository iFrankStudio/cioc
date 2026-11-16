# B28b — explicit post-registration prediction failure and ancestry-based correction

**Status:** transparently recorded **after** preregistering [B28](../studies/B28-cumulative-heritable-resource-mutation.md) and after an independent source-parsed interpreter contradicted one of its registered predictions. This is **not** a newly preregistered confirmatory success.

## What the preregistration predicted incorrectly

B28 expected the δ=+1 genotype under external packets `[128,129,130]` to produce a strict chain `1→2→3→4` and a sequence of successive reproductive request literals `127,128,129,130`. It further expected 128, 129 and 130 charged attempted instructions in the respective supply windows.

That prediction neglected the **one local credit left when a request128 replicator pays for a 127-credit-minimum reproduction**. Parent process2 has one local compute unit after launching its first child3. Its executable next PC is ordinary `Const` at PC0; the residual credit pays that Const and places it at the pool-payable Draw PC1. Process3 also awaits Draw. At the next resource packet130, both are eligible, and the deterministic scheduler may select **parent2 again**, whose lower-cost request128 is payable before child3's request129. Thus process2 can produce a **second child4**, giving genuine branching ancestry.

The corrected independently implemented interpreter predicts:
- δ=+1 under increasing packets: native birth edges **`1→2, 2→3, 2→4`**, genotype literals by process ID **`127,128,129,129`**, paid post-bootstrap attempts `128,129,129` and **one unused pool credit**. Genotype *is* cumulative on each actual parent-child edge; it is **not** cumulative by process ID.
- δ=+1 under flat `[128,128,128]`: first birth creates child128, next packet128 is individually below its 129-unit threshold, third packet accumulates enough to produce a second birth, leaving genotype sequence `127,128,129` and substantial unused pool.
- δ=0 with flat packets: a linear unmutated chain `127→127→127→127` remains possible.
- δ=−1 with increasing supply: first child has literal126 but fails its native `GraphLaunch` after paying graph-copy work, leaving **one birth and one charged trap**, not an omitted failed individual.

## Actual native contradiction and corrective procedure

[GitHub Actions run 43782944682](https://github.com/iFrankStudio/cioc/actions/runs/43782944682), testing SHA `0222e06725cad42fc5c03b3fb13b3714252b4395`, failed at the original **asserted 130 actual attempts in the third increasing-supply window**: Rust returned **129**, not130. The B28 source-parsed independent model initially failed its own strict-linear genotype check, separately revealing `1→2,2→3,2→4`.

The native and observer assays were then corrected **post hoc** to derive each child's expected genotype from its **actual parent PID in the native graph launch event**, rather than assigning "generation = PID−1". The corrected verification must prove every completed child receives *both* mutated executable request literals, and verify all 19 source copies/two graph edits for **each birth**, including multiple births from the same parent. It must preserve original failing CI evidence, experimental conditions and count of all 11 worlds. A passing corrected test would validate the actual **branching** outcome, **not** retroactively confirm the preregistered strict-linear prediction.

## Scientific interpretation

The surprising extra reproduction is a consequence of **ordinary finite compute balance, executable PC reachability and deterministic shared-pool scheduling**. An increased request quantity does not merely change "fitness"; it can leave enough local compute after graph launch for a parent to resume *its own* reproductive circuit, creating sibling branches. The same extra pool opportunity can disadvantage a younger genotype with a larger resource threshold even when that genotype was produced by internal CIL editing.

This is a causal clue for the next research step: true evolutionary claims require following **ancestry and resource opportunity**, not assuming that creation order equals generational depth or that every mutation has independent equal access to fuel. The mutation operator remains **author-designed** and there is no endogenous trait selection, cognitive agency, or open-ended cyberlife in B28.
