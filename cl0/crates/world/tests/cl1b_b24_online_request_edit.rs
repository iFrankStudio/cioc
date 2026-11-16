//! B24 — eight-opcode CIL policy literal is computed and patched in-world.
//! The subtraction and unconditional edit schedule are human-authored.
//! Native experiment pays for every attempted CIL instruction and every
//! identity-neutral environmental packet; no host reward or actor top-up.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug)]
enum Arm { Enabled, Disabled, StaticThree }

fn founder(initial: i64) -> Vec<Op> {
    vec![
        Op::Const { dst: 1, value: 3, next: 1 },
        Op::Const { dst: 2, value: 9, next: 2 },
        Op::Const { dst: 3, value: 1, next: 3 },
        Op::Const { dst: 0, value: initial, next: 4 },
        Op::DrawCompute { amount: 0, next: 5 },
        Op::Write { address: 2, value: 3, next: 6 },
        Op::Sub { dst: 0, lhs: 0, rhs: 3, next: 7 },
        Op::PatchConst { node: 1, value: 0, next: 3 },
    ]
}

#[derive(Debug)]
struct Trial {
    arm: Arm,
    schedule: Vec<u64>,
    service_writes: usize,
    policy_changes: Vec<(i64,i64)>,
    attempted_patches: usize,
    suppressed: usize,
    pc: usize,
    literal: i64,
    final_pool: u64,
    final_local: u64,
    charged_attempts: usize,
}

fn literal(w: &World) -> i64 {
    match &w.process(1).unwrap().code.nodes[3] {
        Op::Const { value, .. } => *value,
        _ => panic!("B24 request source literal lost"),
    }
}
fn run_to_blocked(w: &mut World, changes: &mut Vec<(i64,i64)>, edits: &mut usize) -> usize {
    let mut n = 0;
    loop {
        assert!(n < 200, "finite resource packets must not permit indefinite native activity");
        let before = w.total_compute_inventory().unwrap();
        let events_before = w.events().len();
        let lit_before = literal(w);
        let pc_before = w.process(1).unwrap().pc;
        if !w.step() {
            assert_eq!(w.total_compute_inventory(), Some(before));
            assert_eq!(w.events().len(), events_before);
            break;
        }
        n += 1;
        assert_eq!(w.total_compute_inventory(), Some(before - 1),
            "native compute cannot appear from CIL policy editing");
        assert_eq!(w.events().len(), events_before + 1);
        assert_eq!(w.events().last().unwrap().origin, Origin::Internal(1));
        if pc_before == 7 {
            *edits += 1;
            if literal(w) != lit_before {
                changes.push((lit_before, literal(w)));
            }
        } else {
            assert_eq!(literal(w), lit_before,
                "request code changes must be sourced solely to native PC7");
        }
    }
    n
}

fn test_case(arm: Arm, schedule: &[u64]) -> Trial {
    let initial = if matches!(arm, Arm::StaticThree) { 3 } else { 4 };
    let mut w = World::with_strict_capacity(16, 64, 128)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    if !matches!(arm, Arm::Enabled) {
        w = w.with_native_code_edits_disabled();
    }
    w.load(Process::new(1, founder(initial), 29)).unwrap();
    let mut changes=Vec::new();
    let mut edits=0;
    let startup = run_to_blocked(&mut w, &mut changes, &mut edits);
    assert_eq!(startup, 29);
    assert!(changes.is_empty());
    assert_eq!(w.process(1).unwrap().pc, 4);
    assert_eq!(w.process(1).unwrap().credits, 0);
    assert_eq!(w.total_compute_inventory(), Some(0));
    let mut attempts=startup;
    for &packet in schedule {
        let before = w.total_compute_inventory().unwrap();
        let pool = w.available_compute_pool();
        w.intervene(Intervention::SetComputePool { credits: pool + packet }).unwrap();
        assert_eq!(w.total_compute_inventory(), Some(before + packet));
        assert_eq!(w.events().last().unwrap().origin, Origin::External);
        attempts += run_to_blocked(&mut w, &mut changes, &mut edits);
    }
    let service_writes = w.events().iter().filter(|e| matches!(
        (&e.origin, &e.kind),
        (Origin::Internal(1), EventKind::CellWritten { key:9, value:1 }) if e.pc == 5
    )).count();
    let suppressed = w.events().iter().filter(|e| matches!(
        (&e.origin, &e.kind),
        (Origin::Internal(1), EventKind::NativeCodeEditSuppressed { opcode_tag:9 }) if e.pc == 7
    )).count();
    assert_eq!(w.events().iter().filter(|e| e.origin==Origin::External).count(),
        1+schedule.len(), "one founder load, then only neutral external pool packets");
    assert_eq!(w.total_compute_inventory(), Some(29 + schedule.iter().sum::<u64>() - attempts as u64));
    assert_eq!(w.process(2).is_none(), true);
    assert_eq!(w.occupied_units() <= 128, true);
    assert!(w.events().iter().all(|e| !matches!(
        e.kind, EventKind::Trapped(_) | EventKind::Halted | EventKind::GraphLaunched { .. }
    )));
    let output=Trial{
        arm,schedule:schedule.to_vec(),service_writes,policy_changes:changes,
        attempted_patches:edits,suppressed,pc:w.process(1).unwrap().pc,
        literal:literal(&w),final_pool:w.available_compute_pool(),
        final_local:w.process(1).unwrap().credits,charged_attempts:attempts,
    };
    eprintln!("B24_NATIVE {:?}",output);
    output
}

#[test]
fn b24_native_runtime_derived_request_edit_can_help_then_deadlock_without_resource_observability() {
    let schedules: &[&[u64]] = &[
        &[5,4,4], // development
        &[5,5,5], // heldout H1
        &[4,4,4], // heldout H2
        &[3,3,3,3], // heldout H3
    ];
    for schedule in schedules {
        let enabled = test_case(Arm::Enabled,schedule);
        let disabled = test_case(Arm::Disabled,schedule);
        let static_three = test_case(Arm::StaticThree,schedule);
        assert_eq!(enabled.schedule,disabled.schedule);
        assert_eq!(enabled.schedule,static_three.schedule);
        assert_eq!(enabled.policy_changes.first(),Some(&(4,3)),
            "CIL must truly compute and native-patch its own next resource request");
        assert!(enabled.attempted_patches>=1);
        assert_eq!(enabled.suppressed,0);
        assert_eq!(disabled.literal,4);
        assert!(disabled.policy_changes.is_empty());
        assert_eq!(disabled.suppressed, disabled.attempted_patches);
        assert_eq!(static_three.literal,3);
        assert_eq!(static_three.suppressed,static_three.attempted_patches);
        assert_eq!(static_three.policy_changes.len(),0);
        assert_eq!(enabled.final_local,0);
        assert_eq!(disabled.final_local,0);
        assert_eq!(static_three.final_local,0);
        assert!(enabled.service_writes>0,"all tested schedules permit at least one paid service");
        // Primary hypothesis: a low request plus policy-edit cost can strand
        // the actor at non-Draw PC3, permanently barring additional work.
        assert_eq!(enabled.pc,3,"actor gets stuck at a normal Const, not at a pool-payable Draw");
        assert_eq!(static_three.pc,3,"static3 has the same postdraw compute debt");
    }
}
