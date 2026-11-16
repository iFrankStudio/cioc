//! CL-1B B27: a 12-opcode CIL reproducer copies its WHOLE OWN EXECUTABLE
//! into native graph storage, then launches a child that independently
//! performs the same self-copy operation on later neutral energy packets.
//! All algorithm and pulse schedules are human-authored. Not cyberlife.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

fn founder() -> Vec<Op> {
    vec![
        Op::Const { dst: 3, value: 78, next: 1 }, // 0: compute bundle
        Op::DrawCompute { amount: 3, next: 2 }, // 1: pool acquisition
        Op::GraphNew { dst: 0, next: 3 }, // 2
        Op::Const { dst: 1, value: 0, next: 4 }, // 3: source node index
        Op::GraphCopyNode { graph: 0, source: 1, dst: 2, next: 5 }, // 4
        Op::Const { dst: 3, value: 1, next: 6 }, // 5
        Op::Add { dst: 1, lhs: 1, rhs: 3, next: 7 }, // 6
        Op::Const { dst: 3, value: 12, next: 8 }, // 7
        Op::Sub { dst: 3, lhs: 1, rhs: 3, next: 9 }, // 8
        Op::Branch { test: 3, nonzero: 4, zero: 10 }, // 9
        Op::Const { dst: 1, value: 2, next: 11 }, // 10: child endowment
        Op::GraphLaunch { graph: 0, credits: 1, dst: 2, next: 0 }, // 11
    ]
}

#[derive(Debug)]
struct Arm {
    name: &'static str,
    packets: &'static [u64],
    disable_native_copy: bool,
    expected_births: usize,
}

fn exhaust(w: &mut World) -> usize {
    let mut count = 0_usize;
    loop {
        assert!(count < 110, "finite energy pulse cannot support endless copying");
        let energy_before = w.total_compute_inventory().unwrap();
        let prev_events = w.events().len();
        if !w.step() {
            assert_eq!(w.total_compute_inventory(), Some(energy_before));
            assert_eq!(w.events().len(), prev_events);
            return count;
        }
        count += 1;
        assert_eq!(w.total_compute_inventory(), Some(energy_before - 1),
            "native copying and launching must not mint compute energy");
        assert_eq!(w.events().len(), prev_events + 1);
        assert!(matches!(w.events().last().unwrap().origin,
            Origin::Internal(1) | Origin::Internal(2) | Origin::Internal(3) | Origin::Internal(4)
        ));
    }
}

fn births(w: &World) -> Vec<(u64,u64)> {
    w.events().iter().filter_map(|e| match (&e.origin,&e.kind) {
        (Origin::Internal(owner),EventKind::GraphLaunched {
            parent_process,child_process,transferred_credits:2,moved_nodes:12,..
        }) if owner==parent_process => Some((*parent_process,*child_process)),
        _ => None,
    }).collect()
}

#[test]
fn b27_paid_full_executable_self_copy_yields_verified_native_multi_generation_lineage() {
    let arms=[
        Arm { name:"THREE_GENERATIONS", packets:&[79,79,79], disable_native_copy:false, expected_births:3 },
        Arm { name:"SPLIT_THRESHOLD", packets:&[78,1,79,79], disable_native_copy:false, expected_births:3 },
        Arm { name:"SINGLE_PACKET", packets:&[79], disable_native_copy:false, expected_births:1 },
        Arm { name:"NO_INFLUX", packets:&[], disable_native_copy:false, expected_births:0 },
        Arm { name:"NATIVE_COPY_DISABLED", packets:&[79,79,79], disable_native_copy:true, expected_births:0 },
    ];
    for arm in arms {
        let mut w=World::with_strict_capacity(16,64,128)
            .with_pool_paid_draw()
            .with_recoverable_draw_shortage();
        if arm.disable_native_copy {
            w=w.with_native_code_edits_disabled();
        }
        let source=founder();
        assert_eq!(source.len(),12);
        w.load(Process::new(1,source.clone(),2)).unwrap();
        assert!(w.process(2).is_none());
        assert_eq!(w.total_compute_inventory(),Some(2));
        assert_eq!(exhaust(&mut w),2);
        assert_eq!(w.total_compute_inventory(),Some(0));
        assert_eq!(w.process(1).unwrap().pc,1);
        assert_eq!(w.process(1).unwrap().credits,0);
        let mut attempts=2_usize;

        for (packet_index,&packet) in arm.packets.iter().enumerate() {
            let events_before=w.events().len();
            let previous=w.total_compute_inventory().unwrap();
            let old_pool=w.available_compute_pool();
            w.intervene(Intervention::SetComputePool{credits:old_pool+packet}).unwrap();
            assert_eq!(w.total_compute_inventory(),Some(previous+packet));
            assert_eq!(w.events().len(),events_before+1);
            assert_eq!(w.events().last().unwrap().origin,Origin::External);
            let after_injection=w.events().len();

            let work=exhaust(&mut w);
            attempts+=work;
            if arm.name=="SPLIT_THRESHOLD" && packet_index==0 {
                assert_eq!(work,0,
                    "78 cannot fund 78-credit draw and its own charged instruction");
                assert_eq!(w.events().len(),after_injection,
                    "ineligible Draw must not report phantom internal work");
                assert_eq!(w.available_compute_pool(),78);
                assert_eq!(births(&w).len(),0);
            }
            if !arm.disable_native_copy && !matches!(arm.name,"NO_INFLUX") &&
                !(arm.name=="SPLIT_THRESHOLD" && packet_index==0)
            {
                assert_eq!(work,79,
                    "each complete self-copy plus child bootstrap is paid by one 79-unit packet");
            }
        }

        let line= births(&w);
        assert_eq!(line.len(),arm.expected_births,"{}",arm.name);
        assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),
            1+arm.packets.len(),"host may only load original founder and pulse the common pool");
        assert_eq!(w.total_compute_inventory(),
            Some(2+arm.packets.iter().sum::<u64>()-attempts as u64));
        assert!(w.occupied_units()<=128);
        assert_eq!(w.graph_count(),if arm.disable_native_copy { 1 } else { 0 });
        assert!(w.process((arm.expected_births+2) as u64).is_none(),
            "no hidden extra offspring");

        if !arm.disable_native_copy {
            assert!(w.events().iter().all(|e|
                !matches!(e.kind,EventKind::Trapped(_) | EventKind::Halted)
            ));
            let expected:Vec<(u64,u64)>=(1..=arm.expected_births as u64)
                .map(|parent|(parent,parent+1)).collect();
            assert_eq!(line,expected,"native process lineage must be a real generational chain");
            let made=w.events().iter().filter(|e|matches!(
                e.kind,EventKind::GraphCreated{..}
            )).count();
            let copied=w.events().iter().filter(|e|matches!(
                e.kind,EventKind::GraphNodeCopied{..}
            )).count();
            assert_eq!(made,arm.expected_births);
            assert_eq!(copied,arm.expected_births*12);
            for (i,(parent,child)) in line.iter().enumerate() {
                let own_copies=w.events().iter().filter_map(|e|match (&e.origin,&e.kind) {
                    (Origin::Internal(owner),EventKind::GraphNodeCopied {
                        source_process,source_node,..
                    }) if owner==parent && source_process==parent =>Some(*source_node),
                    _=>None,
                }).collect::<Vec<_>>();
                assert_eq!(own_copies,(0..12).collect::<Vec<usize>>(),
                    "each child must copy all 12 nodes of its own parent's executable in order");
                assert_eq!(*parent,(i+1) as u64);
                let p=w.process(*parent).unwrap();
                let c=w.process(*child).unwrap();
                assert_eq!(p.pc,0,"spent parent stays at an unpayable ordinary Const");
                assert_eq!(p.credits,0);
                assert!(!p.stopped);
                assert_eq!(c.code.nodes,source,
                    "child must inherit the ENTIRE working native reproducer, not just a work marker");
                assert!(!c.stopped);
            }
            assert_eq!(w.process((arm.expected_births+1) as u64).unwrap().pc,1,
                "newest child inherits code and awaits next neutral pool packet");
            assert_eq!(w.process((arm.expected_births+1) as u64).unwrap().credits,0);
            assert_eq!(w.total_compute_inventory(),Some(0));
        } else {
            assert_eq!(arm.expected_births,0);
            assert!(w.process(2).is_none());
            assert!(w.process(1).unwrap().stopped,
                "graph-copy suppression leaves an empty graph and a charged native launch trap");
            assert!(w.events().iter().any(|e|matches!(
                (&e.origin,&e.kind),
                (Origin::Internal(1),EventKind::Trapped(_))
            )));
            assert_eq!(w.events().iter().filter(|e|matches!(
                e.kind,EventKind::GraphNodeCopied { .. }
            )).count(),0);
        }
        eprintln!("B27_NATIVE arm={} births={:?} attempts={} pool={} inventory={} storage={} processes={}",
            arm.name,line,attempts,w.available_compute_pool(),
            w.total_compute_inventory().unwrap(),w.occupied_units(),arm.expected_births+1);
    }
}
