//! C36-R7 frozen engineering gate; exact non-native predictions disclosed.
//! Candidate mailbox12 is not publication13. No host credit refill or VM changes.
use cl0_world::{Intervention, Op, Process, World};
fn producer(mode: &str) -> Vec<Op> {
    let mut g = vec![
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
        Op::Const { dst: 0, value: 15, next: 11 }, // 10
        Op::Write { address: 0, value: 1, next: 12 }, // 11
        Op::Const { dst: 0, value: 15, next: 13 }, // 12
        Op::Read { dst: 1, address: 0, next: 14 }, // 13
        Op::Const { dst: 0, value: 51, next: 15 }, // 14
        Op::Read { dst: 2, address: 0, next: 16 }, // 15
        Op::Add { dst: 1, lhs: 1, rhs: 2, next: 17 }, // 16
        Op::Const { dst: 0, value: 12, next: 18 }, // 17
        Op::Write { address: 0, value: 1, next: 19 }, // 18
        Op::Const { dst: 0, value: 10, next: 20 }, // 19
        Op::Const { dst: 1, value: 2, next: 21 }, // 20
        Op::Write { address: 0, value: 1, next: 22 }, // 21
        Op::Read { dst: 1, address: 0, next: 23 }, // 22
        Op::Branch { test: 1, nonzero: 22, zero: 24 }, // 23
        Op::Const { dst: 0, value: 11, next: 25 }, // 24
        Op::Read { dst: 1, address: 0, next: 26 }, // 25
        Op::Branch { test: 1, nonzero: 27, zero: 0 }, // 26
        Op::Const { dst: 0, value: 12, next: 28 }, // 27
        Op::Read { dst: 1, address: 0, next: 29 }, // 28
        Op::Const { dst: 0, value: 13, next: 30 }, // 29
        Op::Write { address: 0, value: 1, next: 31 }, // 30
        Op::Const { dst: 0, value: 14, next: 32 }, // 31
        Op::Const { dst: 1, value: 1, next: 33 }, // 32
        Op::Write { address: 0, value: 1, next: 12 }, // 33
        Op::Const { dst: 0, value: 14, next: 35 }, // 34
        Op::Const { dst: 1, value: 0, next: 36 }, // 35
        Op::Write { address: 0, value: 1, next: 12 }, // 36
    ];
    if mode == "fresh" { g[33] = Op::Write { address: 0, value: 1, next: 0 }; }
    if mode == "no_retry" { g[26] = Op::Branch { test: 1, nonzero: 27, zero: 34 }; }
    g
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


fn set(w: &mut World, key: i64, value: i64) {
    w.intervene(Intervention::SetCell { key, value }).unwrap();
}
#[derive(Debug, PartialEq, Eq)]
struct Row {
    completed: usize, accepted: usize, rejected: usize, invalid_candidates: usize,
    steps: u64, probes: u64, sales: u64, producer_credit: u64, buyer_credit: u64,
}
fn run(mode: &str, offers: &[(i64,i64)]) -> Row {
    let reward = if mode == "cache_zero" { 0 } else { 100 };
    let mut w = World::with_strict_capacity(8, 64, 100);
    w.load(Process::new(1, producer(mode), 220)).unwrap();
    w.load(Process::new(2, buyer(reward), 15000)).unwrap();
    for key in [10,11,12,13,14,15] { set(&mut w,key,0); }
    set(&mut w,50,offers[0].0); set(&mut w,51,offers[0].1);
    let (mut completed,mut accepted,mut rejected,mut invalid_candidates) = (0,0,0,0);
    let (mut steps,mut probes,mut sales) = (0,0,0);
    while completed < offers.len() && w.process(1).unwrap().credits > 0 {
        assert!(steps < 30000);
        let pre_p = w.process(1).unwrap().clone();
        let pre_b = w.process(2).unwrap().clone();
        assert!(w.step()); steps += 1;
        let p = w.process(1).unwrap();
        let b = w.process(2).unwrap();
        assert!(!p.stopped && !b.stopped);
        assert_eq!(w.total_compute_inventory(), Some(15220-steps));
        assert_eq!(w.available_compute_pool(),0);
        assert!(w.occupied_units() <= 100);
        if pre_p.pc == 4 && p.pc == 5 {
            probes += 1;
            assert_eq!(pre_p.credits-p.credits,7);
            assert_eq!(b.credits-pre_b.credits,6);
        }
        if pre_b.pc == 24 && b.pc == 25 {
            sales += 1;
            assert_eq!(pre_b.credits-b.credits,reward as u64+1);
            assert_eq!(p.credits-pre_p.credits,reward as u64);
        }
        if pre_b.pc == 18 && b.pc == 19 { invalid_candidates += 1; }
        if pre_p.pc == 30 && p.pc == 31 {
            assert_eq!(w.cell(11),Some(1));
            assert_eq!(w.cell(13),Some(offers[completed].0+offers[completed].1));
        }
        if (pre_p.pc == 33 && p.pc != pre_p.pc) || (pre_p.pc == 36 && p.pc != pre_p.pc) {
            if pre_p.pc == 33 { accepted += 1; assert_eq!(w.cell(14),Some(1)); }
            else { rejected += 1; assert_eq!(w.cell(14),Some(0)); }
            completed += 1;
            if completed < offers.len() { set(&mut w,50,offers[completed].0);set(&mut w,51,offers[completed].1); }
        }
    }
    Row { completed, accepted, rejected, invalid_candidates, steps, probes, sales,
      producer_credit: w.process(1).unwrap().credits, buyer_credit: w.process(2).unwrap().credits }
}
#[test]
fn c36_r7_native_verified_publication() {
    let stable = [(3,1),(3,8),(3,15),(3,3),(3,10),(3,17),(3,5),(3,12),(3,19),(3,7),(3,14),(3,2),(7,9),(7,16),(7,4),(7,11),(7,18),(7,6),(7,13),(7,1),(7,8),(7,15),(7,3),(7,10)];
    let mixed = [(3,1),(3,8),(3,15),(3,3),(8,10),(8,17),(8,5),(8,12),(2,19),(2,7),(2,14),(2,2),(9,9),(9,16),(9,4),(9,11),(4,18),(4,6),(4,13),(4,1),(11,8),(11,15),(11,3),(11,10)];
    let fast = [(1,1),(2,8),(3,15),(4,3),(5,10),(6,17),(7,5),(8,12),(9,19),(10,7),(11,14),(1,2),(2,9),(3,16),(4,4),(5,11),(6,18),(7,6),(8,13),(9,1),(10,8),(11,15),(1,3),(2,10)];
    for (mode, offers, expected) in [
        ("fresh", &stable[..], Row { completed: 24, accepted: 24, rejected: 0, invalid_candidates: 0, steps: 3311, probes: 24, sales: 24, producer_credit: 820, buyer_credit: 11089 }),
        ("cache", &stable[..], Row { completed: 24, accepted: 24, rejected: 0, invalid_candidates: 1, steps: 2325, probes: 2, sales: 24, producer_credit: 1445, buyer_credit: 11450 }),
        ("no_retry", &stable[..], Row { completed: 24, accepted: 12, rejected: 12, invalid_candidates: 12, steps: 2039, probes: 1, sales: 12, producer_credit: 394, buyer_credit: 12787 }),
        ("cache_zero", &stable[..], Row { completed: 4, accepted: 4, rejected: 0, invalid_candidates: 0, steps: 427, probes: 1, sales: 4, producer_credit: 0, buyer_credit: 14793 }),
        ("fresh", &mixed[..], Row { completed: 24, accepted: 24, rejected: 0, invalid_candidates: 0, steps: 3311, probes: 24, sales: 24, producer_credit: 820, buyer_credit: 11089 }),
        ("cache", &mixed[..], Row { completed: 24, accepted: 24, rejected: 0, invalid_candidates: 5, steps: 2797, probes: 6, sales: 24, producer_credit: 1185, buyer_credit: 11238 }),
        ("no_retry", &mixed[..], Row { completed: 14, accepted: 4, rejected: 10, invalid_candidates: 11, steps: 1227, probes: 1, sales: 4, producer_credit: 0, buyer_credit: 13993 }),
        ("cache_zero", &mixed[..], Row { completed: 4, accepted: 4, rejected: 0, invalid_candidates: 0, steps: 427, probes: 1, sales: 4, producer_credit: 0, buyer_credit: 14793 }),
        ("fresh", &fast[..], Row { completed: 24, accepted: 24, rejected: 0, invalid_candidates: 0, steps: 3311, probes: 24, sales: 24, producer_credit: 820, buyer_credit: 11089 }),
        ("cache", &fast[..], Row { completed: 19, accepted: 19, rejected: 0, invalid_candidates: 19, steps: 3999, probes: 20, sales: 19, producer_credit: 0, buyer_credit: 11221 }),
        ("no_retry", &fast[..], Row { completed: 7, accepted: 1, rejected: 6, invalid_candidates: 6, steps: 627, probes: 1, sales: 1, producer_credit: 0, buyer_credit: 14593 }),
        ("cache_zero", &fast[..], Row { completed: 2, accepted: 2, rejected: 0, invalid_candidates: 2, steps: 415, probes: 2, sales: 2, producer_credit: 0, buyer_credit: 14805 }),
    ] {
        let observed=run(mode,offers);
        println!("{mode}: {observed:?}");
        assert_eq!(observed,expected);
    }
}
