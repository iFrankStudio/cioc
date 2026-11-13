import unittest
from simulator import scenario, intervene
from analyzer import probe, discover, classify

class TestCIOC(unittest.TestCase):
    def test_call_cycle_not_repair(self):
        case = scenario('call_cycle')
        self.assertFalse(intervene(case, 'B').recovered)
        self.assertEqual(discover('call_cycle')['minimal_disabling_sets'], [])
    def test_internal_cycle(self):
        case = scenario('internal_closure')
        self.assertTrue(intervene(case, 'B').recovered)
        self.assertFalse(intervene(case, 'B', frozenset({'A'})).recovered)
        findings = discover('internal_closure')
        self.assertTrue(classify('internal_closure', findings)['three_node_internal_maintenance_cycle_detected'])
    def test_external_repair(self):
        case = scenario('external_repair')
        self.assertTrue(intervene(case, 'B').recovered)
        self.assertFalse(intervene(case, 'B', frozenset({'F'})).recovered)
        findings = discover('external_repair')
        self.assertFalse(classify('external_repair', findings)['three_node_internal_maintenance_cycle_detected'])
    def test_redundancy(self):
        case = scenario('redundant_maintenance')
        self.assertTrue(intervene(case, 'C', frozenset({'A'})).recovered)
        self.assertTrue(intervene(case, 'C', frozenset({'B'})).recovered)
        self.assertFalse(intervene(case, 'C', frozenset({'A','B'})).recovered)
        findings = discover('redundant_maintenance')
        self.assertTrue(any(x['target']=='C' and x['disabled']==('A','B') for x in findings['minimal_disabling_sets']))
        self.assertTrue(classify('redundant_maintenance', findings)['strongly_connected_internal_support_candidate'])
    def test_resource_constraint(self):
        case = scenario('internal_closure')
        self.assertFalse(intervene(case, 'B', budget=2).recovered)
        self.assertTrue(intervene(case, 'B', budget=3).recovered)
    def test_target_cannot_be_disabled(self):
        with self.assertRaises(ValueError):
            intervene(scenario('internal_closure'), 'B', frozenset({'B'}))
    def test_single_ablation_own_function_not_maintenance(self):
        # Calls are not diagnosed as repair merely from runtime invocation.
        self.assertEqual(probe('call_cycle','C',('B',)).loss_of_recovery, 0)

if __name__ == '__main__':
    unittest.main()
