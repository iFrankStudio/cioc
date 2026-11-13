"""Study-09 provenance and preregistered resource checks."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze_study09 as study


class Study09Tests(unittest.TestCase):
    def test_frozen_longer_epochs_and_independent_seed_range(self):
        self.assertEqual(study.EPOCHS, 32)
        self.assertEqual(study.FIRST, 1_490_000)
        self.assertEqual(study.SEEDS, 128)

    def test_only_actual_cross_owner_success_is_evidence(self):
        events = [
            {"origin": {"Internal": 1}, "kind": {"GraphCopied": {
                "source_owner": 2, "target_owner": 1,
            }}},
            {"origin": {"Internal": 1}, "kind": {"GraphTransferred": {
                "former_owner": 1, "new_owner": 2,
            }}},
            {"origin": {"Internal": 1}, "kind": {"Trapped": "read not granted"}},
        ]
        self.assertEqual(study.successful_exchange(events), (1, 1))
        self.assertEqual(study.successful_exchange(events[-1:]), (0, 0))


if __name__ == "__main__":
    unittest.main()
