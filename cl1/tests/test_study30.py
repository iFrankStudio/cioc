"""Study30 two-source success-only pipeline controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study30 import stages


def ev(tick, actor, kind, pc=0):
    return {"tick": tick, "process": actor, "pc": pc,
            "origin": {"Internal": actor}, "kind": kind}


def read(tick, graph, tag=11, owner=None):
    return ev(tick, 1, {"GraphTagRead": {
        "graph": graph,
        "owner": graph if owner is None else owner,
        "node": 0,
        "tag": tag,
    }})


def written(tick, key):
    return ev(tick, 1, {"CellWritten": {"key": key, "value": 1}})


CODE = [{"Const": {"dst": 0, "value": 0, "next": 1}},
        {"Add": {"dst": 0, "lhs": 0, "rhs": 2, "next": 2}}]


class Study30PipelineTests(unittest.TestCase):
    def test_native_two_source_add_then_expected_output(self):
        events = [read(1, 1), read(2, 2), ev(3, 1, "Executed", 1),
                  written(4, 22)]
        result = stages(events, CODE, 11, 11, 22)
        self.assertTrue(result["both"])
        self.assertTrue(result["add_after_both"])
        self.assertTrue(result["target_write_after_add"])

    def test_failed_or_wrong_native_add_not_accepted(self):
        variations = [
            [read(1, 1), read(2, 2), ev(3, 1, "Executed", 0), written(4, 22)],
            [read(1, 1), read(2, 2), ev(3, 1, {"Trapped": "add failed"}, 1), written(4, 22)],
            [read(1, 1), ev(2, 1, "Executed", 1), read(3, 2), written(4, 22)],
            [read(1, 1), read(2, 2), written(3, 22), ev(4, 1, "Executed", 1)],
        ]
        for events in variations:
            with self.subTest(events=events):
                self.assertFalse(stages(events, CODE, 11, 11, 22)["target_write_after_add"])

    def test_incorrect_graph_owner_not_counted(self):
        seq = [read(1, 1, owner=3), read(2, 2),
               ev(3, 1, "Executed", 1), written(4, 22)]
        self.assertFalse(stages(seq, CODE, 11, 11, 22)["both"])

    def test_correct_pipeline_wrong_address_is_not_strict_target(self):
        events = [read(1, 1), read(2, 2),
                  ev(3, 1, "Executed", 1), written(4, 30)]
        result = stages(events, CODE, 11, 11, 22)
        self.assertTrue(result["write_after_add"])
        self.assertFalse(result["target_write_after_add"])


if __name__ == "__main__":
    unittest.main()
