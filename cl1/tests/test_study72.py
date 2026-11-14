"""Study72: exact twelve-arm SplitMix64 grammar and within-world sensor-switch controls."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from analyze_study72 import expected_code, mix_seed, within_world_pair


def make_write(epoch, value, pc=7, read_epoch=None):
    return {
        "epoch": epoch,
        "pc": pc,
        "tick": epoch * 12 + 3,
        "value": value,
        "source_tainted": True,
        "latest_native_source_read_epoch": epoch if read_epoch is None else read_epoch,
    }


class Study72DynamicSensor(unittest.TestCase):
    def test_registered_genomes_match_across_sensor_arms(self):
        self.assertEqual(mix_seed(4_800_000), mix_seed(4_800_000))
        for seed in range(4_800_000,4_800_256):
            for mode in ("broad","short"):
                dynamic=expected_code(seed,mode+"-dynamic")
                static_low=expected_code(seed,mode+"-static-low")
                static_high=expected_code(seed,mode+"-static-high")
                self.assertEqual(dynamic, static_low)
                self.assertEqual(dynamic, static_high)
                ablated=expected_code(seed,mode+"-ablated-dynamic")
                for pc,(normal,negative) in enumerate(zip(dynamic,ablated)):
                    if isinstance(normal,dict) and "Read" in normal:
                        self.assertEqual(negative,{"Const":{"dst":0,"value":0,"next":pc+1}})
                    else:
                        self.assertEqual(normal,negative)

    def test_source_switch_requires_same_writer_pc_both_windows_and_fresh_read(self):
        dynamic={"writes":[make_write(9,3),make_write(13,8)]}
        low={"writes":[make_write(9,3),make_write(13,3)]}
        high={"writes":[make_write(9,8),make_write(13,8)]}
        self.assertTrue(within_world_pair(dynamic,low,high)["qualified"])
        dynamic["writes"][1]["latest_native_source_read_epoch"]=11
        self.assertFalse(within_world_pair(dynamic,low,high)["qualified"])
        dynamic["writes"][1]["latest_native_source_read_epoch"]=13
        dynamic["writes"][1]["pc"]=8
        self.assertFalse(within_world_pair(dynamic,low,high)["qualified"])


if __name__=="__main__":
    unittest.main()
