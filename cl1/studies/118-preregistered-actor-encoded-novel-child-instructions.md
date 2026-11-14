# CL-1 Study118 — pre-outcome native CIL graph-instruction assembly calibration (NOT autonomous invention)

Study117 successfully demonstrated a strict **storage resource threshold**: same 56-op original human-coded CIL genome, 64 target seeds, capacity5 versus4, 50 true grandchildren per source expressing an activated **already prewritten** source7→Add1→Write10 function [source-verified original-only CI43617599460](https://github.com/iFrankStudio/cioc/actions/runs/43617599460). Crucial next barrier: code topology has so far only *activated an existing original executable module*. Before claiming de novo algorithm synthesis, first verify the native VM already allows a running CIL actor to build a child executable graph containing typed instructions **not present as executable instructions in its own parent code**, using ordinary source-owned graph copies and numeric instruction descriptors entirely written by the CIL actor. This is a ZERO NEW STOCHASTIC GENOTYPES, six-world HUMAN-CODED **substrate mechanism test**. Every graph-builder rule and descriptor format is authored by investigator; positive calibration never counts as spontaneous new digital life.

## Frozen original identical source parent CIL program

Use the existing VM GraphNew, GraphCopyNode, GraphReplace, GraphLaunch, Const, Read, Write and Halt instructions. New CIL instructions or a VM-level assembler/fitness primitive are FORBIDDEN. A single human-authored parent CIL builder program is identical in every condition, funded with exactly500 native local compute credits. Parent code must contain **ZERO Add and ZERO Sub executable opcodes**, and must NOT contain a complete source-processing Read7→Add/Sub1→Write10 program at ANY active original control path. It carries data descriptor-writing instructions, not the desired child's typed algorithm. Parent never receives external code edits.

At native PC0 allocate a fresh graph with GraphNew. Set the source index register to the parent CIL program's existing final Halt opcode index, then execute exactly SEVEN GraphCopyNode instructions to copy that one existing Halt node into positions0..6 of the parent-owned graph. This creates 7 real nodes from native graph operations, not an externally built or host-injected executable graph.

Then in a fixed human-coded CIL sequence, for each graph node index0..5, CIL itself generates and writes all FIVE descriptor words to shared numeric cells100..104 with ordinary Const→Write operations. GraphReplace reads that descriptor from those ACTUAL cells and replaces the corresponding owner-created graph node. Desired final *child* executable graph is:
- node0: Const R2=7,next1, descriptor [0,2,7,1,0].
- node1: Read R0,address R2,next2, descriptor [3,0,2,2,0].
- node2: Const R3=10,next3, descriptor [0,3,10,3,0].
- node3: Const R2=1,next4, descriptor [0,2,1,4,0].
- node4: **typed Add or Sub**, descriptor [tag,0,0,2,5], with opcode tag obtained by a real parent-native Read of its environmental data cell20. Tag1 means Add R0=R0+R2,next5; tag2 means Sub R0=R0−R2,next5. This is the first fully executable Add/Sub instance in the new graph even though **the parent contains neither Add nor Sub**. The opcode repertoire and numeric tag semantics were predefined by the VM and opcode tag comes from EXTERNAL input, not an organism invented math instruction.
- node5: Write address R3,value R0,next6, descriptor [4,3,0,6,0].
- node6: the originally copied Halt (not replaced).

Finally parent executes GraphLaunch to create real child process2 from its parent-owned modified graph, transfer exactly60 LOCAL native credits, then Halt. Child starts PC0, truly Reads environmental source7 and Writes previously absent shared output cell10 to source+1 (Add) or source−1 (Sub). Parent never itself writes output10; CIL output must be caused by child.

## Six mandatory fixed human controls and exact provenance

Cross **source input3 and8** with three external single-data-word values for cell20: **Add tag1**, **Sub tag2**, and **invalid opcode tag99**. These are exactly SIX human-scripted mechanism worlds, not a population of independent random genotypes. Parent source CIL program and initial local500 credits are byte-identical across all six worlds; the only external setup is SetCell7=source and SetCell20=tag. Descriptor shared cells100..104 and result cell10 are initially ABSENT. World is strict shared cell capacity16, maximum256 executable nodes per process, aggregate total storage1024, no public compute pool and no host replenishment.

Run at most250 native scheduler attempted instructions per world. For valid four worlds, assert:
- one real parent-native GraphCreated, seven actor-owned GraphNodeCopied from SAME parent Halt, SIX real GraphReplaced events with descriptor array matching all five actual actor-written words, and one real child GraphLaunched transferring60 credits; no host-spawned code/process.
- parental original executable contains NO Add/Sub, whereas child code node4 typed Add (tag1) or Sub(tag2) and exact complete 7-node child program and actual opcode desc.
- real child PC1 Read(key7,value=source) happens before real same-child PC5 CellWritten(key10,value source±1), and parent has no direct CellWritten(key10). Check both distinct environmental source inputs to avoid constant-output false positive.
- original compute source total500 and charges conserved exactly500−actual attempted instructions across parent/child and pool=0; child-generated data and graph code storage accounted for, no compute minting.

For both invalid tag99 controls: original actor must genuinely reach its fifth GraphReplace and TRAP on unknown descriptor type, with no native GraphLaunched, no child-process2 and no CellWritten(key10). All prior four legitimate GraphReplace events still counted and no host intervention rescues it. If output is not reachable or type replacement fails, report original null and fix only ordinary code errors in a clearly versioned later mechanism study; never call partial graph construction autonomous life.

**Scientific boundary:** Even if all six controls pass, the builder's descriptor layout, chosen math routine and assembler are investigator-written, and opcode tag comes from externally seeded data. The child executes a **newly COMPOSED instance of a predefined VM instruction type not executable in the parent**, not a novel VM primitive, autonomously invented algorithm, internally discovered goal, open-ended evolutionary novelty, stable self-reproducing lineage or life. No new VM creation or repair/fitness oracle. **CL-1 remains OPEN.**
