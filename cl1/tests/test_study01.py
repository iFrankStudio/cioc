"""Focused safeguards for CL-1 independent read-only Study 01 analysis."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze


class StudyAnalysisTests(unittest.TestCase):
    def test_exogenous_cell_difference_is_not_indirect_effect(self):
        common = {
            "processes": {"1": {"pc": 3, "credits": 2}},
            "graph_objects": {},
            "compute_pool": 0,
        }
        lesion = dict(common, cells={"0": 9999, "4": 8})
        sham = dict(common, cells={"1000000": 9999, "4": 8})
        self.assertEqual(analyze.semantic_signature(lesion), analyze.semantic_signature(sham))
        sham["processes"] = {"1": {"pc": 4, "credits": 2}}
        self.assertNotEqual(analyze.semantic_signature(lesion), analyze.semantic_signature(sham))

    def test_strict_storage_prices_processes_and_grants(self):
        state = {
            "strict_accounting": True,
            "cells": {"0": 1},
            "processes": {"1": {"code": {"nodes": ["Halt", "Halt"]}}},
            "graph_objects": {"8": {"nodes": ["Halt"], "readers": [2, 3]}},
        }
        self.assertEqual(analyze.occupied_units(state), 8)

    def test_origin_and_condition_rules(self):
        self.assertEqual(analyze.internal_process({"origin": {"Internal": 2}}), 2)
        self.assertIsNone(analyze.internal_process({"origin": "External"}))
        self.assertEqual(analyze.parse_label("seed=610000/control"), (610000, "control"))
        with self.assertRaises(ValueError):
            analyze.parse_label("seed=610064/control")
        with self.assertRaises(ValueError):
            analyze.parse_label("seed=610001/life-found")


if __name__ == "__main__":
    unittest.main()
