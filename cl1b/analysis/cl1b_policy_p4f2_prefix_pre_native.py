#!/usr/bin/env python3
"""P4F2: independently executable pre-native prefix-identifiability control.

Synthetic observations are NOT original CL0 Rust/CIL outcomes.
"""
import json

WORLDS = {
    "transient_shock": (0, 0, 1, 1, 1, 1),
    "persistent_shift": (0, 0, 0, 0, 0, 0),
    "delayed_recovery": (0, 0, 0, 1, 1, 1),
    "single_shock": (0, 1, 1, 1, 1, 1),
    "no_shock": (1, 1, 1, 1, 1, 1),
}

def rollback_after(prefix, threshold):
    return len(prefix) >= threshold and not any(prefix[-threshold:])

def audit():
    assert WORLDS["transient_shock"][:2] == WORLDS["persistent_shift"][:2]
    assert WORLDS["transient_shock"][2:] != WORLDS["persistent_shift"][2:]
    ambiguous = {}
    for name, seq in WORLDS.items():
        ambiguous.setdefault("".join(map(str, seq[:2])), []).append(name)
    assert set(ambiguous["00"]) == {
        "transient_shock", "persistent_shift", "delayed_recovery"
    }
    decisions = {
        name: {
            str(threshold): [
                rollback_after(seq[:i], threshold) for i in range(1, len(seq) + 1)
            ] for threshold in (2, 3)
        } for name, seq in WORLDS.items()
    }
    return {
        "status": "SOURCE_ONLY_PRE_NATIVE",
        "observation_semantics": "1=truthful current-policy success, 0=failure",
        "worlds": WORLDS,
        "indistinguishable_two_probe_prefixes": ambiguous,
        "fixed_threshold_decisions": decisions,
        "hypothesis": "Two identical initial paid failure observations cannot "
                      "identify whether shock is transient or persistent; "
                      "a third truthful observation can distinguish these two worlds.",
        "controls": [
            "same-source immediate two-failure rollback",
            "equal-budget uninformative observation",
            "feedback inversion",
            "source-patch suppressed while retaining charges"
        ],
        "native_acceptance": [
            "original Rust World with unchanged CL0 resource limits",
            "original charged CIL, one uninterrupted PID per world",
            "freeze matched hidden task sequences before execution",
            "count observation, source patch, task probes and recovery costs",
            "independent opcode replay and GitHub Actions exact-SHA evidence",
            "report both beneficial and harmful environments"
        ],
        "nonclaim": "Synthetic bits are not original native outcomes; no fitness advantage inferred."
    }

if __name__ == "__main__":
    print(json.dumps(audit(), indent=2, sort_keys=True))
