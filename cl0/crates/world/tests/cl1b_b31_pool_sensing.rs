//! CL-1B B31: generic charged neutral-pool telemetry allows a single
//! continuing CIL actor to choose and patch an executable request literal.
//! The threshold, policy branches and service target are human-written.
//! Mere conditional self-edit is NOT endogenous learning or digital life.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(crate) enum Arm { Sense, BlindThree, BlindFour }

pub(crate) fn founder(arm:Arm)->Vec<Op>{
    let observe=match arm {
        Arm::Sense=>Op::SenseComputePool{dst:0,next:1},
        Arm::BlindThree=>Op::Const{dst:0,value:4,next:1},
        Arm::BlindFour=>Op::Const{dst:0,value:0,next:1},
    };
    vec![
        observe,                                               // PC0: paid observation or charged blind control
        Op::Const{dst:2,value:4,next:2},                       // PC1 equality threshold
        Op::Sub{dst:0,lhs:0,rhs:2,next:3},                     // PC2
        Op::Branch{test:0,nonzero:5,zero:4},                    // PC3
        Op::Const{dst:0,value:3,next:6},                       // PC4: pool exactly4
        Op::Const{dst:0,value:4,next:6},                       // PC5: otherwise
        Op::Const{dst:1,value:8,next:7},                       // PC6: target executable literal
        Op::PatchConst{node:1,value:0,next:8},                  // PC7: actual in-CIL code mutation
        Op::Const{dst:0,value:4,next:9},                       // PC8: mutates to requested3 or remains4
        Op::DrawCompute{amount:0,next:10},                     // PC9: only zero-local-credit execution gateway
        Op::Const{dst:2,value:9,next:11},                      // PC10
        Op::Const{dst:3,value:1,next:12},                      // PC11
        Op::Write{address:2,value:3,next:9},                   // PC12: genuine function
    ]
}
fn drain(w:&mut World)->usize{
    let mut steps=0;
    loop{
        assert!(steps<60,"bounded common pool cannot fund an infinite service");
        let energy=w.total_compute_inventory().unwrap();
        let before=w.events().len();
        if !w.step(){
            assert_eq!(w.total_compute_inventory(),Some(energy));
            assert_eq!(w.events().len(),before);
            return steps;
        }
        steps+=1;
        assert_eq!(w.total_compute_inventory(),Some(energy-1),
            "sensor, branch, code edit and service all cost exactly one compute attempt");
        assert_eq!(w.events().len(),before+1);
        assert_eq!(w.events().last().unwrap().origin,Origin::Internal(1));
        assert!(!matches!(w.events().last().unwrap().kind,EventKind::Trapped(_)|EventKind::Halted));
    }
}
fn request_literal(w:&World)->i64{
    match &w.process(1).unwrap().code.nodes[8] {
        Op::Const{value,..}=>*value,
        _=>panic!("PC8 must remain an executable literal"),
    }
}
#[derive(Debug)]
struct Trial {
    arm:Arm,packet:u64,sensed:Vec<u64>,source_literal:i64,
    charged:usize,service_writes:usize,remaining_pool:u64,pc:usize,credits:u64,
    charged_patch:usize,
}
fn observe(arm:Arm,packet:u64)->Trial{
    let mut w=World::with_strict_capacity(16,64,128)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    let source=founder(arm);
    assert_eq!(source.len(),13);
    w.load(Process::new(1,source,8)).unwrap();
    assert!(w.process(2).is_none());
    assert_eq!(w.total_compute_inventory(),Some(8));
    if packet>0 {
        w.intervene(Intervention::SetComputePool{credits:packet}).unwrap();
        assert_eq!(w.events().last().unwrap().origin,Origin::External);
    }
    let charged=drain(&mut w);
    let sensed=w.events().iter().filter_map(|e|match (&e.origin,&e.kind){
        (Origin::Internal(1),EventKind::ComputePoolSensed{available}) if e.pc==0=>Some(*available),
        _=>None,
    }).collect::<Vec<_>>();
    let charged_patch=w.events().iter().filter(|e|
        e.origin==Origin::Internal(1)&&e.pc==7&&!matches!(e.kind,EventKind::Trapped(_))
    ).count();
    let service_writes=w.events().iter().filter(|e|matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(1),EventKind::CellWritten{key:9,value:1}) if e.pc==12
    )).count();
    assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),
        1+usize::from(packet>0));
    assert_eq!(w.total_compute_inventory(),Some(8+packet-charged as u64));
    assert!(w.process(2).is_none());
    assert!(w.occupied_units()<=128);
    assert_eq!(charged_patch,1,"the CIL policy must attempt exactly one actual executable edit");
    assert_eq!(w.process(1).unwrap().credits,0);
    assert_eq!(w.process(1).unwrap().pc,9,
        "the source must remain reachable at pool-paid Draw when energy is exhausted");
    let out=Trial{
        arm,packet,sensed,source_literal:request_literal(&w),
        charged,service_writes,remaining_pool:w.available_compute_pool(),
        pc:w.process(1).unwrap().pc,credits:w.process(1).unwrap().credits,
        charged_patch,
    };
    eprintln!("B31_NATIVE {:?}",out);
    out
}

#[test]
fn b31_paid_generic_pool_sensing_can_condition_native_source_mutation_but_not_create_fitness(){
    assert_eq!(Op::decode_instruction([31,0,1,0,0]).unwrap(),
        Op::SenseComputePool{dst:0,next:1});
    assert_eq!(Op::decode_instruction([23,0,1,0,0]).unwrap(),
        Op::DrawCompute{amount:0,next:1});
    assert!(Op::decode_instruction([31,-1,1,0,0]).is_err());
    assert!(Op::decode_instruction([34,0,1,0,0]).is_err());
    let mut worlds=0_usize;
    for packet in [0_u64,4,5,6] {
        let sensor=observe(Arm::Sense,packet);
        let low=observe(Arm::BlindThree,packet);
        let high=observe(Arm::BlindFour,packet);
        worlds+=3;
        assert_eq!(sensor.sensed,vec![packet],"true ambient state, not a host-defined fitness score");
        assert!(low.sensed.is_empty()&&high.sensed.is_empty(),
            "equally paid fake observations cannot see native shared pool");
        assert_eq!(sensor.source_literal,if packet==4 {3} else {4});
        assert_eq!(low.source_literal,3);
        assert_eq!(high.source_literal,4);
        assert_eq!((sensor.credits,low.credits,high.credits),(0,0,0));
        assert_eq!((sensor.pc,low.pc,high.pc),(9,9,9));
        assert_eq!((sensor.charged_patch,low.charged_patch,high.charged_patch),(1,1,1));
        match packet {
            0=>{
                assert_eq!((sensor.service_writes,low.service_writes,high.service_writes),(0,0,0));
                assert_eq!((sensor.charged,low.charged,high.charged),(8,8,8));
            }
            4=>{
                assert_eq!((sensor.service_writes,low.service_writes,high.service_writes),(1,1,0));
                assert_eq!((sensor.charged,low.charged,high.charged),(12,12,8));
                assert_eq!((sensor.remaining_pool,low.remaining_pool,high.remaining_pool),(0,0,4));
            }
            5=>{
                assert_eq!((sensor.service_writes,low.service_writes,high.service_writes),(1,1,1));
                assert_eq!((sensor.remaining_pool,low.remaining_pool,high.remaining_pool),(0,1,0));
            }
            6=>{
                assert_eq!((sensor.service_writes,low.service_writes,high.service_writes),(1,1,1));
                assert_eq!((sensor.remaining_pool,low.remaining_pool,high.remaining_pool),(1,2,1));
            }
            _=>unreachable!(),
        }
    }
    assert_eq!(worlds,12);
}

#[test]
fn b31_pool_sensor_is_not_a_free_zero_credit_reboot_and_bounds_signed_readback(){
    let mut dead=World::with_strict_capacity(16,64,128)
        .with_pool_paid_draw().with_recoverable_draw_shortage();
    dead.load(Process::new(1,founder(Arm::Sense),0)).unwrap();
    dead.intervene(Intervention::SetComputePool{credits:100}).unwrap();
    let events=dead.events().len();
    assert!(!dead.step(),"only a pending Draw can be pool funded at zero local credits");
    assert_eq!(dead.events().len(),events);
    assert_eq!(dead.process(1).unwrap().pc,0);
    assert_eq!(dead.process(1).unwrap().registers,[0;4]);
    assert_eq!(dead.available_compute_pool(),100);

    let oversized=(i64::MAX as u64)+1;
    let mut overflow=World::with_strict_capacity(16,64,128)
        .with_pool_paid_draw().with_recoverable_draw_shortage();
    overflow.load(Process::new(1,founder(Arm::Sense),8)).unwrap();
    overflow.intervene(Intervention::SetComputePool{credits:oversized}).unwrap();
    let total=overflow.total_compute_inventory().unwrap();
    assert!(overflow.step(),"invalid signed sensor read is a REAL paid trapped attempt");
    assert_eq!(overflow.total_compute_inventory(),Some(total-1));
    assert_eq!(overflow.process(1).unwrap().pc,0);
    assert!(overflow.process(1).unwrap().stopped);
    assert_eq!(overflow.process(1).unwrap().registers,[0;4]);
    assert!(matches!(overflow.events().last().unwrap().kind,
        EventKind::Trapped(ref message) if message.contains("compute pool exceeds i64")));
}
