import unittest
from dynamic import World, OrganismPolicy, run_case, CANONICAL

class TestRegeneration(unittest.TestCase):
    def test_repair_repairer_and_then_other_module(self):
        r=run_case('internal_regeneration')
        self.assertTrue(r['viable'])
        self.assertEqual(r['revisions'],2)
        self.assertEqual(r['external_repairs'],0)
        self.assertGreater(r['candidates_tested'],2)
    def test_fixed_no_regeneration(self):
        self.assertFalse(run_case('fixed')['viable'])
    def test_external_not_attributed_internal(self):
        r=run_case('external')
        self.assertTrue(r['viable']); self.assertEqual(r['external_repairs'],1)
        self.assertFalse(any(e['event']=='commit' for e in r['history']))
    def test_disable_only_provider(self):
        self.assertFalse(run_case('maintenance_cut')['viable'])
    def test_budget_is_binding(self):
        self.assertFalse(run_case('insufficient_resources',budget=3)['viable'])
    def test_broken_internal_backup(self):
        self.assertFalse(run_case('broken_blueprint')['viable'])
    def test_candidate_validation_no_world_auto_repair(self):
        w=World.create(); w.fault('B')
        self.assertFalse(w.attempt('A','B',('VERIFY','WRITE','LOAD')))
        self.assertFalse(w.modules['B'].functional)
        self.assertTrue(w.attempt('A','B',CANONICAL))
        self.assertTrue(w.modules['B'].functional)
    def test_permissions(self):
        w=World.create(); w.fault('C')
        self.assertFalse(w.attempt('A','C',CANONICAL))
    def test_checkpoint_isolation(self):
        w=World.create(); snapshot=w.checkpoint(); w.fault('B')
        self.assertTrue(snapshot.modules['B'].functional)
    def test_invalid_candidate_cannot_commit(self):
        w=World.create(); w.fault('B')
        self.assertFalse(w.attempt('A','B',('LOAD','WRITE','WRITE')))
        self.assertEqual(w.version,0)
    def test_recovery_deterministic(self):
        a=run_case('internal_regeneration'); b=run_case('internal_regeneration')
        self.assertEqual(a,b)

if __name__=='__main__': unittest.main()
