"""Study47 targeted calibration: exact scripted fixture and local-edge semantics."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study47 import expected_nodes, edge_valid


class Study47ScriptedVM(unittest.TestCase):
    def test_positive_child_edge_is_portable(self):
        self.assertEqual(len(expected_nodes("native-local-edge")), 9)
        self.assertEqual(
            expected_nodes("native-local-edge")[3],
            {"GraphSetNext": {"graph": 3, "node": 2, "target": 0, "next": 4}},
        )
        self.assertTrue(edge_valid([
            {"Write": {"address": 0, "value": 3, "next": 0}}
        ]))

    def test_unmodified_absolute_parent_edge_is_invalid_in_child(self):
        self.assertEqual(
            expected_nodes("copied-stale-edge")[3],
            {"Const": {"dst": 2, "value": 0, "next": 4}},
        )
        self.assertFalse(edge_valid([
            {"Write": {"address": 0, "value": 3, "next": 7}}
        ]))

    def test_zero_child_credit_is_a_separate_human_authored_control(self):
        self.assertEqual(
            expected_nodes("zero-child-credit")[4]["Const"]["value"], 0
        )
        self.assertEqual(
            expected_nodes("native-local-edge")[4]["Const"]["value"], 3
        )


if __name__ == "__main__":
    unittest.main()
