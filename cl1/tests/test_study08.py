"""Study-08 event provenance must distinguish successes, denials and owner boundaries."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze_study08 as study


class Study08Tests(unittest.TestCase):
    def test_successful_cross_owner_copy_and_transfer(self):
        events = [
            {"origin": {"Internal": 2}, "kind": {"GraphCopied": {
                "source_graph": 1, "source_owner": 1, "source_node": 0,
                "target_graph": 2, "target_owner": 2, "new_node": 0,
            }}},
            {"origin": {"Internal": 2}, "kind": {"GraphTransferred": {
                "graph": 2, "former_owner": 2, "new_owner": 1,
            }}},
        ]
        self.assertEqual(study.successful_exchange(events), (1, 1))

    def test_denied_attempt_is_not_success(self):
        events = [
            {"origin": {"Internal": 2}, "kind": {"Trapped": "source graph access denied"}},
            {"origin": {"Internal": 2}, "kind": "Executed"},
        ]
        self.assertEqual(study.successful_exchange(events), (0, 0))

    def test_invalid_or_external_provenance_rejected(self):
        event = {"origin": "External", "kind": {"GraphCopied": {
            "source_owner": 1, "target_owner": 2,
        }}}
        with self.assertRaises(ValueError):
            study.successful_exchange([event])
        event["origin"] = {"Internal": 3}
        with self.assertRaises(ValueError):
            study.successful_exchange([event])


if __name__ == "__main__":
    unittest.main()
