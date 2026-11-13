import unittest
from cioc import World, CANONICAL, probe, expected


class V09Tests(unittest.TestCase):
    def test_verifier_recovers_without_full_backup(self):
        w = World(); w.damage("V")
        self.assertTrue(w.run())
        self.assertEqual((w.internal_repairs, w.external_repairs), (1, 0))
        self.assertNotEqual(w.roles["V"], CANONICAL["V"])
        self.assertEqual(probe("V", w.roles["V"], True), expected("V", True))
        self.assertEqual(probe("V", w.roles["V"], False), ())
        self.assertTrue(any(e["event"] == "certified" and e["path"] == "delegated" for e in w.events))

    def test_modifier_recovers_by_delegated_install(self):
        w = World(); w.damage("M")
        self.assertTrue(w.run())
        self.assertNotEqual(w.roles["M"], CANONICAL["M"])
        self.assertTrue(any(e["event"] == "internal_install" and e["actor"] == "G" for e in w.events))

    def test_generator_still_recovers(self):
        w = World(); w.damage("G")
        self.assertTrue(w.run())
        self.assertEqual(w.internal_repairs, 1)

    def test_observer_is_single_point(self):
        w = World(); w.damage("O")
        self.assertFalse(w.run())

    def test_joint_failures(self):
        for names in (("V", "M"), ("G", "V")):
            with self.subTest(names=names):
                w = World(); w.damage(*names)
                self.assertFalse(w.run())

    def test_low_budget(self):
        w = World(budget=4); w.damage("V")
        self.assertFalse(w.run())

    def test_missing_fragment(self):
        w = World(); w.damage("V"); w.fragments["M"].pop("V"); w.fragments["G"].pop("V")
        self.assertFalse(w.run())

    def test_delegated_validation_is_causally_needed(self):
        w = World(allow_delegated_validation=False); w.damage("V")
        self.assertFalse(w.run())

    def test_delegated_installation_is_causally_needed(self):
        w = World(allow_delegated_installation=False); w.damage("M")
        self.assertFalse(w.run())

    def test_sequential_repair_after_regeneration(self):
        for first, second in (("V", "M"), ("M", "V"), ("G", "V")):
            with self.subTest(first=first, second=second):
                w = World(); w.damage(first)
                self.assertTrue(w.run())
                w.damage(second)
                self.assertTrue(w.run())
                self.assertEqual(w.internal_repairs, 2)
                self.assertEqual(w.external_repairs, 0)

    def test_redundant_end_fragment_survives_one_loss(self):
        w = World(); w.damage("V")
        w.fragments["M"].pop("V")
        self.assertTrue(w.run())
        self.assertEqual(w.internal_repairs, 1)

    def test_external_not_internal(self):
        w = World(); w.damage("V"); w.external_restore("V")
        self.assertTrue(w.run())
        self.assertEqual((w.internal_repairs, w.external_repairs), (0, 1))

    def test_no_damage(self):
        w = World()
        self.assertTrue(w.run())
        self.assertEqual(w.internal_repairs, 0)


if __name__ == "__main__":
    unittest.main()
