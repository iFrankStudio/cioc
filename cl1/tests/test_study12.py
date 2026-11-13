"""Structural provenance checks exclude failed operations and respect actor identities."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze_study12 as study


class Study12Tests(unittest.TestCase):
    def test_frozen_96_world_provenance_cohort(self):
        self.assertEqual(study.SEEDS, 96)
        self.assertEqual(study.FIRST, 1_810_000)

    def test_only_successful_cross_owner_events_are_counted(self):
        events = [
            {"origin": {"Internal": 2}, "kind": {"GraphCopied": {
                "source_owner": 1, "target_owner": 2
            }}},
            {"origin": {"Internal": 2}, "kind": {"Trapped": "access denied"}},
        ]
        self.assertEqual(study.successful_exchange(events), (1, 0))

    def test_node_identity_change_excludes_deleted_object_alone(self):
        a = {"processes": {"2": {"pc": 5}}, "graph_objects": {"1": {}}}
        b = {"processes": {"2": {"pc": 5}}, "graph_objects": {}}
        self.assertEqual(study.process_state(a), study.process_state(b))


if __name__ == "__main__":
    unittest.main()
