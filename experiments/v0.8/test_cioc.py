import unittest
from cioc import World, CANONICAL, contract, probe


class RecoveryTests(unittest.TestCase):
    def test_generator_recovers_behaviorally_with_different_source(self):
        world = World()
        world.damage("G")
        self.assertTrue(world.run())
        self.assertEqual(world.internal_repairs, 1)
        self.assertEqual(world.external_repairs, 0)
        self.assertTrue(contract("G", world.roles["G"]))
        self.assertNotEqual(world.roles["G"], CANONICAL["G"])

    def test_recovered_generator_operates_through_alternative_opcode(self):
        world = World()
        world.damage("G")
        self.assertTrue(world.run())
        world.damage("M")
        self.assertFalse(world.run())
        self.assertTrue(any(e.get("event") == "proposal_emitted"
                            and e.get("opcode") == "PROPOSE_ALT"
                            for e in world.events))

    def test_behavioral_contract_rejects_wrong_order_and_noop(self):
        self.assertFalse(contract("G", ("PROPOSE", "ENUMERATE", "IDLE")))
        self.assertFalse(contract("G", ("ENUMERATE", "IDLE", "IDLE")))
        self.assertTrue(contract("G", ("ENUMERATE", "PROPOSE_ALT", "IDLE")))
        self.assertEqual(probe("G", ("ENUMERATE", "PROPOSE_ALT", "IDLE"), False), ())

    def test_critical_roles_remain_single_points(self):
        for role in ("O", "V", "M"):
            with self.subTest(role=role):
                world = World()
                world.damage(role)
                self.assertFalse(world.run())
                self.assertEqual(world.internal_repairs, 0)

    def test_joint_failures(self):
        for roles in (("G", "V"), ("G", "M")):
            with self.subTest(roles=roles):
                world = World()
                world.damage(*roles)
                self.assertFalse(world.run())

    def test_missing_fragment_and_budget(self):
        world = World(); world.damage("G")
        world.fragments["V"].pop("G")
        self.assertFalse(world.run())
        low = World(budget=8); low.damage("G")
        self.assertFalse(low.run())

    def test_external_restore_not_misclassified(self):
        world = World(); world.damage("G"); world.external_restore("G")
        self.assertTrue(world.run())
        self.assertEqual((world.internal_repairs, world.external_repairs), (0, 1))

    def test_no_damage_needs_no_repair(self):
        world = World()
        self.assertTrue(world.run())
        self.assertEqual(world.internal_repairs, 0)


if __name__ == "__main__":
    unittest.main()
