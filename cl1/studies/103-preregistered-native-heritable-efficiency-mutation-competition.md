# CL-1 Study103 Phase0 — identical founder CIL genome, native efficiency mutation, and finite-pool reproductive competition

**REGISTERED BEFORE SCRIPTED EXPERIMENTS. ZERO random ecological initial genotypes.** After source-verified engineered complete self-copy (99), neutral inherited native edits (100), frozen hand-authored fast-versus-slow founder competition (101; F6:S4 with finite common pool; CI43606097528), and independent engineered sibling branching calibration (102), the missing composite mechanism is whether a **true native executable mutation, occurring before CIL whole-genome copying**, can yield descendants that carry a resource-competitive functional advantage without a host directly inserting a different child code genome.

## Identical original 24-opcode program, entirely existing VM primitives

Every founder starts with exactly the **same serialized 24-node executable CIL genome**. Initial program:
0 Const(R2,7)->1; 1 Const(R3,9)->2; 2 Read(R0,key R2)->3; 3 Write(key R3,R0)->4; 4 Branch(test R1, nonzero21, zero5); 5 Const(R2,1)->17 [original **slow** edge]; 6 Const(R1,0)->7; 7 Const(R3,24)->8; 8 GraphNew(R0)->9; 9 GraphCopyNode(graph R0, source R1, dst R1)->10; 10 Add R1=R1+R2->11; 11 Sub R3=R3-R2->12; 12 Branch R3 nonzero9 zero13; 13 Const(R2,200)->14; 14 DrawCompute(R2)->15; 15 GraphLaunch(graph R0, credits R2, dst R3)->16; 16 Halt; 17 Const(R1,20)->18; 18 Const(R2,1)->19; 19 Sub R1=R1-R2->20; 20 Branch R1 nonzero19 zero6; 21 Const(R1,5)->22; 22 Const(R2,6)->23; 23 **native PatchNext(node R1, target R2)->5**.

The only original code path to native PatchNext is BranchPC4 when the actor's initial R1 is nonzero. The parent assigned R1=1 executes PC21-23 and natively changes its own **functionally executed** PC5.next from 17 to 6, avoiding the otherwise costly 42-instruction delay (PC17-20). The other founder assigned R1=0 takes PC4.zero ->PC5, leaving slow edge intact. Both then execute ordinary native 24-node GraphCopyNode whole-genome copy and native public-resource Draw200/GraphLaunch200; all native children start with **zero registers**, skip the mutation branch, and therefore inherit the faster/slower PC5.edge, in perpetuity. Source key7 low3/high8 and output key9 are identical between variants; both must truly run ReadPC2→WritePC3 and transmit that function.

The R1=1 mutation trigger is explicitly **researcher-provided initial process register state**, NOT endogenous stochastic variation, natural adaptation, fitness sensing, environmental preference, or AI genotype creation. The genome is nonetheless genuinely changed by an ordinary internally executed CIL PatchNext and copied into a child by native CIL before any child exists. All founder *code bytes* are identical, though initial process registers are intentionally unequal.

## Exactly ten frozen human-scripted controls

For each independently initialized external source input 3 and8, run each named condition:
1. `mixed-trigger1`: founder process1 initial R1=1 (will mutate to fast), founder2 R1=0 (slow).
2. `mixed-trigger2`: founder process1 initial R1=0, founder2 R1=1; reverse scheduler-ID bias.
3. `both-untriggered`: both R1=0, no internal executable mutation.
4. `both-triggered`: both R1=1, two real internal mutation events.
5. `mixed-trigger1-no-pool`: same two starting code/genotypes/registers as mixed-trigger1 but pool0, exposing fertility/resource dependence.

Each original `Process::new(1|2, identical CIL, 200)` receives only the indicated initial R1 externally before `World::load`. World is `World::with_strict_capacity(4,64,1024).with_pool_paid_draw().with_recoverable_draw_shortage()`; one shared externally initialized source key7; shared world pool exactly2000 or0; no other code/data host changes. `World::run(2600)` native attempt cap. One native attempt uses one real compute credit including native DrawDeferred. Original total inventory=400+pool, and final inventory must equal original minus actual charged native attempts. No individual actor replenishment or host mutation/reproduction.

## Frozen observer hierarchy

- **M:** exactly one native internal `ExecutableEdgePatched{node5,before17,after6}` for each triggered founder, attributable PC23; exactly none for untriggered founders and all descendants. No additional code site changes.
- **H:** descendant executable code has exactly same 24 nodes as founder code after its own mutation, including PC5.next=6 for mutant lineage and PC5.next=17 for unmutated lineage. Actual `GraphNodeCopied(source_node5)` and `GraphLaunched` provenance; copied actor-owned graphs and child code are never externally injected.
- **F:** native source Read(key7) by actor at PC2 followed by real same-actor Write(key9,value=source) at PC3 in every launched lineage; source3 and8 expected to affect outputs but not relative fitness under this script.
- **R:** total `GraphLaunched` descendants per original founder, tracked through actual parent→child native genealogy. Report mutant descendants vs unmutated descendants for mixed-trigger1 and mixed-trigger2 separately, regardless of sign/tie; same-variant controls reveal scheduler/process-order skew; 0-pool must yield zero native offspring. Do NOT cherry-pick winners, change quotas, discard traps or post-select at successful Draw.
- **C:** externally supplied compute inventory, native attempted charges and every native Draw/Launch movement conserved. Report actual charged attempts, pool leftover, native DrawDeferred, genotype ancestry and full original source-variant counts.

There is **no prespecified claim that mutation is beneficial or that the mutant must win**; a negative, tie or counterintuitive result is scientifically relevant. This is still a **human-programmed directed efficiency mutation**, not a spontaneous adaptation: mutation trigger and code-mutating machinery were specifically authored; no random selection over a genuine genetic variant distribution. A confirmed mutant reproductive advantage would establish a composite mechanistic substrate (native heritable mutational functional variation + resource-dependent differential reproduction), NOT autonomous evolution, organization closure, or life. Future genuinely stochastic mutation/selection studies must independently freeze generator and report complete original population denominators. **CL-1 remains OPEN.**
