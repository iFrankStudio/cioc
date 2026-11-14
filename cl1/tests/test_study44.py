"""Study44 calibrated generic source-register diversity and native child Write ancestry."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study44 import (
    expected_continuations, verify_native_child_write, neutral_candidate,
)


def child_event(origin=None, tick=5):
    return {
        "tick": tick, "pc": 0, "process": 4,
        "origin": {"Internal": 4} if origin is None else origin,
        "kind": {"CellWritten": {"key": 3, "value": 7}},
    }


def child_state(op=None):
    return {"processes": {"4": {"code": {"nodes": [
        {"Write": {"address": 0, "value": 3, "next": 1}}
        if op is None else op
    ]}}}}


class Study44ChildOutput(unittest.TestCase):
    def test_valid_native_child_after_launch_with_write_code(self):
        event = child_event()
        got = verify_native_child_write(
            event, child_state(), child_state(), {4: 3}
        )
        self.assertEqual(got, {"key": 3, "value": 7})

    def test_no_launch_external_origin_or_child_before_launch_rejected(self):
        cases = [
            (child_event(), {}, {}),
            (child_event(origin="External"), {4: 3}, {}),
            (child_event(tick=2), {4: 3}, {}),
        ]
        for event, launches, _ in cases:
            with self.subTest(event=event, launches=launches):
                with self.assertRaises(ValueError):
                    verify_native_child_write(
                        event, child_state(), child_state(), launches
                    )

    def test_graph_new_is_not_a_shared_write(self):
        with self.assertRaises(ValueError):
            verify_native_child_write(
                child_event(), child_state({"GraphNew": {"dst": 3, "next": 1}}),
                child_state({"GraphNew": {"dst": 3, "next": 1}}), {4: 3}
            )

    def test_registered_extra_lcg_draw_yields_general_nonzero_source_index(self):
        g = expected_continuations(3_300_000)
        self.assertEqual(set(g), {1, 2, 3})
        self.assertEqual(g, expected_continuations(3_300_000))
        seeded = []
        for seed in range(3_300_000, 3_300_128):
            for program in expected_continuations(seed).values():
                self.assertEqual(len(program), 27)
                for op in program:
                    if "Const" in op and op["Const"]["dst"] == 2:
                        seeded.append(op["Const"]["value"])
        self.assertTrue(seeded)
        self.assertTrue(all(0 <= x < 36 for x in seeded))
        self.assertTrue(any(x > 0 for x in seeded))

    def test_zero_credit_neutral_scheduler_is_not_goal_directed(self):
        s = {"compute_pool": 4, "processes": {
            "1": {"credits": 0, "stopped": True},
            "2": {"credits": 0, "stopped": False},
            "3": {"credits": 0, "stopped": False},
        }}
        self.assertEqual(neutral_candidate(s), (2, 4))


if __name__ == "__main__":
    unittest.main()
