"""Study 18: synthetic event-level lineage controls, not autonomous-life evidence."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study18 import audit_events


def internal(process, name, payload, pc=None):
    return {
        "origin": {"Internal": process},
        "process": process,
        "pc": pc,
        "kind": {name: payload},
    }


def trace(writer="internal", read_all=True, trap=None, child=2,
          wrong_order=False, duplicate=False, graph_created=True):
    events = []
    if graph_created:
        events.append(internal(1, "GraphCreated", {"graph": 1, "owner": 1}))
    # Two executable instructions are necessary for a genuine read-then-write
    # fixture. Both must come from fully recorded descriptors.
    descriptors = ((100, [3, 0, 1, 1, 0]), (110, [4, 1, 0, 0, 0]))
    for node, (base, words) in enumerate(descriptors):
        for offset, word in enumerate(words):
            if writer == "internal":
                events.append(internal(1, "CellWritten", {
                    "key": base + offset, "value": word,
                }))
            elif writer == "external":
                events.append({
                    "origin": "External", "process": 0, "pc": None,
                    "kind": {"Intervened":
                        f"SetCell {{ key: {base + offset}, value: {word} }}"},
                })
        if read_all:
            for offset, word in enumerate(words):
                events.append(internal(1, "CellRead", {
                    "key": base + offset, "value": word,
                }))
        if trap == "encode" and node == 0:
            events.append(internal(1, "Trapped", "incomplete descriptor"))
        else:
            events.append(internal(1, "GraphEncoded", {
                "graph": 1, "owner": 1, "new_node": node,
                "descriptor_base": base, "descriptor": words,
            }))
    if duplicate:
        events.append(internal(1, "GraphCreated", {"graph": 1, "owner": 1}))
    if wrong_order:
        events.append(internal(child, "CellWritten", {"key": 201, "value": 7}, pc=1))
    if trap == "launch":
        events.append(internal(1, "Trapped", "insufficient credits"))
    else:
        events.append(internal(1, "GraphLaunched", {
            "source_graph": 1, "parent_process": 1, "child_process": 2,
            "transferred_credits": 3, "moved_nodes": 2,
        }))
        events.append(internal(2, "CellRead", {"key": 200, "value": 7}, pc=0))
        if not wrong_order:
            events.append(internal(child, "CellWritten", {
                "key": 201, "value": 7,
            }, pc=1))
    return events


class Study18MechanismTests(unittest.TestCase):
    def test_complete_execution_trace_is_not_causal_life_proof(self):
        result = audit_events(trace())
        self.assertEqual(result["counts"].get("traceable_execution_chain"), 1)
        self.assertEqual(len(result["witnesses"]), 1)
        self.assertEqual(result["witnesses"][0]["functional_causality"], "NOT EVALUABLE")

    def test_external_descriptor_never_internal(self):
        result = audit_events(trace(writer="external"))
        self.assertEqual(result["witnesses"], [])
        self.assertEqual(result["counts"].get("external_descriptor_source"), 1)

    def test_unknown_descriptor_never_internal(self):
        result = audit_events(trace(writer="unknown"))
        self.assertEqual(result["witnesses"], [])
        self.assertEqual(result["counts"].get("unknown_descriptor_source"), 1)

    def test_trapped_encode_or_launch(self):
        for operation in ("encode", "launch"):
            with self.subTest(operation=operation):
                result = audit_events(trace(trap=operation))
                self.assertEqual(result["witnesses"], [])
                self.assertEqual(len(result["trapped_event_ids"]), 1)

    def test_other_process_cannot_supply_child_output(self):
        self.assertEqual(audit_events(trace(child=3))["witnesses"], [])

    def test_output_before_launch_never_counts(self):
        self.assertEqual(audit_events(trace(wrong_order=True))["witnesses"], [])

    def test_duplicate_graph_handle_invalidates_original_ancestry(self):
        result = audit_events(trace(duplicate=True))
        self.assertEqual(result["witnesses"], [])
        self.assertGreater(result["counts"].get("graph_identity_reused", 0), 0)

    def test_missing_graph_creation_or_explicit_descriptor_reads(self):
        for options in ({"graph_created": False}, {"read_all": False}):
            with self.subTest(options=options):
                self.assertEqual(audit_events(trace(**options))["witnesses"], [])

    def test_changed_graph_code_invalidates_chain(self):
        events = trace()
        at = next(i for i, e in enumerate(events) if "GraphLaunched" in e["kind"])
        events.insert(at, internal(1, "GraphNodeEdited", {
            "graph": 1, "owner": 1, "node": 0, "edit": "next",
        }))
        self.assertEqual(audit_events(events)["witnesses"], [])

    def test_child_output_pc_must_match_known_write_opcode(self):
        events = trace()
        events[-1]["pc"] = 0
        self.assertEqual(audit_events(events)["witnesses"], [])


if __name__ == "__main__":
    unittest.main()
