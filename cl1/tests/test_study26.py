"""Focused synthetic classification checks for post-outcome Study 26."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study26 import classify


def event(tick, actor, kind):
    return {
        "tick": tick, "pc": tick, "process": actor,
        "origin": {"Internal": actor}, "kind": kind,
    }


def graph(tick, tag=11, actor=3):
    return event(tick, actor, {"GraphTagRead": {
        "graph": 2, "owner": 2, "node": 0, "tag": tag,
    }})


def write(tick, value=11, cell=90, actor=3):
    return event(tick, actor, {"CellWritten": {"key": cell, "value": value}})


CONTROL = [graph(1), write(2)]


class Study26MechanismTests(unittest.TestCase):
    def test_changed_source_value_and_output(self):
        result = classify(CONTROL, [graph(1, 19), write(2, 19)], 3, 90)
        self.assertEqual(result["category"], "changed_numeric_output")

    def test_changed_read_unchanged_output(self):
        result = classify(CONTROL, [graph(1, 19), write(2, 11)], 3, 90)
        self.assertEqual(result["category"], "same_numeric_output_after_changed_tag")

    def test_changed_read_failed_output_and_trap(self):
        modified = [graph(1, 19), event(2, 3, {"Trapped": "halted"})]
        result = classify(CONTROL, modified, 3, 90)
        self.assertEqual(result["category"], "changed_read_no_output")
        self.assertEqual(result["altered_actor_traps"], 1)

    def test_no_read_failed_output(self):
        result = classify(CONTROL, [event(1, 3, {"Trapped": "access denied"})], 3, 90)
        self.assertEqual(result["category"], "no_changed_read_no_output")

    def test_write_but_not_following_source_read_is_ambiguous(self):
        result = classify(CONTROL, [write(1, 19)], 3, 90)
        self.assertEqual(result["category"], "other_or_ambiguous")

    def test_source_was_not_mutated_is_a_hard_audit_error(self):
        with self.assertRaises(ValueError):
            classify(CONTROL, [graph(1, 11), write(2, 11)], 3, 90)


if __name__ == "__main__":
    unittest.main()
