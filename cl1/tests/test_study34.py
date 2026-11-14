"""Study34 retrospective source-read reachability negative and positive controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study34 import eligible_initial_read
from analyze_study33 import expected_program_continuations, FIRST, SEEDS


class Study34SourceReadGate(unittest.TestCase):
    def test_all_frozen_study33_continuations_have_no_source2_node0_read(self):
        for seed in range(FIRST, FIRST + SEEDS):
            for nodes in expected_program_continuations(seed).values():
                self.assertFalse(eligible_initial_read(nodes))

    def test_positive_source2_node0_read_is_detected(self):
        self.assertTrue(eligible_initial_read([
            {"GraphReadTag": {"dst": 0, "graph": 2, "node": 0, "next": 1}}
        ]))

    def test_wrong_graph_or_node_does_not_count(self):
        self.assertFalse(eligible_initial_read([
            {"GraphReadTag": {"dst": 0, "graph": 3, "node": 0, "next": 1}},
            {"GraphReadTag": {"dst": 0, "graph": 2, "node": 2, "next": 2}},
            {"GraphReadTag": {"dst": 0, "graph": 1, "node": 0, "next": 3}},
        ]))

    def test_non_read_graph_operand_does_not_count(self):
        self.assertFalse(eligible_initial_read([
            {"GraphCopyNode": {"graph": 2, "source": 0, "dst": 0, "next": 1}},
            "Halt",
        ]))


if __name__ == "__main__":
    unittest.main()
