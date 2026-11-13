"""Direct safeguards for precommitted graph-target selection and indirect effects."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze_study06 as study


class Study06Tests(unittest.TestCase):
    def test_minimum_nonempty_graph_selection_only(self):
        world = {"graph_objects": {
            "8": {"owner": 2, "nodes": [{"Halt": {}}]},
            "2": {"owner": 1, "nodes": []},
            "4": {"owner": 3, "nodes": [{"Halt": {}}]},
        }}
        self.assertEqual(study.chosen_graph(world), (4, 3))
        world["graph_objects"]["4"]["nodes"] = []
        self.assertEqual(study.chosen_graph(world), (8, 2))
        world["graph_objects"]["8"]["nodes"] = []
        self.assertIsNone(study.chosen_graph(world))

    def test_deletion_alone_is_not_indirect_effect(self):
        baseline = {
            "processes": {"1": {"pc": 3}}, "graph_objects": {"1": {"owner": 1}},
            "cells": {}, "compute_pool": 4,
        }
        altered = {
            "processes": {"1": {"pc": 3}}, "graph_objects": {},
            "cells": {}, "compute_pool": 4,
        }
        self.assertNotEqual(study.semantic_state(baseline), study.semantic_state(altered))
        self.assertEqual(study.process_state(baseline), study.process_state(altered))
        altered["processes"]["1"] = {"pc": 4}
        self.assertNotEqual(study.process_state(baseline), study.process_state(altered))

    def test_new_object_is_not_repair_evidence(self):
        initial = {"graph_objects": {"2": {"owner": 1, "nodes": ["Halt"]}}}
        final = {"graph_objects": {"3": {"owner": 1, "nodes": ["Halt"]}}}
        self.assertTrue(study.postdrop_nonempty_owner_graph(initial, final, 1))
        self.assertFalse(study.postdrop_nonempty_owner_graph(initial, final, 2))


if __name__ == "__main__":
    unittest.main()
