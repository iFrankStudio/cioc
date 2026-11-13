"""Narrow mechanism fixtures for Study29 source1+source2 read then output."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study29 import strict_dual_output


def event(tick, actor, kind):
    return {
        "tick": tick,
        "origin": {"Internal": actor},
        "process": actor,
        "pc": tick,
        "kind": kind,
    }


def read(tick, graph, tag, actor=1, owner=None):
    return event(tick, actor, {"GraphTagRead": {
        "graph": graph, "owner": graph if owner is None else owner,
        "node": 0, "tag": tag,
    }})


def write(tick, address, value=7, actor=1):
    return event(tick, actor, {"CellWritten": {
        "key": address, "value": value,
    }})


def pair(t1, t2, address, value=7):
    return [read(1, 1, t1), read(2, 2, t2), write(3, address, value)]


class Study29SourcePair(unittest.TestCase):
    def test_four_arm_additive_positive_and_conserved_value(self):
        for p in ((11, 11, 22), (19, 11, 30),
                  (11, 19, 30), (19, 19, 38)):
            with self.subTest(pattern=p):
                result = strict_dual_output(pair(*p), *p)
                self.assertTrue(result["qualified"])
                self.assertEqual(result["value"], 7)

    def test_no_false_sources_or_wrong_actor(self):
        fixtures = [
            [read(1, 1, 11), write(3, 22)],
            [read(1, 1, 11), read(2, 2, 11, owner=3), write(3, 22)],
            [read(1, 1, 11), read(2, 2, 11, actor=2), write(3, 22)],
            [read(1, 1, 11), read(2, 2, 19), write(3, 22)],
            [read(1, 1, 11), read(2, 2, 11), write(3, 30)],
            [write(1, 22), read(2, 1, 11), read(3, 2, 11)],
            [read(1, 1, 11), read(2, 2, 11),
             write(3, 22), write(4, 22)],
        ]
        for events in fixtures:
            with self.subTest(events=events):
                self.assertFalse(
                    strict_dual_output(events, 11, 11, 22)["qualified"]
                )


if __name__ == "__main__":
    unittest.main()
