import unittest
from cioc import EvidenceWorld, witness_contract, scenarios

class QuorumTests(unittest.TestCase):
    def test_one_fault(self):
        w=EvidenceWorld(target_code=('SENSE','SILENCE'))
        self.assertTrue(w.step())
        self.assertTrue(witness_contract(w.target_code))
        self.assertEqual(w.internal_repairs,1)
    def test_no_false_alarm(self):
        w=EvidenceWorld();self.assertFalse(w.step());self.assertEqual(w.internal_repairs,0)
    def test_single_byzantine(self):
        w=EvidenceWorld(target_code=('SENSE','SILENCE'))
        w.witnesses['A']=('SENSE','INVERT','REPORT')
        self.assertTrue(w.step())
    def test_two_colluders_limit(self):
        w=EvidenceWorld(target_code=('SENSE','SILENCE'))
        w.witnesses['A']=('SENSE','INVERT','REPORT')
        w.witnesses['B']=('SENSE','INVERT','REPORT')
        self.assertFalse(w.step())
    def test_quorum_unavailable(self):
        w=EvidenceWorld(target_code=('SENSE','SILENCE'))
        w.witnesses['A']=None;w.witnesses['B']=None
        self.assertFalse(w.step())
    def test_replay_and_sybil_dont_vote(self):
        w=EvidenceWorld(target_code=('SENSE','SILENCE'))
        reports=w.collect(injected=[{'name':'A','epoch':0,'fault':False}, {'name':'D','epoch':1,'fault':True}])
        self.assertTrue(w.repair(reports))
    def test_duplicate_conflicting_identity_abstains(self):
        w=EvidenceWorld(target_code=('SENSE','SILENCE'))
        reports=w.collect(injected=[{'name':'A','epoch':1,'fault':False}]);self.assertTrue(w.repair(reports))
    def test_missing_fragments(self):
        w=EvidenceWorld(target_code=('SENSE','SILENCE'));w.fragments.clear()
        self.assertFalse(w.step())
    def test_budget(self):
        w=EvidenceWorld(target_code=('SENSE','SILENCE'),budget=3)
        self.assertFalse(w.step())
    def test_disable(self):
        w=EvidenceWorld(target_code=('SENSE','SILENCE'),allow_recovery=False)
        self.assertFalse(w.step())
    def test_external_separation(self):
        w=EvidenceWorld(target_code=('SENSE','SILENCE'));w.external_restore()
        self.assertFalse(w.step());self.assertEqual((w.internal_repairs,w.external_repairs),(0,1))
    def test_matrix(self):
        s=scenarios();self.assertTrue(s['single_fault']['healthy']);self.assertFalse(s['two_byzantine']['healthy'])
if __name__=='__main__':unittest.main()
