import unittest
from cioc import *

class CIOCTests(unittest.TestCase):
    def test_genesis_functional(self): self.assertTrue(World.create().viable())
    def test_no_blueprint_needed_for_B(self):
        w=World.create(); old=GENESIS['B']; w.damage('B')
        self.assertTrue(ResidentSearch().repair(w,'B'))
        self.assertTrue(w.components['B'].functional())
        self.assertNotEqual(w.components['B'].program, old)
        self.assertFalse(any(x['event']=='external_restore' for x in w.history))
    def test_chain(self):
        r=run_case('chain_regeneration'); self.assertTrue(r['viable']); self.assertEqual(r['internal_commits'],2)
    def test_no_search(self): self.assertFalse(run_case('no_search')['viable'])
    def test_external_baseline_not_internal(self):
        r=run_case('external_baseline'); self.assertTrue(r['viable']); self.assertEqual(r['internal_commits'],0); self.assertEqual(r['external_repairs'],1)
    def test_cut_source(self): self.assertFalse(run_case('disable_source')['viable'])
    def test_budget(self): self.assertFalse(run_case('low_budget')['viable'])
    def test_no_square_cannot_recover_C(self): self.assertFalse(run_case('missing_primitive')['viable'])
    def test_search_engine_limitation(self): self.assertFalse(run_case('destroyed_search')['viable'])
    def test_restore_A(self): self.assertTrue(run_case('recover_A')['viable'])
    def test_invalid_commit_rejected(self):
        w=World.create(); w.damage('B'); self.assertFalse(w.commit('A','B',('INC',))); self.assertIsNone(w.components['B'].program)
    def test_no_wrong_source(self):
        w=World.create(); w.damage('B'); self.assertFalse(w.commit('C','B',('DOUBLE','INC')))
    def test_determinism(self):
        self.assertEqual(run_case('chain_regeneration'),run_case('chain_regeneration'))
    def test_budget_never_negative(self):
        for s in SCENARIOS: self.assertGreaterEqual(run_case(s)['remaining_budget'],0)

if __name__=='__main__': unittest.main()
