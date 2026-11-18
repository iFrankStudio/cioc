//! CL-1B B56-F1d1j: strict single native PID1 paid mutation-rule memory,
//! feedback-based candidate generation, cost-surrogate and shift controls.
//! All 177 original CIL source nodes were PRECOMMITTED before this Rust source.
//! Authored ISA and policy grammar — NOT a new primitive or cyberlife.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const CREDITS:u64=4000;
const KEYS:[i64;16]=[7,11,20,21,31,32,33,50,51,52,53,54,56,61,62,63];
const PROBES:[i64;2]=[-7,5];
const HELDOUT:i64=11;
const FETCH_PC:usize=37;
const SERVICE_PC:usize=59;
const FEEDBACK_PC:usize=66;
const SCORE_PC:usize=72;
const RULE_WRITE_PC:usize=96;
const RULE_SHADOW_PC:usize=98;
const PROGRAM_WRITE_52_PC:usize=138;
const PROGRAM_WRITE_53_PC:usize=140;
const META_RULE_READ_PC:usize=111;
const FALLBACK_RULE_READ_PC:usize=125;
const HELDOUT_STAGE_PC:usize=152;
const INITIAL_RULE_R1_PC:usize=32;

fn founder()->Vec<Op>{
    vec![
        Op::Const{dst:2,value:31,next:1}, // pc 0
        Op::Write{address:2,value:0,next:2}, // pc 1
        Op::Const{dst:2,value:32,next:3}, // pc 2
        Op::Write{address:2,value:0,next:4}, // pc 3
        Op::Const{dst:2,value:33,next:5}, // pc 4
        Op::Write{address:2,value:0,next:6}, // pc 5
        Op::Const{dst:2,value:50,next:7}, // pc 6
        Op::Write{address:2,value:0,next:8}, // pc 7
        Op::Const{dst:2,value:51,next:9}, // pc 8
        Op::Write{address:2,value:0,next:10}, // pc 9
        Op::Const{dst:2,value:61,next:11}, // pc 10
        Op::Write{address:2,value:0,next:12}, // pc 11
        Op::Const{dst:2,value:62,next:13}, // pc 12
        Op::Write{address:2,value:0,next:14}, // pc 13
        Op::Const{dst:2,value:63,next:15}, // pc 14
        Op::Write{address:2,value:0,next:16}, // pc 15
        Op::Const{dst:2,value:52,next:17}, // pc 16
        Op::Const{dst:3,value:3,next:18}, // pc 17
        Op::Write{address:2,value:3,next:19}, // pc 18
        Op::Const{dst:2,value:53,next:20}, // pc 19
        Op::Const{dst:3,value:3,next:21}, // pc 20
        Op::Write{address:2,value:3,next:22}, // pc 21
        Op::Const{dst:2,value:54,next:23}, // pc 22
        Op::Const{dst:3,value:3,next:24}, // pc 23
        Op::Write{address:2,value:3,next:25}, // pc 24
        Op::Const{dst:2,value:21,next:26}, // pc 25
        Op::Read{dst:3,address:2,next:27}, // pc 26
        Op::Const{dst:2,value:2,next:28}, // pc 27
        Op::Sub{dst:3,lhs:3,rhs:2,next:29}, // pc 28
        Op::Branch{test:3,nonzero:33,zero:30}, // pc 29
        Op::Const{dst:2,value:63,next:31}, // pc 30
        Op::Const{dst:3,value:1,next:32}, // pc 31
        Op::Write{address:2,value:3,next:33}, // pc 32
        Op::Const{dst:2,value:50,next:34}, // pc 33
        Op::Read{dst:3,address:2,next:35}, // pc 34
        Op::Const{dst:2,value:51,next:36}, // pc 35
        Op::Add{dst:2,lhs:2,rhs:3,next:37}, // pc 36
        Op::Read{dst:3,address:2,next:38}, // pc 37
        Op::Branch{test:3,nonzero:39,zero:44}, // pc 38
        Op::Const{dst:2,value:1,next:40}, // pc 39
        Op::Sub{dst:3,lhs:3,rhs:2,next:41}, // pc 40
        Op::Branch{test:3,nonzero:42,zero:46}, // pc 41
        Op::Sub{dst:3,lhs:3,rhs:2,next:43}, // pc 42
        Op::Branch{test:3,nonzero:58,zero:48}, // pc 43
        Op::Const{dst:2,value:7,next:45}, // pc 44
        Op::Read{dst:1,address:2,next:50}, // pc 45
        Op::Const{dst:2,value:0,next:47}, // pc 46
        Op::Sub{dst:1,lhs:2,rhs:1,next:50}, // pc 47
        Op::Const{dst:3,value:2,next:49}, // pc 48
        Op::Add{dst:1,lhs:1,rhs:3,next:50}, // pc 49
        Op::Const{dst:2,value:56,next:51}, // pc 50
        Op::Write{address:2,value:1,next:52}, // pc 51
        Op::Const{dst:2,value:50,next:53}, // pc 52
        Op::Read{dst:3,address:2,next:54}, // pc 53
        Op::Const{dst:2,value:1,next:55}, // pc 54
        Op::Add{dst:3,lhs:3,rhs:2,next:56}, // pc 55
        Op::Const{dst:2,value:50,next:57}, // pc 56
        Op::Write{address:2,value:3,next:33}, // pc 57
        Op::Const{dst:2,value:11,next:59}, // pc 58
        Op::Write{address:2,value:1,next:60}, // pc 59
        Op::Const{dst:2,value:32,next:61}, // pc 60
        Op::Read{dst:3,address:2,next:62}, // pc 61
        Op::Const{dst:2,value:2,next:63}, // pc 62
        Op::Sub{dst:3,lhs:3,rhs:2,next:64}, // pc 63
        Op::Branch{test:3,nonzero:65,zero:155}, // pc 64
        Op::Const{dst:2,value:20,next:66}, // pc 65
        Op::Read{dst:1,address:2,next:67}, // pc 66
        Op::Branch{test:1,nonzero:68,zero:73}, // pc 67
        Op::Const{dst:2,value:61,next:69}, // pc 68
        Op::Read{dst:3,address:2,next:70}, // pc 69
        Op::Const{dst:1,value:1,next:71}, // pc 70
        Op::Add{dst:3,lhs:3,rhs:1,next:72}, // pc 71
        Op::Write{address:2,value:3,next:73}, // pc 72
        Op::Const{dst:2,value:32,next:74}, // pc 73
        Op::Read{dst:3,address:2,next:75}, // pc 74
        Op::Const{dst:1,value:1,next:76}, // pc 75
        Op::Add{dst:3,lhs:3,rhs:1,next:77}, // pc 76
        Op::Write{address:2,value:3,next:78}, // pc 77
        Op::Const{dst:2,value:2,next:79}, // pc 78
        Op::Sub{dst:3,lhs:3,rhs:2,next:80}, // pc 79
        Op::Branch{test:3,nonzero:153,zero:81}, // pc 80
        Op::Const{dst:2,value:61,next:82}, // pc 81
        Op::Read{dst:3,address:2,next:83}, // pc 82
        Op::Const{dst:2,value:2,next:84}, // pc 83
        Op::Sub{dst:3,lhs:3,rhs:2,next:85}, // pc 84
        Op::Branch{test:3,nonzero:99,zero:86}, // pc 85
        Op::Const{dst:2,value:33,next:87}, // pc 86
        Op::Read{dst:3,address:2,next:88}, // pc 87
        Op::Branch{test:3,nonzero:89,zero:150}, // pc 88
        Op::Sub{dst:3,lhs:3,rhs:1,next:90}, // pc 89
        Op::Const{dst:2,value:21,next:91}, // pc 90
        Op::Read{dst:2,address:2,next:92}, // pc 91
        Op::Const{dst:1,value:1,next:93}, // pc 92
        Op::Sub{dst:2,lhs:2,rhs:1,next:94}, // pc 93
        Op::Branch{test:2,nonzero:97,zero:95}, // pc 94
        Op::Const{dst:2,value:63,next:96}, // pc 95
        Op::Write{address:2,value:3,next:150}, // pc 96
        Op::Const{dst:2,value:56,next:98}, // pc 97
        Op::Write{address:2,value:3,next:150}, // pc 98
        Op::Const{dst:2,value:62,next:100}, // pc 99
        Op::Read{dst:3,address:2,next:101}, // pc 100
        Op::Const{dst:1,value:1,next:102}, // pc 101
        Op::Add{dst:3,lhs:3,rhs:1,next:103}, // pc 102
        Op::Write{address:2,value:3,next:104}, // pc 103
        Op::Const{dst:2,value:3,next:105}, // pc 104
        Op::Sub{dst:3,lhs:3,rhs:2,next:106}, // pc 105
        Op::Branch{test:3,nonzero:107,zero:124}, // pc 106
        Op::Const{dst:2,value:33,next:108}, // pc 107
        Op::Read{dst:3,address:2,next:109}, // pc 108
        Op::Branch{test:3,nonzero:115,zero:110}, // pc 109
        Op::Const{dst:2,value:63,next:111}, // pc 110
        Op::Read{dst:3,address:2,next:112}, // pc 111
        Op::Add{dst:3,lhs:3,rhs:1,next:113}, // pc 112
        Op::Const{dst:2,value:33,next:114}, // pc 113
        Op::Write{address:2,value:3,next:129}, // pc 114
        Op::Const{dst:2,value:1,next:116}, // pc 115
        Op::Sub{dst:3,lhs:3,rhs:2,next:117}, // pc 116
        Op::Branch{test:3,nonzero:121,zero:118}, // pc 117
        Op::Const{dst:3,value:2,next:119}, // pc 118
        Op::Const{dst:2,value:33,next:120}, // pc 119
        Op::Write{address:2,value:3,next:129}, // pc 120
        Op::Const{dst:3,value:1,next:122}, // pc 121
        Op::Const{dst:2,value:33,next:123}, // pc 122
        Op::Write{address:2,value:3,next:129}, // pc 123
        Op::Const{dst:2,value:63,next:125}, // pc 124
        Op::Read{dst:3,address:2,next:126}, // pc 125
        Op::Add{dst:3,lhs:3,rhs:1,next:127}, // pc 126
        Op::Const{dst:2,value:33,next:128}, // pc 127
        Op::Write{address:2,value:3,next:129}, // pc 128
        Op::Const{dst:2,value:33,next:130}, // pc 129
        Op::Read{dst:3,address:2,next:131}, // pc 130
        Op::Sub{dst:3,lhs:3,rhs:1,next:132}, // pc 131
        Op::Branch{test:3,nonzero:135,zero:133}, // pc 132
        Op::Const{dst:1,value:1,next:134}, // pc 133
        Op::Const{dst:3,value:2,next:137}, // pc 134
        Op::Const{dst:1,value:1,next:136}, // pc 135
        Op::Const{dst:3,value:3,next:137}, // pc 136
        Op::Const{dst:2,value:52,next:138}, // pc 137
        Op::Write{address:2,value:1,next:139}, // pc 138
        Op::Const{dst:2,value:53,next:140}, // pc 139
        Op::Write{address:2,value:3,next:141}, // pc 140
        Op::Const{dst:2,value:62,next:142}, // pc 141
        Op::Read{dst:3,address:2,next:143}, // pc 142
        Op::Const{dst:2,value:3,next:144}, // pc 143
        Op::Sub{dst:3,lhs:3,rhs:2,next:145}, // pc 144
        Op::Branch{test:3,nonzero:146,zero:150}, // pc 145
        Op::Const{dst:2,value:32,next:147}, // pc 146
        Op::Write{address:2,value:0,next:148}, // pc 147
        Op::Const{dst:2,value:61,next:149}, // pc 148
        Op::Write{address:2,value:0,next:153}, // pc 149
        Op::Const{dst:2,value:32,next:151}, // pc 150
        Op::Const{dst:3,value:2,next:152}, // pc 151
        Op::Write{address:2,value:3,next:153}, // pc 152
        Op::Const{dst:2,value:50,next:154}, // pc 153
        Op::Write{address:2,value:0,next:33}, // pc 154
        Op::Const{dst:2,value:31,next:156}, // pc 155
        Op::Read{dst:3,address:2,next:157}, // pc 156
        Op::Const{dst:1,value:1,next:158}, // pc 157
        Op::Add{dst:3,lhs:3,rhs:1,next:159}, // pc 158
        Op::Write{address:2,value:3,next:160}, // pc 159
        Op::Const{dst:2,value:2,next:161}, // pc 160
        Op::Sub{dst:3,lhs:3,rhs:2,next:162}, // pc 161
        Op::Branch{test:3,nonzero:163,zero:176}, // pc 162
        Op::Const{dst:2,value:33,next:164}, // pc 163
        Op::Write{address:2,value:0,next:165}, // pc 164
        Op::Const{dst:2,value:32,next:166}, // pc 165
        Op::Write{address:2,value:0,next:167}, // pc 166
        Op::Const{dst:2,value:61,next:168}, // pc 167
        Op::Write{address:2,value:0,next:169}, // pc 168
        Op::Const{dst:2,value:62,next:170}, // pc 169
        Op::Write{address:2,value:0,next:171}, // pc 170
        Op::Const{dst:3,value:3,next:172}, // pc 171
        Op::Const{dst:2,value:52,next:173}, // pc 172
        Op::Write{address:2,value:3,next:174}, // pc 173
        Op::Const{dst:2,value:53,next:175}, // pc 174
        Op::Write{address:2,value:3,next:153}, // pc 175
        Op::Halt, // pc 176
    ]
}
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Task{Negate,Composed,Unreachable}
impl Task{
    fn name(self)->&'static str{match self{
        Self::Negate=>"NEGATE",Self::Composed=>"COMPOSED",
        Self::Unreachable=>"UNREACHABLE"}}
    fn value(self,x:i64)->i64{match self{
        Self::Negate=>-x,Self::Composed=>-x+2,Self::Unreachable=>x+4}}
}
const SCHEDULES:[[Task;2];4]=[
    [Task::Negate,Task::Negate],
    [Task::Composed,Task::Composed],
    [Task::Negate,Task::Composed],
    [Task::Negate,Task::Unreachable],
];
#[derive(Copy,Clone)]
enum Signal{Truth,BlindSuccess,Inverted,Absent,Phase0CDecoy}
impl Signal{
    fn name(self)->&'static str{match self{
        Self::Truth=>"TRUTH",Self::BlindSuccess=>"BLIND_SUCCESS",
        Self::Inverted=>"INVERTED",Self::Absent=>"ABSENT",
        Self::Phase0CDecoy=>"PHASE0_C_DECOY"}}
    fn bit(self,correct:i64,phase:i64,mode:i64)->Option<i64>{match self{
        Self::Truth=>Some(correct),
        Self::BlindSuccess=>Some(1),
        Self::Inverted=>Some(1-correct),
        Self::Absent=>None,
        Self::Phase0CDecoy=>Some(if phase==0&&mode==1{1}else{correct}),
    }}
}
fn emit(out:&mut BufWriter<File>,row:Value){
    serde_json::to_writer(&mut *out,&row).unwrap();
    out.write_all(b"\n").unwrap();
}
fn fnv_code(code:&[Op])->String{
    let bytes=serde_json::to_vec(code).unwrap();
    let hash=bytes.iter().fold(14695981039346656037_u64,|h,b|{
        (h^u64::from(*b)).wrapping_mul(1099511628211)
    });
    format!("{hash:016x}")
}
fn snap(w:&World,codehash:&str)->Value{
    let p=w.process(1).unwrap();
    json!([p.pc,p.registers,p.credits,p.stopped,p.code.nodes.len(),codehash])
}
fn cells(w:&World)->Value{json!(KEYS.map(|key|w.cell(key)))}
fn program(w:&World)->Value{json!([51,52,53,54].map(|key|w.cell(key)))}
fn external(out:&mut BufWriter<File>,w:&mut World,id:usize,key:i64,value:i64,
    reason:&str,codehash:&str){
    assert!(matches!(key,7|20|21),"host may write ONLY input, feedback, gate");
    w.intervene(Intervention::SetCell{key,value}).unwrap();
    let e=w.events().last().unwrap();
    assert!(matches!(e.origin,Origin::External));
    assert!(w.used_cells()<=16&&w.occupied_units()<=300);
    emit(out,json!(["ext",id,e.tick,key,value,reason,
        snap(w,codehash),w.occupied_units(),cells(w)]));
}
fn case(out:&mut BufWriter<File>,id:usize,schedule_id:usize,
    signal:Signal,gate:i64)->Value{
    let source=founder();
    assert_eq!(source.len(),177);
    let hash=fnv_code(&source);
    let tasks=SCHEDULES[schedule_id];
    let mut w=World::with_strict_capacity(16,180,300);
    w.load(Process::new(1,source.clone(),CREDITS)).unwrap();
    emit(out,json!(["case",id,schedule_id,signal.name(),gate,
        tasks.map(|task|task.name()),source,hash,w.occupied_units(),
        CREDITS,16,180,300]));
    external(out,&mut w,id,21,gate,"single_readonly_experiment_gate",&hash);
    external(out,&mut w,id,7,PROBES[0],"phase0_probe0_input",&hash);

    let mut paid=0_usize;
    let mut services:Vec<Value>=Vec::new();
    let mut paid_feedback_reads=0_usize;
    let mut score_writes=0_usize;
    let mut attempt_writes=0_usize;
    let mut rule_writes:Vec<Value>=Vec::new();
    let mut rule_shadow_writes:Vec<Value>=Vec::new();
    let mut program_writes:Vec<Value>=Vec::new();
    let mut rule_proposal_reads:Vec<Value>=Vec::new();
    let mut fallback_reads:Vec<Value>=Vec::new();
    let mut initial_rule_writes=0_usize;
    let mut opcode_fetches=0_usize;
    let mut accumulator_writes=0_usize;
    let mut peak=w.occupied_units();
    let mut paid_at_heldout=Vec::new();
    let mut env_count=2_usize;
    while w.step(){
        paid+=1;
        assert!(paid<=CREDITS as usize,"strict finite PID1 compute budget exceeded");
        let e=w.events().last().unwrap().clone();
        assert!(matches!(e.origin,Origin::Internal(1)),"actual native CIL must be PID1");
        assert!(w.process(2).is_none(),"same-PID1 experiment spawned child");
        assert_eq!(w.process(1).unwrap().code.nodes,source,
            "native source graph changed during data-program research");
        assert_eq!(w.total_compute_inventory(),Some(CREDITS-paid as u64));
        assert!(w.used_cells()<=16&&w.occupied_units()<=300);
        peak=peak.max(w.occupied_units());
        if e.pc==FETCH_PC{
            let vpc=w.cell(50).unwrap();
            assert!((0..=3).contains(&vpc));
            let key=51+vpc;
            assert!(matches!(&e.kind,EventKind::CellRead{key:k,value}
                if *k==key&&Some(*value)==w.cell(key)));
            opcode_fetches+=1;
        }
        if e.pc==FEEDBACK_PC{
            assert!(matches!(&e.kind,EventKind::CellRead{key:20,value}
                if *value==w.cell(20).unwrap_or(0)));
            paid_feedback_reads+=1;
        }
        if e.pc==SCORE_PC{
            assert!(matches!(&e.kind,EventKind::CellWritten{key:61,..}));
            score_writes+=1;
        }
        if e.pc==INITIAL_RULE_R1_PC{
            assert_eq!(gate,2,"only frozen-R1 control may initialize rule1");
            assert!(matches!(&e.kind,EventKind::CellWritten{key:63,value:1}));
            initial_rule_writes+=1;
        }
        if e.pc==RULE_WRITE_PC{
            assert_eq!(gate,1);
            if let EventKind::CellWritten{key:63,value}= &e.kind{
                rule_writes.push(json!([w.cell(31),w.cell(33),value,e.tick]));
            }else{panic!("paid rule-update native PC without Write63");}
        }
        if e.pc==RULE_SHADOW_PC{
            assert!(gate==0||gate==2);
            if let EventKind::CellWritten{key:56,value}=&e.kind{
                rule_shadow_writes.push(json!([w.cell(31),w.cell(33),value,e.tick]));
            }else{panic!("shadow-rule write PC not shadow key56");}
        }
        if e.pc==PROGRAM_WRITE_52_PC||e.pc==PROGRAM_WRITE_53_PC{
            if let EventKind::CellWritten{key,value}=&e.kind{
                assert!(matches!(*key,52|53));
                program_writes.push(json!([w.cell(31),w.cell(33),key,value,e.tick]));
            }else{panic!("actor code slot not written");}
        }
        if e.pc==META_RULE_READ_PC{
            if let EventKind::CellRead{key:63,value}=&e.kind{
                rule_proposal_reads.push(json!([w.cell(31),w.cell(33),value,e.tick]));
            }else{panic!("source rule-proposal read missing");}
        }
        if e.pc==FALLBACK_RULE_READ_PC{
            if let EventKind::CellRead{key:63,value}=&e.kind{
                fallback_reads.push(json!([w.cell(31),w.cell(33),value,e.tick]));
            }else{panic!("source paid fallback rule read absent");}
        }
        if e.pc==103 {
            assert!(matches!(&e.kind,EventKind::CellWritten{key:62,..}));
            attempt_writes+=1;
        }
        if e.pc==51 {
            assert!(matches!(&e.kind,EventKind::CellWritten{key:56,..}));
            accumulator_writes+=1;
        }
        let mut just_probe:Option<(usize,usize,i64,i64)>=None;
        let mut finished_heldout:Option<usize>=None;
        if let EventKind::CellWritten{key:11,value}=&e.kind{
            assert_eq!(e.pc,SERVICE_PC);
            let phase=w.cell(31).unwrap();
            let stage=w.cell(32).unwrap();
            let mode=w.cell(33).unwrap();
            assert!((0..2).contains(&phase)&&(0..=2).contains(&stage));
            assert!((0..=2).contains(&mode));
            services.push(json!([phase,stage,mode,value,program(&w),w.cell(63)]));
            if stage<2{
                just_probe=Some((phase as usize,stage as usize,mode,*value));
            }else{
                finished_heldout=Some(phase as usize);
                paid_at_heldout.push(paid);
            }
        }
        emit(out,json!(["step",id,paid,e.tick,1,e.pc,e.kind,
            snap(&w,&hash),w.occupied_units(),cells(&w)]));
        if let Some((phase,stage,mode,output))=just_probe{
            let truth=i64::from(output==tasks[phase].value(PROBES[stage]));
            if let Some(bit)=signal.bit(truth,phase as i64,mode){
                external(out,&mut w,id,20,bit,"post_real_probe_feedback",&hash);
                env_count+=1;
            }
            external(out,&mut w,id,7,
                if stage==0{PROBES[1]}else{PROBES[0]},
                "next_probe_default_input",&hash);
            env_count+=1;
        }
        if e.pc==HELDOUT_STAGE_PC{
            assert!(matches!(&e.kind,EventKind::CellWritten{key:32,value:2}));
            external(out,&mut w,id,7,HELDOUT,"actor_requested_heldout_input",&hash);
            env_count+=1;
        }
        if let Some(phase)=finished_heldout{
            if phase==0{
                external(out,&mut w,id,7,PROBES[0],"phase1_new_task_input",&hash);
                env_count+=1;
            }
        }
    }
    assert!(w.process(1).unwrap().stopped);
    assert_eq!(w.cell(31),Some(2));
    assert!(w.process(2).is_none());
    assert_eq!(paid_at_heldout.len(),2);
    assert_eq!(initial_rule_writes,usize::from(gate==2));
    assert!(rule_writes.iter().all(|x|x[2]==0||x[2]==1));
    assert!(fallback_reads.iter().all(|x|x[2]==0||x[2]==1));
    assert!(rule_proposal_reads.iter().all(|x|x[2]==0||x[2]==1));
    if gate==1{assert!(rule_shadow_writes.is_empty())}
    else{
        assert!(rule_writes.is_empty(),"rule-memory suppression was bypassed");
        assert_eq!(w.cell(63),Some(if gate==2{1}else{0}));
    }
    let heldout:Vec<Value>=services.iter()
        .filter(|s|s[1]==2).cloned().collect();
    assert_eq!(heldout.len(),2,"each of two task phases must execute real heldout");
    let phase_results:Vec<Value>=heldout.iter().map(|s|{
        let phase=s[0].as_u64().unwrap() as usize;
        let output=s[3].as_i64().unwrap();
        json!({
            "phase":phase,"task":tasks[phase].name(),
            "retained_mode":s[2],"output":output,
            "oracle_value":tasks[phase].value(HELDOUT),
            "correct":output==tasks[phase].value(HELDOUT),
            "bytecode":s[4],"learned_rule":s[5],
        })
    }).collect();
    let correct=phase_results.iter().filter(|x|x["correct"]==true).count();
    if matches!(signal,Signal::Truth){
        if schedule_id<3{assert_eq!(correct,2)}else{assert_eq!(correct,1)}
        if schedule_id==0&&gate==1{
            assert_eq!(w.cell(63),Some(1));
            assert_eq!(rule_proposal_reads.iter().find(|x|x[0]==1).unwrap()[2],1);
        }
        if schedule_id==3{
            assert_eq!(phase_results[1]["correct"],false);
            assert_eq!(phase_results[1]["oracle_value"],15);
            assert!(!fallback_reads.is_empty(),"unreachable task must reach source paid fallback");
            if gate==1{
                assert_eq!(phase_results[1]["retained_mode"],2);
                assert_eq!(phase_results[1]["output"],-11);
            }
        }
    }
    let phase_costs=[paid_at_heldout[0],
        paid_at_heldout[1]-paid_at_heldout[0]];
    let result=json!({
        "id":id,"schedule_id":schedule_id,"signal":signal.name(),"gate":gate,
        "two_phase_actual_service_events":services,
        "heldout_phase_results":phase_results,
        "heldout_correct_count":correct,
        "rule_write_events":rule_writes,"rule_shadow_events":rule_shadow_writes,
        "original_PID1_paid_program_slot_write_events":program_writes,
        "original_paid_rule_proposal_reads":rule_proposal_reads,
        "original_paid_fallback_rule_reads":fallback_reads,
        "paid_initial_rule1_writes":initial_rule_writes,
        "paid_feedback_reads":paid_feedback_reads,
        "paid_score_writes":score_writes,"paid_failed_candidate_writes":attempt_writes,
        "dynamic_opcode_fetches":opcode_fetches,
        "paid_accumulator_writes":accumulator_writes,
        "phase_costs_at_heldout_services":phase_costs,
        "charged_steps":paid,"external_interventions":env_count,
        "max_total_storage":peak,"final_rule":w.cell(63),
        "final_process":snap(&w,&hash),"final_cells":cells(&w),
        "remaining_credits":w.remaining_credits(),
        "same_PID1":true,"original_code_graph_unchanged":true,
        "invented_new_mutation_grammar":false,
        "b3_met":false,"b4_met":false,"cyberlife":false,
    });
    emit(out,json!(["end",id,result]));
    result
}

#[test]
fn b56_f1d1j_paid_mutation_rule_memory_and_counterfactual_next_program(){
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let mut out=BufWriter::new(File::create(folder.join(
        "b56_f1d1j_native_paid_mutation_rule_memory.jsonl")).unwrap());
    let frozen_manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../cl1b/studies/B56-F1d1j-exact-177-node-rule-memory-and-60-world-matrix-manifest-v1.md");
    let frozen_graph=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../cl1b/studies/B56-F1d1j-frozen-exact-177-node-source-v1.json");
    let checksum=|file:&std::path::Path|{
        let r=std::process::Command::new("sha256sum").arg(file).output().unwrap();
        assert!(r.status.success());
        String::from_utf8(r.stdout).unwrap()
            .split_whitespace().next().unwrap().to_owned()
    };
    let graph_bytes=fs::read(&frozen_graph).unwrap();
    let graph:Value=serde_json::from_slice(&graph_bytes).unwrap();
    let source=founder();
    assert_eq!(source.len(),177);
    for (i,orig) in source.iter().enumerate(){
        let frozen=&graph["code"][i];
        let kind=frozen["kind"].as_str().unwrap();
        let exact=if kind=="Halt"{json!("Halt")}else{
            let mut fields=serde_json::Map::new();
            for (key,value) in frozen.as_object().unwrap(){
                if key!="kind"{fields.insert(key.to_string(),value.clone());}
            }
            let mut entry=serde_json::Map::new();
            entry.insert(kind.to_string(),Value::Object(fields));
            Value::Object(entry)
        };
        assert_eq!(serde_json::to_value(orig).unwrap(),exact,
            "native initial PID1 source differs from frozen prereg node {i}");
    }
    assert_eq!(graph["allowed_cell_keys"],json!(KEYS));
    assert_eq!(graph["starting_credits"],CREDITS);
    assert_eq!(graph["limit_cells"],16);
    assert_eq!(graph["limit_code_nodes"],180);
    assert_eq!(graph["limit_total_units"],300);
    emit(&mut out,json!(["meta","B56-F1d1j",1,
       std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_string()),
       checksum(&frozen_manifest),fs::metadata(&frozen_manifest).unwrap().len(),
       checksum(&frozen_graph),graph_bytes.len(),60,CREDITS,16,180,300,
       source.len(),FETCH_PC,SERVICE_PC,FEEDBACK_PC,SCORE_PC,
       RULE_WRITE_PC,RULE_SHADOW_PC,PROGRAM_WRITE_52_PC,PROGRAM_WRITE_53_PC,
       META_RULE_READ_PC,FALLBACK_RULE_READ_PC,HELDOUT_STAGE_PC,
       INITIAL_RULE_R1_PC]));
    let mut records=Vec::new();
    for sid in 0..4{
        for signal in [
            Signal::Truth,Signal::BlindSuccess,Signal::Inverted,
            Signal::Absent,Signal::Phase0CDecoy,
        ]{
            for gate in [1_i64,0,2]{
                let id=records.len();
                records.push(case(&mut out,id,sid,signal,gate));
            }
        }
    }
    out.flush().unwrap();
    assert_eq!(records.len(),60);
    let selected=|sid:usize,signal:&str,gate:i64|->&Value{
        records.iter().find(|r|r["schedule_id"]==sid
            &&r["signal"]==signal&&r["gate"]==gate).unwrap()
    };
    let native_s0_truth=selected(0,"TRUTH",1);
    let native_s0_frozen=selected(0,"TRUTH",0);
    let native_s0_decoy=selected(0,"PHASE0_C_DECOY",1);
    assert_eq!(native_s0_truth["heldout_correct_count"],2);
    assert_eq!(native_s0_frozen["heldout_correct_count"],2);
    assert_eq!(native_s0_truth["phase_costs_at_heldout_services"][1].as_u64().unwrap()
        <native_s0_frozen["phase_costs_at_heldout_services"][1].as_u64().unwrap(),true,
        "learned B-first mutator must save *paid phase1* work over fixed R0");
    assert_eq!(native_s0_truth["original_paid_rule_proposal_reads"].as_array().unwrap()
        .iter().find(|r|r[0]==1).unwrap()[2],1);
    assert_eq!(native_s0_decoy["original_paid_rule_proposal_reads"].as_array().unwrap()
        .iter().find(|r|r[0]==1).unwrap()[2],0,
        "decoy must corrupt the learned preference, not simply final answer");
    assert_eq!(native_s0_truth["heldout_phase_results"][1]["correct"],true);
    assert_eq!(native_s0_decoy["heldout_phase_results"][0]["correct"],false);
    assert_eq!(selected(1,"TRUTH",1)["heldout_correct_count"],2);
    assert!(selected(1,"TRUTH",1)["phase_costs_at_heldout_services"][1]
        .as_u64().unwrap() < selected(1,"TRUTH",2)
        ["phase_costs_at_heldout_services"][1].as_u64().unwrap());
    let shift=selected(2,"TRUTH",1);
    assert_eq!(shift["heldout_correct_count"],2);
    assert!(shift["phase_costs_at_heldout_services"][1].as_u64().unwrap()
        >selected(2,"TRUTH",0)["phase_costs_at_heldout_services"][1].as_u64().unwrap(),
        "must retain negative-transfer cost evidence");
    let impossible=selected(3,"TRUTH",1);
    assert_eq!(impossible["heldout_correct_count"],1);
    assert_eq!(impossible["heldout_phase_results"][1]["output"],-11);
    assert_eq!(impossible["heldout_phase_results"][1]["oracle_value"],15);
    eprintln!("B56_F1D1J_NATIVE_RULE_MEMORY {}",json!({
        "native_worlds":60,
        "same_PIDs":1,
        "native_source_nodes":177,
        "simultaneous_data_address_union":16,
        "truth_S0_phase1_rule_learning_saves_paid_compute":true,
        "true_S1_mutable_Cfirst_saves_vs_fixed_R1":true,
        "S2_negative_transfer_cost_regression_preserved":true,
        "S3_impossible_fallback_still_wrong":true,
        "targeted_decoy_changes_future_program_mutation_rule":true,
        "authored_rule_grammar":true,"open_ended_invention":false,
        "N56_A_B_C":false,"B3":false,"B4":false,"cyberlife":false,
    }));
}
