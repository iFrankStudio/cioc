import unittest
from cioc import World, run_case, execute_role, ROLE_REQUIREMENTS

class TestControlRegeneration(unittest.TestCase):
    def test_role_program_contract(self):
        for role, ops in ROLE_REQUIREMENTS.items():
            self.assertTrue(execute_role(tuple(ops), role, 0, phase='train'))
            self.assertFalse(execute_role(('NOP',), role, 0, phase='train'))
    def test_all_three_single_damages(self):
        for scenario in ('search_regeneration','verify_regeneration','commit_regeneration'):
            with self.subTest(scenario=scenario):
                r=run_case(scenario)
                self.assertTrue(r['viable'])
                self.assertEqual(r['internal_commits'],1)
                self.assertEqual(r['external_repairs'],0)
    def test_sequential_regeneration(self):
        r=run_case('sequential_regeneration')
        self.assertTrue(r['viable'])
        self.assertEqual(r['internal_commits'],3)
    def test_joint_failure_without_proposer(self):
        r=run_case('joint_failure')
        self.assertFalse(r['viable'])
        self.assertEqual(r['internal_commits'],0)
    def test_no_backup_search(self):
        self.assertFalse(run_case('no_backup_search')['viable'])
    def test_no_contract(self):
        self.assertFalse(run_case('no_contract')['viable'])
    def test_low_budget(self):
        self.assertFalse(run_case('low_budget')['viable'])
    def test_missing_primitive(self):
        self.assertFalse(run_case('missing_primitive')['viable'])
    def test_external_baseline_not_counted_internal(self):
        r=run_case('external_baseline')
        self.assertTrue(r['viable'])
        self.assertEqual((r['internal_commits'],r['external_repairs']),(0,1))
    def test_secondary_capability_regeneration(self):
        r=run_case('destroy_secondary')
        self.assertTrue(r['viable'])
        self.assertEqual(r['internal_commits'],1)
    def test_target_code_erased_before_synthesis(self):
        w=World.create()
        before=w.nodes['S'].program
        w.damage('S')
        self.assertIsNone(w.nodes['S'].program)
        self.assertTrue(w.regenerate('S'))
        self.assertNotEqual(w.nodes['S'].program,before)
    def test_failed_gate_cannot_install(self):
        w=World.create()
        w.damage('S')
        self.assertFalse(w._install('M','S','search',('ENUM','LIMIT3'),verified_train=True,verified_holdout=False))
        self.assertIsNone(w.nodes['S'].program)
    def test_no_self_repair_with_sole_damaged_operator(self):
        w=World.create()
        w.damage('S')
        self.assertFalse(w.regenerate('S',excluded=frozenset({'V','M'})))
    def test_budget_nonnegative(self):
        for scenario in ('search_regeneration','verify_regeneration','commit_regeneration',
                         'sequential_regeneration','low_budget'):
            self.assertGreaterEqual(run_case(scenario)['remaining_budget'],0)
    def test_no_privileged_external_rescue(self):
        w=World.create();w.damage('S')
        self.assertFalse(w.external_restore('S'))
        self.assertEqual(w.external_repairs,0)

if __name__=='__main__': unittest.main()
