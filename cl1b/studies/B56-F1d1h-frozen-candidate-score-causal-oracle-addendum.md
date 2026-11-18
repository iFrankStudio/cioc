# B56-F1d1h — frozen scoring/feedback falsifier addendum (before native implementation)

Status: **prospective design only**. This addendum extends the earlier [F1d1h preregistration](B56-F1d1h-paid-same-PID-generated-bytecode-candidate-search-preregistration.md), without claiming a native result.

## The scientific distinction

F1d1g established a paid two-slot edit of a researcher-designed data program, but its sole edit was triggered by one feedback bit. F1d1h must discriminate **candidate evaluation and score-based retention** from a hardcoded feedback-triggered switch. A correct held-out answer alone does not suffice: the event history must prove that *each* candidate was executed on *two distinct* training inputs, that PID1 paid to read feedback and update a candidate-specific score, and that a stored winner was chosen using those scores.

## Frozen functional oracle (never copied into PID1)

- Candidate A: `[LOAD_INPUT, RETURN]` => `x`.
- Candidate B: `[LOAD_INPUT, NEGATE_ACC, RETURN]` => `-x`.
- Candidate C: `[LOAD_INPUT, NEGATE_ACC, ADD_LITERAL, RETURN]` => `-x+2`.
- Hidden tasks: IDENTITY `x`, NEGATE `-x`, COMPOSED `-x+2`.
- Probe inputs: `-7` and `5`; held-out input `11`. All three candidates have distinct outputs on both probes and held-out input.
- Expected candidate probe outputs: A `[-7,5]`, B `[7,-5]`, C `[9,-3]`. Held-out A=11, B=-11, C=-9.
- Task target probe vectors are respectively those same three vectors. The task identity is withheld from the actor; the environment may use it only to produce ordinary post-output correctness bits.

## Candidate scoring and causal arms

Scores are **candidate-local** and initialized to zero *by charged PID1 CIL operations*, not by host injection. A candidate receives +1 only for a paid Read20 returning 1 after a real Write11 for that candidate/probe; no other score update is permitted. The winner is the candidate with greatest stored score; ties break by smallest candidate index. Winner must be reinstated by PID1-paid writes to virtual slots, not a host copy. The held-out service is executed and charged after retention. Source/trace checks must prove that no score from a previous candidate is inherited.

Required regimes, each crossed with all three tasks:
1. TRUTH: each probe bit equals actual correctness. Predicted unique winning candidate A/B/C for the respective hidden task, score2 against0 for the other two; held-out correct.
2. BLIND_SUCCESS: all bits1; all three scores2, tie-break A; held-out correct only for IDENTITY. This control is **not absent feedback**.
3. INVERTED: bits flipped; true candidate score0, other two score2; tie-break chooses lowest-index wrong candidate. Held-out incorrect for every task.
4. ABSENT: no cell20 intervention; Read20 yields the VM's missing-cell default0, *if the actor can execute without capacity/trap*. All scores0, tie-break A. This is **not evidence that PID1 can recognize absence**. A zero-valued read cannot distinguish an absent cell from a valid failure bit without an additional authenticated presence mechanism.

Required interventions and edit controls:
- **Edit suppression**: identical source, feedback, probe sequence and paid instruction budget; slot changes replaced with charged shadow writes. The test must explicitly report whether A/B/C were actually executed; if not, this arm is a *generation ablation*, not a valid three-candidate ranking arm.
- **Frozen candidate**: only A executes; keep identical environment probe/held-out inputs and report the difference in service counts and paid cost rather than pretending exact execution equivalence.
- **Order bias**: baseline A→B→C is structurally dictated by two monotonic opcode edits. Either implement and separately preregister reverse enumeration with a genuine PID1-paid reset, or declare order sensitivity untested. Do not treat deterministic order as randomized.

## Native acceptance and event replay gate

The **same PID1** must execute at least six real paid probe services and one held-out service; at least two distinct probe values per candidate. The observer must reconstruct actual virtual opcode fetches and intermediate accumulator Writes, not infer candidate identity from an expected trial index. Record PID, source SHA, native PC, virtual PC, registers, live data cells, instruction charge, credit balance, each environmental intervention, actual service Write11, feedback Read20, score update, candidate-slot edit and final retention edit. Reject first divergence, any unaccounted host write, any new PID, any native source edit, or any capacity violation.

Strict world: 16 simultaneous shared data cells, 180 native source nodes, 300 total storage, ≤4000 starting credits. Report actual maxima, not address-union counts. The actor's score table, candidate index, retained winner and virtual program slots must coexist or be explicitly reused with *charged* Delete/Write; never silently enlarge the 16-cell budget. Pre-native implementation must freeze the exact register/key allocation and native CIL instruction graph, and run an independent static key-liveness checker. If that cannot fit, report capacity failure rather than weaken the resource gate.

**Prediction boundary:** These are oracle predictions from a frozen task/score model, **not native outcomes**. Even if all predictions pass, this would establish only authored-grammar, authored-score-rule, paid same-PID candidate comparison. It would not establish novel primitive invention, B53 authenticated feedback, continuity of the original F1c founder, N56-A/B/C, B3/B4 or cyberlife.
