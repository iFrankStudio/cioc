import unittest
from cioc import Organism, VM, Graph, Node, decode_graph, run_case


class CiocV05Tests(unittest.TestCase):
    def test_single_fault_repaired_by_graph_program(self):
        result = run_case('single_fault')
        self.assertTrue(result['viable'])
        self.assertEqual(result['internal_commits'], 1)
        self.assertEqual(result['external_repairs'], 0)
        self.assertEqual(result['repaired'], {'A': 15})

    def test_multiple_sequential_faults(self):
        result = run_case('sequential_faults')
        self.assertTrue(result['viable'])
        self.assertEqual(result['internal_commits'], 3)

    def test_two_faults_recover(self):
        result = run_case('double_fault')
        self.assertTrue(result['viable'])
        self.assertEqual(result['internal_commits'], 2)

    def test_total_maintenance_loss_fails(self):
        result = run_case('all_fault')
        self.assertFalse(result['viable'])
        self.assertEqual(result['internal_commits'], 0)

    def test_contract_ablation_fails(self):
        result = run_case('no_contract')
        self.assertFalse(result['viable'])
        self.assertEqual(result['internal_commits'], 0)

    def test_mutation_permission_ablation_fails(self):
        result = run_case('no_mutation')
        self.assertFalse(result['viable'])
        self.assertEqual(result['internal_commits'], 0)

    def test_budget_limited(self):
        result = run_case('low_budget')
        self.assertFalse(result['viable'])
        self.assertEqual(result['internal_commits'], 0)

    def test_search_space_lacks_solution(self):
        result = run_case('no_valid_genome')
        self.assertFalse(result['viable'])
        self.assertEqual(result['internal_commits'], 0)

    def test_external_restoration_is_separate(self):
        result = run_case('external_baseline')
        self.assertTrue(result['viable'])
        self.assertEqual(result['internal_commits'], 0)
        self.assertEqual(result['external_repairs'], 1)

    def test_no_damage_does_not_propose_repair(self):
        result = run_case('idle')
        self.assertTrue(result['viable'])
        self.assertEqual(result['internal_commits'], 0)

    def test_damage_erases_executable_program(self):
        o = Organism.create()
        o.damage('B')
        self.assertIsNone(o.programs['B'])
        self.assertFalse(o.viable())

    def test_repair_has_actual_vm_instruction_trace(self):
        o = Organism.create()
        o.damage('A')
        VM(o).run()
        ops = [x['op'] for x in o.history if x['event'] == 'step']
        for op in ('READ', 'FIRST', 'DECODE', 'EVAL', 'INC', 'PUT'):
            self.assertIn(op, ops)
        self.assertEqual(o.programs['A'].genome, 15)

    def test_repair_requires_live_controller_not_just_code_available(self):
        o = Organism.create()
        o.damage('A'); o.damage('B'); o.damage('C')
        VM(o).run()
        self.assertEqual(o.version, 0)

    def test_invalid_policy_does_not_repair(self):
        o = Organism({'A': None, 'B': decode_graph(13)}, compute=500)
        VM(o).run()
        self.assertIsNone(o.programs['A'])

    def test_no_target_oracle_in_host_repair_dispatch(self):
        # Public API to host has VM.run, no target-specific World.regenerate.
        self.assertFalse(hasattr(VM, 'regenerate'))

    def test_candidate_graphs_are_distinct_objects(self):
        a = decode_graph(15)
        b = decode_graph(15)
        self.assertIsNot(a, b)
        self.assertIsNot(a.nodes, b.nodes)

    def test_invalid_graph_halts_without_install(self):
        broken = Graph({0: Node('NOT_AN_OPCODE')})
        o = Organism({'A': None, 'B': broken})
        VM(o).run()
        self.assertIsNone(o.programs['A'])


if __name__ == '__main__':
    unittest.main(verbosity=2)
