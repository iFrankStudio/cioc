import unittest
from cioc import Organism

class CIOCV07Tests(unittest.TestCase):
    def test_generator_recovery(self):
        x = Organism(); x.damage("G")
        self.assertTrue(x.run())
        self.assertEqual(x.revision, 1)
        self.assertEqual(x.roles["G"], ("GENERATE", "PROPOSE"))

    def test_critical_roles_fail(self):
        for role in ("O", "V", "M"):
            with self.subTest(role=role):
                x = Organism(); x.damage(role)
                self.assertFalse(x.run())

    def test_joint_loss(self):
        x = Organism(); x.damage("G"); x.damage("M")
        self.assertFalse(x.run())

    def test_missing_fragment(self):
        x = Organism(); x.damage("G")
        x.fragments["V"].pop("G")
        self.assertFalse(x.run())

    def test_budget_constraint(self):
        x = Organism(budget=1); x.damage("G")
        self.assertFalse(x.run())

    def test_external_repair_not_internal(self):
        x = Organism(); x.damage("G"); x.external_restore("G")
        self.assertTrue(x.run())
        self.assertEqual(x.revision, 0)
        self.assertEqual(x.external_restores, 1)

    def test_internal_recovery_not_external(self):
        x = Organism(); x.damage("G"); x.run()
        self.assertEqual(x.external_restores, 0)

if __name__ == "__main__":
    unittest.main()
