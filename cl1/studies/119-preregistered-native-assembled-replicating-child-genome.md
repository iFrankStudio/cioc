# CL-1 Study119 — pre-outcome HUMAN-scripted construction of a *self-reproducing* new typed child program

**Six original investigator-constructed mechanism controls, ZERO independent random ecological genomes.** Study118 [CI43618534539](https://github.com/iFrankStudio/cioc/actions/runs/43618534539) proves a running CIL parent without executable Add or Sub can write numeric descriptions to shared cells and use genuine GraphReplace to assemble a new seven-op child that performs Read7→Add/Sub→Write10, but that child HALTS without reproducing. Study119 asks a strictly stronger substrate question: **can a child program assembled by a CIL parent from CIL-written descriptors itself copy ALL of its executable genome using ordinary GraphCopyNode, use real DrawCompute public credits, and GraphLaunch a grandchild which continues expressing the assembled math?** This is a HUMAN-SCRIPTED blueprint calibration; the child self-copy mechanism and numeric descriptors are investigator-designed, NOT an algorithm spontaneously synthesized by an organism.

## Exact parent and generated child code

One exact parent executable CIL builder program reused across all six original input cases; builder executable contains **no typed Add or Sub instructions at all**. Parent native GraphNew allocates a new graph; source register is set to index of the builder's own terminal Halt. Parent copies its own Halt exactly **18 times** with real actor-native GraphCopyNode (graph nodes0..17), without externally allocating child code. The builder then prepares five numeric descriptor words per graph node0..16 by repeated ordinary CIL Const + Write into shared cells100..104. For child graph node4 descriptor opcode tag is read from ordinary environmental cell20 by parent-native Read. It invokes **17 actual native GraphReplace** operations on the owner graph. Graph node17 remains copied Halt. Desired CHILD 18-node program:

0 Const R2=7→1;
1 Const R3=10→2;
2 Read R0 from cell R2→3;
3 Const R2=1→4;
4 **Add** R0=R0+R2→5 (opcode tag1) OR **Sub** R0=R0−R2→5 (tag2), created from parent-authored numeric descriptor, not a typed parent original instruction;
5 Write cell R3=value R0→6;
6 Const R2=1→7;
7 Const R1=0→8;
8 Const R3=18→9;
9 GraphNew R0→10;
10 GraphCopyNode graph R0, source index R1, dst R1→11;
11 Add R1=R1+R2→12;
12 Sub R3=R3−R2→13;
13 Branch R3 nonzero10 zero14;
14 Const R2=200→15;
15 DrawCompute R2→16;
16 GraphLaunch graph R0, credits R2, dst R3→17;
17 Halt.

The generated child therefore has three typed Add/Sub code sites that are NOT in parent original typed executable program: arithmetic at node4 plus explicit code-copy loop Add11 and Sub12. These code sites are all human planned, and original VM instruction repertoire already includes Add/Sub. CIL parent native GraphLaunch transfers **400 local compute credits** to child2, then original parent Halt. Child2 must independently read and compute source±1 and launch a real grandchild3 via its own 18-node complete code copy; grandchild3 must have exact identical executable nodes to child2, execute same source±1 and native reproducing code path. No host Process3 injection, no per-actor compute refills, no code splices. Further later descendants are permitted but not required; limit fixed execution attempts enough to observe PC0..17 in grandchild3.

## Six fixed original calibration worlds and environment

Source input7=3 or8 crossed with tag20=**1 (Add), 2 (Sub), 99 (invalid)**, fixed six worlds. Exact original parent builder source bytes identical across all, no seed selection. Strict world `World::with_strict_capacity(16, 1024, 10000).with_pool_paid_draw().with_recoverable_draw_shortage()`, original parent1 with exactly**1200 compute credits**, shared pool exactly**1000 compute credits** initialized once using external SetComputePool; input source7 and opcode tag20 externally set, descriptor cells100..104/output10 absent at t0. Max exactly**2500 native attempts**, no other host intervention, no external pool replenishment, explicit process-local charges; total credit inventory2200 minus successful native scheduler attempt count.

For four valid tag1/2 controls, require: parent original code contains zero typed Add/Sub; one genuine parent graph allocation, 18 actual native copies of own source Halt, all17 typed GraphReplaced with exact descriptor words produced by actor-native writes, one parent→child GraphLaunched with400 local credits and18 moved nodes; child2 and grandchild3 both execute source7 Read and Write10 source±1, with exact matching all18 originally assembled child code nodes across generations, child2 itself builds and copies18 nodes from its current executable source, executes its actual DrawCompute200 before transferring200 to grandchild3, all native event/process/PC attribution and conserved compute pool. Parent original code never directly writes output10. Both child2 and grandchild3 process IDs must derive from real GraphLaunched genealogy, not host Process::new. Invalid tag99 controls must trap at native GraphReplace of node4, after only four replaced graph nodes, and must not launch child or grandchild nor Write output10.

**No new VM syntax, graph assembler or fitness oracle allowed.** A positive proves substrate feasibility of cross-generational executable graph **assembly from data** even though initial parent lacks the child's typed executable operations. It does NOT prove spontaneous algorithm invention, autonomous mutator origination, non-directed genetic diversity, multi-component functional closure or life; the exact copying logic and math are scripted by the human parent builder. Later prospective stochastic programs may leverage this generic capability but must not treat this calibration as natural life creation. **CL-1 remains OPEN.**
