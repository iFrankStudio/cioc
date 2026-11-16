# CL-1B B29 — native sibling variation by inherited mutation-rule inversion

**Prospective protocol (2026-11-16), before implementing/observing B29.** B29 is a finite, investigator-authored *lineage affordance* experiment, not proof of endogenous innovation or CL-1B stage acceptance. CL-1B's central scientific target remains **continuity of an adapting individual**; reproduction is an optional substrate stress-test rather than a mandatory requirement or return to archived CL-1A.

## Research question

B28's branching was incidental: an existing parent reproduced twice using the same authored signed mutation step, producing *two genetically equal siblings*. Can the continuing **same CIL parent** instead revise its *own mutation rule at runtime* after its first offspring, generating a second child with a **different complete executable genotype** from the same parent, without host-selected children, free mutations or an added reproduction opcode?

A stronger causal condition than merely finding two process IDs is required: actual native `GraphLaunched` ancestry must show **one parent with ≥2 children**, sibling source genotypes must differ at their own executable request literals, and the original parent must have made an **in-process, paid `PatchConst` of its own signed-delta instruction**, not just produce two prewritten graphs.

## Preregistered 23-opcode founder

One externally loaded process id1, initially **2 local compute credits**, native CL-0 generic restricted world with strict capacity (16 cells, 64 nodes per process, 128 aggregate units), `with_pool_paid_draw()`, `with_recoverable_draw_shortage()`, `with_native_executable_edit_trace()`. No host `SetCredits`, process-specific allocation, winner selection or privileged fitness primitive.

- PC0–18: the B28 19-node *complete source-copy / child-variant* algorithm, changing the genome length constant at PC7 from19 to **23**, with request genotype Q and signed delta δ.
- PC18 `GraphLaunch` transfers2 existing local credits, launches an exact **23-node full self-reproducer**, and continues to PC19.
- PC19 `Const r1=0`; PC20 `Sub r3=r1-r3` (negates the just-used signed mutation δ); PC21 `Const r1=12`; PC22 `PatchConst(node=r1,value=r3,next=0)` makes the **original parent's own PC12 signed mutation operator** alternate from +1 to−1 to+1 after successive births. Unlike B28's child graph edits, this is a native source edit *to the continuing parent*.
- Every copied child inherits the **original parent code as it existed before that birth**, including the then-current δ and the four-instruction post-birth rule-inversion routine. During child construction, GraphSetConst edits only its two copied request literals at PC0/PC11 to current Q+δ. Child request genotype and delta are thus heritable, with the same finite source copier.

A parent's Draw request is **Q=156**. This was fixed by native compute budget analysis: the charged post-Draw copying/graph-edit/launch consumes149 CIL attempts plus transfers2 child credits; the four charged negation/source-patch instructions consume4 more. This leaves **one** local credit from Q156 to execute the next PC0 Const and return to its pool-payable Draw PC1, so it can attempt a second birth. The newborn child spends its inherited2 credits executing PC0 and a deferred PC1 Draw.

First predicted resource window: neutral external packet **157** = one pool-paid Draw156 with instruction charge1 plus exactly156 credits transferred. After one native parent birth, parent has internally flipped δ from+1 to−1, and reentered Draw PC1; its first child has request157 and delta+1, needing packet158 to make its own Draw. On the **second** neutral packet157, the now-eligible parent at request156 should reproduce *again* before its child request157, producing **second child155 with delta−1**. This is a specific engineering prediction to be falsified by the Rust event log, **not** a basis to rewrite supply after inspection.

## Matched controls (initial code generation is investigator-authored)

Four frozen arms, each with exactly one externally loaded founder, equal initial2 compute, and three neutral pool packets `[157,157,156]`:

1. **FLIP:** PC22 is real charged `PatchConst`, initial δ=+1. Tests child1 Q157/δ+1, parent next mutation δ−1, child2 Q155/δ−1; a later packet156 can make the cheaper second sibling viable when both siblings compete for the common pool.
2. **NO_FLIP:** same 23-node parent, same 4-instruction tail and packet budget; PC22 replaced with a same-cost ordinary `Const` that does **not** modify PC12, so successive siblings should have the same Q157/δ+1 and the stronger identification of within-parent rule inversion disappears. This is an authored ablation, not an event-level randomized mutation.
3. **NO_FLUX:** FLIP code but no external packets; no births or mutation-rule change are possible after the initial two charges.
4. **EDITS_DISABLED:** FLIP code, same three packets, `with_native_code_edits_disabled()`. Graph-copy mutations and PatchConst are suppressed at normal instruction charge. This must not create a successful child; a trap and unspent pool credits must remain in the record.

All control code has exactly 23 native instructions. Packet schedules and test denominators are fixed *before* observing outcomes.

## Evidence and acceptance boundaries

- Parent, children and event origins must be **internal**; the only external actions are founder load and fixed pool packets.
- For each actual birth: **23 ordered `GraphNodeCopied`** from that parent's executable, **two `GraphNodeEdited(edit="const")`** for copied child nodes0/11, and **one `GraphLaunched`** with child endowment2 and 23 moved nodes. Inspect **full child code**, not just a metadata trait.
- At native PC22, positive FLIP parent changes its own PC12 from+1 to−1 and next time −1 to+1, while NO_FLIP's PC12 stays+1. Verify source changes by stepping, not by inferring what source "should" do.
- Count all successful birth edges *without assuming* PIDs equal ancestry depth, all traps, residual world-pool compute, paid attempts, and storage. Each charged native instruction decreases total compute by1; each explicit pool pulse increases it exactly by the declared amount. Preserve evidence even if a registered numerical prediction fails.
- Independent deterministic source-parsed simulator should reproduce native event topology, genotype/signed-delta assignment and charge conservation. The earliest returned native SHA-matched CI is decisive for an implementation claim.
- **No autonomous selection claim:** all code construction, δ sign inversion, reproduction budget, and energy schedule were investigator-designed. Showing two different siblings is a precursor to *possible* selection under resource competition, not an internally discovered mutation policy. Moreover, this study is not a prerequisite for the central CL-1B individual-only path; do not conflate copies with individual continuity.
