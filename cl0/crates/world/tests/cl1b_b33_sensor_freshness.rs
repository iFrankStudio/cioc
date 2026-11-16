//! B33: A true paid sensor can be stale before an authored source edit.
//! Contrasts the SAME native B31 founder under three externally varied
//! neutral-pool arrival positions, preserving actual complete service writes.
use cl0_world::{EventKind,Intervention,Origin,Process,World,Op};
#[path="cl1b_b31_pool_sensing.rs"]
mod canonical_b31;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arrival { Early, AfterSense, AfterCommit }

#[derive(Debug)]
struct Trial {
    arm: &'static str,
    arrival: Arrival,
    packet: u64,
    native_senses: Vec<u64>,
    paid_attempts: usize,
    writes: usize,
    patched_request: i64,
    remaining_pool: u64,
    pc: usize,
    local: u64,
    executed_before_packet: usize,
}

fn step_paid(w:&mut World)->bool{
    let old=w.total_compute_inventory().unwrap();
    let len=w.events().len();
    let yes=w.step();
    assert_eq!(w.total_compute_inventory(),Some(old-u64::from(yes)));
    assert_eq!(w.events().len(),len+usize::from(yes));
    if yes{
        assert_eq!(w.events().last().unwrap().origin,Origin::Internal(1));
        assert!(!matches!(w.events().last().unwrap().kind,EventKind::Trapped(_) | EventKind::Halted));
    }
    yes
}
fn drain(w:&mut World)->usize{
    let mut n=0;
    while step_paid(w){
        n+=1;
        assert!(n<64,"finite neutral packet must forbid infinite work");
    }
    n
}
fn native_case(arm:canonical_b31::Arm,arrival:Arrival,packet:u64)->Trial{
    let arm_label=match arm {
        canonical_b31::Arm::Sense=>"SENSE",
        canonical_b31::Arm::BlindThree=>"BLIND3",
        canonical_b31::Arm::BlindFour=>"BLIND4",
    };
    assert_ne!(arm,canonical_b31::Arm::BlindFour);
    let mut w=World::with_strict_capacity(16,64,128)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    w.load(Process::new(1,canonical_b31::founder(arm),8)).unwrap();
    let mut before_packet=0;
    let pre=match arrival { Arrival::Early=>0,Arrival::AfterSense=>1,Arrival::AfterCommit=>8 };
    for _ in 0..pre{
        assert!(step_paid(&mut w),"B33 first eight founder attempts are affordable");
        before_packet+=1;
    }
    if arrival==Arrival::AfterSense{
        assert_eq!(w.process(1).unwrap().pc,1);
    }
    if arrival==Arrival::AfterCommit{
        assert_eq!(w.process(1).unwrap().pc,9);
        assert_eq!(w.process(1).unwrap().credits,0);
    }
    if packet>0{
        assert_eq!(w.available_compute_pool(),0);
        w.intervene(Intervention::SetComputePool{credits:packet}).unwrap();
        assert_eq!(w.events().last().unwrap().origin,Origin::External);
    }
    let attempts=before_packet+drain(&mut w);
    let senses=w.events().iter().filter_map(|e|match (&e.origin,&e.kind){
        (Origin::Internal(1),EventKind::ComputePoolSensed{available}) if e.pc==0=>Some(*available),
        _=>None
    }).collect::<Vec<_>>();
    let writes=w.events().iter().filter(|e|matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(1),EventKind::CellWritten{key:9,value:1}) if e.pc==12
    )).count();
    let edited=w.events().iter().filter(|e|
        e.origin==Origin::Internal(1)&&e.pc==7&&!matches!(e.kind,EventKind::Trapped(_))
    ).count();
    let literal=match &w.process(1).unwrap().code.nodes[8]{
        Op::Const{value,..}=>*value,
        _=>panic!("B31 founder executable policy literal lost")
    };
    assert_eq!(edited,1);
    assert_eq!(w.process(1).unwrap().credits,0);
    assert_eq!(w.process(1).unwrap().pc,9);
    assert!(w.process(2).is_none());
    assert_eq!(w.total_compute_inventory(),Some(8+packet-attempts as u64));
    assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),
        1+usize::from(packet>0));
    let trial=Trial{arm:arm_label,arrival,packet,native_senses:senses,paid_attempts:attempts,
        writes,patched_request:literal,remaining_pool:w.available_compute_pool(),
        pc:w.process(1).unwrap().pc,local:w.process(1).unwrap().credits,
        executed_before_packet:before_packet};
    eprintln!("B33_NATIVE {:?}",trial);
    trial
}

#[test]
fn b33_delayed_neutral_energy_invalidates_truthful_earlier_pool_sensing(){
    let mut number=0;
    for packet in [0_u64,4,5,6]{
        for arrival in [Arrival::Early,Arrival::AfterSense,Arrival::AfterCommit]{
            let dynamic=native_case(canonical_b31::Arm::Sense,arrival,packet);
            let fixed=native_case(canonical_b31::Arm::BlindThree,arrival,packet);
            number+=2;
            assert_eq!(dynamic.arm,"SENSE");
            assert_eq!(fixed.arm,"BLIND3");
            assert_eq!((dynamic.packet,fixed.packet),(packet,packet));
            assert_eq!((dynamic.pc,fixed.pc),(9,9));
            assert_eq!((dynamic.local,fixed.local),(0,0));
            let expected_read=if arrival==Arrival::Early {packet}else{0};
            assert_eq!(dynamic.native_senses,vec![expected_read],
                "truthful CIL pool observation, potentially stale after later external injection");
            assert!(fixed.native_senses.is_empty());
            assert_eq!(dynamic.patched_request,
                if arrival==Arrival::Early&&packet==4 {3}else{4});
            assert_eq!(fixed.patched_request,3);
            assert_eq!((dynamic.executed_before_packet,fixed.executed_before_packet),
                match arrival{Arrival::Early=>(0,0),Arrival::AfterSense=>(1,1),
                    Arrival::AfterCommit=>(8,8)});
            let expected_dynamic=if packet==4&&arrival!=Arrival::Early {0}
                else {usize::from(packet>=4)};
            let expected_fixed=usize::from(packet>=4);
            assert_eq!(dynamic.writes,expected_dynamic);
            assert_eq!(fixed.writes,expected_fixed);
            assert_eq!(dynamic.remaining_pool,
                match packet{
                    0=>0,
                    4 if arrival!=Arrival::Early=>4,
                    4=>0,
                    5=>0,
                    6=>1,
                    _=>unreachable!()
                });
            assert_eq!(fixed.remaining_pool,
                if packet>=4{packet-4}else{packet});
        }
    }
    assert_eq!(number,24);
}
