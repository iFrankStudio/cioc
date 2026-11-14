"""Study77 original CIL generator differs only in registered retry policy and controls."""
import sys
import unittest
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from analyze_study77 import expected_code,MODES

class Study77NativeResourceControls(unittest.TestCase):
    def test_exact_same_random_CIL_with_recoverable_vs_legacy_world_rule(self):
        self.assertEqual(len(MODES),16)
        for seed in range(5_100_000,5_100_128):
            broad=expected_code(seed,"broad-ambient-retry-intact-low")
            legacy=expected_code(seed,"broad-ambient-legacy-intact-low")
            short=expected_code(seed,"short-ambient-retry-intact-low")
            no_draw=expected_code(seed,"broad-ambient-retry-draw-ablated-low")
            self.assertEqual(broad,legacy)
            self.assertEqual(len(broad),28)
            self.assertEqual(broad[-1],"Halt")
            for i,(a,b,c) in enumerate(zip(broad,short,no_draw)):
                if isinstance(a,dict) and "Branch" in a:
                    self.assertEqual({k:v for k,v in a["Branch"].items() if k!="nonzero"},
                                     {k:v for k,v in b["Branch"].items() if k!="nonzero"})
                else:
                    self.assertEqual(a,b)
                if isinstance(a,dict) and "DrawCompute" in a:
                    self.assertEqual(c,{"Const":{"dst":1,"value":8,"next":i+1}})
                else:
                    self.assertEqual(a,c)

if __name__=="__main__":
    unittest.main()
