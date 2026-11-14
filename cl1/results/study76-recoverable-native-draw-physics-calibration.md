# CL-1 Study76 — independently verified generic native Draw shortage/atomic pool-fee calibration

**Status: human-authored VM substrate calibration PASSED; ZERO independent ecological genomes and NO LIFE.**

## Source and original native evidence

- Pre-implementation registration `cl1/studies/76-preregistered-recoverable-native-draw-calibration.md`, committed before new VM code and fixture sources.
- Source-pinned complete [GitHub Actions43582985412](https://github.com/iFrankStudio/cioc/actions/runs/43582985412) **SUCCESS**, exact tested source revision `c366ecacff9bf5c918ef129a052b70301f4b2449`. Artifact `cl1-study76-recoverable-draw-physics-fixtures` ID `14108613754` retains the unchanged six-fixture original SQLite plus independently produced complete JSON. All **144 original SHA256 epochs**, native event ancestry/PC, credits-after, external world-pool interventions and one-credit conservation audited. Complete Rust workspace tests, strict Clippy and format passed.
- Introduced **disabled-by-default** world physics flag `recoverable_draw_shortage`, serialized only when true. On a *valid* native `DrawCompute` for which the existing common pool cannot cover the request, one native **charged** instruction produces `DrawDeferred {requested,available}` rather than permanently stopping; PC remains at Draw. If both this flag and the earlier external `pool_paid_draw` are on, a zero-local-credit Draw may execute only when the world pool already contains the requested energy **plus the one-credit entry fee**, preventing early-fee consumption that otherwise leads to fatal trap. Source/ABI of all original CIL ops unchanged. Invalid operand, invalid edge and other native errors still trap, as do all legacy worlds with flag off.

## Six preregistered human-scripted controls

| Human-authored original condition | Charged native attempts | Real native shared Write | Native DrawDeferred | Fatal trap | Pool-paid native entry |
|---|---:|---:|---:|---:|---:|
| Legacy: short transient pool, later +8 public credits | 2 | **0** | 0 | 1 | 0 |
| Recoverable: **same initial code**, transient shortage and +8 public credits | 6 | **1** | 1 | 0 | 1 |
| Legacy pool-paid: fee consumes insufficient pool before native Draw | 1 | **0** | 0 | 1 | 1 |
| Recoverable pool-paid: wait for fee+native Draw reserve, then +8 pool | 4 | **1** | 0 | 0 | 1 |
| Recoverable with empty world pool | 0 | **0** | 0 | 0 | 0 |
| Recoverable with malformed negative Draw operand | 1 | **0** | 0 | 1 | 0 |

All externally added credits are only put into the **world common pool**, never directly into the actor. The failure mode and later native success are independently verified as actual native events, not inferred from only final cells. Resource conservation holds exactly: initial process credits + initial world pool + external pool input minus native instruction attempts = final actor credits + final world pool, including all failed resource requests. Process code, source port, permissions and new flag are intact throughout.

## What is proven and what remains open

The Rust/CIL VM now supports a **generic nonfatal resource-request / credit-conserving retry law** that demonstrably changes behavior in a controlled human-scripted program without targeting any process's code or survival status. The law is **external researcher-designed environment physics**, so the native Write result is only mechanism calibration. It is **not** an organism discovering the retry, repairing itself, independently generating energy or creating descendants.

The already prospectively frozen new-seed [Study77 protocol](../studies/77-preregistered-recoverable-native-resource-feedback-ecology.md) must now test whether this general physics causes *genuinely sampled stochastic* CIL code to maintain native information-sensitive output and native resource uptake across W2+W3 beyond original local64 credits, relative to unchanged legacy pool-paid law. Full negative controls and all original 128-genome denominators remain essential.

**CL-1 OPEN.**
