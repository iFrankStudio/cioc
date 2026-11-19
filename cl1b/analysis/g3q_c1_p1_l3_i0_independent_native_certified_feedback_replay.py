#!/usr/bin/env python3
"""Independent original-native every-step CIL + interleaved feedback replay.

I0 is a freshly constructed researcher-authored 172-node original PID1.
The actor must itself pay for three certificate trials, write witness flags,
reject incomplete rule coverage, then (ONLY if certified) pay real Write11 task
services, read external feedback20 and adapt its middle-byte policy.

Native input is a read-only source-pinned event stream, never an instruction
or oracle. Semantic task truth stays in the external researcher harness.
"""
import json
import os
import sys
from pathlib import Path

MANIFEST=Path(__file__).resolve().parents[1]/"studies/B56-F1d1k-G3Q-C1-P1-L3-I0-frozen-single-PID-certified-feedback-founder-v1.json"
KEYS=(7,8,9,11,20,21,32,33,50,51,52,53,56,62,63,64)
NAME="B56-F1d1k-G3Q-C1-P1-L3-I0"

def run(source,arm,trace_path=None):
    regs=[0,0,0,0]
    cells={}
    pc=0
    paid=0
    cert_marks=[]
    services=[]
    meta_reads=0
    checkpoints=0
    paid_feedback_reads=[]
    cert_passes=0
    peak=len(source)+1
    def rows():
        if trace_path is None:
            while True:yield None
        else:
            with Path(trace_path).open() as stream:
                for line in stream:yield json.loads(line)
    it=rows()
    def check(expected):
        observed=next(it,None)
        if trace_path is not None and observed!=expected:
            raise AssertionError({"step":paid,"source_PC":expected[2] if len(expected)>2 else None,
                "native":observed,"independent":expected,"first_divergence":True})
    while paid<8000:
        op=source[pc]
        here=pc
        paid+=1
        kind=op["kind"]
        if kind=="Const":
            regs[op["dst"]]=op["value"]
            pc=op["next"]
        elif kind in ("Add","Sub"):
            a,b=regs[op["lhs"]],regs[op["rhs"]]
            regs[op["dst"]]=a+b if kind=="Add" else a-b
            pc=op["next"]
        elif kind=="Read":
            key=regs[op["address"]]
            value=cells.get(key,0)
            regs[op["dst"]]=value
            if here==37:
                meta_reads+=1
                assert key in (62,63,64)
            if here==79:
                assert key==20
                paid_feedback_reads.append(value)
            pc=op["next"]
        elif kind=="Write":
            key=regs[op["address"]]
            value=regs[op["value"]]
            cells[key]=value
            if here==58:
                checkpoints+=1
                assert key==56
            if here==130:
                assert key in (7,8,9) and value==1
                assert cells[21]==0
                cert_marks.append({"step":paid,"key":key,"value":value})
            if here==170:
                assert key==21 and value==1 and len(cert_marks)==3
                assert [cells.get(k) for k in (7,8,9)]==[1,1,1]
                cert_passes+=1
            pc=op["next"]
        elif kind=="Branch":
            pc=op["nonzero"] if regs[op["test"]] else op["zero"]
        elif kind=="Halt":
            assert here in (98,171)
        else:
            raise AssertionError(("unrecognized opcode",here,kind))
        peak=max(peak,len(source)+1+len(cells))
        assert len(cells)<=16 and peak<=300
        check(["cil",paid,here,pc,list(regs),8000-paid,
            [cells.get(k) for k in KEYS],len(source)+1+len(cells)])
        if here==71:
            assert cert_passes==1
            assert kind=="Write" and key==11 and 0<=value<=2
            episode=cells[32]
            assert episode<len(arm["goals"])
            target=arm["goals"][episode]
            truth=int(value==target)
            feedback=truth if arm["feedback"]=="truth" else 1-truth
            evt={"step":paid,"episode":episode,"middle":cells[63],
                "output":value,"truth":truth,"feedback":feedback}
            services.append(evt)
            cells[20]=feedback
            peak=max(peak,len(source)+1+len(cells))
            check(["feedback",paid,episode,target,cells[63],value,truth,feedback,
                  [cells.get(k) for k in KEYS],len(source)+1+len(cells)])
        if kind=="Halt":
            break
    else:
        raise AssertionError("PID1 exceeded finite 8,000-credit budget")
    if trace_path is not None:
        assert next(it,None) is None,"native trace contains records after paid original Halt"
    expected=arm["expected"]
    results={
        "paid_steps":paid,"credits_remaining":8000-paid,
        "halt_pc":pc,"occupied_peak":peak,"cells_used":len(cells),
        "certificate_flags":[cells.get(k) for k in (7,8,9)],
        "certificate_witness_events":cert_marks,
        "meta_fetches":meta_reads,"accumulator_checkpoints":checkpoints,
        "service_events":services,"feedback_reads":paid_feedback_reads,
        "success_claim":cells.get(21),"episodes_completed":cells.get(32),
        "final_active_middle":cells.get(63),
        "bank":[cells.get(k) for k in (51,52,53)]}
    for field,val in expected.items():
        if field in results and val!=results[field]:
            raise AssertionError(("source/frozen prereg divergence",arm["name"],field,results[field],val))
    assert len(services)==0 if cert_passes==0 else len(services)>0
    assert len(cert_marks)==3
    assert meta_reads==checkpoints==9+len(services)*3
    assert paid_feedback_reads==[x["feedback"] for x in services]
    return results,cert_passes

def main():
    frozen=json.loads(MANIFEST.read_text())
    code=frozen["exact_native_founder_code"]
    arms=frozen["prespecified_arms"]
    assert len(code)==172 and len(arms)==4
    native=None
    if len(sys.argv)==3:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==NAME
        assert native["level"]=="NATIVE_ONE_PID_STRUCTURAL_CERTIFICATION_AND_HIDDEN_TASK_FEEDBACK_RELEARNING"
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
        assert len(native["arms"])==4
        directory=Path(sys.argv[2])
    elif len(sys.argv)!=1:
        raise SystemExit("usage: python3 I0_NATIVE_REPLAY.py [SUMMARY.json NATIVE_TRACE_DIR]")
    reports=[]
    total=0
    for i,arm in enumerate(arms):
        source=[dict(node) for node in code]
        m=arm["mutation"]
        if m:
            pc=m["pc"]
            assert pc in (3,6) and source[pc]["value"]==m["value_before"]
            source[pc]["value"]=m["value_after"]
        trace=(directory/f"b56_f1d1k_g3q_c1_p1_l3_i0_native_paid_{arm['name']}.jsonl") if native else None
        result,certified=run(source,arm,trace)
        total+=result["paid_steps"]
        if native:
            actual=native["arms"][i]
            assert actual["arm"]==arm["name"]
            assert actual["paid_steps"]==result["paid_steps"]
            assert actual["credits_remaining"]==result["credits_remaining"]
            assert actual["source_nodes"]==172
            assert actual["initial_bank"]==result["bank"]
            assert actual["certificate_marks"]==result["certificate_witness_events"]
            assert actual["certificate_passes"]==certified
            assert actual["task_services"]==result["service_events"]
            assert actual["paid_meta_fetches"]==result["meta_fetches"]
            assert actual["paid_accumulator_checkpoints"]==result["accumulator_checkpoints"]
            assert actual["paid_feedback_reads"]==result["feedback_reads"]
            assert actual["environment_feedback_writes"]==[x["feedback"] for x in result["service_events"]]
            assert actual["final_halt_pc"]==result["halt_pc"]
            assert actual["native_original_pid_halted"] is True
            assert actual["peak_occupied"]==result["occupied_peak"]
            assert actual["postload_external_rule_or_source_writes"]==0
        reports.append({"arm":arm["name"],"original_paid_steps":result["paid_steps"],
            "actor_certified":certified==1,"real_task_services":len(result["service_events"]),
            "true_task_matches":sum(ev["truth"] for ev in result["service_events"]),
            "claimed_success":result["success_claim"],
            "native_first_divergence":None})
    assert [x["original_paid_steps"] for x in reports]==[1357,1109,362,869]
    assert [x["actor_certified"] for x in reports]==[True,True,False,True]
    assert reports[-1]["claimed_success"]==1 and reports[-1]["true_task_matches"]==0
    print(json.dumps({"study":NAME,
        "evidence_level":"EXACT_NATIVE_EVERY_PID_STEP_AND_FEEDBACK_REPLAY" if native else "INDEPENDENT_FROZEN_SOURCE_PREDICTION",
        "source_sha":native["source_sha"] if native else None,
        "independently_verified_original_paid_steps":total if native else 0,
        "arms":reports,"first_divergence":None,
        "limits":"One PID certifies authored safe grammar but did not create descriptors by in-world rule search; deceptive feedback still yields false success."},indent=2))

if __name__=="__main__":main()
