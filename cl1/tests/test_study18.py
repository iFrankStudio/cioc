"""Study 18 preregistered mechanism controls: time-ordered traces are not life."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study18 import audit_events


def internal(process, name, payload):
    return {"origin": {"Internal": process}, "process": process, "kind": {name: payload}}


def trace(writer="internal", read_all=True, trap=None, child=2, wrong_order=False,
          duplicate=False, graph_created=True):
    events = []
    if graph_created:
        events.append(internal(1, "GraphCreated", {"graph": 1, "owner": 1}))
    for offset, word in enumerate((19, 0, 0, 0, 0)):
        if writer == "internal":
            events.append(internal(1, "CellWritten", {"key": 100 + offset, "value": word}))
        elif writer == "external":
            events.append({"origin": "External", "process": 0, "kind": {
                "Intervened": f"SetCell {{ key: {100 + offset}, value: {word} }}"
            }})
    if read_all:
        for offset, word in enumerate((19, 0, 0, 0, 0)):
            events.append(internal(1, "CellRead", {"key": 100 + offset, "value": word}))
    if trap == "encode":
        events.append(internal(1, "Trapped", "incomplete instruction descriptor"))
    else:
        events.append(internal(1, "GraphEncoded", {
            "graph": 1, "owner": 1, "new_node": 0,
            "descriptor_base": 100, "descriptor": [19, 0, 0, 0, 0],
        }))
    if duplicate:
        events.append(internal(1, "GraphCreated", {"graph": 1, "owner": 1}))
    if wrong_order:
        events.append(internal(child, "CellWritten", {"key": 201, "value": 7}))
    if trap == "launch":
        events.append(internal(1, "Trapped", "insufficient credit"))
    else:
        events.append(internal(1, "GraphLaunched", {
            "source_graph": 1, "parent_process": 1, "child_process": 2,
            "transferred_credits": 3, "moved_nodes": 1,
        }))
        events.append(internal(2, "CellRead", {"key": 200, "value": 7}))
        if not wrong_order:
            events.append(internal(child, "CellWritten", {"key": 201, "value": 7}))
    return events


class Study18MechanismTests(unittest.TestCase):
    def test_complete_internal_execution_witness_is_not_causal_life_proof(self):
        result = audit_events(trace())
        self.assertEqual(result["counts"].get("traceable_execution_chain"), 1)
        self.assertEqual(len(result["witnesses"]), 1)
        self.assertEqual(result["witnesses"][0]["functional_causality"], "NOT EVALUABLE")

    def test_external_descriptor_does_not_count_as_internal(self):
        result = audit_events(trace(writer="external"))
        self.assertEqual(result["witnesses"], [])
        self.assertEqual(result["counts"].get("external_descriptor_source"), 1)

    def test_unknown_descriptor_does_not_count_as_internal(self):
        result = audit_events(trace(writer="unknown"))
        self.assertEqual(result["witnesses"], [])
        self.assertEqual(result["counts"].get("unknown_descriptor_source"), 1)

    def test_trapped_encode_or_launch_never_complete_chain(self):
        for operation in ("encode", "launch"):
            with self.subTest(operation=operation):
                result = audit_events(trace(trap=operation))
                self.assertEqual(result["witnesses"], [])
                self.assertEqual(len(result["trapped_event_ids"]), 1)

    def test_output_by_other_process_is_not_child_output(self):
        result = audit_events(trace(child=3))
        self.assertEqual(result["witnesses"], [])

    def test_output_before_launch_never_counts(self):
        result = audit_events(trace(wrong_order=True))
        self.assertEqual(result["witnesses"], [])

    def test_duplicate_graph_handle_never_claims_original_ancestry(self):
        result = audit_events(trace(duplicate=True))
        self.assertEqual(result["witnesses"], [])
        self.assertGreater(result["counts"].get("graph_identity_reused", 0), 0)

    def test_missing_graph_creation_or_reads_is_not_evaluable(self):
        for options in ({"graph_created": False}, {"read_all": False}):
            with self.subTest(options=options):
                result = audit_events(trace(**options))
                self.assertEqual(result["witnesses"], [])

    def test_no_inferred_causality_from_copied_or_edited_graph(self):
        events = trace()
        insertion = next(i for i, e in enumerate(events) if isinstance(e["kind"], dict) and "GraphLaunched" in e["kind"])
        events.insert(insertion, internal(1, "GraphNodeEdited", {"graph": 1, "owner": 1, "node": 0, "edit": "next"}))
        self.assertEqual(audit_events(events)["witnesses"], [])


if __name__ == "__main__":
    unittest.main()
