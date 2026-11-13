"""Focused preregistered reciprocal-sensitivity safeguards."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import analyze_study02 as study


def state(v):
    return {"processes": {str(i): {"registers": [v if i == 1 else 0]}
                          for i in (1, 2, 3)}}


class Study02Tests(unittest.TestCase):
    def test_wilson_zero_and_all_are_bounded(self):
        lo, hi = study.wilson_95(0, 128)
        self.assertEqual(lo, 0.0)
        self.assertGreater(hi, 0)
        lo, hi = study.wilson_95(128, 128)
        self.assertEqual(hi, 1.0)
        self.assertLess(lo, 1)

    def test_reciprocal_influence_requires_both_directions(self):
        control = state(0)
        treatments = {1: state(0), 2: state(0), 3: state(0)}
        treatments[1]["processes"]["2"] = {"registers": [9]}
        treatments[2]["processes"]["1"] = {"registers": [5]}
        x = study.reciprocal_pairs(control, treatments, {1, 2, 3})
        self.assertEqual(x["reciprocal"], ["1<->2"])
        y = study.reciprocal_pairs(control, treatments, {1})
        self.assertEqual(y["reciprocal"], [])

    def test_sham_semantics_ignore_events_but_not_process_state(self):
        common = {"processes": {"1": {"credits": 3}}, "graph_objects": {},
                  "cells": {}, "compute_pool": 0}
        altered = dict(common, events=[{"origin": "External"}], tick=3)
        self.assertEqual(study.semantic_world(common), study.semantic_world(altered))
        changed = dict(common, cells={"0": 5})
        self.assertNotEqual(study.semantic_world(common), study.semantic_world(changed))


if __name__ == "__main__":
    unittest.main()
