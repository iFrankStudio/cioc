import unittest
from cioc import Organism, evaluate, candidates, make_shares

class TestComplementaryOrganization(unittest.TestCase):
    def create_damaged(self, *damage, **kwargs):
        o=Organism.create(**kwargs)
        for name in damage: o.damage(name)
        return o

    def test_typed_program(self):
        self.assertEqual(evaluate(('DOUBLE','INC'),6),13)
    def test_reject_unknown_op(self):
        with self.assertRaises(ValueError): evaluate(('NOOP',),1)
    def test_candidate_enumeration_bounded(self):
        self.assertEqual(len(list(candidates(('INC','DOUBLE')))), 14)
    def test_shares_reconstruct_bytes(self):
        raw=b'test contract';a,b=make_shares(raw)
        self.assertEqual(bytes(x^y for x,y in zip(a,b)),raw)
        self.assertNotEqual(a,raw);self.assertNotEqual(b,raw)
    def test_task_reconstruction(self):
        o=self.create_damaged('TASK'); r=o.run()
        self.assertTrue(r['task_restored']); self.assertTrue(r['heldout_passed'])
        self.assertEqual(r['internal_task_repairs'],1);self.assertEqual(r['external_repairs'],0)
    def test_contract_reconstruction(self):
        o=self.create_damaged('CONTRACT');r=o.run()
        self.assertTrue(r['contract_restored']);self.assertEqual(r['internal_contract_repairs'],1)
    def test_joint_reconstruction(self):
        o=self.create_damaged('TASK','CONTRACT');r=o.run(400)
        self.assertTrue(r['contract_restored']);self.assertTrue(r['task_restored']);self.assertTrue(r['heldout_passed'])
    def test_all_roles_necessary_for_task(self):
        for role in ('O','G','V','M'):
            with self.subTest(role=role):
                o=self.create_damaged('TASK',role);self.assertFalse(o.run()['task_restored'])
    def test_shares_required(self):
        for shard in ('SHARD_O','SHARD_V'):
            with self.subTest(shard=shard):
                o=self.create_damaged('CONTRACT',shard);self.assertFalse(o.run()['contract_restored'])
    def test_corrupted_share_rejected(self):
        o=self.create_damaged('CONTRACT');o.shard_v=b'bad'+o.shard_v[3:]
        r=o.run();self.assertFalse(r['contract_restored'])
        self.assertTrue(any(e['event']=='contract_reconstruction_rejected' for e in o.history))
    def test_budget_limits(self):
        o=self.create_damaged('TASK',compute=12)
        self.assertFalse(o.run()['task_restored'])
    def test_missing_necessary_primitive(self):
        o=self.create_damaged('TASK',available_ops=('INC','DEC','SQUARE'))
        self.assertFalse(o.run()['task_restored'])
    def test_no_detection(self):
        o=self.create_damaged('TASK');o.detection_enabled=False
        self.assertFalse(o.run()['task_restored'])
    def test_external_restoration_not_internal(self):
        o=self.create_damaged('TASK');o.restore_external('TASK');r=o.run()
        self.assertEqual(r['external_repairs'],1)
        self.assertEqual(r['internal_task_repairs'],0)
    def test_complementary_roles_not_duplicate_repairers(self):
        o=self.create_damaged('TASK')
        o.enabled['V']=False
        self.assertFalse(o.run()['task_restored'])
        o=self.create_damaged('TASK')
        o.enabled['V']=True
        self.assertTrue(o.run()['task_restored'])
    def test_repair_logged_with_causal_messages(self):
        o=self.create_damaged('TASK');o.run()
        self.assertTrue(any(e.get('message_kind')=='CANDIDATE' for e in o.history))
        self.assertTrue(any(e.get('message_kind')=='VERIFIED_CANDIDATE' for e in o.history))
        self.assertTrue(any(e['event']=='task_reconstructed' for e in o.history))

if __name__=='__main__':unittest.main()
