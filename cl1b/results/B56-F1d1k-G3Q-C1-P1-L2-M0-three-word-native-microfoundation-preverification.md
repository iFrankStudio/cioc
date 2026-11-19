# CL-1B G3Q-C1-P1-L2-M0 — source-paid three-word meta-interpreter microfoundation

**Research status (2026-11-19):** frozen exactly **66 original CIL nodes**, native Rust integration, four prospectively frozen founder variants, second independent source interpreter and focused CI registration have been committed. **Native PASS and native event-by-event independent replay are not yet verified.** This is a *separate fresh investigator-authored PID1* and **not** continuation of C0/P0/P1-L1's original executable source or completed C1.

## Why isolate this microfoundation?

The prior P1-L1 179-node source established a paid arithmetic dependency on one bank byte but had no room to execute the entire dynamically fetched three-word meta rule. This new 66-node standalone source isolates a required capability: real paid bank writes, bank reads, active bytecode-slot writes, three dynamically indexed opcode Reads, three paid accumulator checkpoints, RESET semantics, and a causally dependent `Write11`. All are carried by **one original PID1** per arm under original world limits `World::with_strict_capacity(16,180,300)` and 1000 initial credits. The one initial external PID1 load is allowed; subsequent host code/data writes are not.

**Source provenance:** exact [66-node founder and prespecified controls](../studies/B56-F1d1k-G3Q-C1-P1-L2-M0-frozen-66-node-full-three-word-executor-v1.json) committed first at `7e86bb7923de4815d58aaf801892d528b6319ade`; [original Rust integration](../../cl0/crates/world/tests/cl1b_b56_f1d1k_g3q_c1_p1_l2_m0_three_meta_word_executor.rs) committed afterward at `e1c43f4e251834892567d16e93911bd5e528364e`; independent [source interpreter and aggregate crosscheck](../analysis/g3q_c1_p1_l2_m0_source_replay.py) at `f5a8f79e77464f826a2ebe05e2483f50de31102d`; [focused CI registration](../../.github/workflows/cl1b-focused.yml) at `0481921ba8f7e2627ef313c9c1da4021ba6ae924`.

## Exact mechanism and negative/positive predictions

- PID1 pays to store researcher-authored `[2,2,2]` in bank51/52/53, and then pays to load and deploy those same three values to active slots62/63/64. This is **not certified by an internal safe-rule search**.
- The 66-node source runs a three-iteration **dynamic meta PC** through data cell50. At CIL PC30 it pays to fetch the active word at address62+metaPC; the CIL interpreter routes byte3 to **RESET accumulator0**, other bytes0/1/2 to **ADD modulo3**. Every executed meta word has a paid accumulator `Write56` checkpoint at PC51. It pays to emit the final output via `Write11` at PC64.
- Independently interpreted original: 116 paid instructions; reads `[(62,2),(63,2),(64,2)]`, accumulator checkpoints `[2,1,0]`, output0; 9 distinct used data cells, peak occupancy bounded by76/300.
- Researcher **pre-load** source-mutant controls at the first, middle or tail bank initialization (CIL PCs 0,3,6) must causally produce output **2,2,2** rather than original0, with paid steps **115,109,116** respectively. Middle byte3 must actually take the RESET path, reducing execution cost. Mutants are **not autonomous code edits** by PID1.
- Native test checks paid event origins, immutable native source, charged instruction/credit conservation, original PID1 only, all bank Reads, active Writes, 3 real meta opcode Reads, 3 accumulator Writes, one output and strict occupancy. Independent Python interpreter reexecutes the frozen CIL source and compares the produced native **aggregate** JSON if available. This is **not yet a per-native-event independent replay**.

## Important limitations

This result, even if native Rust validates it, establishes **bounded dynamically interpreted three-opcode functional causality**, not a single individual performing its own rule-space certification and subsequent task adaptation. The bank is researcher-selected, target is not hidden, and there is no environment task, regime shift, feedback, fitness authentication, learned rule replacement, self-created opcode, or autonomous structural self-modification. The four founder variants are independent externally authored pre-load experiments, not an individual evolving its own executable source.

## Next integration falsifier

A stronger **P1-L2-M1** must combine this already-small charged three-opcode interpreter with within-PID paid **coverage certification, retained safe-bank provenance and post-certificate deployment**, while keeping one PID, ≤180 CIL nodes, ≤16 data addresses, ≤300 storage, and immutable CIL source. Under source-identical controls, a certification gate should change the actor's **actual subsequent native opcode Reads/task choice**, not merely a report. If that integration cannot fit, document a precise liveness or capacity failure instead of letting the Rust harness supply a rule verdict or task choice. Hidden regime tasks and feedback-conditioned adaptation remain separate stronger milestones.
