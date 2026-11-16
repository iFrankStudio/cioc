// CL-1B B46: source-parsed, independent CIL interpreter for the native B42
// founder and B45 four-op provenance guard. Traces real paid CIL attempts,
// investigator-supplied one-time code interposers and their matched controls.
// No opcode, oracle or reward is inferred from the target results.
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';

const root=new URL('../../cl0/crates/world/tests/',import.meta.url);
const b42=readFileSync(new URL('cl1b_b42_self_assembled_probe.rs',root),'utf8');
const b45=readFileSync(new URL('cl1b_b45_paid_last_writer.rs',root),'utf8');
const b46=readFileSync(new URL('cl1b_b46_same_writer_race.rs',root),'utf8');

const raw=b42.match(/let mut code = vec!\[([\s\S]*?)\n    \];/)?.[1];
const rawStatic=b42.match(/if arm == Arm::StaticCorrect \{\s*return vec!\[([\s\S]*?)\n        \];/)?.[1];
if(!raw||!rawStatic)throw Error('B46 original Rust B42 source not parseable');
const parse=source=>[...source.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}|Op::Halt\b/g)].map(m=>{
  const op={kind:m[1]??'Halt'};
  for(const v of (m[2]??'').matchAll(/(\w+)\s*:\s*(-?\d+)/g))
    op[v[1]]=Number(v[2]);
  return op;
});
const original=parse(raw),staticCode=parse(rawStatic);
if(original.length!==38||staticCode.length!==4||original[7].kind!=='Write'
   ||original[10].kind!=='Write'||original[11].kind!=='Read'
   ||original[13].kind!=='Branch'||original[18].kind!=='CloneNode'
   ||original[27].kind!=='ReadNext'||original[29].kind!=='PatchNext'
   ||original[33].kind!=='Write')
  throw Error('B46 original source/ISA drift');

const guard={
  branch:'code[13] = Op::Branch {test: 1, nonzero: 14, zero: 38};',
  read:'Op::ReadLastWriter {dst: 3, address: 2, next: 39}',
  number:'Op::Const {dst: 2, value: 1, next: 40}',
  compare:'Op::Sub {dst: 3, lhs: 3, rhs: 2, next: 41}',
  decide:'Op::Branch {test: 3, nonzero: 14, zero: 26}'
};
for(const [name,definition] of Object.entries(guard)){
  if(!b45.includes(definition.replace('test: 1, nonzero: 14, zero: 38','test: 1, nonzero: 14, zero: 38').replace('code[13] = Op::Branch {test:','code[13] = Op::Branch { test:'))&&
     !b46.includes(definition))
    throw Error('B46 source guard not matched to B45: '+name);
}
for(const definition of [
  'code[11] = Op::Read {dst: 1, address: 2, next: 42};',
  'Op::Write {address: 2, value: 3, next: 12}',
  'Op::Write {address: 2, value: 0, next: 12}',
  'Op::Const {dst: 3, value: 0, next: 12}',
  'Intervention::PatchEdge{',
  'process:1,node:11,next:12'
])if(!b46.includes(definition))throw Error('B46 source interposer changed: '+definition);

const arms=[
  'NativeWrong','NativeEcho','NopSpoof','OutsideLateWrong',
  'NopGenuine','B45GuardSpoof','StaticCorrect'
];
const staticArm=arm=>arm==='StaticCorrect';
const extra=arm=>!staticArm(arm)&&arm!=='B45GuardSpoof';
const spoof=arm=>arm!=='NopGenuine'&&!staticArm(arm);
function run(arm,train,holdout){
  const isStatic=staticArm(arm);
  const code=(isStatic?staticCode:original).map(op=>({...op}));
  if(!isStatic){
    code[13]={kind:'Branch',test:1,nonzero:14,zero:38};
    code.push(
      {kind:'ReadLastWriter',dst:3,address:2,next:39},
      {kind:'Const',dst:2,value:1,next:40},
      {kind:'Sub',dst:3,lhs:3,rhs:2,next:41},
      {kind:'Branch',test:3,nonzero:14,zero:26}
    );
    if(extra(arm)){
      code[11].next=42;
      code.push(
        arm==='NativeWrong'?{kind:'Write',address:2,value:3,next:12}:
        arm==='NativeEcho'?{kind:'Write',address:2,value:0,next:12}:
        {kind:'Const',dst:3,value:0,next:12}
      );
    }
  }
  const initialCodeSize=code.length,regs=[0,0,0,0];
  const cells=new Map([[7,train]]),writer=new Map([[7,0]]);
  let pc=0,credits=128,restored=false,stopped=false;
  let challenge=null,firstOutput=null,firstReadValue=null,
    nativeInterposerValue=null,firstWriter=null,
    firstWriterWorldValue=null,firstVerdict=null;
  let forged=0,outsideLate=0,patches=0,sensed=0,reads=0,
    wrongCurrentAccepted=false,maxStorage=code.length+3;
  const score=[[0,0],[0,0]];
  const reg=i=>{
    if(!Number.isInteger(i)||i<0||i>=4)
      throw Error('B46 invalid CIL register');
    return regs[i];
  };
  const set=(key,value,source)=>{
    if(!cells.has(key)&&(cells.size>=16||code.length+1+(cells.size+1)*2>192))
      throw Error('B46 expected finite shared world allocation was exceeded');
    cells.set(key,value);writer.set(key,source);
  };
  for(const [phase,input] of [train,holdout].entries()){
    if(phase===1)set(7,input,0);
    for(let n=0;n<64&&!stopped&&credits>0;n++){
      const at=pc,op=code[at];
      if(!op)throw Error('B46 invalid program counter '+at);
      credits--;
      let e={kind:'Executed',pc:at};
      switch(op.kind){
        case 'Const':regs[op.dst]=op.value;pc=op.next;break;
        case 'Add':regs[op.dst]=reg(op.lhs)+reg(op.rhs);pc=op.next;break;
        case 'Sub':regs[op.dst]=reg(op.lhs)-reg(op.rhs);pc=op.next;break;
        case 'Read':{
          const key=reg(op.address),value=cells.get(key)??0;
          regs[op.dst]=value;pc=op.next;
          e={kind:'CellRead',pc:at,key,value};break;
        }
        case 'Write':{
          const key=reg(op.address),value=reg(op.value);
          set(key,value,1);pc=op.next;
          e={kind:'CellWritten',pc:at,key,value};break;
        }
        case 'ReadLastWriter':{
          const key=reg(op.address),observed=writer.get(key)??-1;
          regs[op.dst]=observed;pc=op.next;
          e={kind:'CellWriterSensed',pc:at,key,writer:observed};break;
        }
        case 'Branch':
          pc=reg(op.test)!==0?op.nonzero:op.zero;break;
        case 'ReadNext':{
          const node=code[reg(op.node)];
          if(!node||!Object.hasOwn(node,'next'))throw Error('B46 bad ReadNext');
          regs[op.dst]=node.next;pc=op.next;break;
        }
        case 'CloneNode':{
          const node=code[reg(op.source)];
          if(!node||code.length>=80||code.length+2+(cells.size*2)>192)
            throw Error('B46 clone failed resource check');
          regs[op.dst]=code.length;
          code.push({...node});pc=op.next;break;
        }
        case 'PatchNext':{
          const node=code[reg(op.node)];
          const target=reg(op.target);
          if(!node||!code[target]||!Object.hasOwn(node,'next'))
            throw Error('B46 invalid PatchNext');
          node.next=target;pc=op.next;break;
        }
        case 'Halt':stopped=true;break;
        default:throw Error('B46 unhandled CIL '+op.kind);
      }
      if(phase===0&&!isStatic){
        if(at===7&&e.kind==='CellWritten'&&e.key===7)
          challenge=e.value;
        if(at===10&&e.kind==='CellWritten'&&e.key===11&&firstOutput===null){
          firstOutput=e.value;
          if(spoof(arm)){
            set(11,train+1,0);
            forged++;
          }
        }
        if(at===11&&e.kind==='CellRead'&&e.key===11){
          reads++;
          if(firstReadValue===null){
            firstReadValue=e.value;
            if(arm==='OutsideLateWrong'){
              set(11,0,0);
              outsideLate++;
            }
          }
        }
        if(at===42&&extra(arm)&&patches===0){
          if(e.kind==='CellWritten'&&e.key===11)nativeInterposerValue=e.value;
          // The SAME external one-time source patch in every extra arm.
          code[11].next=12;
          patches++;
        }
        if(e.kind==='CellWriterSensed'&&e.key===11){
          sensed++;
          if(firstWriter===null){
            firstWriter=e.writer;
            firstWriterWorldValue=cells.get(11)??null;
          }
        }
        if(at===41&&firstVerdict===null){
          firstVerdict=pc;
          wrongCurrentAccepted=(train+1!==0&&pc===26&&
            cells.get(11)!==train+1);
        }
        if(at===33&&e.kind==='CellWritten'&&e.key===7){
          if(e.value!==train)throw Error('B46 original task input not restored');
          restored=true;
        }
      }
      if(e.kind==='CellWritten'&&e.key===11&&
         (isStatic||phase===1||restored)){
        score[phase][Number(e.value!==input)]++;
      }
      maxStorage=Math.max(maxStorage,1+code.length+2*cells.size);
    }
  }
  const router=isStatic?3:code[9].next;
  return {
    arm,train,holdout,challenge,firstOutput,firstReadValue,
    nativeInterposerValue,firstWriter,firstWriterWorldValue,firstVerdict,
    forged,outsideLate,patches,sensed,reads,trainTrue:score[0][0],
    trainWrong:score[0][1],holdoutTrue:score[1][0],
    holdoutWrong:score[1][1],charged:128-credits,
    cloned:code.length-initialCodeSize,stopped,restored,
    finalRouter:router,maxStorage,wrongCurrentAccepted
  };
}

const cases=[],grouped={};
for(const arm of arms){
  const stats={worlds:0,trainTrue:0,trainWrong:0,
    holdoutTrue:0,holdoutWrong:0,charged:0,cloned:0,
    sensed:0,firstSource0:0,firstSource1:0,firstAccept:0,
    forged:0,sourceRestorations:0,wrongCurrentAccepted:0,
    halted:0,maxStorage:0};
  for(const train of [-1,0,1,17])for(const holdout of [-9,23]){
    const r=run(arm,train,holdout);
    cases.push(r);
    stats.worlds++;
    for(const k of ['trainTrue','trainWrong','holdoutTrue','holdoutWrong',
      'charged','cloned','sensed','forged'])stats[k]+=r[k];
    stats.firstSource0+=Number(r.firstWriter===0);
    stats.firstSource1+=Number(r.firstWriter===1);
    stats.firstAccept+=Number(r.firstVerdict===26);
    stats.sourceRestorations+=r.patches;
    stats.wrongCurrentAccepted+=Number(r.wrongCurrentAccepted);
    stats.halted+=Number(r.stopped);
    stats.maxStorage=Math.max(stats.maxStorage,r.maxStorage);
    if(r.charged!==128||r.stopped||r.maxStorage>192)
      throw Error('B46 bounded native-style actor resource failure');
    if(extra(arm)&&r.patches!==1)
      throw Error('B46 matched temporary source edit not restored');
  }
  grouped[arm]=stats;
}
const frozen=[
  [14,42,0,104,1024,0,8,0,8,8,8,8,6],
  [14,42,0,104,1024,0,8,0,8,8,8,8,0],
  [24,0,104,0,1024,8,16,8,0,0,8,8,0],
  [24,0,104,0,1024,8,16,8,0,0,8,8,0],
  [24,14,72,26,1024,6,8,0,8,8,0,8,0],
  [24,0,96,8,1024,8,16,8,0,0,8,0,0],
  [128,0,128,0,1024,0,0,0,0,0,0,0,0],
];
for(const [i,arm] of arms.entries()){
  const s=grouped[arm];
  const actual=[
    s.trainTrue,s.trainWrong,s.holdoutTrue,s.holdoutWrong,
    s.charged,s.cloned,s.sensed,s.firstSource0,s.firstSource1,
    s.firstAccept,s.forged,s.sourceRestorations,s.wrongCurrentAccepted
  ];
  if(JSON.stringify(actual)!==JSON.stringify(frozen[i]))
    throw Error('B46 frozen source-model vector drift for '+arm+': '+JSON.stringify(actual));
}
if(cases.length!==56)throw Error('B46 full native-world denominator must be 56');
const output={
  nature:'B46 independent exact-B42-Rust-source interpreted CIL model; NOT native Rust results',
  worlds:cases.length,grouped,cases
};
if(process.argv[1]===fileURLToPath(import.meta.url))
  process.stdout.write(JSON.stringify(output,null,2)+'\n');
export {run,output};
