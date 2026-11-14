"""Study74-P focused source grammar and native W3 Draw gate controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study74_poolpaid import expected_code, evaluate_pair


class Study74NativeResourceGate(unittest.TestCase):
    def test_policy_on_off_preserves_same_unoptimized_native_cil(self):
        a = expected_code(5_000_000, "broad-ambient-on-intact-low")
        b = expected_code(5_000_000, "broad-ambient-off-intact-high")
        self.assertEqual(a, b)
        self.assertEqual(len(a), 28)
        self.assertEqual(a[27], "Halt")

    def test_w3_draw_must_precede_matching_source_sensitive_native_writes(self):
        def side(value):
            return {
                "writes": [
                    {"window": 1, "pc": 7, "value": value,
                     "source_tainted": True, "latest_native_source_read_epoch": 8,
                     "native_attempt_ordinal": 64, "tick": 110},
                    {"window": 2, "pc": 7, "value": value,
                     "source_tainted": True, "latest_native_source_read_epoch": 16,
                     "native_attempt_ordinal": 68, "tick": 210},
                ],
                "draws": [{"window": 1, "tick": 109}],
            }
        low, high = side(3), side(8)
        self.assertFalse(evaluate_pair(low, high)["primary_A_coupled_native_pool_and_feedback"])
        low["draws"].append({"window": 2, "tick": 209})
        high["draws"].append({"window": 2, "tick": 209})
        self.assertTrue(evaluate_pair(low, high)["primary_A_coupled_native_pool_and_feedback"])


if __name__ == "__main__":
    unittest.main()
