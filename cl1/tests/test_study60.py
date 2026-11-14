"""Study60 minimal fake native child and authentic local opcode-lineage calibration."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from analyze_study60 import classify_run, source_history


def e(tick,actor,name,fields=None,origin=None,pc=0):
    return {
        "tick":tick,"process":actor,"pc":pc,
        "origin":{"Internal":actor} if origin is None else origin,
        "kind":{name:fields} if fields is not None else name,
    }


def fixture(forged=False):
    original={"events":[],"processes":{
        "1":{"code":{"nodes":[{"GraphNew":{"dst":3,"next":1}}]}},
    }}
    copy=e(1,1,"GraphNodeCopiedLocal",{
        "graph":1,"owner":1,"source_process":1,
        "source_node":0,"new_node":0,
    })
    launch=e(2,1,"GraphLaunched",{
        "parent_process":1,"child_process":4,"source_graph":1,
        "transferred_credits":3,"moved_nodes":1,
    })
    native=e(3,4,"Executed",origin="External" if forged else None)
    states=[]
    for ep in range(24):
        events=[copy,launch,native] if ep>=16 else []
        states.append({
            "events":events,
            "processes":{
                **original["processes"],
                **({"4":{"code":{"nodes":[{
                    "Const":{"dst":0,"value":2,"next":0},
                }]}}} if ep>=16 else {}),
            },
        })
    return original,states,[]


class Study60NativeChildAncestry(unittest.TestCase):
    def test_valid_child_native_instruction_has_prior_local_copy_and_no_starter(self):
        initial,snapshots,actions=fixture()
        result=classify_run(initial,snapshots,actions)
        self.assertEqual(result["child_success_non_GraphCreated_events"],1)
        self.assertEqual(result["witnesses"][0]["runtime"]["runtime_opcode"],"Const")
        self.assertEqual(result["witnesses"][0]["parent_prior_source_copies"][0]["mode"],"local")

    def test_forged_external_child_instruction_is_rejected(self):
        with self.assertRaises(ValueError):
            classify_run(*fixture(forged=True))


if __name__=="__main__":
    unittest.main()
