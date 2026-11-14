"""Study41 calibrated native executable mutation recognition and host policy."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study41 import (classify_mutation, expected_continuations,
                             expected_actions, neutral_candidate)


def native(op="PatchConst", successful=True):
    old = {"Const": {"dst": 0, "value": 2, "next": 1}}
    patched = {"Const": {"dst": 0, "value": 7, "next": 1}}
    mutator = {op: {"node": 2, "value": 0, "next": 2}}
    initial = [old, mutator, old]
    after = [old, mutator, patched]
    other = {"code": {"nodes": ["Halt"]}}
    before = {"processes": {
        "1": {"code": {"nodes": initial}}, "2": other, "3": other,
    }}
    final = {"processes": {
        "1": {"code": {"nodes": after}}, "2": other, "3": other,
    }}
    event = {"process": 1, "pc": 1, "tick": 2,
             "kind": "Executed" if successful else {"Trapped": "invalid"}}
    return before, final, [event]


class Study41NativePlasticity(unittest.TestCase):
    def test_successful_native_code_mutation_is_attributed(self):
        result = classify_mutation(*native())
        self.assertEqual(result["changed_actors"], [1])
        self.assertEqual(result["unambiguous_actors"], [(1, 1, 2)])

    def test_trapped_or_host_only_code_change_is_rejected(self):
        with self.assertRaises(ValueError):
            classify_mutation(*native(successful=False))
        before, after, _ = native()
        with self.assertRaises(ValueError):
            classify_mutation(before, after, [])

    def test_graph_op_is_not_executable_mutation(self):
        before, after, events = native(op="GraphSetConst")
        with self.assertRaises(ValueError):
            classify_mutation(before, after, events)

    def test_independent_deterministic_native_grammar_has_27_nodes_per_actor(self):
        a = expected_continuations(3_100_000)
        self.assertEqual(set(a), {1, 2, 3})
        self.assertTrue(all(len(p) == 27 for p in a.values()))
        self.assertEqual(a, expected_continuations(3_100_000))

    def test_neutral_access_never_looks_at_opcode_fitness(self):
        s = {"compute_pool": 5, "processes": {
            "1": {"credits": 0, "stopped": False, "code": {"nodes": ["Halt"]}},
            "2": {"credits": 0, "stopped": True, "code": {"nodes": ["Halt"]}},
            "3": {"credits": 0, "stopped": True, "code": {"nodes": ["Halt"]}},
        }}
        self.assertEqual(neutral_candidate(s), (1, 5))
        actions = expected_actions(s, "ambient-general", 1)
        self.assertEqual(actions, [
            {"epoch": 1, "action": {"SetComputePool": {"credits": 13}}},
            {"epoch": 1, "action": {"SetComputePool": {"credits": 12}}},
            {"epoch": 1, "action": {"SetCredits": {"process": 1, "credits": 1}}},
        ])
        s["processes"]["1"]["credits"] = 1
        self.assertIsNone(neutral_candidate(s))


if __name__ == "__main__":
    unittest.main()
