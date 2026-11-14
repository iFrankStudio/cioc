"""Study33 strict native register/source and resource-temporal controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study33 import (
    trace_source_and_pool,
    actions,
    expected_program_continuations,
)


def e(tick, pc, kind, actor=1):
    return {
        "tick": tick, "pc": pc, "kind": kind,
        "process": actor, "origin": {"Internal": actor},
    }


def read(tick=1, tag=11, actor=1, owner=2):
    return e(tick, 0, {"GraphTagRead": {
        "graph": 2, "owner": owner, "node": 0, "tag": tag,
    }}, actor)


def output(tick=4, key=11, value=8, actor=1):
    return e(tick, 3, {"CellWritten": {
        "key": key, "value": value,
    }}, actor)


PROGRAM = [
    {"GraphReadTag": {"dst": 2, "graph": 1, "node": 2, "next": 1}},
    {"Add": {"dst": 0, "lhs": 0, "rhs": 2, "next": 2}},
    {"DrawCompute": {"amount": 1, "next": 3}},
    {"Write": {"address": 0, "value": 3, "next": 4}},
    {"Halt": None},
]


class Study33JointSourceResource(unittest.TestCase):
    def test_source_label_survives_add_and_native_draw_precedes_write(self):
        result = trace_source_and_pool([
            read(), e(2, 1, "Executed"), e(3, 2, "Executed"), output(),
        ], PROGRAM, 11, 11, 45)
        self.assertEqual(result["qualifying_output_events"], 1)
        self.assertTrue(result["unique_source_write"]["prior_draw"])
        self.assertTrue(result["unique_source_write"]["native_budget_extension"])

    def test_mutant_source_tag_relocates_output(self):
        events = [
            read(tag=19), e(2, 1, "Executed"),
            e(3, 2, "Executed"), output(key=19),
        ]
        result = trace_source_and_pool(events, PROGRAM, 19, 19, 0)
        self.assertIsNotNone(result["unique_source_write"])

    def test_wrong_source_or_actor_cannot_qualify(self):
        for data in (
            [read(owner=3), e(2, 1, "Executed"), e(3, 2, "Executed"), output()],
            [read(actor=2), e(2, 1, "Executed"), e(3, 2, "Executed"), output()],
            [read(), e(2, 1, "Executed"), e(3, 2, "Executed"),
             output(actor=2)],
        ):
            with self.subTest(events=data):
                result = trace_source_and_pool(data, PROGRAM, 11, 11, 0)
                self.assertIsNone(result["unique_source_write"])

    def test_native_draw_must_precede_target_output(self):
        result = trace_source_and_pool([
            read(), e(2, 1, "Executed"), output(tick=3),
            e(4, 2, "Executed"),
        ], PROGRAM, 11, 11, 0)
        self.assertIsNotNone(result["unique_source_write"])
        self.assertFalse(result["unique_source_write"]["prior_draw"])

    def test_failed_draw_does_not_qualify(self):
        result = trace_source_and_pool([
            read(), e(2, 1, "Executed"),
            e(3, 2, {"Trapped": "pool empty"}), output(),
        ], PROGRAM, 11, 11, 0)
        self.assertFalse(result["unique_source_write"]["prior_draw"])

    def test_register_override_clears_address_origin(self):
        program = PROGRAM[:2] + [
            {"Const": {"dst": 0, "value": 11, "next": 3}}
        ] + PROGRAM[3:]
        result = trace_source_and_pool([
            read(), e(2, 1, "Executed"), e(3, 2, "Executed"), output(),
        ], program, 11, 11, 0)
        self.assertIsNone(result["unique_source_write"])

    def test_no_overclaim_from_duplicate_target_outputs(self):
        result = trace_source_and_pool([
            read(), e(2, 1, "Executed"), e(3, 2, "Executed"),
            output(), output(tick=5),
        ], PROGRAM, 11, 11, 0)
        self.assertEqual(result["qualifying_output_events"], 2)
        self.assertIsNone(result["unique_source_write"])

    def test_source_mutation_resource_controls_frozen(self):
        initial = {
            "graph_objects": {"2": {
                "owner": 2,
                "readers": [1],
                "nodes": [{"GraphNew": {"dst": 3, "next": 1}}],
            }}
        }
        self.assertEqual(len(actions("control", initial)), 0)
        self.assertEqual(len(actions("mutate-g2", initial)), 2)
        self.assertEqual(actions("mutate-g2", initial)[1]["action"][
            "PutGraph"]["object"]["nodes"][0], "Halt")
        self.assertEqual(len(actions("mutate-g2-empty-pool", initial)), 3)
        self.assertEqual(len(actions("mutate-g2-refill", initial)), 8)
        self.assertEqual(len(actions("refill", initial)), 6)
        self.assertEqual(len(actions("sham", initial)), 1)
        self.assertEqual(len(actions("exact-original-g2", initial)), 2)
        self.assertEqual(len(actions("zero-credit", initial)), 3)
        self.assertEqual(len(expected_program_continuations(2_600_000)), 3)


if __name__ == "__main__":
    unittest.main()
