"""Study48 calibrated same-parent/source-graph event chronology."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study48 import trace_assembly


def event(tick, actor, name, fields):
    return {
        "tick": tick, "pc": 9, "process": actor,
        "origin": {"Internal": actor}, "kind": {name: fields},
    }


def fixture(copy_graph=1, edit_actor=1):
    return [
        event(30, 1, "GraphNodeCopied", {
            "graph": copy_graph, "owner": 1, "source_process": 1,
            "source_node": 15, "new_node": 1,
        }),
        event(31, edit_actor, "GraphNodeEdited", {
            "graph": 1, "owner": edit_actor, "node": 1, "edit": "next",
        }),
        event(33, 1, "GraphLaunched", {
            "source_graph": 1, "parent_process": 1, "child_process": 4,
            "transferred_credits": 3, "moved_nodes": 2,
        }),
    ]


def child_world():
    return {"processes": {
        "4": {"code": {"nodes": [
            {"GraphNew": {"dst": 3, "next": 1}},
            {"Write": {"address": 0, "value": 3, "next": 0}},
        ]}}
    }}


class Study48AssemblyChronology(unittest.TestCase):
    def test_same_parent_graph_copy_then_edit_then_launch(self):
        observed = trace_assembly(fixture(), child_world())
        self.assertEqual(len(observed), 1)
        self.assertTrue(observed[0]["copy_before_edit_before_launch"])
        self.assertTrue(observed[0]["final_child_edges_closed"])
        self.assertTrue(observed[0]["has_Write"])

    def test_cross_graph_and_actor_are_not_false_native_assembly(self):
        row = trace_assembly(fixture(copy_graph=2, edit_actor=2),
                             child_world())[0]
        self.assertEqual(row["copies_nonzero"], [])
        self.assertEqual(row["edits"], [])
        self.assertFalse(row["copy_before_edit_before_launch"])

    def test_forged_child_launch_and_source_are_rejected(self):
        events = fixture()
        events[-1]["kind"]["GraphLaunched"]["parent_process"] = 3
        with self.assertRaises(ValueError):
            trace_assembly(events, child_world())


if __name__ == "__main__":
    unittest.main()
