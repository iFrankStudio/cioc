"""Study57 focused native original-branch exposure provenance and uncertainty gates."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study57 import changed_branch_sites, classify_native_branch_exposure


def process(branch_edge):
    nodes = ["Halt"] * 82
    nodes[9] = {"Branch": {"test": 1, "nonzero": branch_edge, "zero": 10}}
    nodes[10] = {"PatchNext": {"node": 2, "target": 0, "next": 11}}
    return {"code": {"nodes": nodes}}


def initial(edge=7):
    return {
        "events": [],
        "processes": {
            "1": process(edge), "2": process(7), "3": process(7)
        },
    }


def native(tick, actor=1, pc=9, kind="Executed"):
    return {"tick": tick, "process": actor, "pc": pc,
            "origin": {"Internal": actor}, "kind": kind}


class OriginalNativeBranchExposure(unittest.TestCase):
    def test_actual_native_original_branch_is_confirmed_without_prior_mutator(self):
        sites = changed_branch_sites(initial(7), initial(4))
        self.assertEqual(sites, {1: {9}, 2: set(), 3: set()})
        got = classify_native_branch_exposure(
            initial(), [native(10), native(11, pc=9)], sites
        )
        self.assertEqual(len(got), 2)
        self.assertTrue(all(x["unambiguous_initial_branch"] for x in got))

    def test_trap_child_and_prior_native_mutator_do_not_forge_original_exposure(self):
        sites = {1: {9}, 2: set(), 3: set()}
        got = classify_native_branch_exposure(initial(), [
            native(10, kind={"Trapped": "invalid native edge"}),
            native(11, actor=4),
            native(12, pc=10),
            native(13),
        ], sites)
        self.assertEqual(len(got), 1)
        self.assertFalse(got[0]["unambiguous_initial_branch"])

    def test_native_executable_growth_is_not_invalid_and_marks_later_branch_uncertain(self):
        sites = {1: {9}, 2: set(), 3: set()}
        got = classify_native_branch_exposure(
            initial(), [native(10, pc=82), native(11)], sites
        )
        self.assertEqual(len(got), 1)
        self.assertFalse(got[0]["unambiguous_initial_branch"])


if __name__ == "__main__":
    unittest.main()
