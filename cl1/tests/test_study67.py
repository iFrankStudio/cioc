"""Study67 two strict original-state failure-gate calibrations."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from analyze_study67 import resource_state, terminal_cause


class Study67FatalVersusIgnition(unittest.TestCase):
    def test_positive_pool_does_not_make_stopped_or_credit_zero_actor_runnable(self):
        self.assertEqual(resource_state({"stopped":True,"credits":7},80),"stopped_with_pool")
        self.assertEqual(resource_state({"stopped":False,"credits":0},80),
                         "zero_credit_deadlock_with_pool")
        self.assertEqual(resource_state({"stopped":False,"credits":1},80),"still_runnable")

    def test_only_real_native_draw_trap_is_resource_exhaustion_terminal(self):
        program=[
            {"Const":{"dst":1,"value":8,"next":1}},
            {"DrawCompute":{"amount":1,"next":2}},
        ]
        e={"process":1,"pc":1,"origin":{"Internal":1},
           "kind":{"Trapped":"insufficient world compute pool"}}
        self.assertEqual(terminal_cause(e,program),"fatal_native_DrawCompute_pool_insufficient")
        e["pc"]=0
        with self.assertRaises(ValueError):
            terminal_cause(e,program)


if __name__=="__main__":
    unittest.main()
