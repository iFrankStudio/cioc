# CL-1B B07 — source-matched simplified CIL execution audit

**Status:** simplified instruction interpreter / independent deterministic sanity check, **not** native Rust CI, SQLite replay, or proof of cyberlife. Recorded after the [pre-committed B07 preregistration](../studies/B07-two-checkpoint-agreement-tradeoff.md) and before native CI results. A committed [Rust integration test](../../cl0/crates/world/tests/cl1b_b07_checkpoint_agreement.rs) is the required authoritative executable.

One concrete input23 was interpreted from the frozen 24-node CIL graph: 8 bootstrap attempts produced native-analogue checkpoint writes to cells12 and13 (each value9); 100 post-fork attempts were simulated in each of seven arms. The Rust test independently covers five input labels × seven arms = **35 worlds**, no dropping of failures.

| Arm | Correct postfork service writes /100 attempts | Effective code mutations | End PC8.next | Final PC |
| --- | ---: | --- | ---: | ---: |
| CLEAN | 8 | 0 | 9 | 17 |
| CODE_DAMAGE | 7 | 8:10→9 | 9 | 17 |
| SINGLE_POISON_12 | 9 | 0 | 9 | 9 |
| SINGLE_POISON_13 | 9 | 0 | 9 | 9 |
| CORRELATED_DOUBLE_POISON | **1** | harmful 8:9→10 | 10 | 11 |
| CODE_DAMAGE_AND_SINGLE_POISON | **0** | 0 | 10 | 8 |
| CODE_DAMAGE_PATCH_DISABLED | **0** | 0 effective, **7 suppressed** | 10 | 18 |

**Interpretation:** unanimous agreement between two copies blocks **one** independently poisoned checkpoint from damaging originally healthy code, but likewise blocks repair of damaged code if any one stored copy disagrees. Two correlated wrong copies agree and induce *native-looking self-damage*. This is a resource-bounded **programmed agreement guard**, not endogenously generated trust or emergent adaptation. A simple correct-output count again misranks damaged states. The apparent single-poison output increase reflects fewer guard instructions, not greater self-maintenance.

**Scientific limits:** All candidate code, checkpoint locations, lesions, and guard rules were written by researchers. The simplified interpreter did not run Rust VM validation, event hashes, resource accounting or serialization. The native test asserts provenance, exactly100 charged attempts, compute inventory142, 0 births and traps, exact output and edge sequences. No native pass is claimed until GitHub Actions or a local Rust environment supplies verifiable run evidence. 