# CL-1 Study 10 — preregistered neutral, reachable graph opportunity

**Registered 2026-11-13, before writing the runner or examining Study-10 outcomes.** Previous Studies 01–09 are known; this study uses fresh, fixed 64 seed-worlds **1600000 through 1600063**. The experiment is explicitly an **engineered opportunity calibration** and can never establish spontaneous organization or life.

## Fixed purpose
Previous random CIL ecologies produced almost no usable shared nonempty graph objects. This study tests a non-teleological, **externally authored reliable scaffold** enabling three CIL processes to create objects, publish handles, grant read access, copy an instruction from another owner's object, and transfer object ownership. All actions still execute as normal CIL instructions, costing computation and storage, and all owner/permissions are checked by the generic VM. **Nothing programmed by the scaffold counts as emergent behavior.**

## Exact generic initial program template per process
Three process IDs i=1,2,3; each has 21 CIL nodes, initial 40 credits; world strict total storage 160 units, max 32 nodes/process, max 32 shared data cells. First twelve nodes are investigator-scripted opportunities. Their register operands and edges are fixed:

0 `GraphNew {dst:3,next:1}`.
1 `GraphCopyNode {graph:3,source:2,dst:2,next:2}` (copies that process's node 2 to a nonempty owned graph).
2 `Const {dst:0,value:i-1,next:3}`.
3 `Write {address:0,value:3,next:4}` (ordinary public shared-cell handle publication).
4 `Const {dst:1,value:(i+1)%3+1,next:5}` (ID of preceding process in ring: i=1 grants 3, i=2 grants 1, i=3 grants 2).
5 `GraphGrantRead {graph:3,reader:1,next:6}`.
6 `Const {dst:0,value:i%3,next:7}` (next owner's published cell).
7 `Read {dst:1,address:0,next:8}` (retrieve next graph handle via shared data).
8 `Const {dst:2,value:0,next:9}`.
9 `GraphCopyFrom {target:3,source:1,node:2,next:10}` (copy one source node if still authorized).
10 `Const {dst:1,value:(i%3)+1,next:11}`.
11 `GraphTransfer {graph:3,recipient:1,next:12}`.

All three are externally given this *identical role-free loop template*, modulo their ID and ring predecessor/successor. Roles/permissions and copying are **investigator-engineered**. No fitness, repair, reproducer, owner preference, magic address book or damage detection is present in the host.

Nodes 12..19 are eight random CIL instructions sampled with the 64-bit LCG `state=state*6364136223846793005+1442695040888963407` wrapping u64, with initial state=seed; draw one `random()%6` per node, no outcome-dependent selection. Cases: 0 `Add{dst:1,lhs:1,rhs:2,next}`; 1 `Sub{dst:1,lhs:1,rhs:2,next}`; 2 `Read{dst:0,address:0,next}`; 3 `Write{address:0,value:1,next}`; 4 `Branch{test:2,nonzero:next,zero:index-1}`; 5 `GraphReadTag{dst:0,graph:3,node:2,next}`. Node20 is `Halt`. The tail is a **random continuation**, not a selected candidate.

## Common pretreatment fork
Run exactly 18 CIL attempts from initial world under deterministic round-robin order (six per process). No external compute pool added. Expect each process to have a nonempty graph object and published handle in shared cells 0,1,2, with read grants. If any process traps during the fixed first 18 opportunities, **study technical acceptance fails**; do not discard that seed or add host fixes after inspecting it. Capture the identical post-fork world serialized snapshot for every treatment. This initial scaffold success is a precondition and an **engineering calibration**, not a scientific result.

## Five matched conditions, 64 worlds × 5 = 320 runs
Each condition runs 12 independent laboratory epochs × 8 attempted CIL instructions per epoch (96 possible postfork attempts).
1. `control`: no intervention.
2. `sham-drop`: invalid researcher `DropGraph{graph:0}` at epoch0.
3. `target-drop`: researcher deletes lowest nonempty graph handle from the shared fork, expected handle1; record explicit external attribution.
4. `disable-owner`: researcher disables that graph's original process owner.
5. `zero-credit`: researcher sets every extant process's credits to zero at epoch0; CIL attempts must be zero.

All treatment arms begin with the same **preexisting engineered, already published** graph references; deleting such an object is a negative control on the guaranteed scaffold, not evidence of endogenous object discovery. Treatments are scheduled externally and cannot count as CIL-directed repair.

## Frozen outputs and rejection rules
- `scaffold_ready`: all 64 initial worlds have three nonempty objects, public references and expected grants at the pretreatment fork.
- `actual_cross_owner_copy` and `actual_owner_transfer`: only **successful** `GraphCopied` and `GraphTransferred` structured events **after the common fork** with the pre-action owners differing where specified. Failures/traps do not count. These expected successes are **scaffold calibration**, never emergent cross-owner cooperation.
- `indirect_process_effect`: control/target-drop final process-state inequality after excluding graph-object identity and external intervention log; count/64, report sham equivalence.
- `late_random_tail`: at least one successful CIL instruction after the prewritten scaffold node11; this remains a random-continuation activity proxy. Do not call it self-maintenance.
- `recovery`: **NOT EVALUABLE**. No script or prospectively specified independent functional marker exists; “another graph appears” is not repair.
- `organizational_closure`: **NOT EVALUABLE**. A three-way externally scripted circular exchange is not endogenous mutual regeneration.
- All 320 SQLite runs must verify exact replay, all 3,840 epoch snapshots hash-match and replay, strict world capacity holds, sham matches control simulated state and zero-credit attempts are zero. Any violated integrity rule invalidates the entire cohort. Report every trap, inert result and no-exposure case with 64 independent seed-worlds as the binomial denominator.

**Scientific stopping interpretation:** If generic exchanges occur, demonstrate mechanism reachability only. If not, this is an engineering/calibration failure requiring a documented correction and a new revision, not evidence that life cannot emerge. CL-1 positive acceptance still requires a *different, prospective* multi-component functional and internally initiated recovery experiment.
