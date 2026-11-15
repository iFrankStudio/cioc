# CL-1B B06 — checkpoint poisoning / experiment ledger

**Evidence tier:** source-checked, independent simplified CIL path computation, **not native Rust CI**. Five independent input labels are covered by the submitted integration test but only one input23 was manually simulated before test authoring. **No native acceptance asserted.** The experiment's initial two-edit prediction is retained as falsified in [preregistration/discrepancy log](../studies/B06-poisoned-checkpoint-self-damage.md).

## Independently computed path outcomes (100 post-bootstrap CIL steps)

| Condition | Service outputs (all correct) | Effective internal self-code edits | End code PC9.next / PC17.next |
| --- | ---: | --- | --- |
| CLEAN | 5 | none | 10 / 18 |
| POISON_A_REFERENCE (cell13=23) | 7 | node17: 18→23 (damages B monitor) | 10 / 23 |
| POISON_B_REFERENCE (cell12=15) | 6 | node9: 10→15 (damages A monitor) | 15 / 18 |
| DOUBLE_POISON | 7 | **only** node17:18→23; B never gets to its check | 10 / 23 |
| DOUBLE_POISON_PATCH_DISABLED | 5 | 0 effective edits, **10 paid suppressed attempts** | 10 / 18 |

All simulated arms began with intact executable nodes and **actor-authored** checkpoint data from PC3/PC7. No external executable code edit was applied in this B06 experiment. All five conditions remained schedulable through the 100-attempt window in the simplified interpreter. The externally edited checkpoint cell is a separate provenance category: it is not the actor discovering a new value.

## Scientific update

**Checkpoint integrity is a causal precondition for scripted rollback.** The restoration mechanism itself is harmful when its remembered reference is wrong; raw throughput can increase due to loss of integrity checks. The surprisingly asymmetric DOUBLE_POISON case reveals **causal ordering**: monitor A executes first and corrupts monitor B's control edge, thereby preventing B from using its separately poisoned reference to corrupt A. Double contamination does not imply additive damage; in this fixed schedule one faulty act dominates the subsequent causal path.

This is a **negative finding about a hand-engineered maintenance design**. Nothing here demonstrates spontaneous life, emergent diagnosis, novel candidate generation or accumulating autonomous self-improvement. The next scientifically meaningful step is to test whether CIL can form *internally generated, verifiable trust relations* among mutable references and mechanisms without hiding an immutable host authority. That must include alternate schedule/order controls, costs, evidence provenance and unseen faults. See the [native test](../../cl0/crates/world/tests/cl1b_b05_reciprocal_monitors.rs); this file must be updated if the real VM disagrees with simplified path computation.
