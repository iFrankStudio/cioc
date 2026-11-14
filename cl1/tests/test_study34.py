"""Study34 synthetic event provenance and CIL register-indirection controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study34 import source2_read_events
from analyze_study33 import expected_program_continuations, FIRST, SEEDS


def state(op_graph=1, op_node=2, resolved_graph=2, resolved_node=0,
          actor=1, owner=2, tag=11, origin=None):
    return {
        "processes": {str(actor): {"code": {"nodes": [
            {"GraphReadTag": {
                "dst": 2, "graph": op_graph, "node": op_node, "next": 1
            }}
        ]}}},
        "events": [{
            "tick": 6, "pc": 0, "process": actor,
            "origin": {"Internal": actor} if origin is None else origin,
            "kind": {"GraphTagRead": {
                "graph": resolved_graph, "owner": owner,
                "node": resolved_node, "tag": tag
            }},
        }],
    }


class Study34ResolvedSource2(unittest.TestCase):
    def test_register_operand_is_not_graph_id(self):
        out = source2_read_events(state(op_graph=1, op_node=2))
        self.assertEqual(len(out), 1)
        self.assertEqual(out[0]["opcode_graph_register"], 1)
        self.assertEqual(out[0]["resolved_graph"], 2)

    def test_wrong_resolved_graph_or_node_is_excluded(self):
        self.assertEqual(source2_read_events(state(resolved_graph=3)), [])
        self.assertEqual(source2_read_events(state(resolved_node=2)), [])
        self.assertEqual(source2_read_events(state(owner=1)), [])

    def test_mutated_source_tag_is_accepted(self):
        self.assertEqual(source2_read_events(state(tag=19))[0]["tag"], 19)

    def test_unattributed_native_source_is_rejected(self):
        with self.assertRaises(ValueError):
            source2_read_events(state(origin="External"))

    def test_initial_grammar_uses_register_indirection(self):
        # Register operand 1 is not equivalent to resolved graph object 1.
        found = 0
        for seed in range(FIRST, FIRST + SEEDS):
            for nodes in expected_program_continuations(seed).values():
                for op in nodes:
                    if "GraphReadTag" in op and op["GraphReadTag"]["graph"] == 1:
                        found += 1
        self.assertGreater(found, 0)


if __name__ == "__main__":
    unittest.main()
