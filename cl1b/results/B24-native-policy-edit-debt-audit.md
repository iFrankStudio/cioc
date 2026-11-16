# CL-1B B24 — paid executable policy editing can create worse resource access

**Status:** native B24 targeted PASS at [GitHub Actions run 43780673995](https://github.com/iFrankStudio/cioc/actions/runs/43780673995), tested SHA `9fc76c40b8c2fb89bd5fa98fcfa0f8198d0ae19d`. [Source-SHA-pinned native/observer summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/9fc76c40b8c2fb89bd5fa98fcfa0f8198d0ae19d-43780673995-1/summary.txt): **one selected native B24 test passed**, **11/11 independent analyses passed**, no failures; **changed-only scope**, not full 25/25 verification. [Prospective protocol](../studies/B24-online-request-edit-compute-debt.md) · [native Rust test](../../cl0/crates/world/tests/cl1b_b24_online_request_edit.rs) · [independent source-parsed 12-world interpreter](../analysis/b24_online_request_edit_scan.mjs).

## Frozen equal-cost conditions

One founder with **eight CIL nodes, 29 local compute credits**, zero world pool, strict capacity, external neutral world-pool supply. All three policy conditions keep **identical per-instruction compute costs**: (a) CIL-native PatchConst enabled, initial requested Draw4; (b) *same source* but native code edits disabled, still charged, initial Draw4; (c) researcher-fixed Draw3 with edits disabled. Four preregistered neutral packet sequences include development `[5,4,4]` and three held-out supply variations. 3×4 = **12 deterministic worlds**, not twelve independent randomly sampled environmental types.

## Exact actual-output counts

The positive endpoint is internal PC5 `CellWritten(key9,value1)`, not an internal claim that the source policy improved.

| Neutral world-pool packet schedule | Native CIL edit enabled, starts request4 | Same CIL, edit suppressed, request4 | Static request3 with same eight-opcode overhead |
| --- | ---: | ---: | ---: |
| Development `[5,4,4]` | **2** | **2** | 1 |
| Held-out H1 `[5,5,5]` | **2** | **3** | 1 |
| Held-out H2 `[4,4,4]` | **2** | **2** | 1 |
| Held-out H3 `[3,3,3,3]` | **2** | **2** | 1 |

The native test asserts **exact service outputs** in all arms, as well as the end-of-experiment pool balance, compute conservation, externally sourced packet credits, no process births/traps, source code literal and actual edits/suppressed edits. The source-parsed independent interpreter predicts the same complete matrix. This is a case of **no generalization advantage: zero wins, one loss and three ties among the four distinct resource schedules**. These are fixed local resource schedules, **not a population estimate**.

## Why actual source self-modification is still not adaptive selection

The native CIL actor executes a real source mutation: it calculates 4−1=3 in a register and `PatchConst` modifies the source code at PC3 from request4 to request3. After the next Draw3 it calculates3−1=2 and modifies the same literal again. **The mutated program state is real**, not an observer-side fiction. Nonetheless:

- The subtraction1, target node3, timing and choice to always patch after service are **explicitly preauthored**, not chosen from environmental feedback.
- Changing request to3 reduces the amount of local energy acquired per draw **below the cost of the existing Write+Sub+PatchConst+Const work cycle**. After its second output it becomes stuck at **PC3, an ordinary `Const`, with zero local compute**; further world-pool credits cannot run this instruction. In H1 it leaves6 world-pool credits unused.
- With edits suppressed, the actor continues requesting4, staying at pool-payable PC4; under the H1 supply it produces **three outputs** against the self-modifying actor's two, and leaves no compute unused.
- The fixed low-request3 control also gets stuck at PC3 because it still pays for the same unnecessary arithmetic and patch attempt; this is an **overhead effect**, not proof that Draw3 is intrinsically a worse request. The separate B23 streamlined five-node static request3 control is dramatically more efficient under its own protocol.

**Science interpretation:** Native self-editing is expressive and verifiable, but **more edits can reduce genuine work and cause permanent resource starvation**. To claim self-improving adaptation, an actor must itself infer whether a candidate is worth its compute and edit costs, remove unproductive evaluator overhead, and preserve access to the resource-consuming/earning primitive. B24 has no spontaneous mutation discovery, self-originated fitness, open evolution, or CL-1B life acceptance.
