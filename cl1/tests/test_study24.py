"""Study 24 mechanism controls: an ordered source read is not causal function."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study24 import epoch_exposure


def event(tick, actor, kind):
    return {"tick": tick, "origin": {"Internal": actor},
            "process": actor, "pc": tick, "kind": kind}


class Study24ReadSourceTests(unittest.TestCase):
    def test_foreign_source_then_same_actor_successful_write(self):
        events = [
            event(1, 2, {"GraphTagRead": {
                "graph": 5, "owner": 1, "node": 0, "tag": 19,
            }}),
            event(2, 2, {"CellWritten": {"key": 90, "value": 19}}),
        ]
        measured = epoch_exposure(events)
        self.assertEqual(measured["counts"]["foreign_graph_reads"], 1)
        self.assertEqual(measured["counts"]["same_actor_read_then_write"], 1)
        self.assertEqual(measured["pairs"][(1, 2)], 1)

    def test_self_owned_read_not_qualified(self):
        events = [
            event(1, 2, {"GraphTagRead": {
                "graph": 5, "owner": 2, "node": 0, "tag": 0,
            }}),
            event(2, 2, {"CellWritten": {"key": 90, "value": 0}}),
        ]
        self.assertEqual(epoch_exposure(events)["counts"]["same_actor_read_then_write"], 0)

    def test_wrong_actor_or_reverse_event_order(self):
        read = event(2, 2, {"GraphFieldRead": {
            "graph": 5, "owner": 1, "node": 0, "field": 2, "word": 9,
        }})
        foreign_actor_write = event(3, 3, {"CellWritten": {"key": 90, "value": 9}})
        before_read = event(1, 2, {"CellWritten": {"key": 90, "value": 9}})
        for group in ([read, foreign_actor_write], [before_read, read]):
            with self.subTest(events=group):
                self.assertEqual(epoch_exposure(group)["counts"]["same_actor_read_then_write"], 0)

    def test_epoch_boundary_resets_read_ancestry(self):
        first = [event(1, 2, {"GraphTagRead": {
            "graph": 5, "owner": 1, "node": 0, "tag": 0,
        }})]
        second = [event(2, 2, {"CellWritten": {"key": 90, "value": 0}})]
        self.assertEqual(epoch_exposure(first)["counts"]["same_actor_read_then_write"], 0)
        self.assertEqual(epoch_exposure(second)["counts"]["same_actor_read_then_write"], 0)

    def test_missing_source_identity_rejected(self):
        with self.assertRaises(ValueError):
            epoch_exposure([event(1, 2, {"GraphTagRead": {
                "graph": 5, "node": 0, "tag": 0,
            }})])


if __name__ == "__main__":
    unittest.main()
