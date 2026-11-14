"""Study79 source-PC native resource divergence: explicit retry vs fatal source."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from analyze_study79 import event_cause,native_events


class Study79OriginalNativeCause(unittest.TestCase):
    def test_native_deferred_Draw_is_identified_before_legacy_fatal_trap(self):
        retry=[{
            "pc":4,"kind":"DrawDeferred","source_opcode":"DrawCompute",
            "tick":14,"pool_paid_attempt":False,
            "details":{"DrawDeferred":{"requested":8,"available":2}},
        }]
        legacy=[{
            "pc":4,"kind":"Trapped","source_opcode":"DrawCompute",
            "tick":14,"pool_paid_attempt":False,
            "details":{"Trapped":"insufficient world compute pool"},
        }]
        self.assertEqual(event_cause(retry,legacy)["kind"],
                         "deferred-instead-of-legacy-fatal-Draw")

    def test_external_injected_event_cannot_count_as_native_source(self):
        initial={"events":[],"processes":{
            "1":{"code":{"nodes":[{"DrawCompute":{"amount":1,"next":0}}]}}
        }}
        forged={"events":[{
            "tick":1,"origin":{"Internal":2},"process":1,"pc":0,
            "kind":{"DrawDeferred":{"requested":8,"available":0}}
        }]}
        with self.assertRaises(ValueError):
            native_events(initial,forged)


if __name__=="__main__":
    unittest.main()
