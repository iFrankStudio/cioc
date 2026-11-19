#!/usr/bin/env python3
"""Same-SHA P0 native summary vs second frozen-source CIL interpreter.

This checks the native Rust summary only. It is NOT event-by-event native replay.
"""
import json
import os
import sys
from pathlib import Path
from g3q_c1_p0_independent_source_replay import ROOT, SOURCE, ANCESTOR, replay

def main():
    if len(sys.argv) != 2:
        raise SystemExit("usage: python3 g3q_c1_p0_native_summary_crosscheck.py NATIVE_SUMMARY.json")
    artifact = Path(sys.argv[1])
    native = json.loads(artifact.read_text())
    src = json.loads(SOURCE.read_text())["exact_native_founder_code"]
    old = json.loads(ANCESTOR.read_text())["exact_native_founder_code"]
    assert len(src) == 180 and src[:163] == old[:163]
    expected = replay(src)
    # Prevent a valid summary from another run/SHA being relabelled as this CI run.
    # GITHUB_SHA is supplied by GitHub Actions; absence is a verification failure.
    expected_sha = os.environ["GITHUB_SHA"]
    assert len(expected_sha) == 40 and all(c in "0123456789abcdef" for c in expected_sha)
    assert native["github_sha"] == expected_sha, ("source SHA mismatch", native["github_sha"], expected_sha)
    assert native["study"] == "B56-F1d1k-G3Q-C1-P0"
    assert native["level"] == "NATIVE_NARROW_DEPLOYMENT_BRIDGE_NOT_TASK_SERVICE"
    assert native["source_nodes"] == 180
    assert native["original_pid"] == 1
    assert native["credits_start"] == 32000
    assert native["charged_steps"] == expected["steps"] == 24066
    assert native["credits_remain"] == expected["credits_remaining"] == 7934
    assert native["certificate_writes"] == expected["pcs"][106] == 64
    assert native["paid_retention_writes"] == sum(expected["pcs"][pc] for pc in (113, 117, 121)) == 72
    assert native["bank_reads"] == expected["bank_reads"] == [[51,1],[52,3],[53,2]]
    assert native["active_writes"] == expected["active_writes"] == [[62,1],[63,3],[64,2]]
    assert native["diagnostic_output"] == expected["cells"][11] == 2
    assert native["peak_occupied"] <= 197
    assert native["restarts"] == 0
    assert native["postload_external_cell_interventions"] == 0
    assert len(expected["cells"]) == 16
    # Mutated founder is researcher-authored pre-load control, not actor self-edit.
    altered = [dict(op) for op in src]
    altered[163]["value"] = 52
    mutant = replay(altered)
    assert mutant["steps"] == expected["steps"]
    assert mutant["active_writes"] == [[62,3],[63,3],[64,2]]
    print(json.dumps({
      "study":"B56-F1d1k-G3Q-C1-P0",
      "level":"NATIVE_SUMMARY_CROSSCHECK_NOT_INDEPENDENT_EVENT_REPLAY",
      "source_frozen_nodes":len(src),
      "native_source_sha":native["github_sha"],
      "native_charged_steps":native["charged_steps"],
      "source_replay_charged_steps":expected["steps"],
      "native_bank_reads":native["bank_reads"],
      "native_active_writes":native["active_writes"],
      "source_mutant_active_writes":mutant["active_writes"],
      "first_checked_field_mismatch":None,
      "limits":"Tests native Rust aggregate summary, not individual native events or hidden task success."
    },indent=2))

if __name__ == "__main__":
    main()
