"""Independent C33 exact instruction replay, no Rust implementation dependencies."""
import copy,json,sys
from pathlib import Path
m=json.loads(Path(sys.argv[1]).read_text());p=Path(sys.argv[2]);native=json.loads((p/'summary.json').read_text())
keys=[11,12,41,42,43,44,50,51,52,53,58]
def rec(a,b,k):
 z=b
 for _ in range(k):z=a*z+b
 return z
def depth(x):return x.bit_length()-1+x.bit_count()
jobs=[(c,a) for c in m['cases'] for a in m['arms']]+[(c,dict(id='fault',source=c['source'],off=False)) for c in m['faults']]
rows=[]
for c,a in jobs:
 name=c['id']+'_'+a['id'];src=m['sources'][a['source']];g=copy.deepcopy(src['g']);l=src['l'];initial=len(g);q=c['xy'][:];k=c['k'];d=dict(zip(range(50,54),q));d[58]=k;r=[0]*4;pc=paid=prod=kernels=sk=cs=sc=reused_count=0;credit=200000;peak=initial+1+len(d);first=None;reused=False;wrong=None;clones=[];activations=[]
 with (p/(name+'.jsonl')).open() as f:
  while prod<c['n']:
   obs=json.loads(next(f));at=pc;n=copy.copy(g[at]);paid+=1;credit-=1;pc=n.get('next');kind='executed';mutation=None;produced=False
   if at==l['batch']:first=None;reused=False
   if at==l['reuse']:reused=True
   if at==l['kernel_dispatch']:
    i=0 if first is None else 2;assert r[1]==q[i] and r[2]==rec(q[i],q[i+1],sk%k);entry=initial+r[1]-1;sk+=1;kernels+=1
   if at>=initial:cs+=1;sc+=1
   t=n['kind']
   if t=='Const':r[n['dst']]=n['value']
   elif t=='Read':r[n['dst']]=d.get(r[n['address']],0);kind='read'
   elif t=='Write':
    key=r[n['address']];v=r[n['value']];d[key]=v;kind='write'
    if key==11:assert first is None and sk==k and v==rec(q[0],q[1],k);first=v
    if key==12:
     want=rec(q[2],q[3],k);assert first==rec(q[0],q[1],k)
     if a['id']=='fault':assert reused and v!=want;wrong=dict(actual=v,expected=want)
     else:assert v==want and reused==(g[l['gate']]['next']==l['check'] and q[:2]==q[2:])
     assert sk==k*(1 if reused else 2);assert sc==(0 if a['source']=='plain' else k*(depth(q[0])+(0 if reused else depth(q[2]))));reused_count+=int(reused);sk=sc=0;prod+=1;produced=True
   elif t=='Add':r[n['dst']]=r[n['lhs']]+r[n['rhs']]
   elif t=='Sub':r[n['dst']]=r[n['lhs']]-r[n['rhs']]
   elif t=='Branch':pc=n['nonzero'] if r[n['test']] else n['zero']
   elif t=='CloneNode':
    origin=r[n['source']];idx=len(g);assert origin==l['parent_add' if (idx-initial)%2==0 else 'double_src'];g.append(copy.copy(g[origin]));r[n['dst']]=idx;clones.append([paid,origin,idx]);mutation=['clone',origin,idx]
   elif t=='PatchNext':
    i=r[n['node']];to=r[n['target']];assert 0<=to<len(g)
    if i>=initial:
     x=i-initial+1;assert to==(l['out'] if x==1 else i-1 if x%2 else initial+x//2-1)
    elif i==l['special_entry']:assert to==entry
    elif i==l['return_route']:assert to==l['first_result' if first is None else 'second_result']
    else:assert (i,to) in [(l['kernel_dispatch'],l['base']),(l['decision'],l['gate']),(l['gate'],l['check'])]
    old=g[i]['next'];g[i]['next']=to;mutation=['next',i,old,to,False]
   elif t=='PatchConst':
    i=r[n['node']];to=r[n['value']];old=g[i]['value'];suppress=a['off'] and at==l['activate']
    if i==l['flag']:assert at==l['activate'] and to==1 and q[:2]==q[2:] and not activations;activations.append([paid,prod,suppress])
    else:assert i==l['base'] and to==initial-1
    if suppress:kind='suppressed'
    else:g[i]['value']=to
    mutation=['const',i,old,to,suppress]
   else:raise AssertionError(t)
   occupied=len(g)+1+len(d);peak=max(peak,occupied);assert occupied<=220 and len(g)<=200 and len(d)<=16 and credit>=0
   expected=dict(step=paid,pc=at,next_pc=pc,registers=r,credits=credit,cells=[d.get(key) for key in keys],nodes=len(g),gate=g[l['gate']]['next'],occupied=occupied,kind=kind,mutation=mutation)
   assert obs==expected,(name,paid,obs,expected)
   if produced and prod<c['n']:
    for ch in c['changes']:
     if ch[0]==prod:q=ch[1:];assert json.loads(next(f))=={'change':[paid,prod,q]};d.update(zip(range(50,54),q))
  assert not f.readline()
 row=dict(id=name,case=c['id'],cohort=c['cohort'],arm=a['id'],k=k,paid=paid,batches=prod,kernels=kernels,clone_steps=cs,reused=reused_count,initial=initial,nodes=len(g),peak=peak,activations=activations,clones=clones,wrong=wrong)
 assert row==next(x for x in native['rows'] if x['id']==name),(name,row);rows.append(row)
assert len(rows)==82
pairs=[]
for c in m['cases']:
 a={r['arm']:r for r in rows if r['case']==c['id']};first=None
 with (p/(a['on']['id']+'.jsonl')).open() as f,(p/(a['off']['id']+'.jsonl')).open() as g:
  for x,y in zip(f,g):
   x=json.loads(x);y=json.loads(y)
   if x==y:continue
   assert x['pc']==m['sources']['evolve']['l']['activate'] and x['kind']=='executed' and y['kind']=='suppressed'
   assert {k:v for k,v in x.items() if k not in ('kind','mutation')}=={k:v for k,v in y.items() if k not in ('kind','mutation')}
   assert x['mutation'][:-1]==y['mutation'][:-1] and x['mutation'][-1]==False and y['mutation'][-1]==True
   first=x['step'];break
  if first is None:assert not f.readline() and not g.readline() and a['on']['paid']==a['off']['paid']
 assert (first is not None)==bool(a['on']['activations'])
 pairs.append(dict(case=c['id'],cohort=c['cohort'],k=c['k'],first_exposure=first,net=a['shared']['paid']-a['on']['paid'],physical=a['off']['paid']-a['on']['paid'],static_delta=a['static']['paid']-a['on']['paid'],kernels_saved=a['shared']['kernels']-a['on']['kernels'],on=a['on']['paid'],off=a['off']['paid'],shared=a['shared']['paid']))
print(json.dumps(dict(study=m['study'],replayed=len(rows),pairs=pairs,falsifiers=[r for r in rows if r['wrong'] is not None]),indent=2))
