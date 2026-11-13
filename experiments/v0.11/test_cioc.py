import unittest
from cioc import DiagnosticWorld, scenarios


class LiveDiagnosticsTests(unittest.TestCase):
    def test_role_corruption_is_detected_and_repaired_where_possible(self):
        for name in ('G', 'V', 'M'):
            with self.subTest(name=name):
                w = DiagnosticWorld(); w.corrupt(name, ('NOOP',) * 3)
                self.assertTrue(w.run())
                self.assertEqual(w.internal_repairs, 1)
                self.assertEqual(w.detections, 1)
                self.assertTrue(any(e['event'] == 'quarantine' and e['target'] == name for e in w.events))

    def test_live_protocol_corruption(self):
        for name in ('PV', 'PI'):
            with self.subTest(name=name):
                w = DiagnosticWorld(); w.corrupt(name, ('NOOP',) * 3)
                self.assertTrue(w.run())
                self.assertEqual(w.protocol_repairs, 1)
                self.assertEqual(w.external_repairs, 0)

    def test_observer_remains_single_point(self):
        w = DiagnosticWorld(); w.corrupt('O', ('NOOP',) * 3)
        self.assertFalse(w.run())
        self.assertEqual(w.detections, 1)

    def test_equivalent_code_not_quarantined(self):
        samples = {'G': ('ENUMERATE', 'PROPOSE_ALT', 'DELEGATE'),
                   'V': ('TEST', 'CERTIFY_ALT', 'IDLE'),
                   'M': ('CHECK', 'COMMIT_ALT', 'IDLE'),
                   'PV': ('POS_O', 'NEG_M_ALT', 'CERTIFY_P'),
                   'PI': ('VOTE_V', 'PERMIT_G_ALT', 'INSTALL_P')}
        for name, code in samples.items():
            with self.subTest(name=name):
                w = DiagnosticWorld(); w.corrupt(name, code)
                self.assertTrue(w.healthy())
                self.assertTrue(w.run())
                self.assertEqual(w.internal_repairs, 0)
                self.assertEqual(w.detections, 0)

    def test_diagnostic_inhibition(self):
        w = DiagnosticWorld(diagnostic_enabled=False); w.corrupt('PV', ('NOOP',) * 3)
        self.assertFalse(w.run())
        self.assertEqual(w.internal_repairs, 0)

    def test_diagnostic_code_damage(self):
        w = DiagnosticWorld(); w.corrupt('D', ('NOOP', 'NOOP'))
        w.corrupt('V', ('NOOP',) * 3)
        self.assertFalse(w.run())
        self.assertEqual(w.detections, 0)

    def test_budget_bounds_detection(self):
        w = DiagnosticWorld(budget=3); w.corrupt('V', ('NOOP',) * 3)
        self.assertFalse(w.run())
        self.assertEqual(w.internal_repairs, 0)

    def test_joint_damage_still_fails(self):
        w = DiagnosticWorld(); w.corrupt('PV', ('NOOP',) * 3)
        w.corrupt('V', ('NOOP',) * 3)
        self.assertFalse(w.run())

    def test_sequential_recovery(self):
        w = DiagnosticWorld(); w.corrupt('V', ('NOOP',) * 3)
        self.assertTrue(w.run())
        w.corrupt('M', ('NOOP',) * 3)
        self.assertTrue(w.run())
        self.assertEqual((w.detections, w.internal_repairs, w.external_repairs), (2, 2, 0))

    def test_external_repair_not_counted_as_internal(self):
        w = DiagnosticWorld(); w.corrupt('V', ('NOOP',) * 3); w.external_restore('V')
        self.assertTrue(w.run())
        self.assertEqual((w.internal_repairs, w.external_repairs), (0, 1))

    def test_clean_world(self):
        w = DiagnosticWorld()
        self.assertTrue(w.run())
        self.assertEqual(w.detections, 0)

    def test_scenario_matrix(self):
        r = scenarios()
        self.assertTrue(r['corrupt_G']['recovered'])
        self.assertFalse(r['corrupt_O']['recovered'])
        self.assertTrue(r['sequential_V_M']['second'])


if __name__ == '__main__':
    unittest.main()
