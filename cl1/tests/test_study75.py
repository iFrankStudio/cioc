"""Study75 native-resource entry eligibility is distinct from stopped/non-Draw."""
import sys
import unittest
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from analyze_study75 import classify

def state(op,credits,pool,stopped=False,events=None):
    return {
        "compute_pool":pool,
        "processes":{"1":{"pc":0,"credits":credits,"stopped":stopped,
            "code":{"nodes":[op]}}},
        "events":events or [],
    }

class Study75ResourceEntry(unittest.TestCase):
    def test_positive_pool_non_draw_is_not_eligible(self):
        self.assertEqual(classify(state({"Read":{"dst":0,"address":2,"next":0}},0,80)),
                         "zero/local-nonDraw")
    def test_stopped_actors_never_inherit_any_pool_draw_eligibility(self):
        self.assertEqual(classify(state({"DrawCompute":{"amount":1,"next":0}},0,80,True)),
                         "other-stopped")
    def test_pool_paid_native_draw_requires_fee_plus_draw_resources(self):
        op={"DrawCompute":{"amount":1,"next":0}}
        self.assertEqual(classify(state(op,0,8)),"zero/local-Draw-uncapitalized")
        self.assertEqual(classify(state(op,0,9)),"zero/local-pool-paid-eligible")

if __name__=="__main__":
    unittest.main()
