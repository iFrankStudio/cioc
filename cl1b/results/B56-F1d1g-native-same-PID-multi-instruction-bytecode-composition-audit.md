# CL-1B B56-F1d1g — strict same-PID multi-instruction virtual program composition, paid slot edits, and source-identical edit suppression

**Narrow accepted mechanism:** within the SAME continuously alive PID1, an immutable native 77-node CIL interpreter dynamically fetches multiple virtual opcodes from shared data and executes a genuinely chained two-step arithmetic program, after PID1 pays to modify two of its own virtual instruction slots. Feedback, edits, intermediate accumulator states, original native instructions, process identity and strict resources were recorded and independently reexecuted **event by event**. This is **not** autonomous invention of opcode primitives, program-architecture origin or the policy to edit.

## Preregistration and verifiable provenance

- [First prospective multi-instruction gate](../studies/B56-F1d1g-same-PID-multi-instruction-data-bytecode-composition-preregistration.md) committed as `2795b490caf9202665aeff7453564ba0adbf261c`; exact [ISA, test matrix and edit-suppression addendum](../studies/B56-F1d1g-multi-op-bytecode-exact-layout-and-edit-suppression-v1.md) committed at `6a737a5ec9891936c1921c1bf89ea2c93cc9fdf7` **before** native source implementation commit `533d37e2ecac2950528480b8f53581ae31fdc24c`. Immutable addendum SHA-256 `20ca594af1d91d626b380f1bcde8ca92ecd0bae16f416d2a1dcf7bb1551baaf7`. This addendum freezes control key21 and the exact 24-world cohort rather than changing the protocol after results.
- [Full source-pinned GitHub Actions run44266492133](https://github.com/iFrankStudio/cioc/actions/runs/44266492133) at tested source `45ae0bb5de6d3df08cefa55917a888ceb699c085`: **65/65 CL-1B native Rust integration targets pass, 56/56 independent observers pass**, mandatory B31/B34 library ABI checks pass and zero failures. Separate least-privilege evidence job successfully published raw originals. This is full, NOT scoped, native regression.
- Exact source: [native Rust world experiment](../../cl0/crates/world/tests/cl1b_b56_f1d1g_same_pid_multi_bytecode.rs) · [separately authored CIL event interpreter](../analysis/b56_f1d1g_same_pid_multi_bytecode_replay.mjs) · [original native JSONL](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/45ae0bb5de6d3df08cefa55917a888ceb699c085-44266492133-1/b56_f1d1g_same_pid_multi_bytecode_native_steps.jsonl) · [independent complete original history replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/45ae0bb5de6d3df08cefa55917a888ceb699c085-44266492133-1/b56_f1d1g_same_pid_multi_bytecode_replay.json) · [source-pinned run summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/45ae0bb5de6d3df08cefa55917a888ceb699c085-44266492133-1/summary.txt).

## Original paid world and observable 2→4 instruction change

Each isolated world uses the unchanged Rust substrate `World::with_strict_capacity(16,180,300)`, one original PID1 carrying **1,000 finite initial credits**, no compute pool, no extra process and the **same 77-node authored executable CIL graph** across the entire cohort. The program's native source never changes during execution; initial data-program bytes are initialized only through the actor's own charged CIL Write instructions, never by the experiment host. The environment may intervene only at input key7, correctness-feedback key20 and preregistered edit-permission key21. Neither code, program cells50–56, stage31, shadow cell60 nor compute balance may be edited externally.

This native interpreter dynamically reads virtual PC from key50, computes data address `51 + PC` as a paid CIL Add, reads a virtual opcode from cell51–54, and dispatches paid native branches/primitive CIL arithmetic. The four **authored** virtual primitives are:
- `0=LOAD_INPUT`: read key7 into accumulator, store actual intermediate value into key56.
- `1=NEGATE_ACC`: compute `0−acc`, store key56.
- `2=ADD_LITERAL`: read literal2 from key55, compute `acc+literal`, store key56.
- `3=RETURN`: actual native CIL Write11(acc), then stage check; does not increase virtual PC.

For all worlds, the first service program is exactly `[0,3,3,3]` and therefore dynamically executes **virtual instructions [LOAD, RETURN]**. A first-output correctness bit and heldout input arrive only AFTER the first paid service. The same PID1 pays Read20 and, when feedback0, pays a two-slot program update `Write52(1)` and `Write53(2)` in the enabled arm. Slot54 remains3. After reset of virtual PC and stage, it executes **[LOAD, NEGATE, ADD_LITERAL, RETURN]** as individual decoded operations. For heldout11, the observed accumulator checkpoints are `11 → −11 → −9`, followed by actual Write11(−9). For heldout−7, the same instruction sequence produces `−7 → 7 → 9`. There is **no dedicated full COMPOSED service native shortcut**, no new PID, no graph replacement, and no free source mutation.

Under edit suppression (external gate key21=0), the same native source, same feedback and same interpreter execute two ordinary paid Writes to shadow cell60 instead of program slots52/53; they still consume charged operations, but do not alter the interpreted program. This isolates effective *memory update* from merely being exposed to feedback and running the update path. Both arms use identical source code and matching input/feedback; **the environment control key21 is explicit**, not a secret native source patch. The enabled and disabled worlds are not identical in all computed storage-cell semantics, but identical in executable code and cost of the two Write instructions.

## Prespecified crossed causal outcomes

`2 seeds (first5→heldout11; first7→heldout−7) × 2 hidden task types (IDENTITY vs COMPOSED = −x+2) × 3 feedback regimes × 2 edit gate values = 24 isolated native worlds`, with exactly two real service outputs each. First trial always executes identity; for COMPOSED it is deliberately wrong.

| Feedback arm | Edit enabled heldout correct | Edit suppressed heldout correct |
|---|---:|---:|
| TRUTH (actual correctness) | **4 / 4** | 2 / 4 |
| BLIND_SUCCESS (fixed bit1) | 2 / 4 | 2 / 4 |
| INVERTED (opposite of correctness) | 0 / 4 | 2 / 4 |

`BLIND_SUCCESS` is the registered *non-informative* feedback baseline, but it still writes bit1 to feedback cell20 and the actor pays Read20. This is **not** an experiment omitting the feedback cell entirely. The original broad gate requested a no-feedback baseline; this narrower source-frozen three-regime addendum did NOT independently test literal absent feedback, which is therefore an explicit untested extension. All tested task outputs, including negative first attempts, are retained.

## Complete independent evidence and finite accounting

The JS observer independently reconstructs all **77 native CIL opcodes from source**, then reexecutes every native event with state-by-state comparison, not a precalculated correctness table: all registers, native process PC and source code, data cells including dynamic bytecode and accumulator checkpoints, virtual fetch addresses, actor PID, charge/credits, storage and original external interventions. Results:

| Observed quantity | Verified source-pinned native result |
|---|---:|
| Original strict single-PID worlds | 24 |
| **Actual paid CIL steps independently replayed** | **2,516** |
| External input/control/feedback interventions | 96 |
| Dynamic virtual opcode fetches | 104 |
| Actual PID1 paid feedback Reads | 24 |
| Effective real program slot edits | 8 (four cases × two slots) |
| Cost-bearing control shadow Writes | 8 |
| Paid intermediate accumulator checkpoint Writes | 56 |
| Real PID1 service Write11 operations | 48 |
| Paid CIL step range per world | 95–145 |
| **Peak total storage** | **91/300** |
| **First native observer divergence** | **null** |

CIL program code graph is77/180 native nodes. It is byte-equal in every actual native paid event. No PID2 exists in any world. The main trial has at most13 distinguishable data keys, under strict16. The source-pinned raw native test SHA-256 is `49705f317aacb143232e1cb9e00f27a7e5ed1acef8754bf49a3ec21a8c345bcd`.

## Scientific interpretation and next discriminating gate

This makes a precise distinction:
- F1d1d showed **generic native-code node synthesis into PID2** but NOT continuity of PID1.
- F1d1f showed **single-PID data-encoded switch between two existing complete functions**.
- **F1d1g** now demonstrates **a multi-instruction data-bytecode program that is paid-edited in multiple instruction slots, then actually reinterpreted and executed within PID1**. Its semantic output `−x+2` arises from *two discrete primitive operations*, not a preauthored complete service branch.

However, both virtual instruction semantics and their assembly/update plan are **human authored**. The compound `−x+2` target is a toy authored transformation, feedback truth is host-provided and spoofable, only two trials occur, a single fixed two-slot edit is available, no novelty search or selection across many candidate programs is measured, and this founder is distinct from the earlier B51c/F1c task-search chain. These constraints must remain explicit. A next study should test whether PID1 can **generate and evaluate multiple candidate bytecode sequences**, under charged search and storage constraints with an adversarial quality signal and held-out performance. Merely adding more prewritten switch choices cannot establish de-novo algorithm origin.

**Still NOT met:** original F1d1 authenticated B53 receipt monitoring, strict N56-A/B/C, B3/B4, open-ended self-organization or cyberlife.
