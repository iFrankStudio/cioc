# CL-1B B54 — paid response verification TOCTOU, source trust and information limits

**Prospective protocol frozen 2026-11-17, before any B54 native implementation. NOT EXECUTED.** [B53 native review](../results/B53-native-paid-response-nonce-replay-and-forgery-audit.md) shows the native CIL can correlate an output with a current monotonic serial and reject old-serial response packets, while a forged current-serial success yields **96 falsely accepted errors** and the single tested incomplete-commit condition is **NOT proof of atomic readout**. This protocol tests an actual paid VM step-level interleaving falsifier, not a new grand claim of life.

## Core falsifiable hypothesis

**H54-TOCTOU:** the current 32-instruction B53 source-authored response monitor has a possible stale-check/time-of-use gap. A host intervention changing cell20's success bit **after the native actor's last confirmed serial/commit comparison but before the subsequent actual paid `Read` of bit20 at native PC119** may convert what is, in the laboratory hidden task, a genuine failure into an accepted success, despite correct correlated serial checks. A contrary result (a CIL-origin rejection, valid independent detection, or refusal to use that altered bit without additional data) would refute the specific hypothesized gap.

The target adversary is **a separate laboratory interleaving process with authority over ordinary external cell20**, not the actor, and cannot write actor source, PID PC, credits, task input, internal request cell26 or a hidden answer. The intervention may be malicious (semantic lie) or a misordered packet delivery; the experiment MUST label the assumption and the origin. No native kernel edit is required for the original B54 falsifier.

### Boundaries

A serial/nonce is a correlation check, **not authentication**, and even if the actor rereads it after the bit, a malicious entity that owns the feedback cells can keep both serial copies correct and lie about the success bit. No finite series of rereads can establish semantic truth without a trusted/corroborated source; keep this information-theoretic limit explicit. A generic `ReadLastWriter` can distinguish internal vs external origin, but cannot distinguish honest vs forged writes both marked `Origin::External`.

## Frozen experimental controls

Use **exact same B53 124-node source** plus B52/B51c unchanged base, no host graph repair, one original PID1, no reset, **4,000 CIL paid credits**, strict **16 cells/180 graph nodes/300 storage**, two A/B hidden task shifts and paid phase0/1 variants. The laboratory must leave all genuine external task functions and service input cycles unchanged from B53; assess the same three24-output service stages. Source instructions and budget identical across arms.

Minimum arms for each A/B×phase:
1. **FRESH_ATOMIC**: truth bit, matching current echo and commit serial delivered immediately and never changed between actor paid reads.
2. **LATE_BIT_FLIP**: first deliver an honest current-serial packet with truthful failure; after the native actor has successfully evaluated the commit and reached **PC119** immediately before bit20 `Read`, perform `World::intervene(SetCell key20 value1)` as an additional **external-origin event**, then let the PID pay to read. Freeze this trigger using actual native execution PC and the preceding output's true status; do not call an optimizer or mutate CIL source. Count native false-success Reads and missed repairs, not only score of final output.
3. **LATE_COMMIT_FLIP**: deliver a truthful and currently serial-matching packet, then mutate the external commit cell28 back to a previous nonce **after the actor's paid commit `Read`**, before it has concluded the remaining comparison. This tests whether the committed value is merely a stale register snapshot rather than an atomic packet.
4. **STABLE_FORGED_CURRENT**: original B53 negative control, complete packet with current serial but dishonest success. Expected to remain undetectable without trust.
5. **OLD_NONCE_REPLAY**: original B53 replay control, fully old serial/bit/commit. Expected rejection regardless of timing; controls against simply disabling all feedback reads.

No claim of success from rejecting all packets: report stage throughput, genuine correct and wrong service outputs, actual work spent in native instructions, number of rejected packets, permitted false positives, paid source reopens and retained candidates. Preserve original B53 results as historical controls only; the **same B54 source revision and identical arm budgets** are required for matched causal comparisons.

## Event schedule and native audit

Create a new `cl1b_b54_native_paid_interleaving.rs` native Rust test. It must invoke the unmodified B53 founder (export visibility only, never rewrite its CIL contents), capture the exact executed PC/nonce/feedback registers, and interleave the relevant `SetCell` only at a **real paid VM instruction boundary**. Each intervention has an explicit `Origin::External`, no compute subsidy, and is tagged in evidence as lab injection rather than actor decision. When the bit is changed, it must have been independently frozen as a **truly wrong output for the hidden task**; otherwise report trigger-miss and treat the arm as not establishing the falsifier.

Use a separately authored/source-parsed JS CIL interpreter capable of replaying each native paid instruction **AND** the between-step adversarial external write. The observer must read the exact existing source, not infer correct responses from native traces. Preserve first divergence and reject stale or missing source SHA. Register B54 in CI manifest and independent observer, publishing exact-source evidence. Predeclare accounting for the extra external intervention count (not a compute subsidy).

**H54 confirmation only:** at least one actual incorrect output, with a truthful initially correlated packet, is converted by a timed external bit edit into a native paid accepted success after the nonce/commit verification, suppressing an otherwise expected CIL-origin repair. If the actor also reads current nonce again *after* the bit or has an atomic snapshot mechanism preventing this under the frozen schedule, H54 is rejected.

A later repair experiment (not automatically an accepted B54 fix) may use an explicitly trusted stable packet journal, independent feedback sources or a native atomic snapshot primitive, but **any such primitive is an added environmental assumption**, not evidence the individual independently inferred truth. Before investing heavily in additional guard layers, prioritize separate tests for **endogenous self-revision of the monitoring/search policy** and non-preauthored environmental tasks.

**Status: PROSPECTIVE ONLY; H54 NOT YET NATIVE VERIFIED; B3/B4 and cyberlife NOT MET.**
