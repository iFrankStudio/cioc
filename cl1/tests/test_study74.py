"""Study74 only two critical physical-state false-positive gate controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study74 import classify_snapshot


def make_state(op, credits=0, stopped=False, pool=8):
    state = {
        "compute_pool": pool,
        "processes": {"1":{
            "pc":0, "credits":credits, "stopped":stopped,
            "code":{"nodes":[op]},
        }},
        "events": [],
    }
    if stopped:
        state["events"] = [{
            "process":1,"pc":0,"tick":3,
            "origin":{"Internal":1},
            "kind":{"Trapped":"insufficient world compute pool"},
        }]
    return state


class Study74CreditGate(unittest.TestCase):
    def test_only_an_unstopped_draw_at_zero_credits_can_enter_pool_gateway(self):
        draw = {"DrawCompute":{"amount":1,"next":0}}
        self.assertEqual(classify_snapshot(make_state(draw)),
                         "alive_zero_credits_positive_pool_at_Draw")
        self.assertEqual(classify_snapshot(make_state(draw,pool=0)),
                         "alive_zero_credits_empty_pool")
        self.assertEqual(classify_snapshot(make_state(draw,credits=1)),
                         "runnable_positive_local_credits")
        other={"Read":{"dst":0,"address":1,"next":0}}
        self.assertEqual(classify_snapshot(make_state(other)),
                         "alive_zero_credits_positive_pool_not_Draw")

    def test_stopped_fatal_draw_does_not_count_as_live_credit_gateway(self):
        draw = {"DrawCompute":{"amount":1,"next":0}}
        self.assertEqual(
            classify_snapshot(make_state(draw,stopped=True)),
            "stopped_fatal_insufficient_pool_Draw"
        )


if __name__=="__main__":
    unittest.main()
