import unittest
from cioc import Organism, experiment

class CIOCV07Tests(unittest.TestCase):
    def test_generator_recovery(self):
        # Baseline: without G's own program, no internal candidate generation is possible.
        x=Organism();x.damage('G');self.assertTrue(x.run());self.assertEqual(x.revision,1)
    def test_observer_single_point(self):
        x=Organism();x.damage('O');self.assertFalse(x.run())
    def test_verifier_single_point(self):
        x=Organism();x.damage('V');self.assertFalse(x.run())
    def test_modifier_single_point(self):
        x=Organism();x.damage('M');self.assertFalse(x.run())
    def test_external_restore_separate(self):
        x=Organism();x.damage('G');x.external_restore('G');self.assertTrue(x.run());self.assertEqual(x.revision,0)
    def test_no_damage(self):
        self.assertTrue(Organism().run())
    def test_budget(self):
        x=Organism(budget=1);x.damage('G');self.assertFalse(x.run())
    def test_fragment_cannot_be_full_program(self):
        x=Organism();self.assertEqual(len(x.fragments['O']['G']),1)
    def test_missing_fragment_prevents_recovery(self):
        x=Organism();x.damage('G');x.fragments['V'].pop('G')
        self.assertFalse(x.run());self.assertEqual(x.revision,0)
    def test_simultaneous_loss(self):
        x=Organism();x.damage('G');x.damage('M')
        self.assertFalse(x.run());self.assertEqual(x.revision,0)
    def test_generator_program_is_restored(self):
        x=Organism();x.damage('G');self.assertTrue(x.run())
        self.assertEqual(x.roles['G'],('GENERATE','PROPOSE'))
    def test_no_hidden_external_repairs(self):
        x=Organism();x.damage('G');x.run();self.assertEqual(x.external_restores,0)

if __name__=='__main__':unittest.main()
