"""Study59 exact-only original CIL graph-copy opcode treatment calibration."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from analyze_study59 import normalize_local
from analyze_study59_local import expected_continuations as local_grammar
from analyze_study59_absolute import expected_continuations as absolute_grammar


class Study59GenericLocality(unittest.TestCase):
    def test_frozen_256_independent_seed_pairs_change_only_copy_opcode_name(self):
        seen_local=0
        for seed in range(4_000_000,4_000_256):
            a=absolute_grammar(seed)
            b=local_grammar(seed)
            self.assertEqual(set(a),set(b))
            for actor in (1,2,3):
                local_nodes=["Halt"]*9+b[actor]+["Halt"]
                self.assertEqual(normalize_local(local_nodes)[9:81],a[actor])
                seen_local += sum("GraphCopyNodeLocal" in op for op in b[actor])
        self.assertGreater(seen_local,0)

    def test_local_copy_with_unknown_operand_change_is_rejected(self):
        nodes=["Halt"]*82
        nodes[9]={"GraphCopyNodeLocal":{
            "graph":3,"source":1,"dst":2,"next":10,
        }}
        with self.assertRaises(ValueError):
            normalize_local(nodes)


if __name__=="__main__":
    unittest.main()
