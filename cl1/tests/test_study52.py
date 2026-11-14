"""Study52 minimal native source-opcode classification and causal provenance gates."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study52 import classify_strict_launch
from analyze_study50 import trace_assembly


def event(tick, actor, kind, fields):
    return {
        "tick": tick, "pc": 9, "process": actor,
        "origin": {"Internal": actor}, "kind": {kind: fields},
    }


def fixture():
    events = [
        event(30, 1, "GraphNodeCopied", {
            "graph": 1, "owner": 1, "source_process": 1,
            "source_node": 1, "new_node": 1,
        }),
        event(31, 1, "GraphNodeEdited", {
            "graph": 1, "owner": 1, "node": 1, "edit": "next",
        }),
        event(32, 1, "GraphLaunched", {
            "source_graph": 1, "parent_process": 1, "child_process": 4,
            "transferred_credits": 1, "moved_nodes": 2,
        }),
    ]
    original = {
        "events": [],
        "processes": {"1": {"code": {"nodes": [
            {"GraphNew": {"dst": 3, "next": 1}},
            {"Write": {"address": 0, "value": 3, "next": 0}},
        ]}}},
    }
    final = {"processes": {"4": {"code": {"nodes": [
        {"GraphNew": {"dst": 3, "next": 1}},
        {"Write": {"address": 0, "value": 3, "next": 0}},
    ]}}}}
    snapshots = [{"events": events, "processes": original["processes"]}]
    return events, original, final, snapshots


class Study52RuntimeSourceIntegrity(unittest.TestCase):
    def test_original_write_source_is_not_automatic_native_child_success(self):
        events, initial, final, snapshots = fixture()
        path = trace_assembly(events, final)[0]
        got = classify_strict_launch(path, initial, snapshots)
        self.assertEqual(got["ordered_copies"][0]["frozen_parent_source_opcode"], "Write")
        self.assertEqual(got["ordered_copies"][0]["runtime_source_opcode"], "Write")
        self.assertTrue(got["child_has_Write"])
        self.assertFalse(got["native_child_CellWritten"])

    def test_wrong_parent_or_graph_cannot_forge_ordered_assembly(self):
        events, initial, final, snapshots = fixture()
        events[0]["kind"]["GraphNodeCopied"]["graph"] = 2
        found = trace_assembly(events, final)[0]
        self.assertFalse(found["copy_before_edit_before_launch"])
        with self.assertRaises(ValueError):
            classify_strict_launch(found, initial, snapshots)


if __name__ == "__main__":
    unittest.main()
