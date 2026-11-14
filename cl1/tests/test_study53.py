"""Study53 small causal registration controls for exact operand-only topology and native ancestry."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study53 import normalize_program, ancestry


class Study53RegisterTopology(unittest.TestCase):
    def test_separate_register_operand_rewrites_normalize_to_original(self):
        nodes = ["Halt"] * 82
        nodes[9] = {"GraphCopyNode": {
            "graph": 3, "source": 2, "dst": 0, "next": 10
        }}
        nodes[10] = {"GraphSetNext": {
            "graph": 3, "node": 0, "target": 0, "next": 11
        }}
        baseline = normalize_program(nodes)
        self.assertEqual(baseline[9]["GraphCopyNode"]["dst"], 2)
        self.assertEqual(baseline[10]["GraphSetNext"]["node"], 2)
        self.assertEqual(nodes[9]["GraphCopyNode"]["dst"], 0)
        self.assertEqual(nodes[10]["GraphSetNext"]["node"], 0)

    def test_unexpected_register_change_is_rejected_not_misreported(self):
        nodes = ["Halt"] * 82
        nodes[9] = {"GraphCopyNode": {
            "graph": 3, "source": 2, "dst": 1, "next": 10
        }}
        with self.assertRaises(ValueError):
            normalize_program(nodes)


if __name__ == "__main__":
    unittest.main()
