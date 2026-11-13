# CIOC V0.7 — Complementary role-program recovery

This self-contained benchmark investigates whether a missing *generator role program* can be reconstructed using complementary abilities of three surviving internal roles. It is **not a claim of full organizational closure or digital life**.

## Run

Requires Python 3.10+; no external dependencies.

```bash
python run.py
python -m unittest -v
```

## Components

- **O** (observer): scans for missing role programs; possesses a bounded, fallback candidate-enumeration instruction.
- **G** (generator): normal candidate generation; the experiment erases this role's executable program.
- **V** (verifier): receives candidates and checks the externally supplied functional contract.
- **M** (modifier): independently rechecks contracts and atomically installs an accepted program.

Each role is represented by a mutable opcode tuple. Fragments held by O and V provide **partial, predesigned knowledge**, not general automatic rediscovery. The host executes opcode semantics, enumeration, contract checking and installation; the host does not automatically call the recovery workflow. A repair can be performed only when surviving role programs execute the corresponding instructions.

## Main result

With G erased, O proposes a program, V checks it, M installs it. Without O, V or M, recovery of those missing components fails. This asymmetry is scientifically useful: the role network remains brittle and **does not satisfy a strong mutual-maintenance claim**.

## Important limitations

1. Only two-instruction candidate programs are searched; vocabulary, role contracts and fallback policy are supplied by the experimenter.
2. The role contract currently uses a fixed exact sequence. It is *not* a behavioral specification independent of source structure, and thus supports only bounded role reconstruction, not genuine functional program synthesis.
3. The host provides a bounded `product` enumerator, role execution, contract evaluation, and atomic install; those assumptions remain outside the organism.
4. The fragment distribution was engineered for this scenario. A single normal generator loss can recover, but there is no general closure over all maintenance capabilities.
5. This test has no independent identity/boundary inference, stochastic disturbance model, continuous environment or open-ended development.

Next research stage should replace exact-order role contracts with input-output behavioral traces and shift candidate construction into typed graph primitives owned by internal roles. Avoid adding many unrelated tests; prioritize causal controls and the known failure cases.
