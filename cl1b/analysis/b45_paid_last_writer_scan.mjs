// CL-1B B45 independent, SOURCE-PARSED exact B42 CIL interpreter.
// ReadLastWriter is a generic, paid substrate observation, not a truth
// classifier; each stored value+writer consumes two storage units.
// This executable forecast is NOT Rust-native CI verification.
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';

const rust=readFileSync(
  new URL('../../cl0/crates/world/tests/cl1b_b42_self_assembled_probe.rs',import.meta.url),'utf8'
);
const test=readFileSync(
  new URL('../../cl0/crates/world/tests/cl1b_b45_paid_last_writer.rs',import.meta.url),'utf8'
);
const source=rust.match(/let mut code = vec!\[([\s\S]*?)\n    \];/)?.[1];
const cheap=rust.match(/if arm == Arm::StaticCorrect \{\s*return vec!\[([\s\S]*?)\n        \];/)?.[1];
if(!source||!cheap)throw Error('B45 could not parse genuine B42 Rust founder');
const parse=raw=>[...raw.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}|Op::Halt\b/g)].map(m=>{
  const op={kind:m[1]??'Halt'};
  for(const v of (m[2]??'').matchAll(/(\w+)\s*:\s*(-?\d+)/g))
    op[v[1]]=Number(v[2]);
  return op;
});
const founder=parse(source),staticFounder=parse(cheap);
if(founder.length!==38||staticFounder.length!==4||
   founder[7].kind!=='Write'||founder[9].next!==10||
   founder[10].kind!=='Write'||founder[11].kind!=='Read'||
   founder[13].kind!=='Branch'||founder[18].kind!=='CloneNode'||
   founder[27].kind!=='ReadNext'||founder[29].kind!=='PatchNext'||
   founder[33].kind!=='Write')
  throw Error('B45 native source selection drift');
if(!test.includes('code[13] = Op::Branch { test: 1, nonzero: 14, zero: 38 };')||
   !test.includes('Op::ReadLastWriter { dst: 3, address: 2, next: 39 }')||
   !test.includes('Op::Const { dst: 2, value: 1, next: 40 }')||
   !test.includes('Op::Sub { dst: 3, lhs: 3, rhs: 2, next: 41 }')||
   !test.includes('Op::Branch { test: 3, nonzero: 14, zero: 26 }'))
  throw Error('B45 CIL guard CIL source no longer matches independent interpreter');

const arms=[
  'GuardTrackedSpoof',
  'GuardTrackedGenuine',
  'GuardTrackedEcho',
  'GuardUntrackedSpoof',
  'OriginalTrackedSpoof',
  'OriginalUntrackedSpoof',
  'StaticTracked'
];
const tracking=a=>!a.includes('Untracked');
const guard=a=>a.startsWith('Guard');
const attack=a=>a.includes('Spoof')||a.includes('Echo');

function run(arm,train,holdout){
  const isStatic=arm==='StaticTracked';
  const code=(isStatic?staticFounder:founder).map(o=>({...o}));
  if(guard(arm)){
    code[13]={kind:'Branch',test:1,nonzero:14,zero:38};
    code.push(
      {kind:'ReadLastWriter',dst:3,address:2,next:39},
      {kind:'Const',dst:2,value:1,next:40},
      {kind:'Sub',dst:3,lhs:3,rhs:2,next:41},
      {kind:'Branch',test:3,nonzero:14,zero:26}
    );
  }
  const baseLength=code.length,regs=[0,0,0,0];
  const cells=new Map([[7,train]]);
  const lastWriter=tracking(arm)?new Map([[7,0]]):new Map();
  const service=[[0,0],[0,0]], events=[];
  let pc=0,credits=128,stopped=false,restored=false;
  let challenge=null,firstOutput=null,firstRead=null,firstWriter=null,firstVerdict=null;
  let spoofCount=0,sensorEvents=0;
  const reg=i=>{
    if(!Number.isInteger(i)||i<0||i>=4)throw Error('invalid B45 CIL register '+i);
    return regs[i];
  };
  const set=(key,value,writer)=>{
    const newAddress=!cells.has(key);
    // strict allocation counts a process header, code nodes and priced cells
    const occupied=1+code.length+cells.size+lastWriter.size;
    const needed=newAddress?(tracking(arm)?2:1):0;
    if((newAddress&&cells.size>=16)||occupied+needed>192)
      throw Error('B45 unaccounted provenance storage overflow');
    cells.set(key,value);
    if(tracking(arm))lastWriter.set(key,writer);
  };
  for(const [phase,input] of [train,holdout].entries()){
    if(phase===1)set(7,input,0);
    for(let k=0;k<64&&credits>0&&!stopped;k++){
      const op=code[pc],at=pc;
      if(!op)throw Error('B45 missing native node '+pc);
      credits--;
      let e={kind:'Executed',pc:at};
      switch(op.kind){
        case 'Const':regs[op.dst]=op.value;pc=op.next;break;
        case 'Add':regs[op.dst]=reg(op.lhs)+reg(op.rhs);pc=op.next;break;
        case 'Sub':regs[op.dst]=reg(op.lhs)-reg(op.rhs);pc=op.next;break;
        case 'Read':{
          const key=reg(op.address),value=cells.get(key)??0;
          regs[op.dst]=value;pc=op.next;e={kind:'CellRead',pc:at,key,value};break;
        }
        case 'ReadLastWriter':{
          const key=reg(op.address),writer=lastWriter.get(key)??-1;
          regs[op.dst]=writer;pc=op.next;e={kind:'CellWriterSensed',pc:at,key,writer};break;
        }
        case 'Write':{
          const key=reg(op.address),value=reg(op.value);
          set(key,value,1);pc=op.next;e={kind:'CellWritten',pc:at,key,value};break;
        }
        case 'Branch':pc=reg(op.test)!==0?op.nonzero:op.zero;break;
        case 'CloneNode':{
          const i=reg(op.source),node=code[i];
          if(!node||code.length>=80||1+code.length+1+cells.size+lastWriter.size>192)
            throw Error('B45 native clone would trap');
          regs[op.dst]=code.length;code.push({...node});pc=op.next;break;
        }
        case 'ReadNext':{
          const i=reg(op.node);
          if(!code[i]||!Object.hasOwn(code[i],'next'))throw Error('bad native ReadNext');
          regs[op.dst]=code[i].next;pc=op.next;break;
        }
        case 'PatchNext':{
          const i=reg(op.node),dest=reg(op.target);
          if(!code[i]||!Object.hasOwn(code[i],'next')||!code[dest])
            throw Error('bad native PatchNext');
          code[i].next=dest;pc=op.next;break;
        }
        case 'Halt':stopped=true;break;
        default:throw Error('unexpected source opcode '+op.kind);
      }
      events.push({phase,...e});
      if(phase===0&&!isStatic){
        if(e.kind==='CellWritten'&&e.key===7&&at===7)
          challenge=e.value;
        if(e.kind==='CellWritten'&&e.key===11&&at===10&&firstOutput===null){
          firstOutput=e.value;
          if(attack(arm)){
            set(11,arm==='GuardTrackedEcho'?0:train+1,0);
            spoofCount++;
          }
        }
        if(e.kind==='CellRead'&&e.key===11&&at===11&&firstRead===null)
          firstRead=e.value;
        if(e.kind==='CellWriterSensed'&&e.key===11){
          sensorEvents++;
          if(firstWriter===null)firstWriter=e.writer;
        }
        if(e.kind==='CellWritten'&&e.key===7&&at===33){
          if(e.value!==train)throw Error('B45 did not restore input');
          restored=true;
        }
        if(at===13&&firstVerdict===null)firstVerdict=pc;
      }
      if(e.kind==='CellWritten'&&e.key===11&&(phase===1||restored||isStatic))
        service[phase][Number(e.value!==input)]++;
    }
  }
  return {
    arm,train,holdout,challenge,firstOutput,firstRead,firstWriter,firstVerdict,
    spoofCount,sensorEvents,trainTrue:service[0][0],trainWrong:service[0][1],
    holdoutTrue:service[1][0],holdoutWrong:service[1][1],
    paid:128-credits,storage:1+code.length+cells.size+lastWriter.size,
    restored,nodes:code.length,clones:Number(code.length>baseLength),
    finalRouter:isStatic?3:code[9].next,stopped,
    falseAttestation: firstRead===challenge&&firstOutput!==challenge&&firstVerdict===26,
    events
  };
}

const cases=[];
const grouped=Object.fromEntries(arms.map(a=>[a,{
  worlds:0,trainTrue:0,trainWrong:0,holdoutTrue:0,holdoutWrong:0,
  paid:0,clones:0,stopped:0,sensorEvents:0,falseAttestations:0,
  taskCellStorageUnits:null
}]));
for(const train of [-1,0,1,17])for(const holdout of [-9,23])for(const arm of arms){
  const p=run(arm,train,holdout);
  cases.push(p);
  const g=grouped[arm];
  g.worlds++;
  for(const k of ['trainTrue','trainWrong','holdoutTrue','holdoutWrong',
                  'paid','clones','sensorEvents'])g[k]+=p[k];
  g.stopped+=Number(p.stopped);
  g.falseAttestations+=Number(p.falseAttestation);
  const tracked=tracking(arm);
  if(tracked&&p.stopped)throw Error('tracked B45 world failed early');
  if(!tracked&&guard(arm)&&!p.stopped)throw Error('untracked witness failed to halt');
}
if(cases.length!==56)throw Error('B45 model denominator must be 56');
const expected=[
 [24,0,96,8,1024,8,0,16,0],
 [24,14,78,26,1024,6,0,8,0],
 [30,0,102,2,1024,8,0,10,0],
 [0,0,0,0,344,8,8,16,0],
 [16,48,0,104,1024,0,0,0,6],
 [16,48,0,104,1024,0,0,0,6],
 [128,0,128,0,1024,0,0,0,0]
];
for(const [i,arm] of arms.entries()){
  const g=grouped[arm];
  const vector=[
    g.trainTrue,g.trainWrong,g.holdoutTrue,g.holdoutWrong,
    g.paid,g.clones,g.stopped,g.sensorEvents,g.falseAttestations
  ];
  if(JSON.stringify(vector)!==JSON.stringify(expected[i]))
    throw Error('B45 prospective model divergence for '+arm+': '+JSON.stringify(vector));
}
const output={
  nature:'B45 independent B42-source-parsed CIL model, NOT native Rust verification',
  worlds:cases.length,grouped,
  cases:cases.map(({events,...data})=>({...data,internalEvents:events.length}))
};
if(process.argv[1]===fileURLToPath(import.meta.url))
  process.stdout.write(JSON.stringify(output,null,2)+'\n');
export {run,output};
