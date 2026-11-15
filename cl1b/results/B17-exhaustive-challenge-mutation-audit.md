# CL-1B B17 — exhaustive challenge-checker mutation audit

**Evidence tier:** source-parsed restricted JavaScript interpreter of the committed [B16 32-node founder](../../cl0/crates/world/tests/cl1b_b16_functional_challenge.rs), **not native Rust execution or accepted digital life**. The B17 [preregistered protocol](../studies/B17-exhaustive-challenge-mutation-screen.md) preceded the [independent scanner](../analysis/b17_challenge_mutation_scan.mjs) and the outcomes recorded here. A [native Rust differential](../../cl0/crates/world/tests/cl1b_b17_native_challenge_mutation.rs) has been committed but **not confirmed executed**.

## Full finite universe and denominators

The frozen CIL graph has 29 patchable `next` fields. For each, all 31 different successor addresses are tried: **899 distinct one-edge mutation types**, each with five fixed original inputs `[-13,0,7,23,101]`. Thus **4,495 deterministic mutant worlds**, plus five intact baseline controls, each with a maximum of120 attempted CIL instructions and the same initial compute/storage caps. No successful worlds were selected after inspection; traps and early halts are retained.

### Primary source-level results

| Distinct endpoint | Mutant worlds / 4,495 | Unique underlying mutation types |
| --- | ---: | ---: |
| PC16 **accepts** while no genuine PC11 challenge payload was written since current PC0 | **60** | **12 / 899** |
| PC16 **rejects** after a genuine PC11 challenge payload in current epoch | **390** | **78 / 899** |
| Actual PC16 checker branch reached (whether right or wrong) | **2,625** | not independent samples |
| Trap during the 120-instruction window | **154** | varies by input |
| Halt/trap before completion | **394** | varies by input |
| PC16 branches without completing intended PC13→14→15→16 read/compute pipeline | **335** | varies by input |
| PC13 read fails to complete intended downstream PC16 pipeline | **465** | varies by input |

The 60 false-positive worlds arise from **12 structurally distinct edge mutations**, each repeated across all five inputs; 390 false-negative worlds arise from **78 structurally distinct mutations**. The intact baseline always provides real PC11 challenge writes and accepts with no false decision.

At the decision-event level the independent scan recorded **230 erroneous acceptances** and **1,675 erroneous rejections** across the complete mutant set. Many mutants do not even reach the checker (4,495−2,625=1,870 worlds). Rates over all mutants and over checker-reached mutants answer different questions; never hide inability to execute the evaluator in a denominator.

For each input label, the false-positive / false-negative / checker-reached **world counts** are respectively `12 / 78 / 525`, while trap counts differ `[41,32,24,16,41]`. Inputs are fixed deterministic conditions, not independent mutation mechanisms. An arbitrary mutation can route an input-bearing register into an address, making trap rates input-dependent despite identical causal failure categories.

### Mechanism-level interpretation

The B16 nonzero challenge eliminates the **specific** valid-zero and pre-existing stale-output ambiguity when the challenge routine and checker remain intact. This exhaustive test nevertheless shows that some code mutations **fabricate apparent success by changing control flow**: e.g. `node10.next13` skips real PC11 payload production and the instruction that sets the output-cell address, so PC13 reads the actor's own challenge input key7 instead. The read equals expected17, the native-like PC16 branch accepts, and no real challenge output occurred. This failure is entirely internal, without external injection.

Other mutations generate real challenge output but alter the checker downstream, leading to **false rejection** and potentially destructive code edits. A direct functional readback is causally closer to work than a standalone marker, but **does not provide invariant epistemic authority under mutations of its own checker graph**.

**The B10/B13 and B16/B17 raw percentages are not a cost- or graph-size-matched comparison.** B13 has 20 nodes and a different proxy criterion; B17 has 32 nodes, two classes of output writes and a challenge. The only defensible comparative statement is about the *specific frozen lesions* and directly observed causal path, not evolutionary fitness.

### Validation limits and next gate

The companion B17 native Rust test iterates **all 4,495 worlds**, checking actual process PC immediately after PC16 branches, genuine internal PC11 output event provenance, epoch boundaries, compute charges, trap/stop records, and the normal PC13→14→15→16 read/decision chain. Its expected numbers are source-level **post-preregistration compatibility endpoints**, not a pre-registered successful life criterion. An actual CI run with source SHA and per-target logs is required before any of the above are native experimental results.

The research must now move beyond **author-designed diagnostic policies**: a continuing CIL individual would need to propose, test and revise competing evidence hypotheses under faults that also affect the checker, using finite resources and no host-authorized reward or preselected winner. Otherwise extra checking layers remain just ordinary fault-tolerant software.
