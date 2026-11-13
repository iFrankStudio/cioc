"""Study-04 resource-source and state-cost accounting safeguards."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze_study04 as study


class Study04Tests(unittest.TestCase):
    def test_supply_schedule_is_frozen(self):
        self.assertEqual(study.expected_offsets("closed"), [])
        self.assertEqual(study.expected_offsets("replenished"), [0, 128, 256, 384])
        self.assertEqual(study.expected_offsets("zero-credit-pool"), [0, 0, 0, 0])

    def test_externally_attributed_pool_only(self):
        state = {"events": [
            {"origin": "External", "kind": {"Intervened": "SetComputePool { credits: 256 }"}},
            {"origin": {"Internal": 1}, "kind": "Executed"},
        ]}
        self.assertEqual(study.pool_supply_events(state), 1)
        state["events"][0]["origin"] = {"Internal": 1}
        with self.assertRaises(ValueError):
            study.pool_supply_events(state)

    def test_node_count_includes_created_graph_objects(self):
        world = {
            "processes": {"1": {"code": {"nodes": ["Halt"]}}},
            "graph_objects": {"4": {"nodes": ["Halt", "Halt"]}},
        }
        self.assertEqual(study.code_node_count(world), 3)


if __name__ == "__main__":
    unittest.main()
