import unittest
from cioc import World,scenarios

class MaintenanceEcologyTests(unittest.TestCase):
    def test_self_restoration(self):
        w=World(damage_rate=0,mutation_rate=0);uid=w.insert()
        w.corrupt(uid,body=('WAIT','WAIT','DIVIDE'));w.step()
        self.assertEqual(w.repairs,1)
        self.assertIn('EAT_A',w.organisms[uid].body)
    def test_not_full_template_restore(self):
        w=World(damage_rate=0);uid=w.insert()
        w.corrupt(uid,body=('WAIT','WAIT','DIVIDE'))
        w.switch(0,150);w.step()
        self.assertEqual(w.repairs,1)
        self.assertIn('EAT_B',w.organisms[uid].body)
    def test_disabled_repair_negative_control(self):
        w=World(repair_enabled=False,damage_rate=0);uid=w.insert()
        w.corrupt(uid,body=('WAIT','WAIT','DIVIDE'));w.run(25)
        self.assertEqual(len(w.organisms),0)
    def test_controller_damage_blocks(self):
        w=World(damage_rate=0);uid=w.insert()
        w.corrupt(uid,body=('WAIT','WAIT','DIVIDE'),controller=('WAIT','WAIT'));w.run(25)
        self.assertEqual(w.repairs,0)
    def test_anchor_absence_blocks(self):
        w=World(damage_rate=0);uid=w.insert(anchor=None)
        w.corrupt(uid,body=('WAIT','WAIT','DIVIDE'));w.run(25)
        self.assertEqual(w.repairs,0)
    def test_repair_costs_energy(self):
        w=World(damage_rate=0);uid=w.insert(energy=4)
        w.corrupt(uid,body=('WAIT','WAIT','DIVIDE'));w.step()
        self.assertEqual(w.repairs,0)
    def test_healthy_does_not_repair(self):
        w=World(damage_rate=0);w.insert();w.step();self.assertEqual(w.repairs,0)
    def test_scheduled_damage_is_not_fault_signal(self):
        w=World(damage_rate=1.0);w.insert();w.step();self.assertEqual(w.faults,1)
    def test_determinism(self):
        def run():
            w=World(seed=3,damage_rate=.08);w.insert();w.run(40);return w.records
        self.assertEqual(run(),run())
    def test_capacity(self):
        w=World(capacity=8,damage_rate=0);w.insert();w.run(60)
        self.assertLessEqual(len(w.organisms),8)
    def test_scenarios(self):
        r=scenarios();self.assertGreaterEqual(r['single_body_repair']['repaired'],1)
if __name__=='__main__':unittest.main()
