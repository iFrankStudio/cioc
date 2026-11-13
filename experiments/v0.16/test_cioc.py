import unittest
from cioc import MutualWorld,controller_valid,scenarios

class MutualTests(unittest.TestCase):
    def test_primary_rebuilt(self):
        w=MutualWorld(damage_rate=0);uid=w.insert();w.corrupt_controller(uid,'primary');w.step()
        self.assertTrue(controller_valid(w.organisms[uid].controller));self.assertEqual(w.controller_repairs,1)
    def test_secondary_rebuilt(self):
        w=MutualWorld(damage_rate=0);uid=w.insert();w.corrupt_controller(uid,'secondary');w.step()
        self.assertTrue(controller_valid(w.secondary[uid]));self.assertEqual(w.controller_repairs,1)
    def test_joint_loss_cut_set(self):
        w=MutualWorld(damage_rate=0);uid=w.insert();w.corrupt_controller(uid,'primary');w.corrupt_controller(uid,'secondary');w.step()
        self.assertEqual(w.controller_repairs,0)
    def test_controller_recovery_enables_metabolism_repair(self):
        w=MutualWorld(damage_rate=0);uid=w.insert();w.corrupt(uid,body=('WAIT','WAIT','DIVIDE'))
        w.corrupt_controller(uid,'primary');w.step()
        self.assertEqual((w.controller_repairs,w.repairs),(1,1))
    def test_no_backup_fragment(self):
        w=MutualWorld(damage_rate=0);uid=w.insert(fragment=None);w.corrupt_controller(uid,'primary');w.step()
        self.assertEqual(w.controller_repairs,0)
    def test_disable_peer(self):
        w=MutualWorld(damage_rate=0,disable_peer_repair=True);uid=w.insert();w.corrupt_controller(uid,'primary');w.step()
        self.assertEqual(w.controller_repairs,0)
    def test_finite_budget(self):
        w=MutualWorld(damage_rate=0);uid=w.insert(energy=3);w.corrupt_controller(uid,'primary');w.step()
        self.assertEqual(w.controller_repairs,0)
    def test_equivalent_program_valid(self):
        self.assertTrue(controller_valid(('AUDIT','REBUILD','WAIT')))
    def test_no_false_alarm(self):
        w=MutualWorld(damage_rate=0);w.insert();w.step();self.assertEqual(w.controller_repairs,0)
    def test_child_carries_controller(self):
        w=MutualWorld(damage_rate=0);w.insert();w.run(5)
        self.assertTrue(all(uid in w.secondary for uid in w.organisms))
    def test_scenarios(self):
        r=scenarios();self.assertTrue(r['loss_primary']['healthy']);self.assertFalse(r['both_loss']['controller_repairs'])
if __name__=='__main__':unittest.main()
