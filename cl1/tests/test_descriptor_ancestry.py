"""Study-16 read-only lineage attribution: never invent missing writers."""
import unittest
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_descriptor_ancestry import audit_events


class DescriptorLineageAuditTest(unittest.TestCase):
    def test_mixed_external_internal_and_unknown_words(self):
        events = [
            {"origin": "External", "process": 0, "kind": {"Intervened": "SetCell { key: 10, value: 19 }"}},
            {"origin": {"Internal": 1}, "process": 1, "kind": {"CellWritten": {"key": 11, "value": 4}}},
            {"origin": {"Internal": 1}, "process": 1, "kind": {"GraphEncoded": {
                "graph": 2, "owner": 1, "new_node": 0,
                "descriptor_base": 10, "descriptor": [19, 4, 0, 0, 0],
            }}},
        ]
        records, counts = audit_events(events)
        self.assertEqual(len(records), 1)
        self.assertEqual(dict(counts), {
            "external-set": 1, "internal-executed": 1, "unknown": 3,
        })
        self.assertEqual(records[0]["ancestry"][1]["writer"], 1)


if __name__ == "__main__":
    unittest.main()
