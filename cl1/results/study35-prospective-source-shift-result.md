# CL-1 Study 35 — prospective independent native source-conditioned address replication

**Status: CI-VERIFIED PROSPECTIVE RESULT, restricted mechanism replicated, NOT LIFE.**

## Frozen protocol, independent data and provenance

- Prior preregistration (before runner/analysis): `cl1/studies/35-preregistered-independent-native-source-address-shift.md`, commit `998c55cf7621348247df6d4b098bdd9ed9502cd7`.
- Exact executed revision: `9ee57f2536b8cd4eed2a84817f2e7e777d9bcc00`, fresh independent seeds `2,700,000..2,700,191`, **192 independent worlds**, nine matched treatments, **1,728 native executions** and **41,472 fully hashed SQLite epoch snapshots**.
- Independently verified GitHub Actions run: [43546825859](https://github.com/iFrankStudio/cioc/actions/runs/43546825859). Preserved downloadable artifact: `cl1-study35-independent-prospective`, artifact ID `14094670544` (complete SQLite and `cl1-study35-report.json`).
- Rust runner executed full 24-epoch replay per treatment, independent Study33-derived Python baseline audited the unmodified generator, original event/credit ledger, all SHA256 epoch snapshots, sham/exact-original controls and all external interventions. Study35's additional auditor tracked successful native process1 graph2 reads, runtime register def-use, actor identity, event epoch and paired first source-tainted writes, without outcome-selecting a seed.

## Frozen primary result

**1/192 independent worlds** met the prospectively preregistered narrow `paired_native_source_address_shift` criterion; Wilson 95% interval **[0.000920, 0.028907]**. This passes the *mechanistic replication* criterion of at least one qualified matched witness, **not** a CL-1 life criterion.

The sole primary witness is seed **2,700,001**, epoch 3:

| Arm | Native actor | Tick | Program PC | Cell key | Written value |
|---|---:|---:|---:|---:|---:|
| `control` | process1 | 68 | 22 | 22 | 1 |
| `mutate-g2` | process1 | 70 | 22 | 30 | 1 |

The native process1 code and written value were preserved; graph2/node0 source was externally replaced tag11→19; the first source-tainted write moves its actual address **22→30 (+8)**, matching the source perturbation. The auditor verifies same epoch, original process, successful internal read/write events, correct actual graph2 owner and source register chain. The witness has an earlier successful process1 native DrawCompute, but **not in the same epoch**, and its writes occur **before** crossing 47 native postfork attempts. Therefore **same-epoch draw/written joint** = 0/192 and **post-original-budget paired source shift** = 0/192.

## Registered controls and resource limitations

- `control` process1 native DrawCompute 72/192 worlds; resolved native graph2 source reads 42/192; source-tainted address Writes 1/192; the original Study33 exact-address target write **0/192**.
- `sham` and `exact-original-g2` were validated for exact native state equality with control by the independent baseline auditor. `zero-credit` has zero native attempts, draws and source writes. The initial-empty-pool branches are descriptive, not proof of an internally produced compute resource.
- No-refill `control` world activity: W1 (epochs 0–7) **192/192**, W2 (8–15) **86/192**, W3 (16–23) **0/192**. Host-refilled `refill` branches can have W3 activity (139/192), but those are explicitly external resource subsidies.
- Under the strict registered Study33 source address outcome, the prospective result remains **0/192**. The new broader source-dependent address criterion was preregistered as a distinct Study35 mechanism, not retroactively substituted into Study33.

## Scientific interpretation and next necessary question

Study35 prospectively replicated a **limited native, register-mediated, source-conditioned output-address transformation** under a randomized non-goal-directed CIL continuation. The external perturbation of code graph2 and the researcher-endowed initial compute pool are not internal maintenance. The one positive prospective world is rare and not persistent, and does not survive a two-window no-refill criterion. The original Study34 descriptive 3/128 and Study35 prospective 1/192 must be presented separately; do not pool them as a single preregistered sample or claim an effect prevalence without uncertainty.

**CL-1 remains OPEN:** no endogenous code repair, no independently evolved multicomponent function, no internally maintained resources, no unassisted W3 survival, no open-ended evolutionary dynamic. The next study should address the world's finite native compute-access bottleneck independently from information-flow success, rather than optimizing the random grammar to guarantee a result.
