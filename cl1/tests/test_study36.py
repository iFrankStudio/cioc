"""Study36 scheduler eligibility and compute-inventory control calibration."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study36 import classify, inventory, runnable


def world(pool, *actors):
    return {"compute_pool": pool, "processes": {
        str(i): {"credits": credit, "stopped": stopped}
        for i, (credit, stopped) in enumerate(actors, 1)
    }}


class Study36IgnitionControls(unittest.TestCase):
    def test_positive_world_pool_cannot_wake_zero_credit_actor(self):
        s = world(47, (0, False), (10, True))
        self.assertFalse(runnable(s))
        self.assertEqual(inventory(s), 57)
        self.assertEqual(classify(s), "pool_positive_active_zero_credit")

    def test_positive_process_credit_enables_native_step(self):
        s = world(47, (1, False), (10, True))
        self.assertTrue(runnable(s))
        self.assertEqual(classify(s), "runnable")

    def test_stopped_and_empty_pool_categories_remain_distinct(self):
        self.assertEqual(classify(world(1, (4, True))), "pool_positive_all_stopped")
        self.assertEqual(classify(world(0, (0, False))), "pool_zero_active_zero_credit")
        self.assertEqual(classify(world(0, (4, True))), "pool_zero_all_stopped")


if __name__ == "__main__":
    unittest.main()
