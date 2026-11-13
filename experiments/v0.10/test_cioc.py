import unittest
from cioc import World, PROTOCOL_CANONICAL, protocol_contract, protocol_probe


class ProtocolRecoveryTests(unittest.TestCase):
    def test_each_protocol_recovers_without_exact_source(self):
        for name in ('PV', 'PI'):
            with self.subTest(name=name):
                w = World(); w.damage(name)
                self.assertTrue(w.run())
                self.assertEqual((w.protocol_repairs, w.external_repairs), (1, 0))
                self.assertNotEqual(w.protocols[name], PROTOCOL_CANONICAL[name])
                self.assertTrue(protocol_contract(name, w.protocols[name]))

    def test_recovered_protocol_executed_in_later_role_repair(self):
        for protocol, role in (('PV', 'V'), ('PI', 'M')):
            with self.subTest(protocol=protocol):
                w = World(); w.damage(protocol)
                self.assertTrue(w.run())
                w.damage(role)
                self.assertTrue(w.run())
                self.assertEqual(w.internal_repairs, 2)
                self.assertTrue(any(e['event']=='protocol_executed' and
                    e['protocol']==protocol and e['target']==role for e in w.events))

    def test_protocol_and_dependent_role_simultaneous_loss(self):
        for pair in (('PV', 'V'), ('PI', 'M')):
            with self.subTest(pair=pair):
                w = World(); w.damage(*pair)
                self.assertFalse(w.run())
                self.assertEqual(w.protocol_repairs, 0)

    def test_role_recovery_baseline(self):
        for role, expected in (('O',False),('G',True),('V',True),('M',True)):
            with self.subTest(role=role):
                w = World(); w.damage(role)
                self.assertEqual(w.run(), expected)

    def test_missing_fragments_prevent_protocol_recovery(self):
        w=World(); w.damage('PV')
        w.protocol_fragments['G'].pop('PV')
        w.protocol_fragments['V'].pop('PV')
        self.assertFalse(w.run())

    def test_budget_limit(self):
        w=World(budget=5); w.damage('PI')
        self.assertFalse(w.run())

    def test_external_restore_not_internal(self):
        w=World(); w.damage('PV'); w.external_restore('PV')
        self.assertTrue(w.run())
        self.assertEqual((w.internal_repairs,w.external_repairs),(0,1))

    def test_protocol_vm_is_causally_required(self):
        for protocol,role in (('PV','V'),('PI','M')):
            with self.subTest(protocol=protocol):
                w=World(enable_protocol_vm=False); w.damage(role)
                self.assertFalse(w.run())

    def test_corrupted_live_protocol_is_not_auto_detected(self):
        w=World(); w.protocols['PV']=('NOOP','NOOP','NOOP'); w.damage('V')
        self.assertFalse(w.run())

    def test_protocol_negative_behavioral_controls(self):
        self.assertEqual(protocol_probe(('POS_O','NEG_M','CERTIFY_P'),{'O_positive':True}),())
        self.assertEqual(protocol_probe(('VOTE_V','PERMIT_G','INSTALL_P'),{'G_delegates':True}),())
        self.assertFalse(protocol_contract('PV',('POS_O','NOOP','CERTIFY_P')))
        self.assertFalse(protocol_contract('PI',('VOTE_V','NOOP','INSTALL_P')))

    def test_no_damage_no_repair(self):
        w=World()
        self.assertTrue(w.run())
        self.assertEqual(w.internal_repairs,0)


if __name__ == '__main__':
    unittest.main()
