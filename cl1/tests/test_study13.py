"""Study-13 observer test: one-way communication is not reciprocal."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze_study13 as study


class Study13Tests(unittest.TestCase):
    def copied(self, actor, source):
        return {
            "origin": {"Internal": actor}, "pc": 9,
            "kind": {"GraphCopied": {
                "source_owner": source, "target_owner": actor,
            }},
        }

    def test_two_directions_same_window_required(self):
        self.assertEqual(study.reciprocal_pairs([[self.copied(2, 1)]])[0], set())
        self.assertEqual(
            study.reciprocal_pairs([
                [self.copied(2, 1)],
                [self.copied(1, 2)],
            ])[0],
            {(1, 2)},
        )

    def test_incorrect_owner_or_pc_is_rejected(self):
        bad = self.copied(2, 1)
        bad["pc"] = 8
        with self.assertRaises(ValueError):
            study.reciprocal_pairs([[bad]])
        bad["pc"] = 9
        bad["origin"] = "External"
        with self.assertRaises(ValueError):
            study.reciprocal_pairs([[bad]])

    def test_internal_node_addition_is_detected_even_if_graph_later_dropped(self):
        initial = {"graph_objects": {"1": {"nodes": ["Halt"]}}}
        final = {"graph_objects": {"1": {"nodes": ["Halt"]}}}
        events = [
            {"origin": {"Internal": 2},
             "kind": {"GraphCreated": {"graph": 4, "owner": 2}}},
            {"origin": {"Internal": 2},
             "kind": {"GraphNodeCopied": {"graph": 4, "owner": 2,
                                         "source_process": 2,
                                         "source_node": 0, "new_node": 0}}},
        ]
        self.assertTrue(study.newly_populated_graph(initial, final, events))
        self.assertFalse(study.newly_populated_graph(initial, final, events[:1]))

    def test_negative_control_schedule_is_fixed(self):
        self.assertEqual(study.expected_schedule("control", {"processes": {}}), [])
        self.assertEqual(
            [x["action"]["DropGraph"]["graph"] for x in
             study.expected_schedule("drop-both", {"processes": {}})],
            [1, 2],
        )


if __name__ == "__main__":
    unittest.main()
