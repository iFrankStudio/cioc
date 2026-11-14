"""Study39 minimal native world environment-physics controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study39 import candidate, expected_actions, base_actions


def state(pool=7, credit=0, stopped=False, opcode="Const"):
    node = ({"DrawCompute": {"amount": 1, "next": 1}}
            if opcode == "DrawCompute"
            else {"Const": {"dst": 0, "value": 1, "next": 1}})
    return {
        "compute_pool": pool,
        "processes": {
            "1": {"credits": credit, "stopped": stopped, "pc": 0,
                  "registers": [0, 2, 0, 0],
                  "code": {"nodes": [node, "Halt"]}},
            "2": {"credits": 0, "stopped": True, "pc": 0,
                  "registers": [0, 1, 0, 0],
                  "code": {"nodes": ["Halt"]}},
            "3": {"credits": 0, "stopped": True, "pc": 0,
                  "registers": [0, 1, 0, 0],
                  "code": {"nodes": ["Halt"]}},
        },
    }


class Study39EnvironmentRules(unittest.TestCase):
    def test_neutral_starter_does_not_inspect_code_and_does_not_create_credit(self):
        s = state()
        self.assertEqual(candidate(s, "general"), (1, 7))
        self.assertEqual(candidate(s, "draw"), None)
        a = expected_actions(s, "closed-general", 6)
        self.assertEqual(a, [
            {"epoch": 6, "action": {"SetComputePool": {"credits": 6}}},
            {"epoch": 6, "action": {"SetCredits": {"process": 1, "credits": 1}}},
        ])

    def test_ambient_supply_is_first_and_adds_exactly_eight(self):
        s = state(pool=0)
        a = expected_actions(s, "ambient-general", 1)
        self.assertEqual(a, [
            {"epoch": 1, "action": {"SetComputePool": {"credits": 8}}},
            {"epoch": 1, "action": {"SetComputePool": {"credits": 7}}},
            {"epoch": 1, "action": {"SetCredits": {"process": 1, "credits": 1}}},
        ])
        self.assertEqual(expected_actions(s, "ambient", 0), [])

    def test_draw_gate_requires_native_instruction_and_full_pool_headroom(self):
        s = state(pool=2, opcode="DrawCompute")
        self.assertEqual(candidate(s, "draw"), None)
        s["compute_pool"] = 3
        self.assertEqual(candidate(s, "draw"), (1, 3))
        s["processes"]["1"]["stopped"] = True
        self.assertIsNone(candidate(s, "general"))
        s["processes"]["1"]["stopped"] = False
        s["processes"]["1"]["credits"] = 1
        self.assertIsNone(candidate(s, "general"))

    def test_zero_credit_environment_setup_and_explicit_refill_remain_distinct(self):
        s = state(credit=47)
        a = expected_actions(s, "ambient-general-zero-credit", 0)
        self.assertEqual(a[:3], base_actions("ambient-general-zero-credit", 0))
        self.assertEqual(len(a), 5)
        self.assertEqual(a[-1]["action"]["SetCredits"]["process"], 1)
        self.assertEqual(len(base_actions("external-refill", 16)), 3)


if __name__ == "__main__":
    unittest.main()
