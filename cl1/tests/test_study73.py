"""Study73 native source-feedback and compute beyond starting local credit budget.

This calibrates only the frozen paired random-code grammar; ecological assertions
are checked independently against original Rust SQLite in analyze_study73.py.
"""
import sys
import unittest
from pathlib import Path

sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from analyze_study73 import expected_code, MODES, propagate_r0, evaluate_pair


class Study73ConservedNativeEnergy(unittest.TestCase):
    def test_pair_genomes_only_declared_random_branch_and_draw_ablation(self):
        self.assertEqual(len(MODES),16)
        self.assertEqual(len(set(MODES)),16)
        for seed in range(4_900_000,4_900_128):
            broad=expected_code(seed,"broad-ambient-intact-low")
            short=expected_code(seed,"short-ambient-intact-low")
            disabled=expected_code(seed,"broad-ambient-draw-ablated-low")
            self.assertEqual(len(broad),28)
            self.assertEqual(broad[-1],"Halt")
            for pc in range(28):
                b,s,d=broad[pc],short[pc],disabled[pc]
                if isinstance(b,dict) and "Branch" in b:
                    self.assertEqual({k:v for k,v in b["Branch"].items() if k!="nonzero"},
                                     {k:v for k,v in s["Branch"].items() if k!="nonzero"})
                else:
                    self.assertEqual(b,s)
                if isinstance(b,dict) and "DrawCompute" in b:
                    self.assertEqual(d,{"Const":{"dst":1,"value":8,"next":pc+1}})
                else:
                    self.assertEqual(b,d)

    def test_new_sensor_read_taint_and_const_kill_are_distinct(self):
        self.assertEqual(propagate_r0({"Read":{"dst":0,"address":2,"next":4}},
                                      0,False,8),(8,True))
        self.assertEqual(propagate_r0({"Const":{"dst":0,"value":2,"next":5}},
                                      8,True,8),(2,False))
        # A W3 label alone is not proof of spending beyond the initial
        # 64 native credits: provenance must confirm both the Draw and the
        # actual charged native attempt ordinal at the successful output.
        def write(epoch,value,ordinal):
            return {"epoch":epoch,"window":epoch//8,"tick":epoch*12+2,
                    "pc":10,"value":value,"source_tainted":True,
                    "latest_native_source_read_epoch":epoch,
                    "native_attempt_ordinal":ordinal}
        low={"writes":[write(8,3,101),write(16,3,201)],
             "draws":[{"tick":40,"window":0}]}
        high={"writes":[write(8,8,101),write(16,8,201)],
              "draws":[{"tick":40,"window":0}]}
        self.assertTrue(evaluate_pair(low,high)[
            "primary_A_coupled_native_pool_and_feedback"])
        high["writes"][1]["native_attempt_ordinal"]=64
        self.assertFalse(evaluate_pair(low,high)[
            "primary_A_coupled_native_pool_and_feedback"])


if __name__=="__main__":
    unittest.main()
