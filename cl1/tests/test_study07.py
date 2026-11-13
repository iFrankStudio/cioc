"""Study-07 causal controls and admission of observer granularity limits."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze_study07 as study


class Study07Tests(unittest.TestCase):
    def test_bounded_internally_generated_target_selection(self):
        world = {"graph_objects": {
            "2": {"owner": 2, "nodes": []},
            "3": {"owner": 1, "nodes": [{"Halt": {}}]},
            "8": {"owner": 3, "nodes": [{"Const": {}}]},
        }}
        self.assertEqual(study.chosen_graph(world), (3, 1))
        world["graph_objects"]["3"]["nodes"] = []
        self.assertEqual(study.chosen_graph(world), (8, 3))

    def test_raw_dropped_graph_does_not_count_as_indirect_effect(self):
        a = {"processes": {"1": {"pc": 2}}, "graph_objects": {"1": {}},
             "cells": {}, "compute_pool": 10}
        b = dict(a, graph_objects={})
        self.assertEqual(study.process_state(a), study.process_state(b))
        self.assertNotEqual(study.semantic_state(a), study.semantic_state(b))

    def test_new_graph_after_ablation_is_only_structural(self):
        before = {"graph_objects": {"1": {"owner": 1, "nodes": ["Halt"]}}}
        after = {"graph_objects": {"2": {"owner": 1, "nodes": ["Halt"]}}}
        self.assertTrue(study.postdrop_nonempty_owner_graph(before, after, 1))
        self.assertFalse(study.postdrop_nonempty_owner_graph(before, after, 2))


if __name__ == "__main__":
    unittest.main()
