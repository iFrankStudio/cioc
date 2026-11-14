"""Study70 only two targeted calibrations for CIL genotype equality and seed alignment."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from analyze_study70 import genome_hash, program_distance, verify_topology_pair


def base():
    code = [
        {"Const": {"dst":1, "value":1, "next":1}},
        {"Const": {"dst":2, "value":7, "next":2}},
        {"Const": {"dst":3, "value":9, "next":3}},
    ]
    code += [
        {"Const": {"dst":0, "value":pc%8, "next":pc+1}}
        for pc in range(3, 27)
    ]
    code.append("Halt")
    return code


class Study70GenotypeIntegrity(unittest.TestCase):
    def test_original_code_fingerprint_ignores_sensor_metadata_but_not_branch_edge(self):
        a = base()
        b = base()
        self.assertEqual(genome_hash(a), genome_hash(b))
        a[9] = {"Branch": {"test":0, "nonzero":8, "zero":10}}
        b[9] = {"Branch": {"test":0, "nonzero":6, "zero":10}}
        verify_topology_pair(a, b)
        self.assertNotEqual(genome_hash(a), genome_hash(b))

    def test_offset_alignment_is_complete_bijection_not_an_independence_test(self):
        ids = [(73*i + 19)%256 for i in range(256)]
        self.assertEqual(len(set(ids)), 256)
        a, b = base(), base()
        self.assertEqual(program_distance(a, b)["identical_instructions_of24"], 24)
        self.assertEqual(program_distance(a, b)["same_opcode_tags_of24"], 24)


if __name__ == "__main__":
    unittest.main()
