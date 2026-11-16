//! CL-1B B46: last-writer identity is NOT authenticated functional truth.
//! A paid native write under the real actor PID between paid value Read and
//! paid provenance Read can yield a genuine source tag and still cause wrong
//! program retention. The interposed instruction and one-time external
//! source restoration are investigator-authored, not actor-invented.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
#[path = "cl1b_b42_self_assembled_probe.rs"]
mod b42_source;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    NativeWrong,
    NativeEcho,
    NopSpoof,
    OutsideLateWrong,
    NopGenuine,
    B45GuardSpoof,
    StaticCorrect,
}
const ARMS: [Arm; 7] = [
    Arm::NativeWrong, Arm::NativeEcho, Arm::NopSpoof,
    Arm::OutsideLateWrong, Arm::NopGenuine,
    Arm::B45GuardSpoof, Arm::StaticCorrect,
];

fn is_static(arm: Arm) -> bool { arm == Arm::StaticCorrect }
fn has_interposer(arm: Arm) -> bool {
    !matches!(arm, Arm::B45GuardSpoof | Arm::StaticCorrect)
}
fn is_spoof(arm: Arm) -> bool {
    !matches!(arm, Arm::NopGenuine | Arm::StaticCorrect)
}

fn founder(arm: Arm) -> Vec<Op> {
    if is_static(arm) {
        return b42_source::founder(b42_source::Arm::StaticCorrect);
    }
    let mut code = b42_source::founder(b42_source::Arm::ProbePlus);
    assert_eq!(code.len(), 38);
    // The exact B45 authored 4-op last-writer guard, unchanged. No repair
    // oracle and no more powerful observation or VM instruction.
    code[13] = Op::Branch {test: 1, nonzero: 14, zero: 38};
    code.push(Op::ReadLastWriter {dst: 3, address: 2, next: 39});
    code.push(Op::Const {dst: 2, value: 1, next: 40});
    code.push(Op::Sub {dst: 3, lhs: 3, rhs: 2, next: 41});
    code.push(Op::Branch {test: 3, nonzero: 14, zero: 26});
    assert_eq!(code.len(), 42);
    if has_interposer(arm) {
        // A single researcher-installed, charged CIL operation is reached
        // immediately after the real PC11 value Read. The lab will
        // externally restore PC11.next=12 after this FIRST execution.
        // Every such arm gets that restoration and the same one paid step.
        code[11] = Op::Read {dst: 1, address: 2, next: 42};
        code.push(match arm {
            Arm::NativeWrong =>
                Op::Write {address: 2, value: 3, next: 12}, // native PID1 writes 0
            Arm::NativeEcho =>
                Op::Write {address: 2, value: 0, next: 12}, // PID1 writes challenge
            _ =>
                Op::Const {dst: 3, value: 0, next: 12},     // 1 paid no-op-ish control
        });
        assert_eq!(code.len(), 43);
    }
    code
}

#[derive(Debug)]
struct Observation {
    arm: Arm, train: i64, holdout: i64, challenge: Option<i64>,
    first_output: Option<i64>, first_read_value: Option<i64>,
    native_interposer_value: Option<i64>,
    first_writer: Option<i64>, first_writer_world_value: Option<i64>,
    first_writer_pc: Option<usize>, first_verdict: Option<usize>,
    forged: usize, external_after_read: usize, source_restorations: usize,
    writer_senses: usize, read_events: usize,
    train_true: usize, train_wrong: usize,
    holdout_true: usize, holdout_wrong: usize,
    charged: usize, cloned: usize, halted: bool, restored_input: bool,
    final_router: usize, nodes: usize, max_storage: usize,
    wrong_current_value_accepted: bool,
    defective_source_accepted: bool,
}

fn run(arm: Arm, train: i64, holdout: i64) -> Observation {
    let mut world = World::with_strict_capacity(16, 80, 192)
        .with_native_executable_edit_trace()
        .with_cell_write_provenance();
    let code = founder(arm);
    let old_len = code.len();
    world.load(Process::new(1, code, 128)).unwrap();
    world.intervene(Intervention::SetCell{key:7,value:train}).unwrap();
    let mut challenge = None;
    let mut first_output = None;
    let mut first_read_value = None;
    let mut native_interposer_value = None;
    let mut first_writer = None;
    let mut first_writer_world_value = None;
    let mut first_writer_pc = None;
    let mut first_verdict = None;
    let mut forged = 0;
    let mut external_after_read = 0;
    let mut source_restorations = 0;
    let mut writer_senses = 0;
    let mut read_events = 0;
    let mut restored_input = false;
    let mut charged = 0_usize;
    let mut score = [[0_usize;2];2];
    let mut max_storage = world.occupied_units();
    let mut wrong_current_value_accepted = false;

    for (phase,input) in [train,holdout].into_iter().enumerate() {
        if phase==1 {
            world.intervene(Intervention::SetCell{key:7,value:input}).unwrap();
            assert_eq!(world.events().last().unwrap().origin, Origin::External);
        }
        for _ in 0..64 {
            if world.process(1).unwrap().stopped { break; }
            let inventory = world.total_compute_inventory().unwrap();
            assert!(world.step(), "B46 actor must actually pay for its CIL instruction");
            charged += 1;
            assert_eq!(world.total_compute_inventory(), Some(inventory-1));
            assert!(world.occupied_units() <= 192);
            max_storage = max_storage.max(world.occupied_units());
            let event = world.events().last().unwrap().clone();
            assert_eq!(event.origin, Origin::Internal(1));
            assert!(!matches!(&event.kind, EventKind::Trapped(_)));
            if phase==0 && !is_static(arm) {
                match &event.kind {
                    EventKind::CellWritten{key:11,value} if event.pc==10
                        && first_output.is_none() => {
                        first_output=Some(*value);
                        assert_eq!(*value,0);
                        assert_eq!(world.cell_last_writer(11),Some(1));
                        if is_spoof(arm) {
                            world.intervene(Intervention::SetCell{
                                key:11,value:train+1
                            }).unwrap();
                            assert_eq!(world.events().last().unwrap().origin,Origin::External);
                            assert_eq!(world.cell_last_writer(11),Some(0));
                            forged+=1;
                        }
                    }
                    EventKind::CellRead{key:11,value} if event.pc==11 => {
                        read_events+=1;
                        if first_read_value.is_none() {
                            first_read_value=Some(*value);
                            if arm==Arm::OutsideLateWrong {
                                // Same CIL NOP, same credits: externally
                                // change cell AFTER the value was read,
                                // BEFORE the paid writer observation.
                                world.intervene(Intervention::SetCell{
                                    key:11,value:0
                                }).unwrap();
                                assert_eq!(world.cell_last_writer(11),Some(0));
                                external_after_read+=1;
                            }
                        }
                    }
                    EventKind::CellWritten{key:11,value} if event.pc==42 => {
                        assert!(matches!(arm,Arm::NativeWrong|Arm::NativeEcho));
                        assert_eq!(world.cell_last_writer(11),Some(1));
                        native_interposer_value=Some(*value);
                    }
                    EventKind::CellWriterSensed{key:11,writer} => {
                        writer_senses+=1;
                        if first_writer.is_none() {
                            first_writer=Some(*writer);
                            first_writer_pc=Some(event.pc);
                            first_writer_world_value=world.cell(11);
                        }
                    }
                    EventKind::CellWritten{key:7,value} if event.pc==33 => {
                        assert_eq!(*value,train);
                        restored_input=true;
                    }
                    _=>{}
                }
                if event.pc==42 && has_interposer(arm) && source_restorations==0 {
                    // Only AFTER executing the interposer once, restore the
                    // B45 Read successor by a labeled EXTERNAL laboratory
                    // patch in every matched interposer arm. No host repair,
                    // free actor patch, or extra native computation.
                    world.intervene(Intervention::PatchEdge{
                        process:1,node:11,next:12
                    }).unwrap();
                    assert_eq!(world.events().last().unwrap().origin,Origin::External);
                    source_restorations+=1;
                }
                if event.pc==41 && first_verdict.is_none() {
                    first_verdict=Some(world.process(1).unwrap().pc);
                    if train+1!=0 && first_verdict==Some(26)
                        && world.cell(11)!=Some(train+1) {
                        wrong_current_value_accepted=true;
                    }
                }
            }
            if let EventKind::CellWritten{key:11,value} = event.kind {
                // Original diagnostic output is NOT task service.
                if is_static(arm) || phase==1 || restored_input {
                    score[phase][usize::from(value!=input)] += 1;
                }
            }
            if event.pc==7 && phase==0 && !is_static(arm) {
                if let Some(v)=world.cell(7){challenge=Some(v);}
            }
        }
    }
    assert_eq!(world.total_compute_inventory(),Some(128-charged as u64));
    assert!(world.process(2).is_none());
    let nodes=&world.process(1).unwrap().code.nodes;
    let router=if is_static(arm) {3} else {
        match &nodes[9] { Op::Const{next,..} => *next,
            _=>panic!("B46 exact B42 route unexpectedly modified") }
    };
    let result=Observation{
        arm,train,holdout,challenge,first_output,first_read_value,
        native_interposer_value,first_writer,first_writer_world_value,
        first_writer_pc,first_verdict,forged,external_after_read,
        source_restorations,writer_senses,read_events,
        train_true:score[0][0],train_wrong:score[0][1],
        holdout_true:score[1][0],holdout_wrong:score[1][1],
        charged,cloned:nodes.len()-old_len,halted:world.process(1).unwrap().stopped,
        restored_input,final_router:router,nodes:nodes.len(),max_storage,
        wrong_current_value_accepted,
        defective_source_accepted: first_verdict==Some(26)
            && train+1!=0 && router==10,
    };
    eprintln!("B46_NATIVE {:?}",result);
    result
}

#[test]
fn b46_actor_identity_and_disconnected_value_read_are_not_functional_attestation() {
    // Source program contains NO new CIL opcode compared with B45:
    // tag33 is the same paid origin sensor and all extra instructions
    // are ordinary, paid Const/Write with explicit lab one-shot routing.
    assert_eq!(founder(Arm::NopSpoof)[11],
        Op::Read{dst:1,address:2,next:42});
    assert_eq!(founder(Arm::NativeWrong)[42],
        Op::Write{address:2,value:3,next:12});
    assert_eq!(founder(Arm::NativeEcho)[42],
        Op::Write{address:2,value:0,next:12});
    assert_eq!(founder(Arm::NopSpoof)[42],
        Op::Const{dst:3,value:0,next:12});
    assert_eq!(founder(Arm::B45GuardSpoof).len(),42);
    let mut totals=[[0_usize;13];7];
    let mut worlds=0_usize;
    for train in [-1_i64,0,1,17] {
        for holdout in [-9_i64,23] {
            let rs:Vec<_>=ARMS.iter().map(|&arm|run(arm,train,holdout)).collect();
            worlds+=rs.len();
            for (i,o) in rs.iter().enumerate() {
                assert_eq!(o.arm,ARMS[i]);
                assert_eq!((o.train,o.holdout),(train,holdout));
                assert_eq!(o.charged,128);
                assert!(!o.halted);
                assert!(o.max_storage<=192);
                totals[i][0]+=o.train_true;
                totals[i][1]+=o.train_wrong;
                totals[i][2]+=o.holdout_true;
                totals[i][3]+=o.holdout_wrong;
                totals[i][4]+=o.charged;
                totals[i][5]+=o.cloned;
                totals[i][6]+=o.writer_senses;
                totals[i][7]+=usize::from(o.first_writer==Some(0));
                totals[i][8]+=usize::from(o.first_writer==Some(1));
                totals[i][9]+=usize::from(o.first_verdict==Some(26));
                totals[i][10]+=o.forged;
                totals[i][11]+=o.source_restorations;
                totals[i][12]+=usize::from(o.wrong_current_value_accepted);
                if !is_static(o.arm) {
                    assert_eq!(o.challenge,Some(train+1));
                    assert_eq!(o.first_output,Some(0));
                    assert_eq!(o.first_writer_pc,Some(38));
                    assert!(o.restored_input);
                    assert_eq!(o.nodes,42+usize::from(has_interposer(o.arm))
                        +o.cloned);
                }
                if has_interposer(o.arm) {
                    assert_eq!(o.source_restorations,1);
                } else { assert_eq!(o.source_restorations,0); }
            }
            let [wrong,echo,nop,late,genuine,previous,compact]=
                [&rs[0],&rs[1],&rs[2],&rs[3],&rs[4],&rs[5],&rs[6]];
            assert_eq!(wrong.first_read_value,Some(train+1));
            assert_eq!(echo.first_read_value,Some(train+1));
            assert_eq!(wrong.native_interposer_value,Some(0));
            assert_eq!(echo.native_interposer_value,Some(train+1));
            assert_eq!(wrong.first_writer,Some(1));
            assert_eq!(echo.first_writer,Some(1));
            assert_eq!(wrong.first_writer_world_value,Some(0));
            assert_eq!(echo.first_writer_world_value,Some(train+1));
            assert_eq!(wrong.first_verdict,Some(26));
            assert_eq!(echo.first_verdict,Some(26));
            assert_eq!(wrong.defective_source_accepted,train!=-1);
            assert_eq!(echo.defective_source_accepted,train!=-1);
            assert_eq!(wrong.final_router,10);
            assert_eq!(echo.final_router,10);
            assert_eq!(wrong.holdout_true,0);
            assert_eq!(echo.holdout_true,0);
            assert_eq!(wrong.holdout_wrong,13);
            assert_eq!(echo.holdout_wrong,13);
            assert_eq!(nop.first_writer,Some(0));
            assert_eq!(late.first_writer,Some(0));
            assert_eq!(nop.first_verdict,Some(14));
            assert_eq!(late.first_verdict,Some(14));
            assert_eq!(nop.holdout_true,late.holdout_true);
            assert_eq!(nop.holdout_wrong,late.holdout_wrong);
            assert_eq!(late.external_after_read,1);
            assert_eq!(nop.external_after_read,0);
            assert_eq!(previous.first_writer,Some(0));
            assert_eq!((previous.holdout_true,previous.holdout_wrong),(12,1));
            assert_eq!(compact.holdout_true,16);
            assert_eq!(compact.holdout_wrong,0);
            assert_eq!(genuine.forged,0);
        }
    }
    assert_eq!(worlds,56);
    eprintln!("B46_AGGREGATE {:?}",totals);
    // The seven vectors below were predicted by an independent B42/B45
    // Rust-source-parsed CIL interpreter BEFORE first B46 native CI:
    // [train true, train wrong, holdout true, holdout wrong, charged,
    //  clones, senses, first sensor0, first sensor1, first accepts,
    //  external spoofs, external one-shot patches, bad current accepts].
    assert_eq!(totals,[
        [14,42,0,104,1024,0,8,0,8,8,8,8,6],
        [14,42,0,104,1024,0,8,0,8,8,8,8,0],
        [24,0,104,0,1024,8,16,8,0,0,8,8,0],
        [24,0,104,0,1024,8,16,8,0,0,8,8,0],
        [24,14,72,26,1024,6,8,0,8,8,0,8,0],
        [24,0,96,8,1024,8,16,8,0,0,8,0,0],
        [128,0,128,0,1024,0,0,0,0,0,0,0,0],
    ],"B46 native evidence diverged from independent pre-native CIL forecast");
}
