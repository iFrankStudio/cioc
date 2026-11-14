"""Study68 two focused independent CIL backedge/feedback calibrations."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study68 import expected_code, recurrent_pair


class Study68NativeRandomControlFlow(unittest.TestCase):
    def test_all_frozen_genomes_differ_only_in_native_branch_backedges_and_no_tail(self):
        changed = 0
        for seed in range(4_500_000, 4_500_256):
            short = expected_code(seed, "short-low")
            broad = expected_code(seed, "broad-low")
            self.assertEqual(len(short), 28)
            self.assertEqual(short[-1], "Halt")
            self.assertEqual(broad[-1], "Halt")
            for pc, (a, b) in enumerate(zip(short, broad)):
                if "Branch" in a:
                    self.assertEqual(a["Branch"]["test"], b["Branch"]["test"])
                    self.assertEqual(a["Branch"]["zero"], b["Branch"]["zero"])
                    if pc >= 4:
                        self.assertEqual(a["Branch"]["nonzero"], pc - 1)
                        self.assertGreaterEqual(b["Branch"]["nonzero"], 3)
                        self.assertLess(b["Branch"]["nonzero"], pc)
                    changed += int(a != b)
                else:
                    self.assertEqual(a, b)
        self.assertGreater(changed, 0)

    def test_cross_window_source_feedback_requires_new_native_read_each_window(self):
        def write(epoch, value, source_epoch):
            return {
                "epoch": epoch, "window": epoch // 8, "pc": 12,
                "value": value, "source_tainted": True,
                "latest_native_source_read_epoch": source_epoch,
            }
        low = {"writes": [write(10, 3, 9), write(18, 3, 17)]}
        high = {"writes": [write(10, 8, 9), write(18, 8, 17)]}
        self.assertTrue(recurrent_pair(low, high)["sustained_source_sensitive_2_windows"])
        high["writes"][1]["latest_native_source_read_epoch"] = 6
        self.assertFalse(recurrent_pair(low, high)["sustained_source_sensitive_2_windows"])


if __name__ == "__main__":
    unittest.main()
