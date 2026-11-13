import json
import sqlite3
import tempfile
import unittest
from pathlib import Path
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze import analyze, summarize_world, _digest

class AnalysisTests(unittest.TestCase):
    def test_metrics_are_descriptive(self):
        world = {"processes": {"1": {"code": {"nodes": [{"Const": {"dst": 0}}, "Halt"]},
                   "credits": 3, "stopped": False}},
                 "cells": {"7": 42}, "graph_objects": {}, "events": [
                     {"origin": "External", "kind": "Loaded"},
                     {"origin": {"Internal": 1}, "kind": "Executed"}]}
        metrics = summarize_world(world)
        self.assertEqual(metrics["attempt_events"], 1)
        self.assertEqual(metrics["external_events"], 1)
        self.assertEqual(metrics["distinct_opcode_types"], 2)
        self.assertEqual(metrics["cells"], 1)

    def test_sqlite_read_only_both_schema_paths_and_matched_contrast(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "lab.sqlite"
            db = sqlite3.connect(path)
            for table in ("runs", "scheduled_runs"):
                db.execute(f"CREATE TABLE {table} (id INTEGER, source_revision TEXT, condition TEXT,"
                           "initial_sha256 TEXT, initial_json TEXT, final_sha256 TEXT,"
                           "final_json TEXT, attempt_limit INTEGER, attempts INTEGER)")
            initial = '{"cells":{},"events":[],"processes":{},"graph_objects":{}}'
            final = '{"cells":{"3":5},"events":[],"processes":{},"graph_objects":{}}'
            empty = initial
            for ix, condition, value in [(1, "control", final), (2, "zero-budget", empty)]:
                db.execute("INSERT INTO scheduled_runs VALUES (?,?,?,?,?,?,?,?,?)",
                           (ix, "fixture", condition, _digest(initial), initial,
                            _digest(value), value, 5, 3 if ix == 1 else 0))
            db.commit()
            db.close()
            outcome = analyze(path)
            self.assertEqual(outcome["counts"]["runs"], 2)
            self.assertEqual(outcome["counts"]["matched_initials"], 1)
            self.assertEqual(outcome["counts"]["invalid_hash_records"], 0)
            self.assertEqual(len(outcome["descriptive_contrasts"]), 1)
            self.assertEqual(outcome["records"][0]["cells"], 1)

    def test_modified_hash_is_reported_not_trusted(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "bad.sqlite"
            db = sqlite3.connect(path)
            db.execute("CREATE TABLE runs (id INTEGER, source_revision TEXT, condition TEXT,"
                       "initial_sha256 TEXT, initial_json TEXT, final_sha256 TEXT,"
                       "final_json TEXT, attempt_limit INTEGER, attempts INTEGER)")
            db.execute("INSERT INTO runs VALUES (1,'rev','bad','incorrect','{}','incorrect','{}',1,0)")
            db.commit()
            db.close()
            self.assertEqual(analyze(path)["counts"]["invalid_hash_records"], 1)

if __name__ == "__main__":
    unittest.main()
