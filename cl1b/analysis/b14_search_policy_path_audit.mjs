// CL-1B B14: source-parsed observer-only 30-world CIL path audit.
// This script is NOT a native Rust VM result or an agent fitness oracle.
import { readFileSync } from "node:fs";

const src = readFileSync(
  new URL("../../cl0/crates/world/tests/cl1b_b14_search_policy_mutation.rs", import.meta.url),
  "utf8"
);
const body = src.match(/fn founder\(\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if (!body) throw new Error("frozen B14 founder not found");
const ops = [...body.matchAll(/Op::([A-Za-z]+)(?:\s*\{([^}]*)\})?/g)].map((m) => {
  const o = { op: m[1] };
  for (const x of m[2]?.matchAll(/(\w+)\s*:\s*(-?\d+)/g) || []) o[x[1]] = Number(x[2]);
  return o;
});
if (ops.length !== 35) throw new Error("B14 founder changed from 35 nodes");
const allowed = new Set(["Const","Delete","Read","Write","Branch","ReadNext","Sub","PatchConst","PatchNext","Halt"]);
if (ops.some(x => !allowed.has(x.op))) throw new Error("B14 unknown native opcode");

function simulate(input, arm) {
  const code = ops.map(o => ({...o})), r = [0,0,0,0], cells = new Map([[7,input]]);
  let pc = 0, steps = 0, markers = 0, services = 0, incidentalKey1 = 0, suppressed = 0;
  const edges = [], policy = [];
  if (arm !== "SHAM7") code[6].next = arm.startsWith("LESION11") ? 11 : 10;
  if (arm.includes("POLICY_LOCKED")) code[21].next = 23;
  const reg = k => { if (!Number.isInteger(k) || k < 0 || k >= 4) throw Error("invalid register"); return r[k]; };
  const edge = k => { if (!Number.isInteger(k) || k < 0 || k >= 35) throw Error("invalid edge"); return k; };
  while (steps < 120) {
    const at = pc, op = code[at];
    if (!op) throw new Error("no current instruction");
    steps++;
    switch (op.op) {
      case "Const": r[op.dst] = op.value; pc = edge(op.next); break;
      case "Delete": cells.delete(reg(op.address)); pc = edge(op.next); break;
      case "Read": r[op.dst] = cells.get(reg(op.address)) ?? 0; pc = edge(op.next); break;
      case "Write": {
        const key=reg(op.address), value=reg(op.value);
        if (!cells.has(key) && (cells.size >= 16 || cells.size + 36 >= 128))
          throw new Error("world storage capacity exceeded");
        cells.set(key,value); pc=edge(op.next);
        if (key === 13 && value === 1) markers++;
        if (key === 11 && value === input) services++;
        if (key === 1 && value === input) incidentalKey1++;
        break;
      }
      case "Branch": pc=edge(reg(op.test)!==0 ? op.nonzero : op.zero); break;
      case "ReadNext": {
        const target=edge(reg(op.node));
        if (code[target].next === undefined) throw Error("node has no edge");
        r[op.dst]=code[target].next;pc=edge(op.next);break;
      }
      case "Sub": r[op.dst]=reg(op.lhs)-reg(op.rhs);pc=edge(op.next);break;
      case "PatchConst": {
        const node=edge(reg(op.node)), value=reg(op.value);
        if (code[node].op !== "Const") throw Error("not an editable const");
        policy.push([code[node].value,value]);
        code[node].value=value;pc=edge(op.next);break;
      }
      case "PatchNext": {
        const node=edge(reg(op.node)), target=edge(reg(op.target));
        if (code[node].next === undefined) throw Error("node not edge-patchable");
        if (arm === "LESION10_PATCHNEXT_DISABLED") suppressed++;
        else {
          if (code[node].next !== target) edges.push([node,code[node].next,target]);
          code[node].next=target;
        }
        pc=edge(op.next);break;
      }
      default: throw Error("unexpected Halt/opcode");
    }
  }
  return { input, arm, steps, pc, edge:code[6].next, stepLiteral:code[29].value,
    services, markers, incidentalKey1, policy, edges, suppressed };
}

const arms=["SHAM7","LESION10","LESION10_POLICY_LOCKED","LESION10_PATCHNEXT_DISABLED","LESION11","LESION11_POLICY_LOCKED"];
const expected = {
  SHAM7:{pc:8,edge:7,stepLiteral:2,services:7,markers:8,incidentalKey1:0},
  LESION10:{pc:3,edge:7,stepLiteral:1,services:5,markers:4,incidentalKey1:0},
  LESION10_POLICY_LOCKED:{pc:6,edge:6,stepLiteral:2,services:1,markers:0,incidentalKey1:0},
  LESION10_PATCHNEXT_DISABLED:{pc:22,edge:10,stepLiteral:1,services:0,markers:0,incidentalKey1:0},
  LESION11:{pc:8,edge:7,stepLiteral:1,services:3,markers:3,incidentalKey1:1},
  LESION11_POLICY_LOCKED:{pc:6,edge:7,stepLiteral:2,services:4,markers:4,incidentalKey1:1}
};
const worlds=[-13,0,7,23,101].flatMap(input=>arms.map(arm=>simulate(input,arm)));
if (worlds.length !== 30) throw Error("not all B14 worlds included");
for(const w of worlds){
  for(const [field,value] of Object.entries(expected[w.arm])){
    if(w[field]!==value) throw new Error(`B14 observer mismatch ${w.arm} input=${w.input} ${field} ${w[field]}!=${value}`);
  }
}
process.stdout.write(JSON.stringify({
  source:"cl0/crates/world/tests/cl1b_b14_search_policy_mutation.rs",
  evidence:"independent JS simplified CIL path audit, NOT Rust native execution",
  totalWorlds: worlds.length, worlds,
},null,2)+"\n");
