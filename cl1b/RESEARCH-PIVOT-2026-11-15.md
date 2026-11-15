# CL-1B scientific pivot after full native verification — 2026-11-15

**Status:** research design decision, not an experimental pass or a declaration of cyberlife. [Full native verification at SHA `5f4ce96e`](results/CI-2026-11-15-full-native-verification.md) removed the previous source-execution uncertainty. It did **not** turn the authored diagnostic programs in B00–B21 into evolving life.

## Review of the strongest falsification and feasibility evidence

The decisive research results from the same verified source revision are:

1. **Policy self-modification is expressible, but scripted changes do not generalize (B14–B15b).** An actor can modify the actual `Const` literal controlling its next mutation proposal, but a frozen 2→1 step switch has **zero positive effects among 32 distinct held-out one-edge lesion types**; two are harmful. Among 160 input×holdout cases, only31 actually *change and later execute* the modified policy, and those have **0 wins /10 losses/21 ties**. Five input values are correlated repetitions of edge classes, not 160 independent mutation classes.
2. **The actor's evaluator is another mutable piece of executable code (B12–B13b, B16–B17).** Freshness markers can be genuine while promised work never happens; a checker can reject correct work and cause destructive edits. Even direct nonzero payload challenges have **12 distinct one-edge mutation types causing false acceptance** and78 causing false rejection among 899 structural mutants. This is a fundamental fragility of single-path, self-certified "fitness" criteria, not evolution of intentional deception.
3. **Finite closed-world native compute cannot sustain indefinite existence (B18).** The exact resource inventory `local credits + world pool` declines one unit per paid CIL attempt; Draw, Return, Transfer and GraphLaunch move rather than mint credits. A world with finite initial energy and no new source cannot run indefinitely, even if repair is perfect.
4. **Neutral environment-to-actor energy access is mechanically possible (B19–B20).** Generic `DrawCompute` lets native CIL obtain externally available shared-pool compute without researcher-targeted `SetCredits`. However a zero-credit individual blocked at ordinary `Write` cannot reach a new Draw; even an accumulating pool cannot fix its own program counter.
5. **Cooperative resource access is mechanically possible (B21), but not spontaneously formed.** A **separately authored** helper can Draw and `TransferCompute` to that blocked worker. In fully source-matched native controls, only the real CIL transfer restores the worker's service; no-transfer/no-helper controls fail despite identical neutral supply opportunities. This is a potential component of a future organization, not the origin of that organization.

## Primary bottleneck

The most important missing capability is **endogenous organization of resource acquisition and reliable internal decision-making**. Continuing to add researcher-authored repairs, status bits, prechosen checkpoints or step-size switches will mostly re-demonstrate scripted fault tolerance.

An individual need not imitate biological reproduction, aging or anatomy. It *does* need to preserve actionable informational structure, gain access to computation/storage, decide whether modifications help under changed conditions, and avoid dependently trusting a checker's own easily-corrupted output. An organization can include multiple processes and graph objects if its continuity and resource economy are measured as one causal unit rather than declared by process ID alone.

## Highest-priority next experiment — prospective research design, not yet an accepted study

**Test whether a continuing native CIL organization can acquire and retain a new resource-transfer relationship that was *not supplied as an executable helper program by the investigator*.**

- **Initial condition:** a small fixed, general-purpose CIL repertoire and generic graph/compute primitives. No prewired target recipient, immutable repair overseer, trusted "fitness" opcode, or researcher-selected successful offspring. Investigator-originated seed complexity, code templates and environmental inputs must be separately quantified.
- **World physics:** strictly bounded local compute/storage; exogenous compute packets enter only an **identity-neutral world pool**. Trial order/pulse sequences are preregistered, and at least one later sequence is withheld until after all code, policies and analysis logic are frozen. Resource-packet size and event-index order can vary; no chosen process receives host `SetCredits`.
- **Mechanism under test:** native CIL operations themselves must construct or mutate an executable subgraph/recipient relation and perform `DrawCompute` / `TransferCompute` where possible, or else exhibit a real alternative resource access strategy. Trace the first appearance of the relation, graph descriptors, payer, parent-process provenance, resource transfers and subsequent effectiveness.
- **Outcome measures:** actual paid CIL continuation after environmental changes, total energy acquired vs spent, occupied storage, stranded pool credits, irreversible traps and all failed organizations. Any "service" or status readback is a secondary observer measurement, with its own corrupted-checker controls; it cannot be a privileged host reward.
- **Matched ablations:** native executable editing disabled at equal attempted-instruction charge; no-transfer control; frozen founder/helper control from B21; identical resource packets with and without competition; and negative controls in which the newly formed partner cannot access the claimed resource. Preserve all failures in the denominator.
- **Evidence threshold:** successful relationships must have **native lineage provenance** and survive independently specified supply perturbations with an effect not explained solely by more initial compute, a hidden template or researcher top-ups. A single selected anecdote or an externally constructed helper is an engineering control, **not** B3/B4 evidence.
- **Reproducibility:** acquire two independent source-SHA-pinned Rust runs, or one fully verified run plus a separately implemented differential that agrees on exact event and resource accounting; manually review at least one negative/control trace. The existing [full checkpoint](results/CI-2026-11-15-full-native-verification.md) is the baseline.

## Feasibility decision

**Do not weaken the substrate's compute conservation or add a life-specific `REPAIR`, `FITNESS`, `HELPER`, `FEED` opcode to manufacture a result.** If CIL cannot create an unplanned relationship within the available compute budget, record that *negative result* and study which **generic** graph/data primitives or information channels are actually unavailable. New generic substrate affordances must have explicit cost, determinism and negative-control checks, and must be tested separately from life claims.

The next real breakthrough is evidence that organization/policy arises **in the world from native operations**, not that we can write yet another externally designed program that behaves impressively.
