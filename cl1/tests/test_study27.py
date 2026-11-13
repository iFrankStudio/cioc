"""Preregistered Study-27 graph-tag-to-cell-address exposure controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study27 import strict_output


def event(tick, actor, kind):
    return {"tick": tick, "process": actor, "pc": tick,
            "origin": {"Internal": actor}, "kind": kind}


def source(tick, actor=1, graph=2, owner=2, tag=11):
    return event(tick, actor, {"GraphTagRead": {
        "graph": graph, "owner": owner, "node": 0, "tag": tag,
    }})


def output(tick, actor=1, address=11, value=3):
    return event(tick, actor, {"CellWritten": {
        "key": address, "value": value,
    }})


class Study27AddressControls(unittest.TestCase):
    def test_both_parts_of_exact_mechanism(self):
        original = strict_output([source(1), output(2)], 11, 11)
        changed = strict_output([source(1, tag=19), output(2, address=19)], 19, 19)
        self.assertTrue(original["qualified"])
        self.assertTrue(changed["qualified"])
        self.assertEqual(original["value"], changed["value"])

    def test_wrong_source_actor_address_and_value_only_are_not_positive(self):
        fixtures = (
            [source(1, actor=3), output(2)],
            [source(1, graph=1), output(2)],
            [source(1, owner=1), output(2)],
            [source(1), output(2, address=19)],
            [source(1), output(2, value=19, address=19)],
            [output(1), source(2)],
            [source(1)],
        )
        for events in fixtures:
            with self.subTest(events=events):
                self.assertFalse(strict_output(events, 11, 11)["qualified"])

    def test_multioutput_is_not_unambiguous_causal_exposure(self):
        result = strict_output([
            source(1), output(2, value=4), output(3, value=4),
        ], 11, 11)
        self.assertFalse(result["qualified"])


if __name__ == "__main__":
    unittest.main()
