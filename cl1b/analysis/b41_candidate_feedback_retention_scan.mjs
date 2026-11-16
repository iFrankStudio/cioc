// CL-1B B41 independent execution of the actual native Rust CIL source.
// This model parses the Rust literal op graph; it does not execute Rust and
// must not be presented as native CI verification or autonomous evolution.
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';

const src = readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b41_candidate_feedback_retention.rs', import.meta.url), 'utf8');
const baseText = src.match(/let mut code = vec!\[([\s\S]*?)\n    \];/)?.[1];
const compactText = src.match(/if arm == Arm::StaticCorrect \{\s*return vec!\[([\s\S]*?)\n        \];/)?.[1];
if (!baseText || !compactText) throw Error('B41 native CIL source layout changed');
function parse(text) {
 const result = [...text.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}|Op::Halt\b/g)].map(m=>{
  const op={kind:m[1]??'Halt'};
  for(const a of (m[2]??'').matchAll(/(\w+)\s*:\s*(-?\d+)/g)) op[a[1]]=Number(a[2]);
  return op;
 });
 return result;
}
const source=parse(baseText), staticSource=parse(compactText);
if(source.length!==29 || staticSource.length!==4 || source[8].next!==14 ||
 source[14].kind!=='Write' || source[15].kind!=='Write' ||
 source[23].kind!=='Halt' || source[13].kind!=='ReadNext')
 throw Error('B41 native code shape drifted');
if(!src.includes('Arm::AssumePass => code[10] = Op::Sub { dst: 1, lhs: 0, rhs: 0, next: 11 }') ||
 !src.includes('Arm::ForceFallback => code[10] = Op::Const { dst: 1, value: 1, next: 11 }'))
 throw Error('B41 counterfactual mutation sources drifted');

function run(arm,first,second,train,holdout){
 const code=(arm==='StaticCorrect'?staticSource:source).map(op=>({...op}));
 if(arm==='AssumePass')code[10]={kind:'Sub',dst:1,lhs:0,rhs:0,next:11};
 if(arm==='ForceFallback')code[10]={kind:'Const',dst:1,value:1,next:11};
 const r=[0,0,0,0],cells=new Map([[20,first],[21,second],[7,train]]);
 let pc=0,credits=96,charged=0,stopped=false;
 const correct=[0,0], wrong=[0,0], edits=[],fallbacks=[];
 const reg=i=>{if(!Number.isInteger(i)||i<0||i>3)throw Error('invalid CIL register '+i);return r[i];};
 for(const [phase,input] of [train,holdout].entries()){
  if(phase===1) cells.set(7,input);
  for(let k=0;k<48;k++){
   if(stopped||credits===0)break;
   const op=code[pc];
   if(!op)throw Error('unmapped B41 native node: '+pc);
   credits--;charged++;
   switch(op.kind){
    case 'Const':r[op.dst]=op.value;pc=op.next;break;
    case 'Read':r[op.dst]=cells.get(reg(op.address))??0;pc=op.next;break;
    case 'Write':{
      const key=reg(op.address),value=reg(op.value);
      cells.set(key,value);
      if(key===11){if(value===input)correct[phase]++;else wrong[phase]++;}
      if(key===22&&value===1)fallbacks.push(charged);
      pc=op.next;break;
    }
    case 'Sub':r[op.dst]=reg(op.lhs)-reg(op.rhs);pc=op.next;break;
    case 'Branch':pc=reg(op.test)!==0?op.nonzero:op.zero;break;
    case 'PatchNext':{
      const node=reg(op.node),target=reg(op.target);
      if(!Number.isInteger(node)||node<0||node>=code.length ||
         !Number.isInteger(target)||target<0||target>=code.length)
        throw Error('invalid B41 paid self-modification');
      if(!Object.hasOwn(code[node],'next'))throw Error('patch of missing edge');
      const before=code[node].next;
      if(before!==target)edits.push({node,before,after:target,at:charged});
      code[node].next=target;pc=op.next;break;
    }
    case 'ReadNext':{
      const node=reg(op.node);
      if(!code[node]||!Object.hasOwn(code[node],'next'))throw Error('invalid ReadNext');
      r[op.dst]=code[node].next;pc=op.next;break;
    }
    case 'Halt':stopped=true;break;
    default:throw Error('B41 unsupported native opcode '+op.kind);
   }
  }
 }
 const edge=code[arm==='StaticCorrect'?2:8].next;
 return {arm,first,second,train,holdout,
   trainCorrect:correct[0],trainWrong:wrong[0],holdoutCorrect:correct[1],holdoutWrong:wrong[1],
   charged,stopped,finalEdge:edge,
   finalWriteNext:arm==='StaticCorrect'?null:[code[14].next,code[15].next],
   trueSwitches:edits.filter(e=>e.node===8).length,
   trueRetentions:edits.filter(e=>(e.node===14||e.node===15)&&e.before===9&&e.after===5).length,
   fallbackWrites:fallbacks.length,
   edits};
}
const cases=[];
for(const [first,second] of [[14,15],[15,14]])
 for(const train of [0,17,23])
  for(const holdout of [-9,17]){
   const [feedback,assumed,forced,staticCorrect]=
      ['Feedback','AssumePass','ForceFallback','StaticCorrect'].map(a=>run(a,first,second,train,holdout));
   cases.push(feedback,assumed,forced,staticCorrect);
   if(feedback.finalEdge!==(train===0?first:15)||feedback.trueRetentions!==1||feedback.stopped)
    throw Error('B41 positive native causal prediction failed');
   if(feedback.fallbackWrites!==Number(first===14&&train!==0))
    throw Error('B41 feedback-conditioned switch absent');
   if(assumed.fallbackWrites!==0||forced.fallbackWrites<1)
    throw Error('B41 alternative equal-instruction evaluator controls invalid');
   if(staticCorrect.charged!==96||staticCorrect.trainWrong!==0||
      staticCorrect.holdoutWrong!==0||staticCorrect.trainCorrect+staticCorrect.holdoutCorrect!==24)
    throw Error('B41 compact immutable comparator drifted');
   if(staticCorrect.trainCorrect+staticCorrect.holdoutCorrect < feedback.trainCorrect+feedback.holdoutCorrect)
    throw Error('B41 immutable cheap baseline unexpectedly lost');
   if(train===0&&first===14&&feedback.holdoutCorrect!==0)
    throw Error('B41 ambiguous zero case expected overfit');
   if(train!==0&&feedback.holdoutCorrect<=0)
    throw Error('B41 informative task feedback failed to retain true copier');
  }
if(cases.length!==48)throw Error('B41 wrong preregistered world count');
const counts={passed:cases.length,
 feedbackHeldoutCorrect:cases.filter(c=>c.arm==='Feedback').reduce((a,c)=>a+c.holdoutCorrect,0),
 feedbackHeldoutWrong:cases.filter(c=>c.arm==='Feedback').reduce((a,c)=>a+c.holdoutWrong,0),
 staticHeldoutCorrect:cases.filter(c=>c.arm==='StaticCorrect').reduce((a,c)=>a+c.holdoutCorrect,0),
 halted:cases.filter(c=>c.stopped).length,
 feedbackWrongRetentions:cases.filter(c=>c.arm==='Feedback'&&c.train===0&&c.first===14).length};
const output={evidence:'independent Rust-literal-source-parsed CIL execution; NOT native CI',worlds:cases.length,counts,cases};
if(process.argv[1]===fileURLToPath(import.meta.url))process.stdout.write(JSON.stringify(output,null,2)+'\n');
export {run,source,output};
