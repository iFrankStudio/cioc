# CL-1B B56-F1d1e — strict same-PID opcode-origin reachability gate (preregistered, prospective)

**Frozen before F1d1e native results.** This is a mechanism-falsification protocol, not an achieved continuous individual, adaptation, B3/B4 or cyberlife. Starting evidence: F1d1d scoped native run 44259124488 (SHA f78b8f6d2403c18465c7d4e72067d17b8ff4921d), 16 selected native tests and 54 independent analyses; two constructed Branch(test1) nodes executed exclusively by new PID2. F1d1c's unchanged F1c founder has only Branch(test3), no GraphEncode.

## Falsifiable hypothesis and adversarial null

H1: under the **unchanged** Rust VM instruction semantics, an existing PID1 can pay to create a Branch(test1) that was absent from its initial executable source and subsequently **execute that new opcode as PID1**, without an external code replacement, process relabeling or a new PID. H0: available paid graph construction and mutation instructions cannot transfer an arbitrary newly encoded graph opcode into PID1's own executable code; GraphLaunch necessarily executes it in a fresh PID, while direct PID1 PatchNext/PatchConst/CloneNode preserve opcode type.

A counterexample must supply an exact finite CIL instruction trace from a one-PID source with no Branch(test1), including all register values, graph ownership, every paid operation, original PID and final executable opcode at the PC actually executed. A source-only suggested route does not reject H0.

## Source semantics gate before any cohort

Read the exact current cl0/crates/world/src/lib.rs, pin its blob SHA, and enumerate **every** assignment to `process.code.nodes` or process executable source and all executable-code mutation dispatch arms. Check PatchNext, PatchConst, CloneNode, GraphNew, GraphEncode, GraphReplace, GraphCopyFrom, GraphTransfer and GraphLaunch separately; in particular distinguish edits of graph-object nodes from edits of a process's executable nodes. Check whether any instruction copies a graph-object opcode *back into the same PID's executable source*, whether external Intervention can do so (excluded from H1), and whether GraphLaunch allocates a distinct process id. No inference from opcode names alone.

If this exhaustive source audit proves no same-PID opcode-type change is possible, publish a bounded *reachability impossibility certificate* with exact semantics and preconditions. It is not a proof that other founder grammars, PID-spanning lineages or future VM revisions are impossible. Do not add a kernel primitive merely to force H1 to pass.

## Native strong controls, if source gate warrants them

Run strict `World::with_strict_capacity(16,180,300)` and equal finite initial credits in isolated deterministic worlds: (A) PID1 creates and GraphLaunches a novel Branch(test1), demonstrating child PID2 positive construction; (B) same source with GraphEncode disabled, demonstrating absence of generated Branch; (C) PID1 PatchNext/PatchConst/CloneNode attempt to alter opcode type, expected only source-type-preserving edits; (D) an *explicitly separately labeled* externally initialized PID1 Branch(test1) positive execution control, which must **not** count as endogenous construction. No host-injected graph edits, PID renumbering, free storage or extra credits. Report all stopped/trapped worlds and paid instructions. The controls distinguish construction, execution, identity and external preassembly.

Before implementation freeze the exact event schema: initial source op tags, per-step PID/PC/op tag, code-node tag snapshots at each paid mutation, graph-object tags, credits, data cells, occupied storage, graph ownership, launch events, and actual Branch execution PID. Independently reexecute the Rust event stream in a separately authored JS observer; first divergence must be null. Reject any purported success if a new opcode is executed only by PID2.

## Scientific decision

Accept H1 only for a **native-executed same-PID** counterexample with independently replayed paid provenance. Otherwise preserve the negative result and investigate PID-spanning lineage continuity as a **different explicitly preregistered organism identity definition** with measurable inherited state, credit continuity and failure controls. The old B51c feedback chain and strict original F1c origin constraints remain separate; this preflight cannot satisfy N56-A/B/C or B3/B4.
