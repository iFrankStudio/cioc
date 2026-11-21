//! Minimal original-VM regression for an opt-in, researcher-only one-Write
//! counterfactual. Never silently alter original CIL physics without opt-in.
use cl0_world::{EventKind,Op,Origin,Process,World};

fn actor() -> Process {
    Process::new(1,vec![
        Op::Const{dst:0,value:38,next:1},
        Op::Const{dst:1,value:1,next:2},
        Op::Write{address:0,value:1,next:3},
        Op::Const{dst:1,value:0,next:4},
        Op::Write{address:0,value:1,next:5},
        Op::Read{dst:2,address:0,next:6},
        Op::Halt,
    ],20)
}

fn run(config:Option<(usize,i64,i64)>)->World{
 let mut world=World::with_strict_capacity(16,180,300);
 if let Some((pc,key,value))=config{world=world.with_native_write_suppression(1,pc,key,value);}
 world.load(actor()).unwrap();
 while world.step(){}
 world
}

#[test]
fn paid_single_write_effect_is_the_only_suppressed_operation() {
 let native=run(None);
 let suppressed=run(Some((4,38,0)));
 let placebo=run(Some((7,38,0))); // unreachable PC7
 assert_eq!(native.cell(38),Some(0));
 assert_eq!(native.process(1).unwrap().registers[2],0);
 assert_eq!(suppressed.cell(38),Some(1));
 assert_eq!(suppressed.process(1).unwrap().registers[2],1);
 assert_eq!(native.events(),placebo.events(),"unmatched study gate changed native trace");
 assert_eq!(native.process(1),placebo.process(1));
 assert_eq!(native.total_compute_inventory(),Some(13));
 assert_eq!(suppressed.total_compute_inventory(),Some(13));
 assert_eq!(native.process(1).unwrap().pc,suppressed.process(1).unwrap().pc);
 let events=suppressed.events().iter().filter(|e|matches!(e.origin,Origin::Internal(1))).collect::<Vec<_>>();
 assert_eq!(events.len(),7);
 assert!(matches!(&events[2].kind,EventKind::CellWritten{key:38,value:1}));
 assert!(matches!(&events[4].kind,EventKind::NativeCellWriteSuppressed{key:38,value:0}));
 assert_eq!(events[4].pc,4);
 assert_eq!(events.iter().filter(|e|matches!(e.kind,EventKind::NativeCellWriteSuppressed{..})).count(),1);
 assert_eq!(events.iter().filter(|e|matches!(e.kind,EventKind::CellWritten{..})).count(),1);
 assert!(matches!(&events[6].kind,EventKind::Halted));
}

#[test]
fn mismatched_write_operand_and_wrong_opcode_fail_closed_after_charge() {
 let wrong_value=run(Some((4,38,1)));
 assert_eq!(wrong_value.cell(38),Some(1),"wrong value gate mutated cell");
 assert_eq!(wrong_value.total_compute_inventory(),Some(15));
 assert!(matches!(&wrong_value.events().last().unwrap().kind,EventKind::Trapped(message) if message.contains("operand mismatch")));
 let wrong_opcode=run(Some((3,38,0)));
 assert_eq!(wrong_opcode.cell(38),Some(1));
 assert_eq!(wrong_opcode.total_compute_inventory(),Some(16));
 assert!(matches!(&wrong_opcode.events().last().unwrap().kind,EventKind::Trapped(message) if message.contains("not Write")));
 let wrong_key=run(Some((4,39,0)));
 assert_eq!(wrong_key.cell(38),Some(1));
 assert_eq!(wrong_key.total_compute_inventory(),Some(15));
 assert!(matches!(&wrong_key.events().last().unwrap().kind,EventKind::Trapped(message) if message.contains("operand mismatch")));
}
