//! C36-R8: frozen engineering preflight, no native outcome claimed.
//! Same initial adaptive source in on/off; only native PatchNext effect suppressed.
use cl0_world::{Intervention, Op, Process, World};
fn producer(mode: &str) -> Vec<Op> {
    let mut g = vec![
        Op::Const { dst: 0, value: 10, next: 1 },
        Op::Const { dst: 1, value: 1, next: 2 },
        Op::Const { dst: 2, value: 2, next: 3 },
        Op::Const { dst: 3, value: 6, next: 4 },
        Op::TransferCompute { recipient: 2, amount: 3, next: 5 },
        Op::Write { address: 0, value: 1, next: 6 },
        Op::Read { dst: 1, address: 0, next: 7 },
        Op::Branch { test: 1, nonzero: 6, zero: 8 },
        Op::Const { dst: 0, value: 11, next: 9 },
        Op::Read { dst: 1, address: 0, next: 10 },
        Op::Const { dst: 0, value: 15, next: 11 },
        Op::Write { address: 0, value: 1, next: 12 },
        Op::Const { dst: 0, value: 15, next: 13 },
        Op::Read { dst: 1, address: 0, next: 14 },
        Op::Const { dst: 0, value: 51, next: 15 },
        Op::Read { dst: 2, address: 0, next: 16 },
        Op::Add { dst: 1, lhs: 1, rhs: 2, next: 17 },
        Op::Const { dst: 0, value: 12, next: 18 },
        Op::Write { address: 0, value: 1, next: 19 },
        Op::Const { dst: 0, value: 10, next: 20 },
        Op::Const { dst: 1, value: 2, next: 21 },
        Op::Write { address: 0, value: 1, next: 22 },
        Op::Read { dst: 1, address: 0, next: 23 },
        Op::Branch { test: 1, nonzero: 22, zero: 24 },
        Op::Const { dst: 0, value: 11, next: 25 },
        Op::Read { dst: 1, address: 0, next: 26 },
        Op::Branch { test: 1, nonzero: 27, zero: 0 },
        Op::Const { dst: 0, value: 12, next: 28 },
        Op::Read { dst: 1, address: 0, next: 29 },
        Op::Const { dst: 0, value: 13, next: 30 },
        Op::Write { address: 0, value: 1, next: 31 },
        Op::Const { dst: 0, value: 14, next: 32 },
        Op::Const { dst: 1, value: 1, next: 33 },
        Op::Write { address: 0, value: 1, next: 12 },
        Op::Const { dst: 0, value: 14, next: 35 },
        Op::Const { dst: 1, value: 0, next: 36 },
        Op::Write { address: 0, value: 1, next: 12 },
    ];
    if mode != "cache" { g[33] = Op::Write { address: 0, value: 1, next: 0 }; }
    if mode == "adaptive" || mode == "edit_off" {
        g[11] = Op::Read { dst: 2, address: 0, next: 37 };
        g.extend([
        Op::Write { address: 0, value: 1, next: 38 },
        Op::Sub { dst: 3, lhs: 1, rhs: 2, next: 39 },
        Op::Branch { test: 3, nonzero: 40, zero: 41 },
        Op::Const { dst: 1, value: 0, next: 42 },
        Op::Const { dst: 1, value: 12, next: 42 },
        Op::Const { dst: 0, value: 33, next: 43 },
        Op::ReadNext { dst: 2, node: 0, next: 44 },
        Op::Sub { dst: 3, lhs: 2, rhs: 1, next: 45 },
        Op::Branch { test: 3, nonzero: 46, zero: 12 },
        Op::PatchNext { node: 0, target: 1, next: 12 },
        ]);
    }
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
fn edge(p: &Process) -> usize {
    match &p.code.nodes[33] { Op::Write { next, .. } => *next, _ => panic!("dispatch") }
}
#[derive(Debug, PartialEq, Eq)]
struct Row {
    completed: usize, steps: u64, probes: u64, sales: u64, invalid_candidates: usize,
    patch_attempts: usize, edits: usize, producer_credit: u64, buyer_credit: u64,
}
type Snapshot = (u64,usize,u64,[i64;4],usize,u64,[i64;4],Vec<i64>,usize,usize);
fn run(mode: &str, offers: &[(i64,i64)]) -> (Row, Vec<Snapshot>) {
    let mut w = World::with_strict_capacity(8, 64, 100);
    if mode == "edit_off" { w = w.with_native_patchnext_disabled(); }
    w.load(Process::new(1, producer(mode), 220)).unwrap();
    w.load(Process::new(2, buyer(100), 15000)).unwrap();
    let keys = [10,11,12,13,14,15,50,51];
    for key in keys { set(&mut w,key,0); }
    set(&mut w,50,offers[0].0); set(&mut w,51,offers[0].1);
    let (mut completed,mut invalid_candidates,mut patch_attempts,mut edits) = (0,0,0,0);
    let (mut steps,mut probes,mut sales) = (0,0,0);
    let mut trace = Vec::new();
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
            probes += 1; assert_eq!(pre_p.credits-p.credits,7); assert_eq!(b.credits-pre_b.credits,6);
        }
        if pre_b.pc == 24 && b.pc == 25 {
            sales += 1; assert_eq!(pre_b.credits-b.credits,101); assert_eq!(p.credits-pre_p.credits,100);
        }
        if pre_b.pc == 18 && b.pc == 19 { invalid_candidates += 1; }
        if pre_p.pc == 46 && p.pc == 12 {
            patch_attempts += 1;
            assert_eq!(pre_p.credits-p.credits,1);
            if edge(&pre_p)!=edge(p) { edits += 1; assert_ne!(mode,"edit_off"); }
            else { assert_eq!(mode,"edit_off"); }
        } else { assert_eq!(edge(&pre_p),edge(p)); }
        if pre_p.pc == 30 && p.pc == 31 {
            assert_eq!(w.cell(11),Some(1));
            assert_eq!(w.cell(13),Some(offers[completed].0+offers[completed].1));
        }
        if pre_p.pc == 33 && p.pc != pre_p.pc {
            assert_eq!(w.cell(14),Some(1)); completed += 1;
            if completed < offers.len() { set(&mut w,50,offers[completed].0);set(&mut w,51,offers[completed].1); }
        }
        let p = w.process(1).unwrap(); let b = w.process(2).unwrap();
        trace.push((steps,p.pc,p.credits,p.registers,b.pc,b.credits,b.registers,keys.iter().map(|k|w.cell(*k).unwrap()).collect(),edge(p),completed));
    }
    (Row { completed,steps,probes,sales,invalid_candidates,patch_attempts,edits,
        producer_credit: w.process(1).unwrap().credits, buyer_credit: w.process(2).unwrap().credits },trace)
}
#[test]
fn c36_r8_native_sensing_path_edit() {
    let stable = [(3,1),(3,8),(3,15),(3,3),(3,10),(3,17),(3,5),(3,12),(3,19),(3,7),(3,14),(3,2),(7,9),(7,16),(7,4),(7,11),(7,18),(7,6),(7,13),(7,1),(7,8),(7,15),(7,3),(7,10)];
    let mixed = [(3,1),(3,8),(3,15),(3,3),(8,10),(8,17),(8,5),(8,12),(2,19),(2,7),(2,14),(2,2),(9,9),(9,16),(9,4),(9,11),(4,18),(4,6),(4,13),(4,1),(11,8),(11,15),(11,3),(11,10)];
    let fast = [(1,1),(2,8),(3,15),(4,3),(5,10),(6,17),(7,5),(8,12),(9,19),(10,7),(11,14),(1,2),(2,9),(3,16),(4,4),(5,11),(6,18),(7,6),(8,13),(9,1),(10,8),(11,15),(1,3),(2,10)];
    let switch = [(1,1),(2,8),(3,15),(4,3),(5,10),(6,17),(7,5),(8,12),(9,19),(10,7),(11,14),(1,2),(7,9),(7,16),(7,4),(7,11),(7,18),(7,6),(7,13),(7,1),(7,8),(7,15),(7,3),(7,10)];
    let reverse = [(7,1),(7,8),(7,15),(7,3),(7,10),(7,17),(7,5),(7,12),(7,19),(7,7),(7,14),(7,2),(2,9),(3,16),(4,4),(5,11),(6,18),(7,6),(8,13),(9,1),(10,8),(11,15),(1,3),(2,10)];
    {
        let mut on = Vec::new(); let mut off = Vec::new();
        for (mode,expected) in [
            ("fresh", Row { completed: 24, steps: 3311, probes: 24, sales: 24, invalid_candidates: 0, patch_attempts: 0, edits: 0, producer_credit: 820, buyer_credit: 11089 }),
            ("cache", Row { completed: 24, steps: 2325, probes: 2, sales: 24, invalid_candidates: 1, patch_attempts: 0, edits: 0, producer_credit: 1445, buyer_credit: 11450 }),
            ("adaptive", Row { completed: 24, steps: 2491, probes: 4, sales: 24, invalid_candidates: 1, patch_attempts: 3, edits: 3, producer_credit: 1350, buyer_credit: 11379 }),
            ("edit_off", Row { completed: 24, steps: 3739, probes: 24, sales: 24, invalid_candidates: 0, patch_attempts: 22, edits: 0, producer_credit: 606, buyer_credit: 10875 }),
        ] {
            let (observed,trace)=run(mode,&stable);
            println!("stable {mode}: {observed:?}");
            assert_eq!(observed,expected);
            if mode=="adaptive" { on=trace; } else if mode=="edit_off" { off=trace; }
        }
        assert_eq!(on.iter().zip(off.iter()).take_while(|(a,b)|a==b).count(),216);
    }
    {
        let mut on = Vec::new(); let mut off = Vec::new();
        for (mode,expected) in [
            ("fresh", Row { completed: 24, steps: 3311, probes: 24, sales: 24, invalid_candidates: 0, patch_attempts: 0, edits: 0, producer_credit: 820, buyer_credit: 11089 }),
            ("cache", Row { completed: 24, steps: 2797, probes: 6, sales: 24, invalid_candidates: 5, patch_attempts: 0, edits: 0, producer_credit: 1185, buyer_credit: 11238 }),
            ("adaptive", Row { completed: 24, steps: 3299, probes: 12, sales: 24, invalid_candidates: 5, patch_attempts: 11, edits: 11, producer_credit: 898, buyer_credit: 11023 }),
            ("edit_off", Row { completed: 24, steps: 3731, probes: 24, sales: 24, invalid_candidates: 0, patch_attempts: 18, edits: 0, producer_credit: 610, buyer_credit: 10879 }),
        ] {
            let (observed,trace)=run(mode,&mixed);
            println!("mixed {mode}: {observed:?}");
            assert_eq!(observed,expected);
            if mode=="adaptive" { on=trace; } else if mode=="edit_off" { off=trace; }
        }
        assert_eq!(on.iter().zip(off.iter()).take_while(|(a,b)|a==b).count(),216);
    }
    {
        let mut on = Vec::new(); let mut off = Vec::new();
        for (mode,expected) in [
            ("fresh", Row { completed: 24, steps: 3311, probes: 24, sales: 24, invalid_candidates: 0, patch_attempts: 0, edits: 0, producer_credit: 820, buyer_credit: 11089 }),
            ("cache", Row { completed: 19, steps: 3999, probes: 20, sales: 19, invalid_candidates: 19, patch_attempts: 0, edits: 0, producer_credit: 0, buyer_credit: 11221 }),
            ("adaptive", Row { completed: 24, steps: 3695, probes: 24, sales: 24, invalid_candidates: 0, patch_attempts: 0, edits: 0, producer_credit: 628, buyer_credit: 10897 }),
            ("edit_off", Row { completed: 24, steps: 3695, probes: 24, sales: 24, invalid_candidates: 0, patch_attempts: 0, edits: 0, producer_credit: 628, buyer_credit: 10897 }),
        ] {
            let (observed,trace)=run(mode,&fast);
            println!("fast {mode}: {observed:?}");
            assert_eq!(observed,expected);
            if mode=="adaptive" { on=trace; } else if mode=="edit_off" { off=trace; }
        }
        assert_eq!(on.iter().zip(off.iter()).take_while(|(a,b)|a==b).count(),3695);
    }
    {
        let mut on = Vec::new(); let mut off = Vec::new();
        for (mode,expected) in [
            ("fresh", Row { completed: 24, steps: 3311, probes: 24, sales: 24, invalid_candidates: 0, patch_attempts: 0, edits: 0, producer_credit: 820, buyer_credit: 11089 }),
            ("cache", Row { completed: 24, steps: 3599, probes: 13, sales: 24, invalid_candidates: 12, patch_attempts: 0, edits: 0, producer_credit: 742, buyer_credit: 10879 }),
            ("adaptive", Row { completed: 24, steps: 3053, probes: 14, sales: 24, invalid_candidates: 0, patch_attempts: 1, edits: 1, producer_credit: 1009, buyer_credit: 11158 }),
            ("edit_off", Row { completed: 24, steps: 3693, probes: 24, sales: 24, invalid_candidates: 0, patch_attempts: 11, edits: 0, producer_credit: 629, buyer_credit: 10898 }),
        ] {
            let (observed,trace)=run(mode,&switch);
            println!("switch {mode}: {observed:?}");
            assert_eq!(observed,expected);
            if mode=="adaptive" { on=trace; } else if mode=="edit_off" { off=trace; }
        }
        assert_eq!(on.iter().zip(off.iter()).take_while(|(a,b)|a==b).count(),2064);
    }
    {
        let mut on = Vec::new(); let mut off = Vec::new();
        for (mode,expected) in [
            ("fresh", Row { completed: 24, steps: 3311, probes: 24, sales: 24, invalid_candidates: 0, patch_attempts: 0, edits: 0, producer_credit: 820, buyer_credit: 11089 }),
            ("cache", Row { completed: 24, steps: 3647, probes: 13, sales: 24, invalid_candidates: 12, patch_attempts: 0, edits: 0, producer_credit: 718, buyer_credit: 10855 }),
            ("adaptive", Row { completed: 24, steps: 3125, probes: 14, sales: 24, invalid_candidates: 1, patch_attempts: 2, edits: 2, producer_credit: 973, buyer_credit: 11122 }),
            ("edit_off", Row { completed: 24, steps: 3693, probes: 24, sales: 24, invalid_candidates: 0, patch_attempts: 11, edits: 0, producer_credit: 629, buyer_credit: 10898 }),
        ] {
            let (observed,trace)=run(mode,&reverse);
            println!("reverse {mode}: {observed:?}");
            assert_eq!(observed,expected);
            if mode=="adaptive" { on=trace; } else if mode=="edit_off" { off=trace; }
        }
        assert_eq!(on.iter().zip(off.iter()).take_while(|(a,b)|a==b).count(),216);
    }
}
