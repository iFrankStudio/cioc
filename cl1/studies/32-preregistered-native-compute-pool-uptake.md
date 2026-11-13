# CL-1 Study 32 — preregistered independent native compute-pool uptake ecology

**Registered 2026-11-14 BEFORE new runner, seed execution or results.** Prior Studies 22, 27, 29, 30 and 31 are known. Study22 demonstrated that *external* compute refills yield late activity; Study31 showed a two-world source-tag-to-output-address register chain in an independently sampled *single-source* context. Neither implies autonomous energy supply. Study32 asks whether ordinary native CIL compute-pool operations become **successfully executed under a random continuation** without an externally scripted refill to that process during the observation window.

## Immutable generator and ecology

Exactly **192 new independent seed-worlds `2,500,000..2,500,191`**, disjoint from all prior cohorts. Copy the previous Study29 base generator verbatim except for **two explicitly specified random opcode substitutions**. Same three process IDs 1–3, externally authored nine-node neutral scaffold, prefork 27 charged attempts, owners/readers, strict storage limits (cells32/nodes40/total192), 56 initial credits per process, 18 continuation nodes with one frozen 64-bit LCG `random(state)%12` draw per node, sequential next edges, 27-node program + Halt. Fixed continuation mapping:

0 `GraphCopyFrom{target:3,source:1,node:2}`; 1 `GraphCopyNode{graph:3,source:2,dst:2}`; **2 `DrawCompute{amount:1}`**; 3 `GraphReadTag{dst:0,graph:3,node:2}`; 4 `GraphReadTag{dst:2,graph:1,node:2}`; 5 `Add{dst:0,lhs:0,rhs:2}`; 6 and 7 `Write{address:0,value:3}`; 8 `Const{dst:2,value:0}`; 9 `Branch{test:1,nonzero:index-2,zero:next}`; **10 `ReturnCompute{amount:1}`**; 11 `GraphGrantRead{graph:3,reader:1}`.

The original prefork process register1 holds the circular predecessor ID, a *known strictly positive constant* `2/3/1` for process1/2/3. None of the continuation instructions modify register1; native successful `DrawCompute` transfers that many compute units *from world pool to process*, whereas `ReturnCompute` transfers them back. Each successful instruction itself costs one native compute credit. The program **does not get a repair, fitness, evolution or energy-foraging objective**. Code and operand selection are authored by the experimenter; any successful resource movement is *native execution of a supplied primitive*.

**Initial world pool=48 compute units**, supplied **externally** with `Intervention::SetComputePool{credits:48}` **before the 27-step prefork**, and included in every same-seed common initial hash. Note: this is an experimenter endowed environment reservoir, NOT resource regeneration. Initially each process still has 56 credits, 47 after common prefork.

## Frozen six matched branches

For **24 epochs ×12 charged native instruction attempts**, with windows W1=0–7, W2=8–15, W3=16–23, run **6 branches per independent world** (**1,152 paired runs, 27,648 hashed SQLite epoch snapshots**):

1. `control`: same prefork state, initial pool48, no external postfork interventions.
2. `sham`: externally rejected `DropGraph(0)` before epoch0; simulated state must match control.
3. `empty-pool`: external `SetComputePool(0)` at epoch0, removing the remaining pool; NO process-credit refill. Successful later native Draw in this arm is possible **only if native ReturnCompute has already replenished the pool**, and is recorded as resource *recycling*, never new energy.
4. `refill`: pool48, externally `SetCredits(process,47)` in ID order at epochs8 and16, with no process restart.
5. `empty-pool-refill`: pool externally zeroed at epoch0 and processes externally refilled at epochs8,16; resource provenance is separately recorded.
6. `zero-credit`: `SetCredits(process,0)` for each process ID1–3 at epoch0, no further intervention; no internal CIL executions.

There is no graph ablation, graph restoration or output-target fitness intervention. Initial resource budgets intentionally differ in `empty-pool` and `refill`, **so late activity contrasts measure resource availability under these interventions, NOT matched-energy effect of a hypothetical native maintenance trait**.

## Frozen registered endpoints and controls

For each branch and W1/W2/W3, report distinct independent seed worlds with (a) successful positive `DrawCompute` from the external-or-recycled pool (native `Origin::Internal(actor)` `Executed`, source program PC opcode `DrawCompute`, positive known operand); (b) successful native `ReturnCompute`, (c) any charged native attempt, (d) successful source2 `GraphTagRead` and source2-read-before-write temporal opportunity, and (e) native per-process attempted instructions in that postfork window. Additionally report paired discordances for `control` vs `empty-pool` on any successful Draw in W1 and W3 charged activity; do not treat the 1,152 branches as independent worlds.

Strongest narrow native resource test: a given process in an arm **without postfork externally refilled process credits** executes **more than its original 47 postfork charged instruction attempts** (since no `TransferCompute` opcode exists in this grammar), and a successful native positive `DrawCompute` occurred in that actor. Count distinct seed-worlds and actual actor examples. Merely calling DrawCompute successfully at fewer than 47 attempts is a resource *operation*, **not evidence that its execution life was prolonged**. If programs trap, die or stop before such usage, report a real zero.

**Independent pool ledger:** starting from each branch's actual prefork `compute_pool=48`, replay epoch0 external SetComputePool0 if specified, then reconcile all native **successful** Draw/Return events in *native tick order*, with exactly quantity `2,3,1` depending on process ID. Failed/trapped native attempts must change no pool quantity. At every one of the 27,648 stored epoch snapshots, reconstructed pool must match serialized `compute_pool`, never go negative; this plus source-pinned full Rust epoch/world replay and independent exact schedule/hash/capacity/zero-credit/sham verification is required. Native compute transfers conserve inventory, subject to 1 credit consumed per native attempt. A no-pool arm can recycle through a **successful Return**; do not falsely demand permanent zero pool.

## Interpretation and stopping

Report all seeds, including inert/trapped/negative outcomes, and Wilson confidence intervals for independent seed-world proportions. This experiment is a **resource accounting/availability and CIL primitive uptake test**, not a prospective two-component functional maintenance or independent life test. The initial 48-unit world pool, initial code, process authorization and branch policy all come from the researcher. Pool recycling does not create energy and finite pool use cannot imply open-ended sustainable life. Strict CL-1 scientific acceptance still demands internally formed causally necessary multiple components, function loss after actual damage, source-verified endogenous repair over two separated late windows, paired external functional restoration and negative controls. **CL-1 remains OPEN** regardless of result.
