# CL-1B / B17 — exhaustive functional-challenge evaluator mutation robustness

**Prospective protocol: 2026-11-15, before authoring B17 scanner or examining its results.** This is a systematic robustness falsifier of the **researcher-authored** B16 challenge-checker, not a life-origination claim, an agent-proposed mutation, or native validation.

## Question

B16 replaces B10's separate success marker with CIL-issued nonzero challenge17, internal deletion of output11 and readback of the **actual** payload cell. Under all possible **one-edge mutations** of B16's 32-node executable graph, how often does the checker nevertheless **accept without performing its own challenge payload write** or **reject despite doing that write**? Do these faults arise only where the researcher deliberately chose a checker-bypass, or over a broader legal mutation space?

## Frozen source and mutation universe

Read exactly the committed `fn founder()` in `cl0/crates/world/tests/cl1b_b16_functional_challenge.rs`; do not embed a redesigned founder. There are 32 CIL nodes: 29 with ordinary mutable `next` edges; one Branch node16 and two Halt nodes22/23 have no `next`. The laboratory enumerates each of the **29 × 31 = 899** one-edge changes `(node, alternate_next)` excluding the original edge for that instruction, across all five fixed original-user inputs `[-13,0,7,23,101]`: **4,495 mutant worlds**, plus five intact baseline worlds.

Every world externally loads one identical founder, seeds input key7, externally patches one `next` edge *before* native execution (or no patch for baseline), gives the individual 240 local credits and strict capacity (16 cells, 64 max nodes, 128 total units), then runs **at most 120 charged CIL instructions**. No after-start host edits or rescue. All traps, halts and early stops stay in the denominator.

## Primary evaluator metrics (actual decision, actual work)

An **epoch** is delimited by an internally attempted PC0, which is the founder's CIL trial setup. An **actual challenge payload** is `Origin::Internal(1)` `CellWritten(key11,value17)` executed at **PC11** since the most recent PC0. Ordinary work at PC31, even if correct, is a different observable and **never substitutes** for the challenge payload.

- **False positive:** a native execution of PC16 `Branch` takes success edge24, even though **no PC11 payload write to key11 with value17** occurred in the current epoch. This is an *actual decision*, not merely a possibly bypassed PC13 read. Count per-world incidence and total events.
- **False negative:** a native PC16 `Branch` takes repair edge17 after one or more legitimate PC11 challenge writes in the epoch. Count per-world incidence and events.
- **Bypassed evaluator:** a native PC13 read may occur but no downstream PC16 branch consumes its value before a new PC13/PC0; record separately. A PC16 branch may also be executed without a preceding genuine PC13 read. These constitute additional evidence-integrity defects, but not necessarily the primary false-positive criterion.
- **Structural output mismatch:** PC11 produced at least one genuine challenge in the 120-attempt window but no actual PC16 success branch ever occurred, or PC16 accepted at least once despite no genuine PC11 challenge write in the whole window. This whole-window indicator is secondary and may be censored by 120 attempts.
- Preserve wrong-key/wrong-value writes, traps, credits, actual repair edits, marker-free direct evidence and fresh-probe deletion attempts; do not classify readback value alone as a successful service.

The native event stream records PC, `Origin`, and `CellWritten`. To distinguish whether PC16 branched to24 or17, the Rust experiment must examine the **actual process PC immediately after** the charged PC16 step, rather than guessing from the later event trace.

## Controls and scientific interpretation

- All five intact founders must have at least one actual PC11 payload, PC16 success branch and **zero** false-positive/negative decisions.
- The single externally altered source edge **node10.next13** from B16 is a known human-selected *positive false-acceptance control*; the full mutation set must include it without special-casing it as success.
- Inputs represent deterministic conditions under the same 899 underlying mutation mechanisms, **not 4,495 independent mutation types**. Report distinct mutated edges with any false positive, input dependence and exact per-input strata.
- Trap/early stop is a failure condition and is counted even if the program never reaches the checker. Do not report “low failure rate” by burying unrunnable mutants in the denominator without separate **evaluator-reached** counts.
- This mutational robustness rate **must not be directly compared as an organism fitness measure** with B13: B10 and B16 have different graph lengths, instrumentation, work per cycle and diagnostic objectives. A relative claim requires a separately preregistered cost- and fault-matched design.

## Evidence and gate

First implement a source-parsed independent restricted interpreter that enforces register bounds, valid edges, storage limits, halted/trapped accounting and actual branch transitions. Then implement a native Rust full mutation screen or deterministic stratified native differential of the same **frozen** universe. Both must preserve unverified status until actual native CI evidence is retrievable. No world or code change may be silently excluded based on whether it helps the hypothesis.

The robust-life question remains **unresolved** even if B16's direct challenge is better than the old marker under selected faults: the actor must generate and validate *its own* diagnostic policies with resource-paid trials, rather than follow a fixed researcher-coded evaluator.
