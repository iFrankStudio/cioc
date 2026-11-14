"""Study55: frozen RNG alignment and exact allowable branch topology differences."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study55_local import expected_continuations as local_program
from analyze_study55_uniform import expected_continuations as uniform_program
from analyze_study55 import normalize_program


class Study55CodeTopology(unittest.TestCase):
    def test_paired_seeds_preserve_every_opcode_choice_and_nonbranch_operand(self):
        changed = 0
        for seed in range(3_800_000, 3_800_256):
            local, uniform = local_program(seed), uniform_program(seed)
            for actor in (1, 2, 3):
                a = local[actor]
                b = uniform[actor]
                self.assertEqual(len(a), 72)
                self.assertEqual(normalize_program(["Halt"] * 9 + b + ["Halt"])[9:81], a)
                for index, (left, right) in enumerate(zip(a, b), start=9):
                    if "Branch" in left:
                        self.assertEqual(left["Branch"]["nonzero"], index - 2)
                        self.assertTrue(0 <= right["Branch"]["nonzero"] < index)
                        changed += int(left != right)
        self.assertGreater(changed, 0)

    def test_unregistered_branch_target_is_rejected(self):
        nodes = ["Halt"] * 82
        nodes[9] = {"Branch": {"test": 1, "nonzero": 81, "zero": 10}}
        with self.assertRaises(ValueError):
            normalize_program(nodes)


if __name__ == "__main__":
    unittest.main()
