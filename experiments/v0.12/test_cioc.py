import unittest
from cioc import TwinWorld, challenges, scenarios


class CrossDiagnosisTests(unittest.TestCase):
    def test_single_diagnostic_loss_is_repaired(self):
        for name in ('DR', 'DP'):
            with self.subTest(name=name):
                w = TwinWorld(); w.damage(name)
                self.assertTrue(w.run())
                self.assertEqual(w.repairs_diagnostics, 1)

    def test_silent_corruption_is_detected(self):
        for name in ('DR', 'DP'):
            with self.subTest(name=name):
                w = TwinWorld(); w.corrupt(name, ('NOOP', 'NOOP'))
                self.assertTrue(w.run())
                self.assertEqual(w.cross_detections, 1)
                self.assertEqual(w.repairs_diagnostics, 1)

    def test_replacement_is_behavioral_not_source_equal(self):
        w = TwinWorld(); w.damage('DR')
        self.assertTrue(w.run())
        self.assertNotEqual(w.diagnostics['DR'], ('SCAN_ROLE', 'REPORT'))
        self.assertTrue(challenges(w.diagnostics['DR'], 'role'))

    def test_repaired_auditor_performs_real_audit(self):
        w = TwinWorld(); w.damage('DR'); self.assertTrue(w.run())
        w.corrupt('G', ('NOOP',) * 3)
        self.assertTrue(w.run())
        self.assertGreaterEqual(w.detections, 1)

    def test_single_functional_corruption(self):
        for name in ('G', 'V', 'M', 'PV', 'PI'):
            with self.subTest(name=name):
                w = TwinWorld(); w.corrupt(name, ('NOOP',) * 3)
                self.assertTrue(w.run())

    def test_observer_remains_single_point(self):
        w = TwinWorld(); w.corrupt('O', ('NOOP',) * 3)
        self.assertFalse(w.run())

    def test_simultaneous_diagnostic_loss_fails(self):
        w = TwinWorld(); w.damage('DR', 'DP')
        self.assertFalse(w.run())
        self.assertEqual(w.repairs_diagnostics, 0)

    def test_untrusted_peer_cannot_authorize_recovery(self):
        w = TwinWorld(); w.damage('DR'); w.corrupt('DP', ('NOOP', 'NOOP'))
        self.assertFalse(w.run())

    def test_fragment_and_budget_controls(self):
        w = TwinWorld(); w.damage('DR'); w.diagnostic_fragments.pop('DR')
        self.assertFalse(w.run())
        x = TwinWorld(budget=4); x.damage('DR')
        self.assertFalse(x.run())

    def test_inhibit_peer_repair(self):
        w = TwinWorld(allow_peer_repair=False); w.damage('DP')
        self.assertFalse(w.run())

    def test_equivalent_diagnostic_does_not_raise_alarm(self):
        w = TwinWorld(); w.corrupt('DR', ('SCAN_ROLE', 'REPORT_ALT'))
        self.assertTrue(w.run())
        self.assertEqual(w.cross_detections, 0)
        self.assertEqual(w.repairs_diagnostics, 0)

    def test_sequential_recovery(self):
        w = TwinWorld(); w.damage('DR'); self.assertTrue(w.run())
        w.damage('DP'); self.assertTrue(w.run())
        self.assertEqual(w.repairs_diagnostics, 2)
        self.assertEqual(w.external_repairs, 0)

    def test_scenario_matrix(self):
        s = scenarios()
        self.assertTrue(s['loss_DR']['recovered'])
        self.assertFalse(s['loss_both']['recovered'])
        self.assertTrue(s['sequential_DR_DP']['second'])


if __name__ == '__main__':
    unittest.main()
