//! Study103: HUMAN-authored identical CIL genotypes; native executable mutation
//! before whole-code heredity, then natural VM resource-mediated descendant counts.
//! This is a scripted mechanism fixture, not stochastic evolutionary emergence.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::collections::{BTreeMap, BTreeSet};

const NODES: usize = 24;
const GRANT: u64 = 200;
const LIMIT: usize = 2600;

fn common_genome() -> Vec<Op> {
    let nodes = vec![
        Op::Const { dst: 2, value: 7, next: 1 },
        Op::Const { dst: 3, value: 9, next: 2 },
        Op::Read { dst: 0, address: 2, next: 3 },
        Op::Write { address: 3, value: 0, next: 4 },
        Op::Branch { test: 1, nonzero: 21, zero: 5 },
        Op::Const { dst: 2, value: 1, next: 17 },
        Op::Const { dst: 1, value: 0, next: 7 },
        Op::Const { dst: 3, value: 24, next: 8 },
        Op::GraphNew { dst: 0, next: 9 },
        Op::GraphCopyNode { graph: 0, source: 1, dst: 1, next: 10 },
        Op::Add { dst: 1, lhs: 1, rhs: 2, next: 11 },
        Op::Sub { dst: 3, lhs: 3, rhs: 2, next: 12 },
        Op::Branch { test: 3, nonzero: 9, zero: 13 },
        Op::Const { dst: 2, value: 200, next: 14 },
        Op::DrawCompute { amount: 2, next: 15 },
        Op::GraphLaunch { graph: 0, credits: 2, dst: 3, next: 16 },
        Op::Halt,
        Op::Const { dst: 1, value: 20, next: 18 },
        Op::Const { dst: 2, value: 1, next: 19 },
        Op::Sub { dst: 1, lhs: 1, rhs: 2, next: 20 },
        Op::Branch { test: 1, nonzero: 19, zero: 6 },
        Op::Const { dst: 1, value: 5, next: 22 },
        Op::Const { dst: 2, value: 6, next: 23 },
        Op::PatchNext { node: 1, target: 2, next: 5 },
    ];
    assert_eq!(nodes.len(), NODES);
    nodes
}

fn fixture(source: i64, first_trigger: bool, second_trigger: bool, pool: u64) -> World {
    assert!([3, 8].contains(&source));
    let mut world = World::with_strict_capacity(4, 64, 1024)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    let code = common_genome();
    for (id, trigger) in [(1_u64, first_trigger), (2_u64, second_trigger)] {
        let mut parent = Process::new(id, code.clone(), GRANT);
        parent.registers[1] = i64::from(trigger);
        world.load(parent).unwrap();
    }
    assert_eq!(world.process(1).unwrap().code.nodes, world.process(2).unwrap().code.nodes);
    world.intervene(Intervention::SetCell { key: 7, value: source }).unwrap();
    world.intervene(Intervention::SetComputePool { credits: pool }).unwrap();
    world
}

#[derive(Debug)]
struct Outcome {
    attempts: usize,
    first_children: usize,
    second_children: usize,
    native_mutations: usize,
    native_draws: usize,
    deferred: usize,
    source_function_actors: usize,
    pool_left: u64,
}

fn audit(
    world: &World,
    source: i64,
    first_trigger: bool,
    second_trigger: bool,
    pool: u64,
    attempts: usize,
) -> Outcome {
    assert!(attempts <= LIMIT);
    assert_eq!(world.total_compute_inventory(), Some(2 * GRANT + pool - attempts as u64));
    assert_eq!(world.events().iter().filter(|e| e.origin == Origin::External).count(), 4);
    assert_eq!(world.cell(9), Some(source));

    let mut ancestor = BTreeMap::<u64, u64>::from([(1, 1), (2, 2)]);
    let mut phenotype = BTreeMap::<u64, bool>::from([(1, first_trigger), (2, second_trigger)]);
    let mut descendants = BTreeMap::<u64, usize>::new();
    let mut copies = BTreeMap::<u64, BTreeSet<usize>>::new();
    let mut mutations = Vec::<u64>::new();
    let mut read_actors = BTreeSet::<u64>::new();
    let mut output_actors = BTreeSet::<u64>::new();
    let mut draws = 0_usize;
    let mut deferred = 0_usize;

    for e in world.events() {
        if e.origin != Origin::Internal(e.process) {
            continue;
        }
        match &e.kind {
            EventKind::ExecutableEdgePatched { node, before, after } => {
                assert_eq!(e.pc, 23, "mutation must be executed by authored native PatchNext");
                assert_eq!((*node, *before, *after), (5, 17, 6));
                assert!([1, 2].contains(&e.process), "child unexpectedly mutated itself");
                assert_eq!(phenotype.get(&e.process), Some(&true));
                mutations.push(e.process);
            }
            EventKind::GraphLaunched {
                parent_process,
                child_process,
                transferred_credits,
                moved_nodes,
                ..
            } => {
                assert_eq!(e.pc, 15);
                assert_eq!(*parent_process, e.process);
                assert_eq!((*transferred_credits, *moved_nodes), (GRANT, NODES));
                let original = *ancestor.get(parent_process).expect("missing native ancestor");
                let mutated = phenotype[parent_process];
                assert!(ancestor.insert(*child_process, original).is_none());
                assert!(phenotype.insert(*child_process, mutated).is_none());
                *descendants.entry(original).or_default() += 1;
                let inherited = &world.process(*child_process).unwrap().code.nodes;
                assert_eq!(inherited.len(), NODES);
                let expected = if mutated { 6 } else { 17 };
                assert!(matches!(inherited[5], Op::Const { next, .. } if next == expected));
                for (i, (a, b)) in common_genome().iter().zip(inherited).enumerate() {
                    if i != 5 {
                        assert_eq!(a, b, "whole executable child code must be faithfully inherited");
                    }
                }
            }
            EventKind::GraphNodeCopied { source_process, source_node, .. } => {
                assert_eq!(*source_process, e.process);
                copies.entry(e.process).or_default().insert(*source_node);
            }
            EventKind::CellRead { key: 7, value } => {
                assert_eq!(e.pc, 2);
                assert_eq!(*value, source);
                read_actors.insert(e.process);
            }
            EventKind::CellWritten { key: 9, value } => {
                assert_eq!(e.pc, 3);
                assert_eq!(*value, source);
                assert!(read_actors.contains(&e.process));
                output_actors.insert(e.process);
            }
            EventKind::Executed if e.pc == 14 => draws += 1,
            EventKind::DrawDeferred { requested: GRANT, .. } if e.pc == 14 => deferred += 1,
            _ => {}
        }
    }
    let triggered: Vec<u64> = [(1_u64, first_trigger), (2_u64, second_trigger)]
        .into_iter().filter_map(|(id, yes)| yes.then_some(id)).collect();
    assert_eq!(mutations, triggered, "exact registered internal founder mutations");
    assert_eq!(draws, descendants.values().sum());
    assert_eq!(output_actors.len(), ancestor.len(), "each resident native lineage retains actual source function");

    for (&actor, &mutated) in &phenotype {
        let code = &world.process(actor).unwrap().code.nodes;
        let edge = if mutated { 6 } else { 17 };
        assert!(matches!(code[5], Op::Const { next, .. } if next == edge));
        assert_eq!(code.len(), NODES);
    }
    for (&actor, copied_nodes) in &copies {
        if copied_nodes.len() == NODES {
            assert!(ancestor.contains_key(&actor));
        }
    }
    if pool == 0 {
        assert!(descendants.is_empty(), "closed external compute must block all native offspring");
        assert_eq!(draws, 0);
        assert!(deferred > 0);
    } else {
        assert!(*descendants.get(&1).unwrap_or(&0) >= 1);
        assert!(*descendants.get(&2).unwrap_or(&0) >= 1);
    }
    Outcome {
        attempts,
        first_children: *descendants.get(&1).unwrap_or(&0),
        second_children: *descendants.get(&2).unwrap_or(&0),
        native_mutations: mutations.len(),
        native_draws: draws,
        deferred,
        source_function_actors: output_actors.len(),
        pool_left: world.available_compute_pool(),
    }
}

#[test]
fn native_in_genome_efficiency_mutation_is_inherited_before_shared_pool_competition() {
    let conditions = [
        ("mixed-trigger1", true, false, 2000),
        ("mixed-trigger2", false, true, 2000),
        ("both-untriggered", false, false, 2000),
        ("both-triggered", true, true, 2000),
        ("mixed-trigger1-no-pool", true, false, 0),
    ];
    for source in [3, 8] {
        for (name, first_trigger, second_trigger, pool) in conditions {
            let mut world = fixture(source, first_trigger, second_trigger, pool);
            let attempts = world.run(LIMIT);
            let result = audit(&world, source, first_trigger, second_trigger, pool, attempts);
            println!(
                "Study103 source={source}/{name}: attempts={} founder1_descendants={} founder2_descendants={} native_mutations={} native_draws={} deferred={} functional_actors={} pool_left={} [SCRIPTED-TRIGGER, ZERO RANDOM ECOLOGY]",
                result.attempts, result.first_children, result.second_children,
                result.native_mutations, result.native_draws, result.deferred,
                result.source_function_actors, result.pool_left,
            );
        }
    }
}
