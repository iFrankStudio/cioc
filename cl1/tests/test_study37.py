"""Study37 minimal independent environmental resource-access policy calibration."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study37 import base_actions, candidate, expected_actions


def world(pool=8, credits=(0, 0, 0), stopped=(False, False, False),
          current=("DrawCompute", "Halt", "Halt"), quantities=(2, 1, 1)):
    processes = {}
    for n in (1, 2, 3):
        op = ({"DrawCompute": {"amount": 1, "next": 1}}
              if current[n - 1] == "DrawCompute" else "Halt")
        processes[str(n)] = {
            "credits": credits[n - 1],
            "stopped": stopped[n - 1],
            "pc": 0,
            "registers": [0, quantities[n - 1], 0, 0],
            "code": {"nodes": [op, "Halt"]},
        }
    return {"compute_pool": pool, "processes": processes}


class Study37EnvironmentAccess(unittest.TestCase):
    def test_positive_chooses_first_actor_with_native_draw_and_conserves_pool(self):
        state = world()
        self.assertEqual(candidate(state), (1, 8))
        actions = expected_actions(state, "pool-access", 6)
        self.assertEqual(actions, [
            {"epoch": 6, "action": {"SetComputePool": {"credits": 7}}},
            {"epoch": 6, "action": {"SetCredits": {
                "process": 1, "credits": 1,
            }}},
        ])

    def test_no_action_when_actor_runnable_stopped_pool_inadequate_or_no_draw(self):
        self.assertIsNone(candidate(world(credits=(1, 0, 0))))
        self.assertIsNone(candidate(world(stopped=(True, True, True))))
        self.assertIsNone(candidate(world(pool=2)))
        self.assertIsNone(candidate(world(current=("Halt", "Halt", "Halt"))))
        self.assertEqual(expected_actions(world(), "control", 6), [])

    def test_zero_credit_arm_applies_pre_action_before_selecting(self):
        state = world(credits=(47, 47, 47))
        actions = expected_actions(state, "pool-access-zero-credit", 0)
        self.assertEqual(actions[:3], base_actions("pool-access-zero-credit", 0))
        self.assertEqual(len(actions), 5)
        self.assertEqual(actions[4]["action"]["SetCredits"]["process"], 1)

    def test_empty_pool_blocks_ignition_at_epoch_zero_not_forever(self):
        state = world(pool=8)
        self.assertEqual(
            expected_actions(state, "pool-access-empty-pool", 0),
            base_actions("pool-access-empty-pool", 0),
        )
        self.assertEqual(
            len(expected_actions(world(pool=5), "pool-access-empty-pool", 7)),
            2,
        )


if __name__ == "__main__":
    unittest.main()
