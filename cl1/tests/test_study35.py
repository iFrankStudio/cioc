"""Study35 pre-registered native address-shift parser: minimal calibration."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study35 import actor1_native_source_trace, paired_first_shift


def e(tick, pc, kind, actor=1):
    return {
        "tick": tick, "pc": pc, "kind": kind,
        "process": actor, "origin": {"Internal": actor},
    }


PROGRAM = [
    {"GraphReadTag": {"dst": 0, "graph": 1, "node": 2, "next": 1}},
    {"DrawCompute": {"amount": 1, "next": 2}},
    {"Write": {"address": 0, "value": 3, "next": 3}},
]


def trace(tag=11, key=12, overwrite=False, source_graph=2):
    nodes = PROGRAM.copy()
    events = [
        e(1, 0, {"GraphTagRead": {
            "graph": source_graph, "owner": 2, "node": 0, "tag": tag,
        }}),
        e(2, 1, "Executed"),
    ]
    if overwrite:
        nodes.insert(2, {"Const": {"dst": 0, "value": 10, "next": 3}})
        events.extend([
            e(3, 2, "Executed"),
            e(4, 3, {"CellWritten": {"key": key, "value": 5}}),
        ])
        nodes.append(PROGRAM[2])
    else:
        events.append(e(3, 2, {"CellWritten": {"key": key, "value": 5}}))
    initial = {"events": [], "processes": {"1": {
        "code": {"nodes": nodes},
    }}}
    final = {"events": events}
    return actor1_native_source_trace(initial, final,
                                       "mutate-g2" if tag == 19 else "control",
                                       [3 + int(overwrite)] * 24)


class Study35SourceShift(unittest.TestCase):
    def test_successful_source_shift_and_draw(self):
        left, right = trace(), trace(tag=19, key=20)
        result = paired_first_shift(left, right)
        self.assertEqual(result["original"]["key"], 12)
        self.assertEqual(result["mutation"]["key"], 20)
        self.assertTrue(result["original"]["same_epoch_prior_draw"])

    def test_unrelated_source_and_overwrite_do_not_pass(self):
        self.assertEqual(trace(source_graph=3)["source_address_writes"], [])
        self.assertEqual(trace(overwrite=True)["source_address_writes"], [])

    def test_paired_epoch_and_output_value_must_match(self):
        left, right = trace(), trace(tag=19, key=20)
        right["source_address_writes"][0]["epoch"] = 1
        self.assertIsNone(paired_first_shift(left, right))
        right["source_address_writes"][0]["epoch"] = 0
        right["source_address_writes"][0]["value"] = 99
        self.assertIsNone(paired_first_shift(left, right))


if __name__ == "__main__":
    unittest.main()
