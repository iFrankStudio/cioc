"""Focused Study-11 provenance checks: random continuation vs scripted prefix."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze_study11 as study


class Study11Tests(unittest.TestCase):
    def test_96_independent_worlds_and_cross_owner_success_only(self):
        self.assertEqual(study.SEEDS, 96)
        events = [
            {"origin": {"Internal": 2}, "pc": 12, "kind": {"GraphCopied": {
                "source_owner": 1, "target_owner": 2, "source_graph": 1,
                "target_graph": 2, "source_node": 0, "new_node": 1,
            }}},
            {"origin": {"Internal": 1}, "pc": 13,
             "kind": {"Trapped": "access denied"}},
        ]
        self.assertEqual(study.successful_exchange(events), (1, 0))

    def test_sham_data_deletion_alone_not_counted_as_process_effect(self):
        a = {"processes": {"2": {"pc": 10}}, "graph_objects": {"1": {}}}
        b = {"processes": {"2": {"pc": 10}}, "graph_objects": {}}
        self.assertEqual(study.process_state(a, 1), study.process_state(b, 1))


if __name__ == "__main__":
    unittest.main()
