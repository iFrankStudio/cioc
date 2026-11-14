"""Stage67-A: two narrow counterfactual *eligibility* checks, no VM change."""
import sys
import unittest
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study67a import native_eligibility


def fixture(op="DrawCompute", pool=9, stopped=False):
    code = [{"DrawCompute":{"amount":1,"next":0}}
            if op=="DrawCompute" else {"Write":{"address":2,"value":0,"next":0}}]
    return {"compute_pool":pool,"processes":{
        "1":{"stopped":stopped,"credits":0,"pc":0,
             "registers":[0,8,7,9],"code":{"nodes":code}}
    }}


class Stage67ANativeDrawGate(unittest.TestCase):
    def test_only_current_Draw_with_funded_fee_and_transfer_is_eligible(self):
        x=native_eligibility(fixture())
        self.assertTrue(x["pending_Draw_pool_sufficient_for_toll_and_transfer"])
        self.assertEqual(x["native_draw_requested"],8)
        self.assertFalse(native_eligibility(fixture(op="Write"))[
            "pending_Draw_pool_sufficient_for_toll_and_transfer"])
        self.assertFalse(native_eligibility(fixture(stopped=True))[
            "pending_Draw_pool_sufficient_for_toll_and_transfer"])

    def test_pool_eight_cannot_fund_nine_credit_draw_and_attempt(self):
        x=native_eligibility(fixture(pool=8))
        self.assertTrue(x["pending_Draw"])
        self.assertTrue(x["pending_Draw_pool_insufficient"])
        self.assertFalse(x["pending_Draw_pool_sufficient_for_toll_and_transfer"])


if __name__=="__main__":
    unittest.main()
