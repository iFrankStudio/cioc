#!/usr/bin/env python3
"""I6N expected-negative *native* audit: stale witness evidence after patch2.

The original I6N frozen source and Rust run remain immutable evidence.
A passing negative auditor MUST reproduce the defect, not hide it: each of
eight original-native hard genotypes pays for two original PatchConst edits
but only one set of three reset witnesses, occurring BEFORE the first edit.
Thus no fresh isolation occurs between first and second modifications.
The corrected I6R must separately prove resets for BOTH edits.
"""
import json
import os
import sys
from pathlib import Path
NAME="B56-F1d1k-G3Q-C1-P1-L3-I6N"
FREEZE=(Path(__file__).resolve().parents[1]/
    "studies/B56-F1d1k-G3Q-C1-P1-L3-I6N-frozen-179-node-two-paid-repairs-baseline-v1.json")

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: python3 I6N_NEGATIVE.py NATIVE_SUMMARY.json NATIVE_TRACE_DIR")
    source=json.loads(FREEZE.read_text())
    assert source["study"]==NAME and source["source_node_count"]==179
    actual=json.loads(Path(sys.argv[1]).read_text())
    assert actual["study"]==NAME
    assert actual["level"]=="NATIVE_ORIGINAL_PID_179_NODE_BOUNDED_TWO_CHARGED_SELF_EDITS"
    assert actual["source_sha"] not in ("","local")
    if os.environ.get("GITHUB_SHA"):
        assert actual["source_sha"]==os.environ["GITHUB_SHA"]
    groups=actual["original64_worlds"]
    assert len(groups)==64
    hard={tuple(x) for x in source["controls"] if False}
    hard={tuple(x["seed"]) for x in source["all64_original_PID_arms"]
        if len(x["expected"]["paid_source_edits"])==2}
    assert len(hard)==8
    missing=[]
    for i,world in enumerate(groups):
        seed=world["seed"]
        assert i==(seed[0]*4+seed[1])*4+seed[2]
        assert world["original_PID1_naturally_halted"] is True
        assert world["postload_host_source_writes"]==0
        patches=world["original_PID1_source_edits"]
        reset=world["paid_full_witness_reset_events"]
        assert len(reset)==3*min(1,len(patches)),("native source changed",seed)
        if len(patches)==2:
            assert tuple(seed) in hard
            assert [x["key"] for x in reset]==[7,8,9]
            assert all(x["value"]==0 for x in reset)
            assert reset[-1]["step"]<patches[0]["step"]<patches[1]["step"]
            name=f"a{seed[0]}_b{seed[1]}_c{seed[2]}"
            path=Path(sys.argv[2])/f"b56_f1d1k_g3q_c1_p1_l3_i6n_original_{name}.jsonl"
            paid=[json.loads(row) for row in path.read_text().splitlines()
                if row.strip()]
            original_cil=[x for x in paid if x[0]=="cil"]
            native_reset=[x[1] for x in original_cil if x[2] in (156,158,160)]
            native_patch=[x[1] for x in original_cil if x[2] in (170,172)]
            assert native_reset==[x["step"] for x in reset]
            assert native_patch==[x["step"] for x in patches]
            assert len(native_reset)==3 and len(native_patch)==2
            assert not any(native_patch[0]<step<native_patch[1] for step in native_reset)
            missing.append({"seed":seed,"first_original_paid_patch":patches[0],
                "second_original_paid_patch":patches[1],
                "paid_resets_observed":len(reset),
                "paid_resets_required":6,
                "missing_second_witness_isolation":True})
    assert len(missing)==8 and {tuple(x["seed"]) for x in missing}==hard
    print(json.dumps({
        "study":NAME,"evidence_level":"NATIVE_SOURCE_PINNED_EXPECTED_NEGATIVE_PROTOCOL_FALSIFIER",
        "source_sha":actual["source_sha"],
        "original_native_hard_genotypes_with_proven_missing_reset":8,
        "missing_three_witness_reset_events":24,
        "false_promotion_allowed":False,
        "hard_cases":missing,
        "conclusion":"I6N native PID pays two edits but never resets flags after first patch before second. This violates preregistered per-edit evidence isolation; I6N is a negative, not a successful I6.",
        "corrected_variant":"I6R source 180 under same 16/180/300 limits",
    },indent=2))
if __name__=="__main__":
    main()
