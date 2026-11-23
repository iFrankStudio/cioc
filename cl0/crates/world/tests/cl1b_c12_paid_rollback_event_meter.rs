//! C12: original native event-triggered paid rollback-attempt meter; frozen C10 task corpus.
//! Engineering cost calibration only; the C10 rollback policy is NOT learned.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::{BufWriter, Write};

const MANIFEST: &str =
    "../../../cl1b/studies/CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10-frozen-274-worlds-v1.json";
const CREDIT: u64 = 6000;
const CELLS: [i64; 16] = [11, 20, 21, 30, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43];

fn decode(v: &Value) -> Op {
    let tag = v["kind"].as_str().unwrap();
    let raw = if tag == "Halt" {
        json!("Halt")
    } else {
        let mut map = v.as_object().unwrap().clone();
        map.remove("kind");
        let mut obj = serde_json::Map::new();
        obj.insert(tag.to_string(), Value::Object(map));
        Value::Object(obj)
    };
    serde_json::from_value(raw).expect("frozen C10 source opcode")
}

fn source(m: &Value, metered: bool) -> Vec<Op> {
    let mut nodes = m["frozen_source_g2_rollback"]
        .as_array().unwrap().iter().map(decode).collect::<Vec<_>>();
    assert_eq!(nodes.len(), 180);
    if metered {
        assert_eq!(nodes[95], Op::Branch { test: 0, nonzero: 99, zero: 86 });
        nodes[95] = Op::Branch { test: 0, nonzero: 180, zero: 86 };
        let extension = [
            json!({"kind":"Const","dst":2,"value":43,"next":181}),
            json!({"kind":"Read","dst":0,"address":2,"next":182}),
            json!({"kind":"Const","dst":2,"value":1,"next":183}),
            json!({"kind":"Add","dst":0,"lhs":0,"rhs":2,"next":184}),
            json!({"kind":"Const","dst":2,"value":43,"next":185}),
            json!({"kind":"Write","address":2,"value":0,"next":99})
        ];
        nodes.extend(extension.iter().map(decode));
        assert_eq!(nodes.len(), 186);
    }
    nodes
}

fn theta(nodes: &[Op]) -> i64 {
    match &nodes[173] {
        Op::Const { dst: 2, value, .. } => *value,
        x => panic!("not PC173 Const: {x:?}"),
    }
}

fn signature(nodes: &[Op]) -> [i64; 5] {
    let edge = |i: usize| match &nodes[i] {
        Op::Write { next, .. } => *next as i64,
        x => panic!("not signature Write {i}: {x:?}"),
    };
    let stride = match &nodes[46] {
        Op::Const { value, .. } => *value,
        x => panic!("not PC46 Const {x:?}"),
    };
    [edge(12), edge(34), stride, edge(133), theta(nodes)]
}

struct Trial {
    summary: Value,
    path: Vec<usize>,
    services: Vec<Value>,
    edits: Vec<Value>,
    paid: u64,
}

fn run(
    m: &Value,
    name: &str,
    goals: &[i64],
    metered: bool,
    suppress_restore: bool,
    mode: &str,
    dir: &std::path::Path,
) -> Trial {
    assert_eq!(goals.len(), 48);
    assert!(goals.iter().all(|&g| (0..=2).contains(&g)));
    assert!(matches!(mode, "truth" | "inverted" | "forced0"));
    let code = source(m, metered);
    let mut expected = code.clone();
    let mut world = World::with_strict_capacity(16, 186, 300);
    if suppress_restore {
        world = world.with_native_patchconst_suppression(1, 102, 173);
    }
    world.load(Process::new(1, code, CREDIT)).unwrap();
    assert_eq!(world.occupied_units(), if metered { 187 } else { 181 });
    let trace_path = dir.join(format!("cl1b_c12_original_{name}.jsonl"));
    let mut trace = BufWriter::new(File::create(trace_path).unwrap());
    let mut paid = 0_u64;
    let mut peak = world.occupied_units();
    let mut path = Vec::new();
    let mut services = Vec::new();
    let mut edits = Vec::new();
    let mut paid_counter_writes = 0_u64;
    let mut rollback_attempts = 0_u64;
    let mut physical_reverts = 0_u64;

    while world.step() {
        paid += 1;
        assert!(paid <= CREDIT, "{name} exhausted its private budget");
        let e = world.events().last().unwrap().clone();
        assert_eq!(e.origin, Origin::Internal(1));
        let pc = e.pc;
        let pid = world.process(1).unwrap();
        assert_eq!(pid.credits, CREDIT - paid);
        assert_eq!(world.total_compute_inventory(), Some(CREDIT - paid));
        assert!(world.process(2).is_none());
        assert!(world.used_cells() <= 16);
        assert!(world.occupied_units() <= 300);
        assert_eq!(pid.code.nodes.len(), if metered { 186 } else { 180 });
        if pc < 180 { path.push(pc); }
        if matches!(pc, 45 | 88 | 102 | 122 | 28) {
            let suppressed = pc == 102 && suppress_restore;
            let target = pid.registers[1] as usize;
            let value = if pc == 28 { pid.registers[3] } else { pid.registers[0] };
            let before = match &expected[target] {
                Op::Const { value, .. } => *value,
                x => panic!("expected actual Const patch target: {x:?}"),
            };
            if !suppressed {
                if let Op::Const { value: old, .. } = &mut expected[target] {
                    *old = value;
                }
                assert!(matches!(e.kind, EventKind::Executed));
            } else {
                assert!(matches!(e.kind, EventKind::NativeCodeEditSuppressed { opcode_tag: 9 }));
            }
            if pc == 102 {
                assert_eq!(target, 173);
                rollback_attempts += 1;
                if !suppressed && before != value { physical_reverts += 1; }
            }
            edits.push(json!([pc, target, before, value, suppressed]));
        }
        if matches!(pc, 26 | 82 | 131 | 136 | 139 | 155) {
            let target = pid.registers[1] as usize;
            let new_next = pid.registers[0] as usize;
            if let Op::Write { next, .. } = &mut expected[target] {
                edits.push(json!([pc, target, *next, new_next, false]));
                *next = new_next;
            } else {
                panic!("expected Write PatchNext destination");
            }
        }
        assert_eq!(pid.code.nodes, expected, "unpaid source mutation in {name} at paid={paid}");
        if pc == 185 {
            assert!(metered);
            paid_counter_writes += 1;
            assert!(matches!(&e.kind, EventKind::CellWritten { key: 43, value }
                if *value == (rollback_attempts + 1) as i64));
            assert_eq!(world.cell(43), Some((rollback_attempts + 1) as i64));
        }
        peak = peak.max(world.occupied_units());
        serde_json::to_writer(&mut trace, &json!([
            "cil", paid, pc, pid.pc, pid.registers, pid.credits,
            CELLS.map(|k| world.cell(k)), world.occupied_units(),
            signature(&pid.code.nodes),
            if pc == 102 && suppress_restore { "suppressed_pc102" }
            else if pc == 185 { "paid_meter_write" } else { "normal" }
        ])).unwrap();
        trace.write_all(b"\n").unwrap();

        if pc == 3 {
            let guess = match &e.kind {
                EventKind::CellWritten { key: 11, value } => *value,
                x => panic!("not a native Write11: {x:?}"),
            };
            let stage = world.cell(32).unwrap_or(0) as usize;
            assert!(stage < 48);
            let truth = i64::from(guess == goals[stage]);
            let feedback = match mode {
                "truth" => truth, "inverted" => 1 - truth, "forced0" => 0,
                _ => unreachable!(),
            };
            services.push(json!([stage, guess, truth, feedback]));
            world.intervene(Intervention::SetCell { key: 20, value: feedback }).unwrap();
            assert!(matches!(world.events().last().unwrap().origin, Origin::External));
            peak = peak.max(world.occupied_units());
            serde_json::to_writer(&mut trace, &json!([
                "feedback", paid, stage, goals[stage], guess, truth, feedback,
                CELLS.map(|k| world.cell(k)), world.occupied_units(),
                signature(&world.process(1).unwrap().code.nodes)
            ])).unwrap();
            trace.write_all(b"\n").unwrap();
        }
    }
    trace.flush().unwrap();
    assert_eq!(paid_counter_writes, if metered { rollback_attempts } else { 0 });
    assert_eq!(world.cell(43), if metered && rollback_attempts > 0 { Some(rollback_attempts as i64) } else { None });
    let successes = services.iter().filter(|x| x[2] == 1).count();
    let completed = world.cell(32).unwrap_or(0);
    let claim = world.cell(21).unwrap_or(0);
    if mode == "truth" {
        assert_eq!((completed, successes as i64, claim), (48, 48, 1), "{name}");
    } else if mode == "forced0" {
        assert_eq!((completed, claim), (0, 0), "{name}");
    } else {
        assert_eq!(claim, 1, "{name} inverted control failed false-claim probe");
        assert!(successes < services.len());
    }
    let summary = json!({
        "name": name, "metered": metered,
        "suppress_restore": suppress_restore, "feedback": mode,
        "goals": goals, "paid": paid, "service_attempts": services.len(),
        "meter_writes": paid_counter_writes, "meter_value": world.cell(43),
        "completed_stages": completed, "true_matches": successes, "claim": claim,
        "rollback_attempts": rollback_attempts,
        "physical_value_changes_at_pc102": physical_reverts,
        "peak_occupied": peak, "final_signature": signature(&world.process(1).unwrap().code.nodes)
    });
    Trial { summary, path, services, edits, paid }
}

#[test]
fn c12_182_original_worlds_paid_cil_service_meter_economic_calibration() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let m: Value = serde_json::from_slice(&fs::read(root.join(MANIFEST)).unwrap()).unwrap();
    assert_eq!(m["study"], "CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10");
    assert_eq!(m["cases"].as_array().unwrap().len(), 45);
    let dir = root.join("../../cl1b-ci-evidence");
    fs::create_dir_all(&dir).unwrap();
    let mut records = Vec::new();
    let mut savings_metered = 0_i64;
    let mut savings_reference = 0_i64;
    let mut total_meter_fees = 0_u64;
    let mut total_native_paid = 0_u64;

    for (case_idx, case) in m["cases"].as_array().unwrap().iter().enumerate() {
        let goals = case["goals"].as_array().unwrap()
            .iter().map(|v| v.as_i64().unwrap()).collect::<Vec<_>>();
        let base = case["name"].as_str().unwrap();
        let mut row = Vec::new();
        for (label, metered, suppress) in [
            ("metered_live", true, false),
            ("metered_pc102_suppressed", true, true),
            ("reference_live", false, false),
            ("reference_pc102_suppressed", false, true),
        ] {
            let name = format!("{base}_{label}");
            let trial = run(&m, &name, &goals, metered, suppress, "truth", &dir);
            total_native_paid += trial.paid;
            row.push(trial);
        }
        for (a, b) in [(0, 2), (1, 3)] {
            assert_eq!(row[a].path, row[b].path, "{base} meter changed old source PC sequence");
            assert_eq!(row[a].services, row[b].services, "{base} meter changed real feedback history");
            assert_eq!(row[a].edits, row[b].edits, "{base} meter changed physical source edits");
            assert_eq!(row[a].paid, row[b].paid + 6 * row[b].summary["rollback_attempts"].as_u64().unwrap(),
                "{base} meter fee mismatch");
            assert_eq!(row[a].summary["final_signature"], row[b].summary["final_signature"]);
            total_meter_fees += 6 * row[b].summary["rollback_attempts"].as_u64().unwrap();
        }
        let d_meter = row[1].paid as i64 - row[0].paid as i64;
        let d_ref = row[3].paid as i64 - row[2].paid as i64;
        savings_metered += d_meter;
        savings_reference += d_ref;
        records.push(json!({
            "case": base, "regime": case["regime"],
            "metered_live": row[0].summary, "metered_suppressed": row[1].summary,
            "reference_live": row[2].summary, "reference_suppressed": row[3].summary,
            "metered_same_source_saving": d_meter, "reference_same_source_saving": d_ref
        }));
        assert_eq!(case_idx + 1, records.len());
    }
    let controls = m["controls"].as_array().unwrap();
    let mut adversarial = Vec::new();
    for control in [ &controls[0], &controls[2] ] {
        let goals = control["goals"].as_array().unwrap()
            .iter().map(|v| v.as_i64().unwrap()).collect::<Vec<_>>();
        let mode = control["feedback"].as_str().unwrap();
        let name = format!("c12_{}", mode);
        let trial = run(&m, &name, &goals, true, false, mode, &dir);
        total_native_paid += trial.paid;
        adversarial.push(trial.summary);
    }
    let result = json!({
        "study": "CL1B-C12-PAID-ROLLBACK-EVENT-METER",
        "status": "ORIGINAL_NATIVE_PAID_METER_CALIBRATION_NOT_ADAPTATION",
        "original_c10_goal_corpus": MANIFEST,
        "native_worlds": 182, "truthful_worlds": 180, "deceptive_worlds": 2,
        "total_native_private_cil": total_native_paid,
        "total_meter_fee_across_truthful_metered_arms": total_meter_fees,
        "reference_same_source_PC102_effect_net_saving": savings_reference,
        "metered_same_source_PC102_effect_net_saving": savings_metered,
        "original_c10_published_reference_saving": 1840,
        "all_cases": records, "deceptive_records": adversarial
    });
    fs::write(dir.join("cl1b_c12_paid_meter_native_182_worlds.json"),
        serde_json::to_vec_pretty(&result).unwrap()).unwrap();
    assert_eq!(savings_reference, 1840, "published C10 source-reference mismatch");
    println!("C12 ORIGINAL NATIVE: worlds=182 private_CIL={total_native_paid} paid_telemetry_fee={total_meter_fees} C10_ref_saving={savings_reference} C12_metered_saving={savings_metered} all_45_cases_completed");
}
