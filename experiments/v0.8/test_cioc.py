import unittest
from cioc import World, ROLE_SPEC

class RecoveryTests(unittest.TestCase):
    def test_generator_recovery(self):
        w = World(); w.damage("G")
        self.assertTrue(w.run())
        self.assertEqual(w.roles["G"], ROLE_SPEC["G"])
        self.assertEqual((w.internal_repairs, w.external_repairs), (1, 0))

    def test_critical_roles(self):
        for role in ("O", "V", "M"):
            with self.subTest(role=role):
                w = World(); w.damage(role)
                self.assertFalse(w.run())

    def test_joint_loss(self):
        w = World(); w.damage("G", "M")
        self.assertFalse(w.run())

    def test_budget(self):
        w = World(budget=4); w.damage("G")
        self.assertFalse(w.run())

    def test_missing_fragment(self):
        w = World(); w.damage("G")
        w.fragments["M"].pop("G")
        self.assertFalse(w.run())

    def test_external_control(self):
        w = World(); w.damage("G"); w.external_restore("G")
        self.assertTrue(w.run())
        self.assertEqual((w.internal_repairs, w.external_repairs), (0, 1))

    def test_idle(self):
        w = World()
        self.assertTrue(w.run())
        self.assertEqual(w.internal_repairs, 0)

if __name__ == "__main__":
    unittest.main()
