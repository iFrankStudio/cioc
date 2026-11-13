"""Study31 native register def-use lineage, with overwrite adversarial controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study31 import trace_native_address


def e(tick, pc, kind, actor=1):
    return {"tick": tick, "pc": pc, "process": actor,
            "origin": {"Internal": actor}, "kind": kind}


def tag(tick=1, pc=0, value=11, graph=2, owner=2, actor=1):
    return e(tick, pc, {"GraphTagRead": {
        "graph": graph, "owner": owner, "node": 0, "tag": value,
    }}, actor)


def write(tick=3, pc=2, key=11, value=1, actor=1):
    return e(tick, pc, {"CellWritten": {
        "key": key, "value": value,
    }}, actor)


PROG = [
    {"GraphReadTag": {"dst": 0, "graph": 1, "node": 2, "next": 1}},
    {"Branch": {"test": 1, "nonzero": 1, "zero": 2}},
    {"Write": {"address": 0, "value": 3, "next": 3}},
]


class Study31NativeLineage(unittest.TestCase):
    def test_successful_graph2_tag_written_as_address(self):
        result = trace_native_address([
            tag(), e(2, 1, "Executed"), write(),
        ], PROG, 11, 11)
        self.assertTrue(result["qualified_unique"])
        self.assertEqual(result["qualified_writes"][0]["address_register"], 0)

    def test_registered_code_tag_mutant_preserves_lineage(self):
        result = trace_native_address([tag(value=19), write(2, key=19)],
                                      PROG, 19, 19)
        self.assertTrue(result["qualified_unique"])

    def test_const_override_eliminates_graph_source_label(self):
        program = PROG[:1] + [
            {"Const": {"dst": 0, "value": 11, "next": 2}},
        ] + PROG[2:]
        result = trace_native_address([
            tag(), e(2, 1, "Executed"), write(),
        ], program, 11, 11)
        self.assertFalse(result["qualified_unique"])

    def test_wrong_source_cannot_claim_lineage(self):
        result = trace_native_address([
            tag(graph=1, owner=1), write(2),
        ], PROG, 11, 11)
        self.assertFalse(result["qualified_unique"])

    def test_native_add_propagates_graph_source_label(self):
        program = PROG[:1] + [
            {"Add": {"dst": 0, "lhs": 0, "rhs": 3, "next": 2}},
        ] + PROG[2:]
        result = trace_native_address([
            tag(), e(2, 1, "Executed"), write(),
        ], program, 11, 11)
        self.assertTrue(result["qualified_unique"])

    def test_write_using_another_register_does_not_pass(self):
        program = PROG[:2] + [
            {"Write": {"address": 2, "value": 3, "next": 3}},
        ]
        result = trace_native_address([tag(), write(2)], program, 11, 11)
        self.assertFalse(result["qualified_unique"])

    def test_graph_copy_from_node_register_overwrite_prevents_false_witness(self):
        program = PROG[:1] + [
            {"GraphCopyFrom": {"target": 3, "source": 1, "node": 0, "next": 2}},
        ] + PROG[2:]
        result = trace_native_address([
            tag(), e(2, 1, {"GraphCopied": {}}), write(),
        ], program, 11, 11)
        self.assertFalse(result["qualified_unique"])

    def test_wrong_pc_and_wrong_internal_actor_rejected_or_excluded(self):
        with self.assertRaises(ValueError):
            trace_native_address([tag(pc=22)], PROG, 11, 11)
        result = trace_native_address([tag(actor=2), write(2)], PROG, 11, 11)
        self.assertFalse(result["qualified_unique"])

    def test_failed_tag_read_never_sets_source(self):
        result = trace_native_address([
            e(1, 0, {"Trapped": "bad handle"}), write(2),
        ], PROG, 11, 11)
        self.assertFalse(result["qualified_unique"])


if __name__ == "__main__":
    unittest.main()
