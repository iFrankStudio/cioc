import unittest
from cioc import Ecology, scenarios

class EcologyTests(unittest.TestCase):
    def test_instructions_cost_energy(self):
        w=Ecology();uid=w.insert(('WAIT',),energy=4);w.step()
        self.assertEqual(w.alive[uid].energy,3)
    def test_extinction_without_resource(self):
        w=Ecology();w.insert(('WAIT','WAIT','DIVIDE'));w.run(40)
        self.assertEqual(len(w.alive),0)
    def test_replication_without_host_repair(self):
        w=Ecology(mutation_rate=0);w.insert(('EAT_A','EAT_A','DIVIDE'));w.run(30)
        self.assertGreater(w.births,0);self.assertEqual(w.external_insertions,1)
    def test_mutations_are_in_instruction_alphabet(self):
        w=Ecology(mutation_rate=1);w.insert(('EAT_A','EAT_A','DIVIDE'));w.run(20)
        self.assertGreater(w.mutations,0)
        self.assertTrue(all(set(x.code)<=set(('EAT_A','EAT_B','DIVIDE','WAIT')) for x in w.alive.values()))
    def test_deterministic(self):
        def run():
            w=Ecology(seed=12,mutation_rate=0.2);w.insert(('EAT_A','EAT_A','DIVIDE'));w.run(25)
            return w.stats
        self.assertEqual(run(),run())
    def test_capacity(self):
        w=Ecology(capacity=10);w.insert(('EAT_A','EAT_A','DIVIDE'));w.run(100)
        self.assertLessEqual(len(w.alive),10)
    def test_no_exogenous_reseeding(self):
        w=Ecology();w.insert(('EAT_A','EAT_A','DIVIDE'));w.run(30)
        self.assertEqual(w.external_insertions,1)
    def test_scenarios(self):
        s=scenarios();self.assertGreater(s['replication']['births'],0);self.assertEqual(s['no_food']['population'],0)
if __name__=='__main__':unittest.main()
