# CL-1 Study55 — preregistered independent generic CIL back-edge topology experiment

**Status: FROZEN PROSPECTIVELY before any Study55 implementation, code generation, seed inspection or outcome.** Following Studies49–54, spontaneous same-actor same-graph node-copy→edge-edit→child-launch was independently observed (1/256 retrospective, 3/256 prospective), but **native child CellWritten remained zero**. Retrospective Study54 confirmed a parent can copy an actual original Write node without ever launching its graph. Study55 tests whether *generic executable control-flow topology* changes the likelihood of compositional native operations; it does not write a reproductive sequence or organism.

## Frozen population and paired factors

- Exactly **256 new independently sampled seed pairs**, integer seeds **3,800,000..3,800,255**, no exclusions, pilot seed selection or optional extension. Two paired CIL initial-program topology regimes, each with four matched environment arms: **2,048 actual native runs**, 24 epochs/run ×12 charged attempts/epoch = **49,152 SHA256 epoch snapshots**. Each seed is ONE independent pair, not two independent seed-worlds.
- Common native world: unchanged existing Rust world kernel and CIL instruction semantics; strict cells32, max executable nodes96, aggregate storage512. Three original programs each with Study49 nine-instruction scaffold, 72 stochastic nodes9..80 and Halt81; 56 initial actor credits, 27 shared charged native prefork attempts, 47 actor credits each and externally endowed pool48 at fork. The full original 24-way Study49 random opcode choice table and **all operand registers remain unchanged**; only the nonzero branch edge of choice9 changes. Each choice8 consumes its original second LCG call to sample original source index0..80.
- **Unbiased paired RNG alignment:** On *every* continuation whose 24-way choice is 9 (`Branch`), consume **one additional LCG call** (64-bit wrapping state `state=state*6364136223846793005 +1442695040888963407`). Do so in BOTH topology regimes and for all original actors. In `local` topology keep existing `Branch {test:1,nonzero:index-2,zero:next}`; the extra draw is deliberately unused for branch destination. In `uniform-prefix` topology use `Branch {test:1,nonzero:(draw%index) as usize,zero:next}`; its nonzero edge is thus uniformly sampled among **all preceding program nodes0..index-1**. No other structural or data operations change. The extra draw is essential for the two worlds to have **identical subsequent random opcode choices and source operands**, except those branch edges. The two initial CIL programs are not byte-identical across regimes by design; they are code-choice paired, not state-identity paired.
- All actors/graphs remain randomly sampled; no manual graph-local closure, favored Write, repair/reproduce opcode, outcome-dependent seed selection, external child injection or winner-reward.

## Frozen matched environment conditions (four within each regime)

Same Study49 conditions and source-independent neutral scheduler:
1. `ambient-general`: +8 **External** world-pool credits before native work at each epoch1..23; one existing pool credit transfer to the lowest-id unstopped zero-credit actor **only when no actor runnable**, without code inspection.
2. `ambient-only`: same ambient +8 input, without host actor ignition.
3. `closed-general`: no new environmental credits, neutral ignition as above.
4. `zero-credit-ambient-general`: set all original actors credits0 at epoch0, then ambient+general law.

Use identical 24 epochs×12 charged native attempts per epoch, source-pinned run provenance, original SQLite and Rust laboratory exact replay; external energy and starter actions never count as native self-maintenance. Within each topology/seed the four branches start from a byte-identical prefork world.

## Preregistered endpoints and falsifiable direction

**Primary A — spontaneous strictly ordered native code graph assembly.** For each topology's `ambient-general` arm, count worlds/256 with at least one real `GraphLaunched` native child, preceded by the same original parent and same actual source graph's successful internally attributed `GraphNodeCopied` of a parent source_node≠0, followed by a successful `GraphNodeEdited` of that graph by the parent, with copy.tick < edit.tick < launch.tick. Report two arms, paired `uniform-only`, `local-only`, `both`, `neither`, Wilson95 intervals. **Directional exploratory mechanism claim**, frozen prospectively: uniform-prefix > local in paired discordance; 0 or reversed outcome is a valid negative. This does not constitute life.

**Primary B — native-launched child shared-cell functional output.** Count native child `CellWritten` by actor child>3 with earlier `GraphLaunched` actor/child ancestry and executable `Write` at recorded child PC, successful internal event origin and cell key/value, per topology/256 and exact paired discordance. This is not inheritance or self-maintenance. Zero is an acceptable null.

**Secondary**: worlds with native source `Write` copied (frozen initial source opcode and runtime source uncertainty separately), copied `Write`→later same-parent/graph native edit→launch, worlds with launched child containing `Write`, graph-local final edge closure versus actual execution, native non-`GraphCreated` child op success, original actor self-code modification, actor/child W1/W2/W3 work, native source/graph lineage and all environmental pool transfers. No retrospective endpoint promotion to prospective success.

## Strict independent verification

- Independently reconstruct both LCG program grammars, including identical random choice stream and **extra RNG draw for every Branch in both**; reject any initial CIL differences besides the declared `Branch.nonzero` field. Verify original scaffold, seed IDs, all 2×4 matched conditions, actor credits47, total inventory189 and strict storage caps.
- Independently verify all **49,152 original SHA256 epoch snapshots**, run start/final hashes, true native actor/child/graph attribution and full event ledger, native compute depletion 1 per attempted instruction, external ambient input +8 and environment neutral-start transfer (net zero), and Rust `verify_epoch_replay` per arm. No post-result code edits or seed pruning.
- Minimal focused positive and negative lineage fixtures; two Python native-grammar auditors for each regime plus one paired contrast auditor. Archive original SQLite separately for each topology plus the independently checked paired report, each linked to the executed source revision.
- Do not combine the 256 correlated pairs as 512 independent worlds or claim an effect from one lucky outcome. A complete native graph assembly ordering or child Write is still not damage repair, internally produced energy, continuing lineage, fitness adaptation or life.

**CL-1 remains OPEN.**
