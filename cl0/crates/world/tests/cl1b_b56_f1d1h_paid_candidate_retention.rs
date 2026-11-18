//! CL-1B B56-F1d1h: exact frozen 166-node, strictly paid native PID1
//! candidate generation, 2-probe per-candidate score and reexecuted retention.
//! Researcher authored small ISA and exhaustive feedback rules: NOT cyberlife.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const STARTING_CREDITS:u64=1000;
const KEYS:[i64;16]=[7,11,20,21,31,32,50,51,52,53,54,55,56,61,62,63];
const PROBES:[i64;2]=[-7,5];
const HELDOUT:i64=11;
const FETCH_PC:usize=31;
const SERVICE_PC:usize=54;
const FEEDBACK_PC:usize=61;
const PAID_SCORE_WRITE_PC:usize=70;
const GENERATED_WRITE_PC:usize=104;
fn founder()->Vec<Op>{
    vec![
        Op::Const{dst:0,value:0,next:1}, // pc 0
        Op::Const{dst:2,value:31,next:2}, // pc 1
        Op::Write{address:2,value:0,next:3}, // pc 2
        Op::Const{dst:2,value:32,next:4}, // pc 3
        Op::Write{address:2,value:0,next:5}, // pc 4
        Op::Const{dst:2,value:50,next:6}, // pc 5
        Op::Write{address:2,value:0,next:7}, // pc 6
        Op::Const{dst:2,value:51,next:8}, // pc 7
        Op::Write{address:2,value:0,next:9}, // pc 8
        Op::Const{dst:2,value:52,next:10}, // pc 9
        Op::Const{dst:3,value:3,next:11}, // pc 10
        Op::Write{address:2,value:3,next:12}, // pc 11
        Op::Const{dst:2,value:53,next:13}, // pc 12
        Op::Const{dst:3,value:3,next:14}, // pc 13
        Op::Write{address:2,value:3,next:15}, // pc 14
        Op::Const{dst:2,value:54,next:16}, // pc 15
        Op::Const{dst:3,value:3,next:17}, // pc 16
        Op::Write{address:2,value:3,next:18}, // pc 17
        Op::Const{dst:2,value:55,next:19}, // pc 18
        Op::Const{dst:3,value:2,next:20}, // pc 19
        Op::Write{address:2,value:3,next:21}, // pc 20
        Op::Const{dst:2,value:61,next:22}, // pc 21
        Op::Write{address:2,value:0,next:23}, // pc 22
        Op::Const{dst:2,value:62,next:24}, // pc 23
        Op::Write{address:2,value:0,next:25}, // pc 24
        Op::Const{dst:2,value:63,next:26}, // pc 25
        Op::Write{address:2,value:0,next:27}, // pc 26
        Op::Const{dst:2,value:50,next:28}, // pc 27
        Op::Read{dst:3,address:2,next:29}, // pc 28
        Op::Const{dst:2,value:51,next:30}, // pc 29
        Op::Add{dst:2,lhs:2,rhs:3,next:31}, // pc 30
        Op::Read{dst:3,address:2,next:32}, // pc 31
        Op::Branch{test:3,nonzero:33,zero:38}, // pc 32
        Op::Const{dst:2,value:1,next:34}, // pc 33
        Op::Sub{dst:3,lhs:3,rhs:2,next:35}, // pc 34
        Op::Branch{test:3,nonzero:36,zero:40}, // pc 35
        Op::Sub{dst:3,lhs:3,rhs:2,next:37}, // pc 36
        Op::Branch{test:3,nonzero:53,zero:42}, // pc 37
        Op::Const{dst:2,value:7,next:39}, // pc 38
        Op::Read{dst:1,address:2,next:45}, // pc 39
        Op::Const{dst:2,value:0,next:41}, // pc 40
        Op::Sub{dst:1,lhs:2,rhs:1,next:45}, // pc 41
        Op::Const{dst:2,value:55,next:43}, // pc 42
        Op::Read{dst:3,address:2,next:44}, // pc 43
        Op::Add{dst:1,lhs:1,rhs:3,next:45}, // pc 44
        Op::Const{dst:2,value:56,next:46}, // pc 45
        Op::Write{address:2,value:1,next:47}, // pc 46
        Op::Const{dst:2,value:50,next:48}, // pc 47
        Op::Read{dst:3,address:2,next:49}, // pc 48
        Op::Const{dst:2,value:1,next:50}, // pc 49
        Op::Add{dst:3,lhs:3,rhs:2,next:51}, // pc 50
        Op::Const{dst:2,value:50,next:52}, // pc 51
        Op::Write{address:2,value:3,next:27}, // pc 52
        Op::Const{dst:2,value:11,next:54}, // pc 53
        Op::Write{address:2,value:1,next:55}, // pc 54
        Op::Const{dst:2,value:32,next:56}, // pc 55
        Op::Read{dst:3,address:2,next:57}, // pc 56
        Op::Const{dst:2,value:2,next:58}, // pc 57
        Op::Sub{dst:3,lhs:3,rhs:2,next:59}, // pc 58
        Op::Branch{test:3,nonzero:60,zero:165}, // pc 59
        Op::Const{dst:2,value:20,next:61}, // pc 60
        Op::Read{dst:1,address:2,next:62}, // pc 61
        Op::Branch{test:1,nonzero:63,zero:71}, // pc 62
        Op::Const{dst:2,value:31,next:64}, // pc 63
        Op::Read{dst:3,address:2,next:65}, // pc 64
        Op::Const{dst:2,value:61,next:66}, // pc 65
        Op::Add{dst:2,lhs:2,rhs:3,next:67}, // pc 66
        Op::Read{dst:3,address:2,next:68}, // pc 67
        Op::Const{dst:1,value:1,next:69}, // pc 68
        Op::Add{dst:3,lhs:3,rhs:1,next:70}, // pc 69
        Op::Write{address:2,value:3,next:71}, // pc 70
        Op::Const{dst:2,value:32,next:72}, // pc 71
        Op::Read{dst:3,address:2,next:73}, // pc 72
        Op::Const{dst:1,value:1,next:74}, // pc 73
        Op::Add{dst:3,lhs:3,rhs:1,next:75}, // pc 74
        Op::Write{address:2,value:3,next:76}, // pc 75
        Op::Const{dst:2,value:2,next:77}, // pc 76
        Op::Sub{dst:3,lhs:3,rhs:2,next:78}, // pc 77
        Op::Branch{test:3,nonzero:79,zero:81}, // pc 78
        Op::Const{dst:2,value:50,next:80}, // pc 79
        Op::Write{address:2,value:0,next:27}, // pc 80
        Op::Const{dst:2,value:32,next:82}, // pc 81
        Op::Write{address:2,value:0,next:83}, // pc 82
        Op::Const{dst:2,value:21,next:84}, // pc 83
        Op::Read{dst:3,address:2,next:85}, // pc 84
        Op::Const{dst:2,value:2,next:86}, // pc 85
        Op::Sub{dst:3,lhs:3,rhs:2,next:87}, // pc 86
        Op::Branch{test:3,nonzero:88,zero:107}, // pc 87
        Op::Const{dst:2,value:31,next:89}, // pc 88
        Op::Read{dst:3,address:2,next:90}, // pc 89
        Op::Const{dst:2,value:2,next:91}, // pc 90
        Op::Sub{dst:3,lhs:3,rhs:2,next:92}, // pc 91
        Op::Branch{test:3,nonzero:93,zero:107}, // pc 92
        Op::Const{dst:2,value:31,next:94}, // pc 93
        Op::Read{dst:3,address:2,next:95}, // pc 94
        Op::Const{dst:1,value:1,next:96}, // pc 95
        Op::Add{dst:3,lhs:3,rhs:1,next:97}, // pc 96
        Op::Write{address:2,value:3,next:98}, // pc 97
        Op::Const{dst:2,value:51,next:99}, // pc 98
        Op::Add{dst:2,lhs:2,rhs:3,next:100}, // pc 99
        Op::Const{dst:1,value:21,next:101}, // pc 100
        Op::Read{dst:1,address:1,next:102}, // pc 101
        Op::Branch{test:1,nonzero:103,zero:105}, // pc 102
        Op::Const{dst:0,value:0,next:104}, // pc 103
        Op::Write{address:2,value:3,next:79}, // pc 104
        Op::Const{dst:2,value:55,next:106}, // pc 105
        Op::Write{address:2,value:3,next:79}, // pc 106
        Op::Const{dst:2,value:61,next:108}, // pc 107
        Op::Read{dst:3,address:2,next:109}, // pc 108
        Op::Const{dst:2,value:2,next:110}, // pc 109
        Op::Sub{dst:3,lhs:3,rhs:2,next:111}, // pc 110
        Op::Branch{test:3,nonzero:112,zero:137}, // pc 111
        Op::Const{dst:2,value:62,next:113}, // pc 112
        Op::Read{dst:3,address:2,next:114}, // pc 113
        Op::Const{dst:2,value:2,next:115}, // pc 114
        Op::Sub{dst:3,lhs:3,rhs:2,next:116}, // pc 115
        Op::Branch{test:3,nonzero:117,zero:144}, // pc 116
        Op::Const{dst:2,value:63,next:118}, // pc 117
        Op::Read{dst:3,address:2,next:119}, // pc 118
        Op::Const{dst:2,value:2,next:120}, // pc 119
        Op::Sub{dst:3,lhs:3,rhs:2,next:121}, // pc 120
        Op::Branch{test:3,nonzero:122,zero:152}, // pc 121
        Op::Const{dst:2,value:61,next:123}, // pc 122
        Op::Read{dst:3,address:2,next:124}, // pc 123
        Op::Const{dst:2,value:1,next:125}, // pc 124
        Op::Sub{dst:3,lhs:3,rhs:2,next:126}, // pc 125
        Op::Branch{test:3,nonzero:127,zero:137}, // pc 126
        Op::Const{dst:2,value:62,next:128}, // pc 127
        Op::Read{dst:3,address:2,next:129}, // pc 128
        Op::Const{dst:2,value:1,next:130}, // pc 129
        Op::Sub{dst:3,lhs:3,rhs:2,next:131}, // pc 130
        Op::Branch{test:3,nonzero:132,zero:144}, // pc 131
        Op::Const{dst:2,value:63,next:133}, // pc 132
        Op::Read{dst:3,address:2,next:134}, // pc 133
        Op::Const{dst:2,value:1,next:135}, // pc 134
        Op::Sub{dst:3,lhs:3,rhs:2,next:136}, // pc 135
        Op::Branch{test:3,nonzero:137,zero:152}, // pc 136
        Op::Const{dst:2,value:31,next:138}, // pc 137
        Op::Write{address:2,value:0,next:139}, // pc 138
        Op::Const{dst:3,value:3,next:140}, // pc 139
        Op::Const{dst:2,value:52,next:141}, // pc 140
        Op::Write{address:2,value:3,next:142}, // pc 141
        Op::Const{dst:2,value:53,next:143}, // pc 142
        Op::Write{address:2,value:3,next:160}, // pc 143
        Op::Const{dst:3,value:1,next:145}, // pc 144
        Op::Const{dst:2,value:31,next:146}, // pc 145
        Op::Write{address:2,value:3,next:147}, // pc 146
        Op::Const{dst:2,value:52,next:148}, // pc 147
        Op::Write{address:2,value:3,next:149}, // pc 148
        Op::Const{dst:3,value:3,next:150}, // pc 149
        Op::Const{dst:2,value:53,next:151}, // pc 150
        Op::Write{address:2,value:3,next:160}, // pc 151
        Op::Const{dst:3,value:2,next:153}, // pc 152
        Op::Const{dst:2,value:31,next:154}, // pc 153
        Op::Write{address:2,value:3,next:155}, // pc 154
        Op::Const{dst:2,value:53,next:156}, // pc 155
        Op::Write{address:2,value:3,next:157}, // pc 156
        Op::Const{dst:3,value:1,next:158}, // pc 157
        Op::Const{dst:2,value:52,next:159}, // pc 158
        Op::Write{address:2,value:3,next:160}, // pc 159
        Op::Const{dst:2,value:32,next:161}, // pc 160
        Op::Const{dst:3,value:2,next:162}, // pc 161
        Op::Write{address:2,value:3,next:163}, // pc 162
        Op::Const{dst:2,value:50,next:164}, // pc 163
        Op::Write{address:2,value:0,next:27}, // pc 164
        Op::Halt, // pc 165
    ]
}
#[derive(Copy,Clone)]
enum Task{Identity,Negate,Composed}
impl Task{
    fn name(self)->&'static str{match self{
        Self::Identity=>"IDENTITY",Self::Negate=>"NEGATE",Self::Composed=>"COMPOSED"}}
    fn expected(self,x:i64)->i64{match self{
        Self::Identity=>x,Self::Negate=>-x,Self::Composed=>-x+2}}
    fn index(self)->usize{match self{
        Self::Identity=>0,Self::Negate=>1,Self::Composed=>2}}
}
#[derive(Copy,Clone)]
enum Signal{Truth,BlindSuccess,Inverted,Absent}
impl Signal{
    fn name(self)->&'static str{match self{
        Self::Truth=>"TRUTH",Self::BlindSuccess=>"BLIND_SUCCESS",
        Self::Inverted=>"INVERTED",Self::Absent=>"ABSENT"
    }}
    fn bit(self,correct:i64)->Option<i64>{match self{
        Self::Truth=>Some(correct),Self::BlindSuccess=>Some(1),
        Self::Inverted=>Some(1-correct),Self::Absent=>None}}
}
fn emit(out:&mut BufWriter<File>,row:Value){
    serde_json::to_writer(&mut *out,&row).unwrap();
    out.write_all(b"\n").unwrap();
}
fn fnv_code(code:&[Op])->String{
    let bytes=serde_json::to_vec(code).unwrap();
    let h=bytes.iter().fold(14695981039346656037_u64,|hash,b|{
        (hash^u64::from(*b)).wrapping_mul(1099511628211)
    });
    format!("{h:016x}")
}
fn snap(w:&World,codehash:&str)->Value{
    let p=w.process(1).unwrap();
    json!([p.pc,p.registers,p.credits,p.stopped,p.code.nodes.len(),codehash])
}
fn cells(w:&World)->Value{json!(KEYS.map(|key|w.cell(key)))}
fn external(out:&mut BufWriter<File>,w:&mut World,id:usize,key:i64,value:i64,
            tag:&str,codehash:&str){
    assert!(matches!(key,7|20|21),"host may only supply I/O and gate");
    w.intervene(Intervention::SetCell{key,value}).unwrap();
    let e=w.events().last().unwrap();
    assert!(matches!(e.origin,Origin::External));
    assert!(w.used_cells()<=16&&w.occupied_units()<=300);
    emit(out,json!(["ext",id,e.tick,key,value,tag,
        snap(w,codehash),w.occupied_units(),cells(w)]));
}
fn program(w:&World)->Vec<i64>{
    [51,52,53,54].map(|key|w.cell(key).unwrap()).to_vec()
}
fn run_case(out:&mut BufWriter<File>,id:usize,task:Task,signal:Signal,gate:i64)->Value{
    let code=founder();
    assert_eq!(code.len(),166);
    let codehash=fnv_code(&code);
    let mut w=World::with_strict_capacity(16,180,300);
    w.load(Process::new(1,code.clone(),STARTING_CREDITS)).unwrap();
    let frozen=gate==2;
    let expected_probe_services=if frozen{2}else{6};
    emit(out,json!(["case",id,task.name(),signal.name(),gate,
        code,codehash,w.occupied_units(),STARTING_CREDITS,
        PROBES,HELDOUT,expected_probe_services]));
    external(out,&mut w,id,21,gate,"gate_before_first_charge",&codehash);
    external(out,&mut w,id,7,PROBES[0],"initial_probe_input",&codehash);
    let mut paid=0_usize;
    let mut services:Vec<Value>=Vec::new();
    let mut paid_reads=0_usize;
    let mut scores=0_usize;
    let mut native_generated_writes=0_usize;
    let mut fetches=Vec::new();
    let mut checkpoints=Vec::new();
    let mut peak=w.occupied_units();
    while w.step(){
        paid+=1;
        assert!(paid<=STARTING_CREDITS as usize,"finite compute exhaustion");
        let e=w.events().last().unwrap().clone();
        assert!(matches!(e.origin,Origin::Internal(1)),"no child/external inside CIL");
        assert!(w.process(2).is_none(),"PID continuity failed");
        assert_eq!(w.process(1).unwrap().code.nodes,code,
            "native executable graph mutation forbidden");
        assert_eq!(w.total_compute_inventory(),Some(STARTING_CREDITS-paid as u64));
        assert!(w.used_cells()<=16&&w.occupied_units()<=300);
        peak=peak.max(w.occupied_units());
        if e.pc==FETCH_PC{
            let virtual_pc=w.cell(50).unwrap();
            assert!((0..=3).contains(&virtual_pc));
            let addr=51+virtual_pc;
            assert!(matches!(e.kind,EventKind::CellRead{key,value}
                if key==addr&&Some(value)==w.cell(addr)));
            fetches.push(json!([w.cell(31),w.cell(32),virtual_pc,w.cell(addr)]));
        }
        if e.pc==FEEDBACK_PC{
            let observed=w.cell(20).unwrap_or(0);
            assert!(matches!(e.kind,EventKind::CellRead{key:20,value} if value==observed));
            paid_reads+=1;
        }
        if e.pc==PAID_SCORE_WRITE_PC{
            if let EventKind::CellWritten{key,..}=e.kind{
                assert!((61..=63).contains(&key));scores+=1;
            }else{panic!("source scoring PC executed without score Write");}
        }
        if e.pc==GENERATED_WRITE_PC{
            if let EventKind::CellWritten{key,..}=e.kind{
                assert!(gate==1&&matches!(key,52|53));
                native_generated_writes+=1;
            }else{panic!("generation PC executed without program Write");}
        }
        if let EventKind::CellWritten{key:56,value}=e.kind{checkpoints.push(value);}
        let mut just_probe:Option<(usize,i64,i64)> = None;
        if let EventKind::CellWritten{key:11,value}=e.kind{
            assert_eq!(e.pc,SERVICE_PC);
            let candidate=w.cell(31).unwrap();
            let probe=w.cell(32).unwrap();
            services.push(json!([candidate,probe,value]));
            if probe!=2{
                assert!((0..=1).contains(&probe));
                assert!((0..=2).contains(&candidate));
                assert!(services.len()<=expected_probe_services);
                just_probe=Some((candidate as usize,probe,value));
            }else{
                assert_eq!(probe,2,"heldout event must carry reserved stage2");
            }
        }
        emit(out,json!(["step",id,paid,e.tick,1,e.pc,e.kind,
            snap(&w,&codehash),w.occupied_units(),cells(&w)]));
        if let Some((candidate,probe,output))=just_probe{
            let truth=i64::from(output==task.expected(PROBES[probe as usize]));
            if let Some(bit)=signal.bit(truth){
                external(out,&mut w,id,20,bit,"probe_correctness_feedback",&codehash);
            }
            let next=if probe==0{PROBES[1]}else if frozen||candidate==2{HELDOUT}
                else{PROBES[0]};
            external(out,&mut w,id,7,next,"next_probe_or_heldout_input",&codehash);
        }
    }
    assert!(w.process(1).unwrap().stopped);
    assert!(w.process(2).is_none());
    assert_eq!(services.len(),expected_probe_services+1);
    assert_eq!(paid_reads,expected_probe_services);
    assert!(paid<=STARTING_CREDITS as usize);
    let score_cells:[i64;3]=[61,62,63].map(|k|w.cell(k).unwrap());
    let mut best=0_usize;
    for i in 1..3{if score_cells[i]>score_cells[best]{best=i;}}
    let winner=w.cell(31).unwrap();
    assert_eq!(winner,best as i64,"winner MUST use paid candidate-local score comparisons");
    assert_eq!(w.cell(32),Some(2));
    let expected_code=match winner{
        0=>vec![0,3,3,3],1=>vec![0,1,3,3],2=>vec![0,1,2,3],
        _=>panic!("candidate winner is not a valid index")
    };
    let actual_code=program(&w);
    if gate==1 {assert_eq!(actual_code,expected_code)}
    else {assert_eq!(actual_code,vec![0,3,3,3])}
    if gate==2{
        assert_eq!(native_generated_writes,0);
        assert_eq!(scores,score_cells[0] as usize);
    }else if gate==1{
        assert_eq!(native_generated_writes,2);
    }else{
        assert_eq!(native_generated_writes,0);
    }
    let final_output=services.last().unwrap()[2].as_i64().unwrap();
    let heldout_correct=final_output==task.expected(HELDOUT);
    if gate==1&&matches!(signal,Signal::Truth){assert!(heldout_correct)}
    if gate==1&&matches!(signal,Signal::Inverted){assert!(!heldout_correct)}
    if gate==1&&matches!(signal,Signal::BlindSuccess|Signal::Absent){
        assert_eq!(heldout_correct,task.index()==0);
    }
    if gate!=1{assert_eq!(heldout_correct,task.index()==0)}
    let result=json!({
        "id":id,"task":task.name(),"signal":signal.name(),"gate":gate,
        "initial_program":[0,3,3,3],
        "probe_services":services.len()-1,"actual_paid_services":services,
        "actual_virtual_fetches":fetches,"accumulator_checkpoints":checkpoints,
        "score_cells":score_cells,"paid_score_updates":scores,
        "paid_generated_program_slot_edits":native_generated_writes,
        "retained_winner":winner,"reinstated_program":actual_code,
        "heldout_correct":heldout_correct,
        "charged_instructions":paid,"peak_total_storage":peak,
        "remaining_credits":w.remaining_credits(),"final_process":snap(&w,&codehash),
        "single_PID1":true,"source_code_unchanged":true,
        "native_opcode_generation":false,
        "b3_met":false,"b4_met":false,"cyberlife":false
    });
    emit(out,json!(["end",id,result]));
    result
}
#[test]
fn b56_f1d1h_paid_candidate_scoring_and_single_pid_retention(){
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let mut out=BufWriter::new(File::create(folder.join(
        "b56_f1d1h_native_paid_candidate_retention.jsonl")).unwrap());
    let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../cl1b/studies/B56-F1d1h-frozen-exact-source-and-36-world-controls-v1.md");
    let spec=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../cl1b/studies/B56-F1d1h-frozen-exact-CIL-source-v1.json");
    let hash=|path:&std::path::Path|{
        let o=std::process::Command::new("sha256sum").arg(path).output().unwrap();
        assert!(o.status.success());
        String::from_utf8(o.stdout).unwrap()
           .split_whitespace().next().unwrap().to_owned()
    };
    let text=fs::read(&spec).unwrap();
    let parsed:Value=serde_json::from_slice(&text).unwrap();
    let nodes=parsed["code"].as_array().unwrap();
    assert_eq!(nodes.len(),founder().len());
    for (i,op) in founder().iter().enumerate(){
        let data=serde_json::to_value(op).unwrap();
        let kind=nodes[i]["kind"].as_str().unwrap();
        let found=if kind=="Halt"{json!("Halt")}else{
            let mut fields=serde_json::Map::new();
            for (k,v) in nodes[i].as_object().unwrap(){
                if k!="kind"{fields.insert(k.clone(),v.clone());}
            }
            let mut outer=serde_json::Map::new();
            outer.insert(kind.to_string(),Value::Object(fields));
            Value::Object(outer)
        };
        assert_eq!(data,found,"native CIL differs from frozen preregistered opcode {i}");
    }
    let digest=hash(&manifest);let code_spec_digest=hash(&spec);
    emit(&mut out,json!(["meta","B56-F1d1h",1,
        std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_owned()),
        digest,fs::metadata(&manifest).unwrap().len(),
        code_spec_digest,text.len(),36,STARTING_CREDITS,16,180,300,founder().len(),
        FETCH_PC,SERVICE_PC,FEEDBACK_PC,PAID_SCORE_WRITE_PC,GENERATED_WRITE_PC]));
    let mut outcomes=Vec::new();
    for task in [Task::Identity,Task::Negate,Task::Composed]{
        for signal in [Signal::Truth,Signal::BlindSuccess,Signal::Inverted,Signal::Absent]{
            for gate in [1_i64,0,2]{
                let id=outcomes.len();
                outcomes.push(run_case(&mut out,id,task,signal,gate));
            }
        }
    }
    out.flush().unwrap();
    assert_eq!(outcomes.len(),36);
    eprintln!("B56_F1D1H_PAID_CANDIDATE_SEARCH_RETENTION {}",json!({
        "native_worlds":outcomes.len(),
        "full_three_candidate_true_correct":outcomes.iter()
            .filter(|r|r["gate"]==1&&r["signal"]=="TRUTH"&&r["heldout_correct"]==true).count(),
        "full_three_candidate_inverted_correct":outcomes.iter()
            .filter(|r|r["gate"]==1&&r["signal"]=="INVERTED"&&r["heldout_correct"]==true).count(),
        "source_nodes":founder().len(),"single_PID":true,
        "author_primitive_semantics":true,"new_primitive_invented":false,
        "strict_N56_A_B_C":false,"B3":false,"B4":false,"cyberlife":false
    }));
}
