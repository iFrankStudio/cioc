# B49 — independent dynamic interpreter acceptance forecast, not yet native verified

**Frozen before the first B49 native CI run.** [Preregistered protocol](../studies/B49-independent-dynamic-native-trace-replay.md) · [real Rust trace emitter](../../cl0/crates/world/tests/cl1b_b49_dynamic_replay.rs) · [independent dynamic JS CIL machine](../analysis/b49_dynamic_native_trace_replay.mjs).

Source model assumptions are deliberately narrow: same actual B42 38-node founding CIL graph and four-op static comparator, no privilege to inspect the observer-only Rust event history from within CIL, graph-edit operations `CloneNode` and `PatchNext` as real charged native instructions, and ordinary task cells with no source authentication. B49 does **not** change the VM or add a proposal algorithm.

**Frozen input plans:** eight previously unseen B48 sequences and four new B49 sequences as enumerated in the protocol, crossed with two phase offsets and four feedback policies = 96 worlds and 288 external input-transition records. Every actual native attempted CIL instruction is counted, including early exits. The exact total number of paid steps is to be independently measured; **do not silently impute 144 attempts for stopped BLIND_ZERO worlds**.

**Frozen expectations:**
- Realized attempts must be 144 per continuing TRUE, BLIND_SELF and STATIC world. BLIND_ZERO's early stops remain included and must be reported.
- The independent JS VM must reproduce **each actual native PC, event type and payload, registers, shared cells, executed node/edge changes, credit balance, tick and occupied storage**; not just aggregate outputs or source patterns. First mismatch is a hard failure.
- B48 existing unseen source replication should give original raw holdout true/wrong counts for base TRUE `64/15,64/16`, base BLIND_SELF `18/54,20/60`, base BLIND_ZERO `0/9,0/10`, base STATIC `96/0,96/0`; phase+1 TRUE `64/15,63/10`, BLIND_SELF `18/54,20/60`, BLIND_ZERO `0/9,0/10`, STATIC `89/7,89/7`. These are **previous B48 native observations**, not claims of independently forecast values.
- New B49 prospective sequences `[2,0,-7]`, `[-4,31,0]`, `[0,7,-4]`, `[31,2,7]` are included regardless of outcomes, with no numerical answer selected in advance. Successful parity on those cases still does **not** establish B3.

**Status at registration: NATIVE CI NOT RUN; INDEPENDENT DYNAMIC PARITY NOT VERIFIED.** H48 remains not accepted; any B49 pass only strengthens measurement fidelity, not the organism's ability to autonomously invent the founder's mutation/feedback policy. **B3/B4 NOT MET**.
