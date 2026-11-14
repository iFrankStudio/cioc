"""Study45 minimal executable graph closed-edge and stale-parent-PC controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study45 import invalid_edges, op_type


class Study45ChildControlFlow(unittest.TestCase):
    def test_two_node_copied_write_with_parent_absolute_next_is_invalid(self):
        code = [
            {"GraphNew": {"dst": 3, "next": 1}},
            {"Write": {"address": 0, "value": 3, "next": 12}},
        ]
        self.assertEqual(invalid_edges(code), [{
            "pc": 1, "op": "Write", "edge": "next",
            "target": 12, "code_node_count": 2,
        }])

    def test_portable_two_node_program_is_edge_closed(self):
        program = [
            {"GraphNew": {"dst": 3, "next": 1}},
            {"Write": {"address": 0, "value": 3, "next": 0}},
        ]
        self.assertEqual(invalid_edges(program), [])
        self.assertEqual(op_type(program[1]), "Write")

    def test_branch_requires_both_edges_valid_and_halt_has_no_edge(self):
        self.assertEqual(invalid_edges([
            {"Branch": {"test": 0, "nonzero": 1, "zero": 12}},
            "Halt",
        ]), [{
            "pc": 0, "op": "Branch", "edge": "zero",
            "target": 12, "code_node_count": 2,
        }])
        self.assertEqual(invalid_edges(["Halt"]), [])


if __name__ == "__main__":
    unittest.main()
