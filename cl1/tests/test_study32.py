"""Study32 registered native-resource grammar and paired intervention controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study32 import expected_program_continuations, actions


class Study32ResourceProtocolTests(unittest.TestCase):
    def test_independent_lcg_generator_contains_only_registered_native_opcodes(self):
        programs = expected_program_continuations(2_500_000)
        self.assertEqual(set(programs), {1, 2, 3})
        self.assertTrue(all(len(nodes) == 18 for nodes in programs.values()))
        allowed = {
            "GraphCopyFrom", "GraphCopyNode", "DrawCompute",
            "GraphReadTag", "Add", "Write", "Const",
            "Branch", "ReturnCompute", "GraphGrantRead",
        }
        self.assertTrue(all(
            next(iter(op)) in allowed
            for program in programs.values() for op in program
        ))
        self.assertNotEqual(programs, expected_program_continuations(2_500_001))

    def test_initial_pool_zeroing_and_explicit_external_refill_are_distinct(self):
        self.assertEqual(actions("control"), [])
        self.assertEqual(len(actions("sham")), 1)
        self.assertEqual(len(actions("empty-pool")), 1)
        self.assertEqual(actions("empty-pool")[0]["action"],
                         {"SetComputePool": {"credits": 0}})
        self.assertEqual(len(actions("refill")), 6)
        self.assertEqual([a["epoch"] for a in actions("refill")],
                         [8, 8, 8, 16, 16, 16])
        self.assertEqual(len(actions("empty-pool-refill")), 7)
        self.assertEqual(len(actions("zero-credit")), 3)

    def test_unknown_intervention_branch_fails_closed(self):
        with self.assertRaises(ValueError):
            actions("hidden-repair")


if __name__ == "__main__":
    unittest.main()
