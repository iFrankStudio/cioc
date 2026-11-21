//! CL1B-REFILL-GATE-G1: prospective 184 strict-capacity, original-native
//! worlds comparing researcher-authored, actor-paid r3-conditional refill
//! with the same 180-node G1 architecture always/never baselines.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::{BufWriter, Write};

const STUDY: &str = "../../../cl1b/studies/CL1B-REFILL-GATE-G1-frozen-180-node-native-184-worlds-v1.json";
const CELLS: [i64; 12] = [11, 20, 21, 30, 32, 33, 34, 35, 36, 37, 38, 39];
const CREDIT: u64 = 6000;

fn decode(v: &Value) -> Op {
    let kind = v["kind"].as_str().unwrap();
    let obj = if kind == "Halt" { json!("Halt") } else {
        let mut props = v.as_object().unwrap().clone();
        props.remove("kind");
        let mut outer = serde_json::Map::new();
        outer.insert(kind.into(), Value::Object(props));
        Value::Object(outer)
    };
    serde_json::from_value(obj).expect("frozen valid original CIL opcode")
}
fn edges(w: &World) -> [i64; 4] {
    let p = w.process(1).unwrap();
    let next = |n: usize| match &p.code.nodes[n] {
        Op::Write {next, ..} => *next as i64,
        x => panic!("PC{n} not Write: {x:?}")
    };
    let stride = match &p.code.nodes[46] {
        Op::Const {value, ..} => *value,
        x => panic!("PC46 not Const: {x:?}")
    };
    [next(12),next(34),stride,next(133)]
}
fn src<'a>(m: &'a Value, name: &str) -> &'a Vec<Value> {
    let key = match name {
        "F6_parent" => "source_f6_original",
        "G1_conditional" => "source_g1_conditional",
        "G1_always" => "source_g1_always",
        "G1_never" => "source_g1_never",
        other => panic!("unrecognized source {other}"),
    };
    m[key].as_array().unwrap()
}
fn run(m: &Value, arm: &Value, output: &std::path::Path) -> Value {
    let name = arm["name"].as_str().unwrap();
    let source = arm["source"].as_str().unwrap();
    let goals = arm["goals"].as_array().unwrap().iter()
        .map(|v| v.as_i64().unwrap()).collect::<Vec<_>>();
    assert_eq!(goals.len(),24);
    assert!(goals.iter().all(|x| (0..=2).contains(x)));
    let mode = arm["feedback"].as_str().unwrap();
    assert!(matches!(mode,"truth"|"inverted"|"forced0"));
    let program = src(m,source).iter().map(decode).collect::<Vec<_>>();
    assert_eq!(program.len(),180);
    let mut w = World::with_strict_capacity(16,180,300);
    w.load(Process::new(1,program,CREDIT)).unwrap();
    assert_eq!(w.occupied_units(),181);
    let file = File::create(output.join(format!("cl1b_refill_gate_g1_original_{name}.jsonl"))).unwrap();
    let mut trace = BufWriter::new(file);
    let mut paid=0_u64;
    let mut peak=181_usize;
    let mut services=Vec::<Value>::new();
    let mut decisions=Vec::<Value>::new();
    let mut refills=Vec::<Value>::new();
    let mut patches=Vec::<Value>::new();
    while w.step() {
        paid+=1;
        assert!(paid<=CREDIT);
        let ev=w.events().last().unwrap().clone();
        assert!(matches!(ev.origin,Origin::Internal(1)));
        let pc=ev.pc;
        let pid=w.process(1).unwrap();
        assert_eq!(pid.credits,CREDIT-paid);
        assert_eq!(w.total_compute_inventory(),Some(CREDIT-paid));
        assert_eq!(pid.code.nodes.len(),180);
        assert!(w.used_cells()<=16);
        assert!(w.occupied_units()<=300);
        assert!(w.process(2).is_none());
        peak=peak.max(w.occupied_units());
        if source.starts_with("G1_") && pc==173 {
            assert!(matches!(&ev.kind,EventKind::Executed));
            let guess=pid.registers[3];
            let jump=pid.pc;
            let expected=match source {
                "G1_conditional"=>if guess==0{176}else{174},
                "G1_always"=>174,
                "G1_never"=>176,
                _=>unreachable!()
            };
            assert_eq!(jump,expected,"paid actor-only gate decision");
            decisions.push(json!({"step":paid,"guess":guess,"next":jump}));
        }
        if (source=="F6_parent" && pc==174) || (source.starts_with("G1_") && pc==175) {
            assert!(matches!(&ev.kind,EventKind::CellWritten{key:38,value:0}));
            assert_eq!(w.cell(38),Some(0));
            refills.push(json!({"step":paid,"pc":pc}));
        }
        if [45,88,102,122,82,131,136,139,155,177,178].contains(&pc) {
            if matches!(&ev.kind,EventKind::Executed) && ((source=="F6_parent" && pc!=178) || (source.starts_with("G1_") && pc!=177)) {
                patches.push(json!({"step":paid,"pc":pc}));
            }
        }
        serde_json::to_writer(&mut trace,&json!(["cil",paid,pc,pid.pc,pid.registers,
            pid.credits,CELLS.map(|key|w.cell(key)),w.occupied_units(),edges(&w)])).unwrap();
        trace.write_all(b"\n").unwrap();
        if pc==3 {
            let guess=match &ev.kind {
                EventKind::CellWritten{key:11,value}=>*value,
                x=>panic!("original task service missing Write11: {x:?}")
            };
            let stage=w.cell(32).unwrap_or(0) as usize;
            assert!(stage<24,"PC3 emitted after 24 task stages");
            let truth=i64::from(guess==goals[stage]);
            let feedback=match mode {
                "truth"=>truth,
                "inverted"=>1-truth,
                "forced0"=>0,
                _=>unreachable!()
            };
            services.push(json!({"step":paid,"stage":stage,"guess":guess,"truth":truth,"feedback":feedback}));
            w.intervene(Intervention::SetCell{key:20,value:feedback}).unwrap();
            assert!(matches!(w.events().last().unwrap().origin,Origin::External));
            peak=peak.max(w.occupied_units());
            serde_json::to_writer(&mut trace,&json!(["feedback",paid,stage,goals[stage],guess,
                truth,feedback,CELLS.map(|key|w.cell(key)),w.occupied_units(),edges(&w)])).unwrap();
            trace.write_all(b"\n").unwrap();
        }
    }
    trace.flush().unwrap();
    let pid=w.process(1).unwrap();
    assert!(pid.stopped,"process did not finish within original paid compute");
    assert_eq!(pid.credits,CREDIT-paid);
    assert!(paid<CREDIT);
    let truthful=services.iter().filter(|e|e["truth"]==1).count();
    let native_completed=w.cell(32).unwrap_or(0);
    if mode=="truth" {
        assert_eq!(native_completed,24,"true task completion lost");
        assert_eq!(truthful,24,"inconsistent true completion");
        assert_eq!(w.cell(21),Some(1),"false native claim");
    }
    json!({"name":name,"source":source,"regime":arm["regime"],"feedback":mode,"goals":goals,
        "steps":paid,"services":services,"decisions":decisions,"refills":refills,
        "patches":patches,"completed_stages":native_completed,
        "truthful_matches":truthful,"claim":w.cell(21).unwrap_or(0),
        "final_edges":edges(&w),"peak_occupied":peak,"host_source_edits":0})
}
#[test]
fn frozen_g1_184_worlds_original_native_paid_gate() {
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let m:Value=serde_json::from_slice(&fs::read(root.join(STUDY)).unwrap()).unwrap();
    assert_eq!(m["study"],"CL1B-REFILL-GATE-G1");
    assert_eq!(m["status"],"PRE_NATIVE_FROZEN");
    assert_eq!(m["counts"]["total_native_worlds"],184);
    let names=["F6_parent","G1_conditional","G1_always","G1_never"];
    for name in names {assert_eq!(src(&m,name).len(),180);}
    let parent:Value=serde_json::from_slice(&fs::read(root.join(
        "../../../cl1b/studies/CL1B-REFILL-CAUSAL-R1-frozen-source-identical-pc174-ablation-472-worlds.json")).unwrap()).unwrap();
    assert_eq!(m["source_f6_original"],parent["original_loaded_CIL_source_f6"],"F6 must be unchanged");
    for name in ["G1_conditional","G1_always","G1_never"] {
        for (pc,(a,b)) in src(&m,name).iter().zip(src(&m,"F6_parent")).enumerate() {
            if !(171..=179).contains(&pc){assert_eq!(a,b,"unexpected source change at PC{pc}");}
        }
    }
    for a in ["G1_conditional","G1_always","G1_never"] {
        for b in ["G1_conditional","G1_always","G1_never"] {
            for (pc,(x,y)) in src(&m,a).iter().zip(src(&m,b)).enumerate() {
                if pc!=173 {assert_eq!(x,y,"G1 variants differ outside PC173");}
            }
        }
    }
    let dir=root.join("../../cl1b-ci-evidence");
    fs::create_dir_all(&dir).unwrap();
    let arms=m["arms"].as_array().unwrap();
    let controls=m["controls"].as_array().unwrap();
    assert_eq!(arms.len(),180);
    assert_eq!(controls.len(),4);
    let mut records=Vec::<Value>::new();
    let mut extra=Vec::<Value>::new();
    let mut total_paid=0_u64;
    for arm in arms {
        let result=run(&m,arm,&dir);
        total_paid+=result["steps"].as_u64().unwrap();
        records.push(result);
    }
    for arm in controls {
        let result=run(&m,arm,&dir);
        total_paid+=result["steps"].as_u64().unwrap();
        extra.push(result);
    }
    let conditional_decisions=records.iter().filter(|r|r["source"]=="G1_conditional")
        .flat_map(|r|r["decisions"].as_array().unwrap().iter()).collect::<Vec<_>>();
    assert!(conditional_decisions.iter().any(|e|e["next"]==174)
        &&conditional_decisions.iter().any(|e|e["next"]==176),
        "G1 did not exercise both actor-conditional paths");
    let report=json!({"study":"CL1B-REFILL-GATE-G1","status":"ORIGINAL_NATIVE_PROSPECTIVE",
        "frozen_manifest":"cl1b/studies/CL1B-REFILL-GATE-G1-frozen-180-node-native-184-worlds-v1.json",
        "worlds":records.len()+extra.len(),"total_private_paid_cil":total_paid,
        "truthful_records":records,"adversarial_records":extra,
        "actor_conditional_gates":conditional_decisions.len()});
    fs::write(dir.join("cl1b_refill_gate_g1_native_184_worlds.json"),
        serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    println!("CL1B G1 native 184 worlds; original fully charged CIL={total_paid}");
}
