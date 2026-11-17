// B51d independent event-level replay of ORIGINAL B51c CIL founder.
// Imports an independently authored CIL interpreter; native step rows are
// comparisons only, never used as inputs for model execution.
import fs from 'node:fs';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';
import {CIL,founder} from './b51c_native_paid_search_replay.mjs';

const source=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b51d_midlife_regime_shift.rs',import.meta.url),'utf8');
for(const token of [
  'mod b51c;', 'b51c::founder()', 'const BUDGET:usize=1300',
  'const WINDOW:usize=24', 'const WINDOWS:usize=3',
  'with_native_executable_edit_trace()', 'phase==1',
  'post_shift_feedback', 'feedback_reads_after_retain'
])if(!source.includes(token))throw Error('B51d actual Rust protocol/source drift: '+token);
if(founder.length!==72||founder[33].kind!=='Read'||founder[33].next!==33)
  throw Error('B51d imported source interpreter is NOT the preregistered B51c founder');

const file='cl1b-ci-evidence/b51d_native_steps.jsonl';
if(!fs.existsSync(file))throw Error('B51d same-run native evidence missing');
const lines=fs.readFileSync(file,'utf8').trimEnd().split(/\r?\n/);
let pos=0,independentSteps=0,externalRows=0,labTransitions=0;
function native(){
  if(pos===lines.length)throw Error('B51d source-pinned native trace truncated at row '+pos);
  try{return JSON.parse(lines[pos++]);}
  catch(e){throw Error('B51d non-JSONL evidence at row '+pos+': '+e);}
}
function cmp(expected,detail){
  const observed=native();
  if(!isDeepStrictEqual(expected,observed))
    throw Error('B51d FIRST DIVERGENCE at '+detail+' row '+pos+
      '\nexpected='+JSON.stringify(expected)+
      '\nobserved='+JSON.stringify(observed));
}
const meta=native();
if(meta.type!=='meta'||meta.study!=='B51d'||meta.version!==1||
   meta.founder_source!=='cl1b_b51c_paid_search_retention.rs'||!meta.source_sha)
  throw Error('B51d missing or altered source provenance');
if(process.env.GITHUB_SHA&&meta.source_sha!==process.env.GITHUB_SHA)
  throw Error('B51d stale native trace from other source SHA');

const HELD=[11,-7,0],TRAIN=[5,-3],BUDGET=1300,WINDOW=24,WINDOWS=3;
const transitions=[
  ['A',[1,-2],[-1,3]],
  ['B',[-1,-3],[1,2]]
];
const arms=['STATIONARY','SHIFT_NO_FEEDBACK','SHIFT_WITH_FEEDBACK','SHIFT_RETURN'];
function hiddenMode(from,to,arm,stage){
  return stage===0||arm==='STATIONARY'||(stage===2&&arm==='SHIFT_RETURN')
    ?from:to;
}
let id=0;
const cases=[],staticReference=[];
for(const [transition,from,to] of transitions){
  for(const phase of [0,1]){
    const matched=[];
    for(const arm of arms){
      const caseId=id++;
      const model=new CIL(from[0],from[1],'TRUE');
      cmp({type:'case',case:caseId,transition,arm,phase,from,to,
        initial_len:72,initial_credits:BUDGET,
        initial_storage:model.storage()},'case header '+caseId);
      if(phase===1){
        const before=model.step(caseId);
        cmp(before,'charged task-unobserving phase preamble '+caseId);
        independentSteps++;
        assert.equal(before.pc,3);
        assert.equal(before.event.type,'executed');
      }
      cmp(model.external(caseId,7,TRAIN[0],'initial_training'),
        'initial task input '+caseId);
      externalRows++;
      model.peak=Math.max(model.peak,model.storage());
      let input=TRAIN[0],trainingCount=0,trainCorrect=0,trainWrong=0;
      let retained=false,retainStep=null,chosen=null;
      let heldCount=0;
      const correct=[0,0,0],wrong=[0,0,0];
      let postShiftFeedbackWrites=0,postRetainFeedbackReads=0,
          postRetainPatches=0,postRetainClones=0;
      while(model.paid<BUDGET&&!model.stopped&&heldCount<WINDOW*WINDOWS){
        const oldLen=model.code.length;
        const row=model.step(caseId);
        cmp(row,'case '+caseId+' paid step '+model.paid);
        independentSteps++;
        const e=row.event;
        if(retained){
          if(e.type==='read'&&e.key===20)postRetainFeedbackReads++;
          if(e.type==='patch')postRetainPatches++;
          postRetainClones+=model.code.length-oldLen;
        }
        if(e.type==='write'&&e.key===11){
          if(!retained){
            const good=e.value===from[0]*input+from[1];
            trainingCount++;
            if(good)trainCorrect++;else trainWrong++;
            cmp(model.external(caseId,20,Number(good),'post_output_bit'),
              'training feedback '+caseId+'/'+trainingCount);
            externalRows++;
            input=TRAIN[trainingCount%TRAIN.length];
            model.input=input;
            cmp(model.external(caseId,7,input,'training_next'),
              'training input '+caseId+'/'+trainingCount);
            externalRows++;
          }else{
            const stage=Math.floor(heldCount/WINDOW);
            const [s,k]=hiddenMode(from,to,arm,stage);
            const good=e.value===s*input+k;
            if(good)correct[stage]++;else wrong[stage]++;
            heldCount++;
            if(arm==='SHIFT_WITH_FEEDBACK'&&stage>=1){
              cmp(model.external(caseId,20,Number(good),'post_shift_feedback'),
                'post-retention feedback '+caseId+'/'+heldCount);
              externalRows++;postShiftFeedbackWrites++;
            }
            if(heldCount%WINDOW===0&&heldCount<WINDOW*WINDOWS){
              const newStage=heldCount/WINDOW;
              cmp({type:'lab_transition',case:caseId,
                stage:newStage,after_outputs:heldCount,
                from:hiddenMode(from,to,arm,newStage-1),
                to:hiddenMode(from,to,arm,newStage),
                charged:model.paid,tick:model.tick},
                'lab-only task shift '+caseId+'/'+newStage);
              labTransitions++;
            }
            if(heldCount<WINDOW*WINDOWS){
              input=HELD[((heldCount%WINDOW)+phase)%HELD.length];
              model.input=input;
              cmp(model.external(caseId,7,input,'heldout_next'),
                'heldout input '+caseId+'/'+heldCount);
              externalRows++;
            }
          }
        }else if(e.type==='patch'&&e.node===2&&
                 e.before===42&&e.after===30){
          if(retained||trainingCount<2||trainingCount%2!==0)
            throw Error('B51d invalid unearned native retention');
          retained=true;retainStep=model.paid;
          chosen=[model.cell(22),model.cell(21)];
          model.accepted=true;model.acceptStep=retainStep;model.chosen=chosen;
          input=HELD[phase];model.input=input;
          cmp(model.external(caseId,7,input,'retained_holdout_start'),
            'retained source initial heldout '+caseId);
          externalRows++;
        }
      }
      const end={type:'end',case:caseId,transition,arm,phase,from,to,
        paid:model.paid,credits:model.credits,stopped:model.stopped,
        retained,retain_step:retainStep,chosen,
        training_outputs:trainingCount,training_correct:trainCorrect,
        training_wrong:trainWrong,heldout_outputs:heldCount,
        window_correct:correct,window_wrong:wrong,
        post_shift_feedback_writes:postShiftFeedbackWrites,
        feedback_reads_after_retention:postRetainFeedbackReads,
        patches_after_retention:postRetainPatches,
        clones_after_retention:postRetainClones,
        final_len:model.code.length,final_pc:model.pc,
        final_regs:[...model.regs],edge2:model.edge(2),
        edge33:model.edge(33),peak_storage:model.peak,
        final_cells:model.stateCells()};
      cmp(end,'case final state '+caseId);
      cases.push(end);matched.push(end);
    }
    for(const row of matched){
      if(row.paid!==matched[0].paid)
        throw Error('B51d matched arm compute opportunity not equal');
      if(row.heldout_outputs!==72||row.retained!==true)
        throw Error('B51d missing retained source or truncated 24x3 window');
    }
    for(let stage=0;stage<WINDOWS;stage++){
      const mode=stage===0?from:to;
      cmp({type:'static',transition,phase,stage,
        result:{mode,phase,paid:144,correct:24,wrong:0}},
        'actual native privileged oracle '+transition+'/'+phase+'/'+stage);
      staticReference.push({transition,phase,stage,mode,correct:24,paid:144});
    }
  }
}
if(pos!==lines.length)throw Error('B51d extra unverified native rows '+(lines.length-pos));
const grouped=Object.fromEntries(arms.map(arm=>{
  const group=cases.filter(c=>c.arm===arm);
  return [arm,{
    worlds:group.length,paid:group.reduce((sum,c)=>sum+c.paid,0),
    correct_by_stage:[0,1,2].map(i=>group.reduce((s,c)=>s+c.window_correct[i],0)),
    wrong_by_stage:[0,1,2].map(i=>group.reduce((s,c)=>s+c.window_wrong[i],0)),
    post_shift_feedback_writes:group.reduce((n,c)=>n+c.post_shift_feedback_writes,0),
    post_retention_feedback_reads:group.reduce((n,c)=>n+c.feedback_reads_after_retention,0),
    post_retention_edge_edits:group.reduce((n,c)=>n+c.patches_after_retention,0),
    retained_source_sign_k:group.map(c=>({case:c.case,chosen:c.chosen}))
  }];
}));
const noRevalidationConfirmed=cases.every(c=>
  c.feedback_reads_after_retention===0&&c.patches_after_retention===0&&
  c.clones_after_retention===0)&&
  grouped.SHIFT_NO_FEEDBACK.wrong_by_stage[1]===96&&
  grouped.SHIFT_WITH_FEEDBACK.wrong_by_stage[1]===96&&
  grouped.SHIFT_WITH_FEEDBACK.post_retention_feedback_reads===0&&
  grouped.SHIFT_RETURN.correct_by_stage[2]===96;
console.log(JSON.stringify({
  study:'B51d',native_source_sha:meta.source_sha,
  matched_continuous_actor_worlds:cases.length,
  independent_native_paid_steps:independentSteps,
  independent_external_cell_interventions:externalRows,
  independent_lab_only_transition_records:labTransitions,
  independent_privileged_static_native_segments:staticReference.length,
  first_divergence:null,
  hidden_regime_shift_revalidated:false,
  no_revalidation_falsifier_confirmed:noRevalidationConfirmed,
  grouped,
  cases:cases.map(c=>({transition:c.transition,arm:c.arm,phase:c.phase,
    chosen:c.chosen,paid:c.paid,windows_correct:c.window_correct,
    windows_wrong:c.window_wrong,
    feedback_reads_after_retention:c.feedback_reads_after_retention,
    edits_after_retention:c.patches_after_retention})),
  b51d_full_transfer_accepted:false,
  b3_met:false,b4_met:false,cyberlife_established:false
},null,2));
