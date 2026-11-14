//! CL-1 Study99: HUMAN-AUTHORED native CIL whole-program self-copy.
//! Mechanism calibration only, ZERO independent stochastic ecological genomes.
//! No organism-origin, adaptive evolution or cyberlife claims.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::collections::{BTreeMap, BTreeSet};

fn complete_self_copier(local_edges: bool) -> Vec<Op> {
    vec![
        Op::Const { dst: 2, value: 7, next: 1 },      // 0: native source address
        Op::Const { dst: 3, value: 9, next: 2 },      // 1: native writer address
        Op::Read { dst: 0, address: 2, next: 3 },     // 2: actually reads source
        Op::Write { address: 3, value: 0, next: 4 },  // 3: writes own source value
        Op::Const { dst: 2, value: 1, next: 5 },      // 4: native copy index step
        Op::Const { dst: 1, value: 0, next: 6 },      // 5: source code starts at0
        Op::Const { dst: 3, value: 16, next: 7 },     // 6: exactly16 source code nodes
        Op::GraphNew { dst: 0, next: 8 },             // 7: owned graph, no host copy
        if local_edges {
            Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 1, next: 9 }
        } else {
            Op::GraphCopyNode { graph: 0, source: 1, dst: 1, next: 9 }
        },                                             // 8: native self-code copy
        Op::Add { dst: 1, lhs: 1, rhs: 2, next: 10 },  // 9: next source index
        Op::Sub { dst: 3, lhs: 3, rhs: 2, next: 11 },  // 10: decrement remaining
        Op::Branch { test: 3, nonzero: 8, zero: 12 },  // 11: copy all16 instructions
        Op::Const { dst: 2, value: 100, next: 13 },   // 12: desired pool Draw+child grant
        Op::DrawCompute { amount: 2, next: 14 },      // 13: internal resource uptake
        Op::GraphLaunch { graph: 0, credits: 2, dst: 3, next: 15 }, //14: native descendant
        Op::Halt,                                      //15: end actor, child starts atPC0
    ]
}

fn world(source: i64, local_edges: bool, common_pool: u64) -> World {
    assert!([3, 8].contains(&source));
    let mut w = World::with_strict_capacity(4, 64, 512)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage();
    w.load(Process::new(1, complete_self_copier(local_edges), 128))
        .expect("valid engineered CIL seed parent");
    w.intervene(Intervention::SetCell { key: 7, value: source })
        .expect("valid environment source");
    w.intervene(Intervention::SetComputePool { credits: common_pool })
        .expect("valid public environmental resource");
    w
}

fn native_genealogy(w: &World) -> Vec<(u64, u64)> {
    let mut links = Vec::new();
    for e in w.events() {
        if let EventKind::GraphLaunched {
            parent_process,
            child_process,
            transferred_credits,
            moved_nodes,
            ..
        } = &e.kind
        {
            assert_eq!(e.origin, Origin::Internal(*parent_process));
            assert_eq!(e.process, *parent_process);
            assert_eq!(*transferred_credits, 100);
            assert_eq!(*moved_nodes, 16);
            links.push((*parent_process, *child_process));
        }
    }
    links
}

#[test]
fn native_whole_program_replication_yields_functional_multigeneration_descendants() {
    for source in [3, 8] {
        let seed = complete_self_copier(false);
        let mut growing = world(source, false, 1000);
        let initial = growing.total_compute_inventory().unwrap();
        assert_eq!(initial, 1128);
        let attempts = growing.run(1500);
        assert!(attempts < 1500, "finite world must stop at a real resource boundary");
        assert_eq!(
            growing.total_compute_inventory(),
            Some(initial - attempts as u64),
            "native Draw/Launch must conserve all externally initialized compute"
        );

        let links = native_genealogy(&growing);
        assert!(links.len() >= 3, "native descendants must make grandchildren");
        for (index, &(parent, child)) in links.iter().enumerate() {
            assert_eq!(parent, (index + 1) as u64);
            assert_eq!(child, (index + 2) as u64);
            assert_eq!(
                &growing.process(child).expect("native child exists").code.nodes,
                &seed,
                "all descendants must inherit complete executable copying machinery"
            );
        }
        assert!(links.iter().any(|(parent, _)| *parent >= 3),
                "a grandchild itself must reproduce via native CIL");
        let generations: BTreeSet<_> = links.iter().flat_map(|(p, c)| [*p, *c]).collect();
        assert!(generations.len() >= 4);

        let mut read_seen = BTreeMap::<u64, i64>::new();
        let mut writer_processes = BTreeSet::new();
        let mut native_draws = BTreeSet::new();
        let mut copied_code_nodes = BTreeMap::<u64, BTreeSet<usize>>::new();
        for event in growing.events() {
            if event.process == 0 || event.origin != Origin::Internal(event.process) {
                continue;
            }
            match &event.kind {
                EventKind::CellRead { key: 7, value } => {
                    assert_eq!(event.pc, 2);
                    assert_eq!(*value, source);
                    read_seen.insert(event.process, *value);
                }
                EventKind::CellWritten { key: 9, value } => {
                    assert_eq!(event.pc, 3);
                    assert_eq!(*value, source);
                    assert_eq!(read_seen.get(&event.process), Some(&source));
                    writer_processes.insert(event.process);
                }
                EventKind::GraphNodeCopied { source_process, source_node, .. } => {
                    assert_eq!(*source_process, event.process);
                    copied_code_nodes.entry(event.process).or_default().insert(*source_node);
                }
                EventKind::Executed if event.pc == 13 => {
                    native_draws.insert(event.process);
                }
                _ => {}
            }
        }
        for actor in 1..=4_u64 {
            assert!(writer_processes.contains(&actor),
                    "both parent and first 3 native children must execute their own source-sensitive Write");
            assert!(native_draws.contains(&actor),
                    "every reproducing native generation needs its own successful DrawCompute");
            assert_eq!(copied_code_nodes.get(&actor).map(BTreeSet::len), Some(16),
                    "each native generation must copy all16 actual code nodes");
            assert_eq!(&growing.process(actor).unwrap().code.nodes, &seed);
        }
        let external = growing.events().iter().filter(|e| e.origin == Origin::External).count();
        assert_eq!(external, 3, "only original parent load/source cell/public pool are external");
        assert_eq!(growing.cell(9), Some(source));

        let mut wrong_relocation = world(source, true, 1000);
        let wrong_initial = wrong_relocation.total_compute_inventory().unwrap();
        let wrong_attempts = wrong_relocation.run(700);
        assert_eq!(wrong_relocation.total_compute_inventory(),
                   Some(wrong_initial - wrong_attempts as u64));
        assert_eq!(native_genealogy(&wrong_relocation).len(), 1,
                "local-address copying must not accidentally preserve whole-program identity");
        assert!(matches!(
            wrong_relocation.process(2).unwrap().code.nodes[0],
            Op::Const { dst: 2, value: 7, next: 0 }
        ));
        assert_ne!(wrong_relocation.process(2).unwrap().code.nodes, complete_self_copier(true));
        assert!(!wrong_relocation.events().iter().any(|e| {
            e.origin == Origin::Internal(2)
                && matches!(e.kind, EventKind::CellWritten { key: 9, .. })
        }));

        let mut no_energy = world(source, false, 0);
        let cold_initial = no_energy.total_compute_inventory().unwrap();
        let cold_attempts = no_energy.run(250);
        assert_eq!(no_energy.total_compute_inventory(),
                   Some(cold_initial - cold_attempts as u64));
        assert!(native_genealogy(&no_energy).is_empty(),
                "cannot manufacture native offspring without common-pool credits");
        assert!(no_energy.events().iter().any(|e| {
            e.origin == Origin::Internal(1)
                && matches!(e.kind, EventKind::DrawDeferred { requested: 100, .. })
        }), "credit-starved native actor must expose its deferred Draw attempt");
        assert!(!no_energy.events().iter().any(|e| {
            e.origin == Origin::External
                && matches!(e.kind, EventKind::GraphLaunched { .. })
        }));
    }
}
