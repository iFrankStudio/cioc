#![recursion_limit = "512"]
//! B56-F1c pilot: ONE PID1 assembles and runs both policies in balanced order;
//! same source-built EVERY and FOURTH under stationary / hidden in-trial shifts.
//! The constructor and selector are author-scripted; order from cell43,
//! no monitor-only edit-law ablation: full F1/N56/B3/B4 NOT met.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
#[path="cl1b_b51c_paid_search_retention.rs"] mod b51c;
const BUDGET:usize=4000;
const WINDOW:usize=24;
const SEEDS:[i64;4]=[101,211,307,401];
const TRAIN:[i64;2]=[5,-3];
const HELD:[i64;3]=[11,-7,0];
const KEYS:[i64;16]=[7,11,20,21,22,24,25,27,28,29,30,31,40,41,42,43];

fn build_two()->Vec<Op>{
    let mut code=b51c::founder();
    assert_eq!(code.len(),72);
    code[50]=Op::PatchNext{node:2,target:3,next:72};
    code.resize(162,Op::Halt);
    code[72]=Op::Const{dst:2,value:27,next:73};
    code[73]=Op::Read{dst:3,address:2,next:74};
    code[74]=Op::Branch{test:3,nonzero:75,zero:30};
    code[75]=Op::Const{dst:2,value:35,next:76};
    code[76]=Op::CloneNode{dst:3,source:2,next:77};
    code[77]=Op::Const{dst:2,value:40,next:78};
    code[78]=Op::Write{address:2,value:3,next:79};
    code[79]=Op::Const{dst:2,value:36,next:80};
    code[80]=Op::CloneNode{dst:3,source:2,next:81};
    code[81]=Op::Const{dst:2,value:41,next:82};
    code[82]=Op::Write{address:2,value:3,next:83};
    code[83]=Op::Const{dst:2,value:40,next:84};
    code[84]=Op::Read{dst:1,address:2,next:85};
    code[85]=Op::Const{dst:2,value:41,next:86};
    code[86]=Op::Read{dst:3,address:2,next:87};
    code[87]=Op::PatchNext{node:1,target:3,next:88};
    code[88]=Op::Const{dst:2,value:41,next:89};
    code[89]=Op::Read{dst:1,address:2,next:90};
    code[90]=Op::Const{dst:3,value:98,next:91};
    code[91]=Op::PatchNext{node:1,target:3,next:92};
    code[92]=Op::Const{dst:2,value:40,next:93};
    code[93]=Op::Read{dst:3,address:2,next:94};
    code[94]=Op::Const{dst:1,value:2,next:95};
    code[95]=Op::PatchNext{node:1,target:3,next:102};
    code[98]=Op::Branch{test:3,nonzero:110,zero:99};
    code[99]=Op::Const{dst:2,value:25,next:100};
    code[100]=Op::Const{dst:3,value:1,next:101};
    code[101]=Op::Write{address:2,value:3,next:110};
    code[102]=Op::Const{dst:2,value:42,next:103};
    code[103]=Op::Write{address:2,value:3,next:130};
    code[110]=Op::Const{dst:2,value:29,next:111};
    code[111]=Op::Read{dst:1,address:2,next:112};
    code[112]=Op::Const{dst:3,value:1,next:113};
    code[113]=Op::Add{dst:1,lhs:1,rhs:3,next:114};
    code[114]=Op::Write{address:2,value:1,next:115};
    code[115]=Op::Const{dst:3,value:-4,next:116};
    code[116]=Op::Add{dst:3,lhs:1,rhs:3,next:117};
    code[117]=Op::Branch{test:3,nonzero:118,zero:104};
    code[118]=Op::Const{dst:3,value:-8,next:119};
    code[119]=Op::Add{dst:3,lhs:1,rhs:3,next:120};
    code[120]=Op::Branch{test:3,nonzero:30,zero:150};
    code[130]=Op::Const{dst:2,value:94,next:131};
    code[131]=Op::Const{dst:3,value:148,next:132};
    code[132]=Op::PatchConst{node:2,value:3,next:133};
    code[133]=Op::Const{dst:2,value:95,next:134};
    code[134]=Op::Const{dst:3,value:159,next:135};
    code[135]=Op::PatchNext{node:2,target:3,next:75};
    code[136]=Op::Const{dst:2,value:28,next:137};
    code[137]=Op::Read{dst:1,address:2,next:138};
    code[138]=Op::Const{dst:3,value:1,next:139};
    code[139]=Op::Add{dst:1,lhs:1,rhs:3,next:140};
    code[140]=Op::Write{address:2,value:1,next:141};
    code[141]=Op::Const{dst:3,value:-4,next:142};
    code[142]=Op::Add{dst:3,lhs:1,rhs:3,next:144};
    code[144]=Op::Branch{test:3,nonzero:110,zero:145};
    code[145]=Op::Const{dst:2,value:28,next:146};
    code[146]=Op::Const{dst:1,value:0,next:147};
    code[147]=Op::Write{address:2,value:1,next:148};
    code[148]=Op::Const{dst:2,value:20,next:30};
    code[150]=Op::Const{dst:2,value:25,next:151};
    code[151]=Op::Read{dst:3,address:2,next:152};
    code[152]=Op::Branch{test:3,nonzero:153,zero:149};
    code[153]=Op::Const{dst:2,value:42,next:154};
    code[154]=Op::Read{dst:3,address:2,next:155};
    code[155]=Op::Const{dst:1,value:2,next:156};
    code[156]=Op::PatchNext{node:1,target:3,next:30};
    code[159]=Op::Const{dst:2,value:2,next:160};
    code[160]=Op::Const{dst:3,value:136,next:161};
    code[161]=Op::PatchNext{node:2,target:3,next:121};
    code[96]=Op::Const{dst:3,value:136,next:97};
    code[97]=Op::PatchNext{node:2,target:3,next:30};
    code[104]=Op::Const{dst:2,value:43,next:105};
    code[105]=Op::Read{dst:3,address:2,next:106};
    code[106]=Op::Branch{test:3,nonzero:107,zero:128};
    code[107]=Op::Const{dst:2,value:2,next:108};
    code[108]=Op::Const{dst:3,value:136,next:109};
    code[109]=Op::PatchNext{node:2,target:3,next:30};
    code[121]=Op::Const{dst:2,value:43,next:122};
    code[122]=Op::Read{dst:3,address:2,next:123};
    code[123]=Op::Branch{test:3,nonzero:124,zero:30};
    code[124]=Op::Const{dst:2,value:42,next:125};
    code[125]=Op::Read{dst:3,address:2,next:126};
    code[126]=Op::Const{dst:1,value:2,next:127};
    code[127]=Op::PatchNext{node:1,target:3,next:30};
    code[128]=Op::Const{dst:2,value:42,next:129};
    code[129]=Op::Read{dst:3,address:2,next:157};
    code[149]=Op::Const{dst:2,value:2,next:96};
    code[157]=Op::Const{dst:1,value:2,next:158};
    code[158]=Op::PatchNext{node:1,target:3,next:30};
    assert_eq!(code.len(),162);
    code
}
fn build_static(fourth:bool)->Vec<Op>{
    let mut code=b51c::founder();
    code.resize(126,Op::Halt);
    if fourth{code[49]=Op::Const{dst:3,value:110,next:50};}
    else{code[49]=Op::Const{dst:3,value:120,next:50};}
    code[110]=Op::Const{dst:2,value:28,next:111};
    code[111]=Op::Read{dst:1,address:2,next:112};
    code[112]=Op::Const{dst:3,value:1,next:113};
    code[113]=Op::Add{dst:1,lhs:1,rhs:3,next:114};
    code[114]=Op::Write{address:2,value:1,next:115};
    code[115]=Op::Const{dst:3,value:-4,next:116};
    code[116]=Op::Add{dst:3,lhs:1,rhs:3,next:117};
    code[117]=Op::Branch{test:3,nonzero:30,zero:118};
    code[118]=Op::Const{dst:1,value:0,next:119};
    code[119]=Op::Write{address:2,value:1,next:120};
    code[120]=Op::Const{dst:2,value:20,next:121};
    code[121]=Op::Read{dst:3,address:2,next:122};
    code[122]=Op::Branch{test:3,nonzero:30,zero:123};
    code[123]=Op::Const{dst:2,value:25,next:124};
    code[124]=Op::Const{dst:3,value:1,next:125};
    code[125]=Op::Write{address:2,value:3,next:30};
    code
}
#[derive(Copy,Clone)]
enum Arm{TruthTwo,BlindTwo,InvertedTwo,NoBuild,StaticEvery,StaticFourth}
impl Arm{
    fn name(self)->&'static str{
        match self{
            Self::TruthTwo=>"A_TRUTH_TWO",
            Self::BlindTwo=>"B_BLIND_TWO",
            Self::InvertedTwo=>"C_INVERTED_TWO",
            Self::NoBuild=>"D_NO_ASSEMBLY",
            Self::StaticEvery=>"E_STATIC_EVERY",
            Self::StaticFourth=>"F_STATIC_FOURTH",
        }
    }
    fn code(self)->Vec<Op>{
        match self{
            Self::StaticEvery=>build_static(false),
            Self::StaticFourth=>build_static(true),
            _=>build_two(),
        }
    }
}
fn emit(out:&mut BufWriter<File>,v:Value){
    serde_json::to_writer(&mut *out,&v).unwrap();
    out.write_all(b"\n").unwrap();
}
fn cells(w:&World)->Value{json!(KEYS.map(|k|w.cell(k)))}
fn event(e:&EventKind)->Value{
    match e{
        EventKind::Executed=>json!(["exec"]),
        EventKind::CellRead{key,value}=>json!(["read",key,value]),
        EventKind::CellWritten{key,value}=>json!(["write",key,value]),
        EventKind::ExecutableEdgePatched{node,before,after}=>
            json!(["patch",node,before,after]),
        EventKind::NativeCodeEditSuppressed{opcode_tag}=>
            json!(["suppressed",opcode_tag]),
        EventKind::Trapped(msg)=>json!(["trap",msg]),
        EventKind::Halted=>json!(["halt"]),
        _=>panic!("B56 F1b unknown paid native event: {e:?}")
    }
}
fn ext(out:&mut BufWriter<File>,w:&mut World,id:usize,k:i64,v:i64,why:&str){
    let inv=w.total_compute_inventory();
    w.intervene(Intervention::SetCell{key:k,value:v}).unwrap();
    assert_eq!(w.total_compute_inventory(),inv);
    let e=w.events().last().unwrap();
    assert_eq!(e.origin,Origin::External);
    emit(out,json!(["ext",id,e.tick,k,v,why,w.remaining_credits(),
        w.occupied_units(),cells(w)]));
}
fn edge(w:&World,node:usize)->Option<usize>{
    match w.process(1).unwrap().code.nodes.get(node){
        Some(Op::Const{next,..})|Some(Op::Add{next,..})|
        Some(Op::Sub{next,..})|Some(Op::Read{next,..})|
        Some(Op::Write{next,..})|Some(Op::ReadNext{next,..})|
        Some(Op::PatchNext{next,..})|Some(Op::PatchConst{next,..})|
        Some(Op::CloneNode{next,..})=>Some(*next),
        _=>None
    }
}
fn const94(w:&World)->Option<i64>{
    match w.process(1).unwrap().code.nodes.get(94){
        Some(Op::Const{value,..})=>Some(*value),_=>None
    }
}
fn step(out:&mut BufWriter<File>,w:&mut World,id:usize,paid:&mut usize)->EventKind{
    let inv=w.total_compute_inventory().unwrap();
    let p=w.process(1).unwrap();
    let old=if matches!(p.code.nodes[p.pc],Op::CloneNode{..}|
        Op::PatchNext{..}|Op::PatchConst{..}){
        Some(p.code.nodes.clone())
    }else{None};
    assert!(w.step());
    *paid+=1;
    assert_eq!(w.total_compute_inventory(),Some(inv-1));
    let e=w.events().last().unwrap().clone();
    assert_eq!(e.origin,Origin::Internal(1));
    let p=w.process(1).unwrap();
    assert_eq!(e.credits_after,p.credits);
    assert!(p.code.nodes.len()<=180 && w.occupied_units()<=300);
    let mut diff=Vec::<Value>::new();
    if let Some(old)=old{
        for (i,op) in p.code.nodes.iter().enumerate(){
            if old.get(i)!=Some(op){
                diff.push(json!([i,old.get(i),op]));
            }
        }
    }
    emit(out,json!(["step",id,*paid,e.tick,e.pc,p.pc,event(&e.kind),
        p.registers,p.credits,p.stopped,p.code.nodes.len(),
        edge(w,2),edge(w,95),edge(w,148),const94(w),
        w.occupied_units(),cells(w),diff]));
    e.kind
}
#[derive(Copy,Clone,PartialEq,Eq)]
enum Ecology{Stationary,EarlyShiftReturn}
impl Ecology{
    fn name(self)->&'static str{
        match self{Self::Stationary=>"S_STATIONARY",
            Self::EarlyShiftReturn=>"D_EARLY_SHIFT_RETURN"}
    }
}
fn target(idx:usize,service_before:usize,ecology:Ecology)->(i64,i64){
    let base=if idx%2==0{(1,-2)}else{(-1,-3)};
    let shift=if idx%2==0{(-1,3)}else{(1,2)};
    if ecology==Ecology::EarlyShiftReturn && (2..24).contains(&service_before)
       {shift}else{base}
}
fn case(out:&mut BufWriter<File>,id:usize,seed:i64,ecology:Ecology,arm:Arm)->Value{
    let idx=SEEDS.iter().position(|s|*s==seed).unwrap();
    let code=arm.code();
    let mut proc=Process::new(1,code.clone(),BUDGET as u64);
    proc.pc=3;
    let mut w=World::with_strict_capacity(16,180,300)
        .with_native_executable_edit_trace();
    w.load(proc).unwrap();
    emit(out,json!(["case",id,seed,ecology.name(),arm.name(),
        target(idx,0,Ecology::Stationary),if idx%2==0{(-1,3)}else{(1,2)},
        if idx<2{"A_THEN_B"}else{"B_THEN_A"},
        BUDGET,code,w.occupied_units()]));
    ext(out,&mut w,id,27,if matches!(arm,Arm::NoBuild){0}else{1},
        "prestart_authorized_assembly_gate");
    let order_a_first=idx<2;
    ext(out,&mut w,id,43,if order_a_first{1}else{0},
        "prestart_balanced_trial_order");
    let mut input=TRAIN[0];
    ext(out,&mut w,id,7,input,"initial_training_probe");
    let mut paid=0_usize;
    let mut training=true;
    let mut training_writes=0_usize;
    let mut service_writes=0_usize;
    let mut counts=[0_usize;3];
    let mut wrong=[0_usize;3];
    let mut all_correct=0_usize;
    let mut all_wrong=0_usize;
    let mut first72_paid=None::<usize>;
    let mut retained=false;
    let mut native_clones=0_usize;
    let mut patch_edges=0_usize;
    let mut graph_changes=0_usize;
    let mut monitor_a_linked=false;
    let mut monitor_b_linked=false;
    let mut first_route=None::<usize>;
    let mut after_four_route=None::<usize>;
    let mut trial_reads_a=0_usize;
    let mut trial_reads_b=0_usize;
    let mut observed_a=0_usize;
    let mut observed_b=0_usize;
    let mut reads_a=0_usize;
    let mut reads_b=0_usize;
    let mut final_choice=None::<usize>;
    let mut trial_selection_paid=None::<usize>;
    let mut saw_shifted_failure=0_usize;
    let mut sampled_failure_markers=0_usize;
    let mut peak=w.occupied_units();
    while paid<BUDGET&&!w.process(1).unwrap().stopped{
        let old_code_len=w.process(1).unwrap().code.nodes.len();
        let old_pc=w.process(1).unwrap().pc;
        let outcome=step(out,&mut w,id,&mut paid);
        peak=peak.max(w.occupied_units());
        let new_code_len=w.process(1).unwrap().code.nodes.len();
        native_clones+=new_code_len-old_code_len;
        if matches!(outcome,EventKind::ExecutableEdgePatched{..}){
            patch_edges+=1;
        }
        if [132,135,87,91,95,161,156].contains(&old_pc){graph_changes+=1;}
        if (old_pc==156||old_pc==97)&&w.cell(29)==Some(8){
            final_choice=edge(&w,2);
            trial_selection_paid=Some(paid);
        }
        if old_pc==127&&w.cell(29).unwrap_or(0)==0{
            first_route=edge(&w,2);
        }
        if [109,158].contains(&old_pc)&&w.cell(29)==Some(4){
            after_four_route=edge(&w,2);
        }
        match outcome{
            EventKind::CellWritten{key:11,value}=>{
                let phase=(service_writes/WINDOW).min(2);
                let (s,k)=target(idx,service_writes,ecology);
                let good=value==s*input+k;
                if training{
                    training_writes+=1;
                    ext(out,&mut w,id,20,i64::from(good),"truthful_training_feedback");
                    input=TRAIN[training_writes%TRAIN.len()];
                    ext(out,&mut w,id,7,input,"next_training_probe");
                }else{
                    if service_writes<3*WINDOW{
                        if good{counts[phase]+=1;}else{wrong[phase]+=1;}
                    }
                    if good{all_correct+=1;}else{all_wrong+=1;}
                    service_writes+=1;
                    if service_writes==72{first72_paid=Some(paid);}
                    let bit=match arm{
                        Arm::BlindTwo=>1,
                        Arm::InvertedTwo=>i64::from(!good),
                        _=>i64::from(good)
                    };
                    ext(out,&mut w,id,20,bit,
                        match arm{
                            Arm::BlindTwo=>"service_constant_false_success",
                            Arm::InvertedTwo=>"service_inverted_feedback",
                            _=>"truthful_service_feedback"
                        });
                    input=HELD[((service_writes%WINDOW)+idx%HELD.len())%HELD.len()];
                    ext(out,&mut w,id,7,input,"next_service_probe");
                    if service_writes<=8{
                        if (service_writes<=4)==order_a_first{
                            observed_a+=1;
                        }else{observed_b+=1;}
                    }
                    if !good&&phase==1{saw_shifted_failure+=1;}
                }
            },
            EventKind::ExecutableEdgePatched{node:2,before:42,after} if !retained && [30,110,120].contains(&after)=>{
                assert!(training && training_writes>=2 && training_writes%2==0);
                training=false;retained=true;
                input=HELD[idx%HELD.len()];
                ext(out,&mut w,id,7,input,"paid_source_retained_service");
            },
            EventKind::ExecutableEdgePatched{node:2,before:30,after:164}=>{
                assert!(retained && !monitor_a_linked);
                monitor_a_linked=true;
                assert_eq!(new_code_len,166);
            },
            EventKind::ExecutableEdgePatched{node:2,before:164,after:136}=>{
                assert!(retained && monitor_a_linked && !monitor_b_linked);
                monitor_b_linked=true;
                assert_eq!(new_code_len,168);
            },
            EventKind::CellRead{key:20,..} if !training=>{
                if edge(&w,2)==Some(164){
                    reads_a+=1;
                    if service_writes<=8{trial_reads_a+=1;}
                }else if edge(&w,2)==Some(136){
                    reads_b+=1;
                    if service_writes<=8{trial_reads_b+=1;}
                }
            },
            EventKind::CellWritten{key:25,value:1} if !training=>{
                sampled_failure_markers+=1;
            },
            _=>{}
        }
    }
    let scored=counts.iter().sum::<usize>();
    let wrong_n=wrong.iter().sum::<usize>();
    let missing=72-service_writes.min(72);
    assert_eq!(scored+wrong_n+missing,72);
    let p=w.process(1).unwrap();
    let row=json!({"id":id,"seed":seed,"ecology":ecology.name(),
        "order":if order_a_first{"A_THEN_B"}else{"B_THEN_A"},"arm":arm.name(),
        "paid":paid,"retained":retained,"training_writes":training_writes,
        "service_writes":service_writes,
        "stage_correct":counts,"stage_wrong":wrong,
        "correct":scored,"wrong":wrong_n,"missing":missing,
        "all_correct":all_correct,"all_wrong":all_wrong,
        "first72_paid":first72_paid,
        "native_clone_count":native_clones,
        "native_changed_edges":patch_edges,"paid_builder_edit_attempts":graph_changes,
        "monitor_a_linked":monitor_a_linked,
        "monitor_b_linked":monitor_b_linked,
        "a_trial_outputs":observed_a,"b_trial_outputs":observed_b,
        "a_feedback_reads":reads_a,"b_feedback_reads":reads_b,
        "trial_a_reads":trial_reads_a,"trial_b_reads":trial_reads_b,
        "first_route":first_route,"after_four_route":after_four_route,
        "selected_service_edge":final_choice,
        "selection_paid":trial_selection_paid,
        "shifted_wrong_outputs":saw_shifted_failure,
        "failure_marker_events":sampled_failure_markers,
        "peak_storage":peak,"final_nodes":p.code.nodes.len(),
        "final_pc":p.pc,"final_regs":p.registers,"final_graph":p.code.nodes,
        "final_cells":cells(&w),"stopped":p.stopped,
        "historical_u":(scored as f64-2.0*wrong_n as f64-missing as f64)/(paid.max(1) as f64)});
    emit(out,json!(["end",id,row]));
    row
}
#[test]
fn b56_f1c_native_paid_balanced_order_and_discriminative_trial(){
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let path=folder.join("b56_f1c_pilot_native_steps.jsonl");
    let mut out=BufWriter::new(File::create(&path).unwrap());
    let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../cl1b/studies/B56-F1c-pilot-native-order-balanced-discriminating-trials-manifest-v1.md");
    let bytes=fs::read(&manifest).unwrap();
    let sha=std::process::Command::new("sha256sum").arg(&manifest).output().unwrap();
    assert!(sha.status.success());
    let hash=String::from_utf8(sha.stdout).unwrap().split_whitespace()
        .next().unwrap().to_owned();
    emit(&mut out,json!(["meta","B56-F1c-PILOT",1,
        std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_owned()),
        hash,bytes.len(),BUDGET,16,180,300]));
    let arms=[Arm::TruthTwo,Arm::BlindTwo,Arm::InvertedTwo,
        Arm::NoBuild,Arm::StaticEvery,Arm::StaticFourth];
    let ecologies=[Ecology::Stationary,Ecology::EarlyShiftReturn];
    let mut rows=Vec::new();
    for seed in SEEDS{
        for ecology in ecologies{
            for arm in arms{
                let id=rows.len();
                rows.push(case(&mut out,id,seed,ecology,arm));
            }
        }
    }
    out.flush().unwrap();
    assert_eq!(rows.len(),48);
    for r in &rows{
        assert_eq!(r["paid"],json!(4000));
        assert!(r["final_nodes"].as_u64().unwrap()<=180);
        assert!(r["peak_storage"].as_u64().unwrap()<=300);
    }
    eprintln!("B56_F1C_BALANCED_TWO_MONITOR_NATIVE {}",serde_json::to_string(&rows.iter()
        .map(|r|json!({"id":r["id"],"arm":r["arm"],"seed":r["seed"],
            "ecology":r["ecology"],"order":r["order"],
            "A":r["monitor_a_linked"],"B":r["monitor_b_linked"],
            "trialA":r["a_trial_outputs"],"trialB":r["b_trial_outputs"],
            "selection":r["selected_service_edge"],
            "correct":r["correct"],"wrong":r["wrong"],
            "all_correct":r["all_correct"]})).collect::<Vec<_>>()).unwrap());
}
