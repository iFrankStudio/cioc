"""Study43 minimal native child ancestry and output safety controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study43 import scan_child_events


def launch():
    return {
        "tick": 3, "pc": 10, "process": 1,
        "origin": {"Internal": 1},
        "kind": {"GraphLaunched": {
            "source_graph": 2, "parent_process": 1,
            "child_process": 4, "moved_nodes": 2,
            "transferred_credits": 1,
        }},
        "_study43_epoch": 0, "_study43_starter": False,
    }


def child(kind):
    return {
        "tick": 4, "pc": 0, "process": 4,
        "origin": {"Internal": 4}, "kind": kind,
        "_study43_epoch": 1, "_study43_starter": True,
    }


def states(events):
    initial = {"events": []}
    final = {
        "events": events,
        "processes": {
            "1": {"code": {"nodes": ["Halt"]}},
            "4": {"code": {"nodes": [
                {"GraphNew": {"dst": 3, "next": 1}}, "Halt"
            ]}},
        },
    }
    return final, initial


class Study43Provenance(unittest.TestCase):
    def test_native_graph_created_child_is_not_shared_cell_function(self):
        result = scan_child_events(*states([
            launch(), child({"GraphCreated": {"graph": 3, "owner": 4}}),
        ]))
        self.assertTrue(result["worlds_with_child_success"])
        self.assertFalse(result["worlds_with_child_cell_write"])
        self.assertEqual(result["native_child_event_counts"]["GraphCreated"], 1)

    def test_child_without_launch_is_rejected(self):
        with self.assertRaises(ValueError):
            scan_child_events(*states([
                child({"CellWritten": {"key": 1, "value": 9}}),
            ]))

    def test_trapped_child_not_accepted_as_success(self):
        result = scan_child_events(*states([
            launch(), child({"Trapped": "invalid"}),
        ]))
        self.assertFalse(result["worlds_with_child_success"])
        self.assertEqual(result["native_child_event_counts"]["Trapped"], 1)


if __name__ == "__main__":
    unittest.main()
