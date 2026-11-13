"""Small independent scientific source/epoch checks, not exhaustive fixtures."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze_study05 as study


class Study05Tests(unittest.TestCase):
    def test_environment_arrives_even_when_inert(self):
        self.assertEqual(study.expected_actions("pulse-pool"), [0, 4, 8, 12, 16, 20])
        self.assertEqual(study.expected_actions("zero-credit-pulse"), [0, 0, 0, 0, 4, 8, 12, 16, 20])

    def test_external_resource_source_is_not_internal(self):
        state = {"events": [
            {"origin": "External", "kind": {"Intervened": "SetComputePool { credits: 32 }"}},
            {"origin": {"Internal": 1}, "kind": "Executed"},
        ]}
        self.assertEqual(study.supply_events(state), 1)
        state["events"][0]["origin"] = {"Internal": 1}
        with self.assertRaises(ValueError):
            study.supply_events(state)

    def test_semantic_state_ignores_directly_injected_pool(self):
        state = {"processes": {}, "graph_objects": {}, "cells": {}, "compute_pool": 1, "events": []}
        another = dict(state, compute_pool=32)
        self.assertEqual(study.semantic_state(state), study.semantic_state(another))
        another = dict(state, cells={"0": 20})
        self.assertNotEqual(study.semantic_state(state), study.semantic_state(another))


if __name__ == "__main__":
    unittest.main()
