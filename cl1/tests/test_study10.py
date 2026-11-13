"""Prespecified Study-10 analyst safeguards for engineered cross-owner exchange."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze_study10 as study


class Study10Tests(unittest.TestCase):
    def test_all_64_worlds_count_and_source_attribution(self):
        self.assertEqual(study.SEEDS, 64)
        events = [
            {"origin": {"Internal": 1}, "kind": {"GraphCopied": {
                "source_graph": 2, "source_owner": 2, "source_node": 0,
                "target_graph": 1, "target_owner": 1, "new_node": 1,
            }}},
            {"origin": {"Internal": 1}, "kind": {"GraphTransferred": {
                "graph": 1, "former_owner": 1, "new_owner": 2,
            }}},
        ]
        self.assertEqual(study.successful_exchange(events), (1, 1))

    def test_failed_copy_is_never_successful_exchange(self):
        self.assertEqual(
            study.successful_exchange([{"origin": {"Internal": 1}, "kind": {
                "Trapped": "source graph access denied"}}]),
            (0, 0),
        )

    def test_indirect_effect_does_not_count_deletion_alone(self):
        before = {"processes": {"1": {"pc": 9}}, "graph_objects": {"1": {"owner": 1}}}
        after = {"processes": {"1": {"pc": 9}}, "graph_objects": {}}
        self.assertEqual(study.process_state(before), study.process_state(after))


if __name__ == "__main__":
    unittest.main()
