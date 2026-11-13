# CIOC V0.7 — bounded complementary role regeneration

This benchmark tests whether O/V/M can reconstruct G after its executable program is erased. It is a toy causal-intervention experiment, **not** proof of digital life or full organizational closure.

## Run

Python 3.10+; standard library only.

```bash
cd experiments/v0.7
python -m unittest -v
python run.py
```

## Roles

- O: detect missing roles and perform bounded fallback search for missing G.
- G: generate program candidates.
- V: check a candidate against fixed experimenter-provided role contracts.
- M: commit a candidate following the contract check.

The program fragments are predesigned and incomplete. The host supplies the instruction interpretation, bounded program enumeration, exact-sequence contract checking and atomic assignment. There is no host-level automatic repair workflow.

## Scope

G-only loss is recoverable; O/V/M loss and G+M loss remain unrecoverable. Reduced budgets and lost fragments block recovery. This model does not meet a strong claim of mutual organizational maintenance. The next stage must replace exact program-sequence verification with independent behavioral contracts and test reconstruction of other critical roles.

The version tracked here is an adapted, inspectable V0.7 benchmark. Historical experiment source ZIPs remain separate archives pending exact-byte import.
