"""Study66 two focused native source and credit-provenance calibration cases."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from analyze_study66 import propagate_r0, evaluate_pair


class Study66CausalNativeEnergyAndInformation(unittest.TestCase):
    def test_native_read_add_sub_and_const_invalidate_taint_as_registered(self):
        r,taint=propagate_r0({"Read":{"dst":0,"address":2,"next":4}},0,False,8)
        self.assertEqual((r,taint),(8,True))
        r,taint=propagate_r0({"Add":{"dst":0,"lhs":0,"rhs":1,"next":5}},r,taint,8)
        self.assertEqual((r,taint),(16,True))
        r,taint=propagate_r0({"Sub":{"dst":0,"lhs":0,"rhs":1,"next":6}},r,taint,8)
        self.assertEqual((r,taint),(8,True))
        r,taint=propagate_r0({"Const":{"dst":0,"value":0,"next":7}},r,taint,8)
        self.assertEqual((r,taint),(0,False))

    def test_native_draw_events_are_required_beyond_external_pool_inflow(self):
        def writer(epoch,value,tick):
            return {"epoch":epoch,"window":epoch//8,"pc":9,"tick":tick,
                    "value":value,"source_tainted":True,
                    "latest_native_source_read_epoch":epoch}
        low={"writes":[writer(9,3,111),writer(17,3,211)],
             "draws":[{"epoch":9,"window":1,"tick":109,"pc":12},
                      {"epoch":17,"window":2,"tick":209,"pc":12}]}
        high={"writes":[writer(9,8,111),writer(17,8,211)],
              "draws":[{"epoch":9,"window":1,"tick":109,"pc":12},
                       {"epoch":17,"window":2,"tick":209,"pc":12}]}
        observed=evaluate_pair(low,high)
        self.assertTrue(observed["primary_A_coupled_native_pool_and_feedback"])
        self.assertTrue(observed["secondary_B_fresh_native_pool_each_late_window"])
        high["draws"]=[]
        observed=evaluate_pair(low,high)
        self.assertTrue(observed["paired_source_sensitive_both_windows"])
        self.assertFalse(observed["primary_A_coupled_native_pool_and_feedback"])
        self.assertFalse(observed["secondary_B_fresh_native_pool_each_late_window"])


if __name__=="__main__":
    unittest.main()
