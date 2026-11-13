# CL-0 Stage 2 — controlled external intervention

The initial Stage-2 increment adds a distinguishable **experimenter-origin** world action. It does not grant CIL any new privileged behavior.

External actions: `Disable`, `SetCredits`, `SetCell`, `PatchEdge`. Every accepted or rejected action is logged with `Origin::External`. It consumes a logical event index, not process compute credit. A failed action must leave the scientific world state unchanged (except its rejection log and event counter). Target functions, survival rules and repair policies remain absent.

Externally setting credits or cells is an **intervention**, never an internal gain. Baseline and treatment must both start from the same serialized pre-intervention world. Stage 2 will next add a durable SQLite runner and exact replay checks.

Limitations: `SetCell` is a whole-value replacement, not a generic allocation permission; `Disable` is irreversible through current CIL instructions. These are laboratory manipulations, not world physics. Resource conservation must be evaluated with these external contributions subtracted.
