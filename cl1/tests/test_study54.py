"""Study54 native Write source copy, temporal graph edit and launch minimal tests."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study54 import stages


def event(tick, actor, op, fields):
    return {
        "tick": tick, "pc": 9, "process": actor,
        "origin": {"Internal": actor}, "kind": {op: fields},
    }


def example(copy_graph=1, edit_graph=1):
    initial = {
        "events": [],
        "processes": {"1": {"code": {"nodes": [
            {"GraphNew": {"dst": 3, "next": 1}},
            {"Write": {"address": 0, "value": 3, "next": 0}},
        ]}}},
    }
    events = [
        event(10, 1, "GraphNodeCopied", {
            "graph": copy_graph, "owner": 1, "source_process": 1,
            "source_node": 1, "new_node": 1,
        }),
        event(11, 1, "GraphNodeEdited", {
            "graph": edit_graph, "owner": 1, "node": 1, "edit": "next",
        }),
        event(12, 1, "GraphLaunched", {
            "source_graph": 1, "parent_process": 1,
            "child_process": 4, "transferred_credits": 2,
            "moved_nodes": 2,
        }),
    ]
    return initial, {"events": events}


class Study54WriteOpportunity(unittest.TestCase):
    def test_resolved_original_Write_source_is_not_assumed_runtime_function(self):
        result = stages(*example())
        self.assertEqual(len(result["write_copies"]), 1)
        self.assertTrue(result["write_copy_before_launch"])
        self.assertTrue(result["write_copy_edit_launch"])
        self.assertIn("NOT EVALUABLE", result["write_copies"][0]["runtime_source_opcode"])

    def test_unrelated_copy_or_edit_graph_is_not_a_complete_path(self):
        self.assertFalse(stages(*example(copy_graph=2))["write_copy_before_launch"])
        result = stages(*example(edit_graph=3))
        self.assertTrue(result["write_copy_before_launch"])
        self.assertFalse(result["write_copy_edit_launch"])


if __name__ == "__main__":
    unittest.main()
