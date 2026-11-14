"""Study40 minimal retrospective attribution: trap, native write, host starter."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study40 import summarize_run


PROGRAM = [
    {"Const": {"dst": 0, "value": 1, "next": 1}},
    {"Write": {"address": 0, "value": 1, "next": 1}},
]


def fixture(items):
    """Create 24 immutable cumulative-event snapshots from a few W3 events."""
    initial = {"events": [], "processes": {
        "1": {"code": {"nodes": PROGRAM}},
    }}
    attempts = [0] * 24
    actions = []
    event_stream = []
    snapshots = []
    for epoch in range(24):
        for mode in items.get(epoch, ()):
            if mode == "starter":
                actions.extend([
                    {"epoch": epoch, "action": {"SetCredits": {
                        "process": 1, "credits": 1,
                    }}},
                ])
                event_stream.append({
                    "process": 0, "pc": 0, "origin": "External",
                    "kind": {"Intervened": "SetCredits"},
                })
            else:
                kind = {
                    "trap": {"Trapped": "invalid native code"},
                    "write": {"CellWritten": {"key": 1, "value": 2}},
                    "success": "Executed",
                }[mode]
                event_stream.append({
                    "process": 1, "pc": 1 if mode == "write" else 0,
                    "origin": {"Internal": 1}, "kind": kind,
                })
                attempts[epoch] += 1
        snapshots.append({"events": list(event_stream)})
    return initial, snapshots, actions, attempts


class Study40NativeFunctionalGates(unittest.TestCase):
    def test_trapped_native_attempt_is_not_successful(self):
        result = summarize_run(*fixture({16: ["trap"]}))
        self.assertTrue(result["W3_active"])
        self.assertFalse(result["W3_successful_world"])
        self.assertEqual(result["W3_successful_epochs"], 0)

    def test_native_write_two_distinct_epochs_without_host_starter(self):
        result = summarize_run(*fixture({
            16: ["write"], 18: ["success"],
        }))
        self.assertEqual(result["W3_native_write_events"], 1)
        self.assertEqual(result["W3_successful_epochs"], 2)
        self.assertTrue(result["W3_success_without_starter_2_epochs"])

    def test_same_epoch_external_starter_is_not_native_success(self):
        result = summarize_run(*fixture({16: ["starter", "write"]}))
        self.assertTrue(result["W3_successful_world"])
        self.assertFalse(result["W3_success_without_starter_2_epochs"])
        self.assertEqual(result["W3_starter_plus_success_epochs"], 1)
        self.assertEqual(result["W3_external_starter_epochs"], [16])


if __name__ == "__main__":
    unittest.main()
