"""Study-03 independent restoration/provenance proxy safeguards."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze_study03 as study


def world(next_edge):
    return {
        "processes": {"1": {"code": {"nodes": [
            {"Const": {"dst": 0, "value": 1, "next": 1}}
            for _ in range(8)
        ] + [{"Add": {"dst": 1, "lhs": 1, "rhs": 2, "next": next_edge}}]}}},
        "events": [],
    }


class Study03Tests(unittest.TestCase):
    def test_restore_only_when_edge_matches_original(self):
        self.assertEqual(study.node_next(world(9)), 9)
        self.assertEqual(study.node_next(world(0)), 0)

    def test_exogenous_restoration_does_not_count_as_internal(self):
        w = world(9)
        w["events"] = [
            {"origin": "External", "kind": {"Intervened": "PatchEdge { process: 1, node: 8, next: 9 }"}},
            {"origin": {"Internal": 2}, "kind": "Executed"},
        ]
        self.assertFalse(study.active_after_lesion(w))
        w["events"][0]["kind"]["Intervened"] = "PatchEdge { process: 1, node: 8, next: 0 }"
        w["events"].append({"origin": {"Internal": 1}, "kind": "Executed"})
        self.assertTrue(study.active_after_lesion(w))

    def test_frozen_condition_schedules(self):
        sham = [{"after_attempts": o, "action": {"Disable": {"process": 9999}}}
                for o in study.OFFSETS]
        self.assertTrue(study.valid_control_actions("sham", sham))
        self.assertFalse(study.valid_control_actions("lesion-only", sham))
        self.assertFalse(study.valid_control_actions("zero-credit", sham))


if __name__ == "__main__":
    unittest.main()
