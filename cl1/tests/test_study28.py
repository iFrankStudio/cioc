"""Targeted counterexamples for Study28 graph1 source necessity classification."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study28 import classify


def event(tick, actor, kind):
    return {"tick": tick, "origin": {"Internal": actor},
            "process": actor, "pc": tick, "kind": kind}


def source2(tag):
    return event(1, 1, {"GraphTagRead": {
        "graph": 2, "owner": 2, "node": 0, "tag": tag,
    }})


def source1(tick=2):
    return event(tick, 1, {"GraphTagRead": {
        "graph": 1, "owner": 1, "node": 0, "tag": 11,
    }})


def output(address, tick=3):
    return event(tick, 1, {"CellWritten": {"key": address, "value": 1}})


class SecondSourceControls(unittest.TestCase):
    def base(self):
        return {
            "control": [source2(11), output(11)],
            "mutate-g1": [source2(11), output(11)],
            "mutate-g2": [source2(19), output(19)],
            "mutate-both": [source2(19), output(19)],
        }

    def test_graph1_not_needed_when_outputs_survive_both_mutations(self):
        result = classify(self.base())
        self.assertTrue(result["original_exposure"])
        self.assertEqual(result["category"], "graph1_not_necessary_observed")
        self.assertFalse(result["second_component_source_read"])

    def test_loss_without_graph1_read_is_not_validated_two_component(self):
        arms = self.base()
        arms["mutate-g1"] = [source2(11)]
        result = classify(arms)
        self.assertEqual(result["category"], "graph1_interference_observed")
        self.assertFalse(result["two_component_read_mediated_dependency"])

    def test_read_mediated_secondary_interference_remains_only_exposure(self):
        arms = self.base()
        arms["control"] = [source2(11), source1(), output(11)]
        arms["mutate-g1"] = [source2(11), source1()]
        result = classify(arms)
        self.assertTrue(result["second_component_source_read"])
        self.assertTrue(result["two_component_read_mediated_dependency"])

    def test_wrong_source_owner_cannot_create_secondary_read(self):
        arms = self.base()
        arms["control"] = [source2(11), event(2, 1, {
            "GraphTagRead": {"graph": 1, "owner": 3, "node": 0, "tag": 11},
        }), output(11)]
        arms["mutate-g1"] = [source2(11)]
        self.assertFalse(classify(arms)["second_component_source_read"])


if __name__ == "__main__":
    unittest.main()
