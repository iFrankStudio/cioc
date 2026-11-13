"""Study 25 strict graph-content causal-audit calibration controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study25 import _targeted_first_outputs


def e(tick, actor, kind):
    return {"origin": {"Internal": actor}, "tick": tick,
            "process": actor, "pc": tick, "kind": kind}


def read(tick, actor=2, graph=1, tag=11, owner=1):
    return e(tick, actor, {"GraphTagRead": {
        "graph": graph, "owner": owner, "node": 0, "tag": tag,
    }})


def write(tick, actor=2, key=90, value=11):
    return e(tick, actor, {"CellWritten": {"key": key, "value": value}})


class Study25CausalCalibrationTests(unittest.TestCase):
    def test_same_actor_target_source_and_output(self):
        old, amb_old = _targeted_first_outputs([read(1), write(2, value=11)], 1)
        mutated, amb_new = _targeted_first_outputs([
            read(1, tag=19), write(2, value=19),
        ], 1)
        self.assertFalse(amb_old or amb_new)
        self.assertEqual(old[(2, 90)], {"tag": 11, "value": 11})
        self.assertEqual(mutated[(2, 90)], {"tag": 19, "value": 19})

    def test_foreign_actor_or_wrong_graph_never_matches(self):
        for sequence in (
            [read(1), write(2, actor=3)],
            [read(1, graph=2), write(2)],
            [write(1), read(2)],
            [read(1, owner=2), write(2)],
        ):
            with self.subTest(sequence=sequence):
                qualified, _ = _targeted_first_outputs(sequence, 1)
                self.assertFalse(qualified)

    def test_duplicate_output_is_ambiguous_not_fake_causal_witness(self):
        result, ambiguous = _targeted_first_outputs([
            read(1), write(2, value=11), write(3, value=19),
        ], 1)
        self.assertFalse(result)
        self.assertIn((2, 90), ambiguous)

    def test_epoch_boundary_requires_new_read(self):
        first, _ = _targeted_first_outputs([read(1)], 1)
        second, _ = _targeted_first_outputs([write(2)], 1)
        self.assertFalse(first)
        self.assertFalse(second)


if __name__ == "__main__":
    unittest.main()
