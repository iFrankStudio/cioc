"""Only two Study65 synthetic calibrations: native dataflow and two-window pairing."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study65 import propagate_r0, recurrent_pair


class Study65SourceFeedbackGates(unittest.TestCase):
    def test_read_add_sub_preserve_native_taint_but_const_invalidates(self):
        r, t = propagate_r0({"Read": {"dst":0, "address":2, "next":4}}, 0, False, 8)
        self.assertEqual((r, t), (8, True))
        r, t = propagate_r0({"Add": {"dst":0, "lhs":0, "rhs":1, "next":5}}, r, t, 8)
        r, t = propagate_r0({"Sub": {"dst":0, "lhs":0, "rhs":1, "next":6}}, r, t, 8)
        self.assertEqual((r, t), (8, True))
        r, t = propagate_r0({"Const": {"dst":0, "value":0, "next":7}}, r, t, 8)
        self.assertEqual((r, t), (0, False))

    def test_recurrent_pair_requires_native_read_in_both_late_windows(self):
        def record(epoch, val, source_epoch):
            return {"epoch":epoch, "window":epoch//8, "pc":8,
                    "value":val, "source_tainted":True,
                    "latest_native_source_read_epoch":source_epoch}
        low={"writes":[record(10,3,9),record(19,3,18)]}
        high={"writes":[record(10,8,9),record(19,8,18)]}
        self.assertTrue(recurrent_pair(low,high)["sustained_source_sensitive_2_windows"])
        high["writes"][1]["latest_native_source_read_epoch"]=4
        self.assertFalse(recurrent_pair(low,high)["sustained_source_sensitive_2_windows"])


if __name__ == "__main__":
    unittest.main()
