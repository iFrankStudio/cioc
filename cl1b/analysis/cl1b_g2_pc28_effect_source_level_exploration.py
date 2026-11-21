#!/usr/bin/env python3
"""Exploratory G2 PC28-effect counterfactual, SOURCE-LEVEL SIMULATION ONLY.

Reads the already-published G2 frozen manifest. No new holdout, native Rust
intervention, or independent native trace is produced. PC28 still costs one
instruction; only its effect on PC173 is suppressed in the counterfactual.
This is NOT an original-native causal result.
"""
import json
from collections import defaultdict
from pathlib import Path

MANIFEST = Path(__file__).resolve().parents[1] / "studies/CL1B-REFILL-RULE-G2-frozen-180-node-self-modifying-184-world-v1.json"
BASELINE = {
    "stationary_terminal_switch": 9432,
    "persistent_drift_two_shifts": 13791,
    "rapid_reversal_pulses": 15635,
    "late_phase_flip": 13425,
    "seeded_noise_impulses": 17998,
}

def simulate(source, arm, suppress_pc28):
    code = [dict(op) for op in source]
    regs, cells, pc, paid, patches, effective = [0, 0, 0, 0], {}, 0, 0, 0, 0
    while paid < 6000:
        at = pc
        op = code[at]
        kind = op["kind"]
        paid += 1
        if kind == "Const":
            regs[op["dst"]] = op["value"]
            pc = op["next"]
        elif kind == "Read":
            regs[op["dst"]] = cells.get(regs[op["address"]], 0)
            pc = op["next"]
        elif kind == "Write":
            cells[regs[op["address"]]] = regs[op["value"]]
            pc = op["next"]
        elif kind in ("Add", "Sub"):
            a, b = regs[op["lhs"]], regs[op["rhs"]]
            regs[op["dst"]] = a + b if kind == "Add" else a - b
            pc = op["next"]
        elif kind == "Branch":
            pc = op["nonzero"] if regs[op["test"]] else op["zero"]
        elif kind == "PatchConst":
            target = regs[op["node"]]
            assert 0 <= target < 180 and code[target]["kind"] == "Const"
            if at == 28:
                assert target == 173
                patches += 1
                effective += int(code[target]["value"] != regs[op["value"]])
            if not (suppress_pc28 and at == 28):
                code[target]["value"] = regs[op["value"]]
            pc = op["next"]
        elif kind == "PatchNext":
            target, destination = regs[op["node"]], regs[op["target"]]
            assert 0 <= target < 180 and 0 <= destination < 180
            assert code[target]["kind"] == "Write"
            code[target]["next"] = destination
            pc = op["next"]
        elif kind == "Halt":
            break
        else:
            raise AssertionError((kind, at))
        if at == 3:
            stage = cells.get(32, 0)
            assert 0 <= stage < 24
            truth = int(cells[11] == arm["goals"][stage])
            mode = arm["feedback"]
            cells[20] = truth if mode == "truth" else 1-truth if mode == "inverted" else 0
        assert 0 <= pc < 180 and len(cells) <= 16
    else:
        raise AssertionError(("private compute exhausted", arm["name"]))
    return {"paid": paid, "completed": cells.get(32, 0),
            "claim": cells.get(21, 0), "patches": patches, "effective": effective}

def main():
    m = json.loads(MANIFEST.read_text(encoding="utf-8"))
    assert m["status"] == "PRE_NATIVE_FROZEN"
    source = m["source_g2_changed_allow"]
    assert len(source) == 180
    totals = defaultdict(lambda: {"on": 0, "suppressed": 0, "wins": 0,
                                  "losses": 0, "ties": 0, "exposed": 0})
    rows = []
    for arm in m["arms"]:
        if arm["source"] != "G2_changed_allow":
            continue
        assert arm["feedback"] == "truth"
        on = simulate(source, arm, False)
        off = simulate(source, arm, True)
        assert on["completed"] == off["completed"] == 24
        assert on["claim"] == off["claim"] == 1
        delta = off["paid"] - on["paid"]
        t = totals[arm["regime"]]
        t["on"] += on["paid"]
        t["suppressed"] += off["paid"]
        t["wins"] += int(delta > 0)
        t["losses"] += int(delta < 0)
        t["ties"] += int(delta == 0)
        t["exposed"] += int(on["effective"] > 0)
        rows.append({"name": arm["name"], "delta_suppressed_minus_on": delta,
                     "on": on, "suppressed": off})
    assert len(rows) == 45
    assert {k: v["on"] for k, v in totals.items()} == BASELINE, (
        "source-level baseline disagrees with published original-native family totals")
    assert sum(t["suppressed"] - t["on"] for t in totals.values()) == 156
    print(json.dumps({"evidence_level": "EXPLORATORY_SOURCE_LEVEL_SIMULATION_NOT_NATIVE",
                      "frozen_source": str(MANIFEST.relative_to(MANIFEST.parents[2])),
                      "families": totals, "paired_worlds": rows},
                     ensure_ascii=False, indent=2))

if __name__ == "__main__":
    main()
