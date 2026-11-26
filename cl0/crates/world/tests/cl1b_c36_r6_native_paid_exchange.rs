//! C36-R6 engineering gate. Fixed affine offset task, two paid native actors.
//! No host credit refill, no kernel change. JS preflight predictions disclosed.
use cl0_world::{Intervention, Op, Process, World};

const OFFERS: [(i64, i64); 12] = [(4, 3), (9, 7), (2, 11), (13, 5), (6, 8), (15, 4), (3, 9), (12, 6), (7, 14), (1, 2), (10, 13), (5, 10)];

fn producer(wrong: bool) -> Vec<Op> {
    let mut nodes = vec![
        Op::Const { dst: 0, value: 10, next: 1 }, // 0
        Op::Const { dst: 1, value: 1, next: 2 }, // 1
        Op::Const { dst: 2, value: 2, next: 3 }, // 2
        Op::Const { dst: 3, value: 6, next: 4 }, // 3
        Op::TransferCompute { recipient: 2, amount: 3, next: 5 }, // 4
        Op::Write { address: 0, value: 1, next: 6 }, // 5
        Op::Read { dst: 1, address: 0, next: 7 }, // 6
        Op::Branch { test: 1, nonzero: 6, zero: 8 }, // 7
        Op::Const { dst: 0, value: 11, next: 9 }, // 8
        Op::Read { dst: 1, address: 0, next: 10 }, // 9
        Op::Const { dst: 0, value: 51, next: 11 }, // 10
        Op::Read { dst: 2, address: 0, next: 12 }, // 11
        Op::Add { dst: 1, lhs: 1, rhs: 2, next: 13 }, // 12
        Op::Const { dst: 0, value: 12, next: 14 }, // 13
        Op::Write { address: 0, value: 1, next: 15 }, // 14
        Op::Const { dst: 0, value: 10, next: 16 }, // 15
        Op::Const { dst: 1, value: 2, next: 17 }, // 16
        Op::Write { address: 0, value: 1, next: 18 }, // 17
        Op::Read { dst: 1, address: 0, next: 19 }, // 18
        Op::Branch { test: 1, nonzero: 18, zero: 20 }, // 19
        Op::SenseOwnCredits { dst: 3, next: 21 }, // 20
        Op::Const { dst: 0, value: 11, next: 22 }, // 21
        Op::Read { dst: 1, address: 0, next: 23 }, // 22
        Op::Const { dst: 0, value: 13, next: 24 }, // 23
        Op::Write { address: 0, value: 1, next: 25 }, // 24
        Op::Const { dst: 0, value: 0, next: 0 }, // 25
    ];
    if wrong {
        nodes[12] = Op::Sub { dst: 1, lhs: 1, rhs: 2, next: 13 };
    }
    nodes
}

fn buyer(reward: i64) -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: 10, next: 1 }, // 0
        Op::Read { dst: 1, address: 0, next: 2 }, // 1
        Op::Branch { test: 1, nonzero: 3, zero: 1 }, // 2
        Op::Const { dst: 2, value: 1, next: 4 }, // 3
        Op::Sub { dst: 2, lhs: 1, rhs: 2, next: 5 }, // 4
        Op::Branch { test: 2, nonzero: 10, zero: 6 }, // 5
        Op::Const { dst: 0, value: 50, next: 7 }, // 6
        Op::Read { dst: 1, address: 0, next: 8 }, // 7
        Op::Const { dst: 0, value: 11, next: 9 }, // 8
        Op::Write { address: 0, value: 1, next: 26 }, // 9
        Op::Const { dst: 0, value: 51, next: 11 }, // 10
        Op::Read { dst: 1, address: 0, next: 12 }, // 11
        Op::Const { dst: 0, value: 50, next: 13 }, // 12
        Op::Read { dst: 2, address: 0, next: 14 }, // 13
        Op::Add { dst: 1, lhs: 1, rhs: 2, next: 15 }, // 14
        Op::Const { dst: 0, value: 12, next: 16 }, // 15
        Op::Read { dst: 2, address: 0, next: 17 }, // 16
        Op::Sub { dst: 1, lhs: 1, rhs: 2, next: 18 }, // 17
        Op::Branch { test: 1, nonzero: 19, zero: 22 }, // 18
        Op::Const { dst: 1, value: 0, next: 20 }, // 19
        Op::Const { dst: 0, value: 11, next: 21 }, // 20
        Op::Write { address: 0, value: 1, next: 26 }, // 21
        Op::Const { dst: 2, value: 1, next: 23 }, // 22
        Op::Const { dst: 3, value: reward, next: 24 }, // 23
        Op::TransferCompute { recipient: 2, amount: 3, next: 25 }, // 24
        Op::Const { dst: 1, value: 1, next: 20 }, // 25
        Op::Const { dst: 0, value: 10, next: 27 }, // 26
        Op::Const { dst: 1, value: 0, next: 28 }, // 27
        Op::Write { address: 0, value: 1, next: 1 }, // 28
    ]
}

fn set(world: &mut World, key: i64, value: i64) {
    world.intervene(Intervention::SetCell { key, value }).unwrap();
}

#[derive(Debug, PartialEq, Eq)]
struct ResultRow {
    completed: usize,
    correct: usize,
    wrong: usize,
    steps: u64,
    probes: u64,
    sales: u64,
    producer_credit: u64,
    buyer_credit: u64,
}

fn run(reward: i64, wrong: bool) -> ResultRow {
    let mut world = World::with_strict_capacity(8, 64, 80);
    world.load(Process::new(1, producer(wrong), 120)).unwrap();
    world.load(Process::new(2, buyer(reward), 3000)).unwrap();
    for key in [10, 11, 12, 13] { set(&mut world, key, 0); }
    set(&mut world, 50, OFFERS[0].0);
    set(&mut world, 51, OFFERS[0].1);
    let mut completed = 0;
    let mut correct = 0;
    let mut incorrect = 0;
    let mut steps = 0;
    let mut probes = 0;
    let mut sales = 0;
    let initial = world.total_compute_inventory().unwrap();
    assert_eq!(initial, 3120);
    while completed < OFFERS.len() && world.process(1).unwrap().credits > 0 {
        assert!(steps < 10000, "bounded gate");
        let before_p = world.process(1).unwrap().clone();
        let before_b = world.process(2).unwrap().clone();
        assert!(world.step());
        steps += 1;
        let p = world.process(1).unwrap();
        let b = world.process(2).unwrap();
        assert!(!p.stopped && !b.stopped, "unexpected trap or halt");
        assert_eq!(world.total_compute_inventory(), Some(initial - steps));
        assert_eq!(world.available_compute_pool(), 0);
        assert!(world.occupied_units() <= 80);
        if before_p.pc == 4 && p.pc == 5 {
            probes += 1;
            assert_eq!(before_p.credits - p.credits, 7);
            assert_eq!(b.credits - before_b.credits, 6);
        }
        if before_b.pc == 24 && b.pc == 25 {
            sales += 1;
            assert_eq!(before_b.credits - b.credits, reward as u64 + 1);
            assert_eq!(p.credits - before_p.credits, reward as u64);
        }
        if before_p.pc == 24 && p.pc == 25 {
            let (a, x) = OFFERS[completed];
            let ok = world.cell(12) == Some(a + x);
            assert_eq!(world.cell(13), Some(i64::from(ok)));
            if ok { correct += 1; } else { incorrect += 1; }
            completed += 1;
            if completed < OFFERS.len() {
                set(&mut world, 50, OFFERS[completed].0);
                set(&mut world, 51, OFFERS[completed].1);
                assert_eq!(world.total_compute_inventory(), Some(initial - steps));
            }
        }
    }
    ResultRow {
        completed, correct, wrong: incorrect, steps, probes, sales,
        producer_credit: world.process(1).unwrap().credits,
        buyer_credit: world.process(2).unwrap().credits,
    }
}

#[test]
fn c36_r6_native_paid_probe_and_reciprocal_production() {
    for (reward, wrong, expected) in [
        (100, false, ResultRow { completed: 12, correct: 12, wrong: 0, steps: 1461, probes: 12, sales: 12, producer_credit: 517, buyer_credit: 1142 }),
        (0, false, ResultRow { completed: 1, correct: 1, wrong: 0, steps: 215, probes: 2, sales: 2, producer_credit: 0, buyer_credit: 2905 }),
        (100, true, ResultRow { completed: 1, correct: 0, wrong: 1, steps: 215, probes: 2, sales: 0, producer_credit: 0, buyer_credit: 2905 }),
    ] {
        let observed = run(reward, wrong);
        println!("reward={reward} wrong={wrong} {observed:?}");
        assert_eq!(observed, expected);
    }
}
