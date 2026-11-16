# CL-1B B46 — model-only same-PID writer race forecast, native pending

**Frozen before first B46 Rust CI**, using the independently interpreted native B42 source with B45's four ordinary charged provenance-guard instructions and an investigator-installed one-time native CIL post-read write. This is NOT native verification.

[Prospective 56-world protocol](../studies/B46-same-pid-writer-race.md) · [Rust B46 native source](../../cl0/crates/world/tests/cl1b_b46_same_writer_race.rs) · [independent source-parsed CIL scan](../analysis/b46_same_writer_race_scan.mjs).

The **first paid CIL value Read at PC11** returns the externally supplied diagnostic challenge in all spoof worlds. In the same-PID **NativeWrong** variant the native actor then really writes **zero** to output cell11 using a researcher-seeded ordinary CIL Write at PC42; only after this does the original paid B45 provenance reader at PC38 see the **truthful actor PID1**. Its existing authored guard accepts the previously read challenge and retains the original defective PC10 service code, despite the *current* output differing from challenge in **six** nonzero-challenge worlds. In **NativeEcho** it genuinely writes the correct challenge and obtains the same source tag, but the temporary correct Write is not a guarantee that the steady router remains correct. This is source-authenticated yet functionally misleading evidence, without a forged source tag.

The experimenter's identical one-time external PC11 edge restoration in all five extra-instruction arms prevents the added Write from becoming a permanent service instruction. The four trust-control arms distinguish the cause: an equally paid post-read no-op with no new native write, an external post-read wrong-value write (source remains0), the same control with no original forgery, and the original B45 guard without an extra instruction. The compact four-op correct copier remains the strong resource-matched comparator.

**Model forecast: eight worlds per arm, each128 charged CIL instructions.**

| Arm | Training correct/wrong | Heldout correct/wrong | New clones | First PID1 source reads | First external0 source reads | False current-value approvals |
|---|---:|---:|---:|---:|---:|---:|
| NativeWrong | 14/42 | **0/104** | 0 | 8 | 0 | **6** |
| NativeEcho | 14/42 | **0/104** | 0 | 8 | 0 | 0 |
| NopSpoof | 24/0 | **104/0** | 8 | 0 | 8 | 0 |
| OutsideLateWrong | 24/0 | **104/0** | 8 | 0 | 8 | 0 |
| NopGenuine | 24/14 | **72/26** | 6 | 8 | 0 | 0 |
| Original B45GuardSpoof | 24/0 | **96/8** | 8 | 0 | 8 | 0 |
| STATIC_CORRECT | **128/0** | **128/0** | 0 | 0 | 0 | 0 |

Interpret the no-op's `104/0` **only as a fixed-phase scheduling / instruction-count confound**, never as autonomous improvement: one charged no-op can shift which outputs are counted in each 64-step task phase. The same-PID native written data and tag are accurate; the *decision using an earlier value + later source is invalid*. Neither identity nor one real successful write proves competence of retained code.

The exact seven vectors of 13 integer aggregates are frozen in B46's native test. The future CI outcome must match them *and* all old ABI/CL-1B targets before B46 becomes native verified. **B3/B4 NOT MET.**
