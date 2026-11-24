"""Independent C30 replay of dynamic source allocation, mutation and execution."""
import copy,json,sys
from pathlib import Path
m=json.loads(Path(sys.argv[1]).read_text());folder=Path(sys.argv[2]);native=json.loads((folder/'summary.json').read_text())
keys=[11,40,42,43,44,45,46,47,50,51]
jobs=[(c,a) for c in m['cases'] for a in m['arms']]+[(c,dict(id='fault',source='fault',off=False)) for c in m['faults']]
rows=[]
for case,arm in jobs:
 name=case['id']+'_'+arm['id'];src=m['sources'][arm['source']];g=copy.deepcopy(src['g']);l=src['l'];initial=len(g)
 guard=l.get('guard');r=[0]*4;d=dict(zip((50,51),case['xy']));pc=0;credit=20000;paid=prod=trial_steps=production_steps=0;peak=len(g)+3
 clones=[];patches=[];trials=[]
 with (folder/(name+'.jsonl')).open() as f:
  for line in f:
   obs=json.loads(line);p=pc;n=copy.copy(g[p]);paid+=1;credit-=1;pc=n.get('next');kind='executed';mutation=None;produced=False
   if p>=initial:
    if g[l['out']]['next']==l['prod_out']:production_steps+=1
    else:trial_steps+=1
   t=n['kind']
   if t=='Const':r[n['dst']]=n['value']
   elif t=='Read':r[n['dst']]=d.get(r[n['address']],0);kind='read'
   elif t=='Write':
    k=r[n['address']];v=r[n['value']];d[k]=v;kind='write'
    if k==11:assert v==d[50]*d[51];prod+=1;produced=True
    if k==46:trials.append(dict(output=d[44],cost=v,x=d[50],y=d[51]))
   elif t=='SenseOwnCredits':r[n['dst']]=credit
   elif t=='Add':r[n['dst']]=r[n['lhs']]+r[n['rhs']]
   elif t=='Sub':r[n['dst']]=r[n['lhs']]-r[n['rhs']]
   elif t=='Branch':pc=n['nonzero'] if r[n['test']] else n['zero']
   elif t=='CloneNode':
    origin=r[n['source']];idx=len(g);assert origin==l['parent_add'];g.append(copy.copy(g[origin]));r[n['dst']]=idx
    clones.append([paid,origin,idx]);mutation=['clone',origin,idx]
   elif t=='PatchNext':
    at=r[n['node']];to=r[n['target']];assert at>=initial or at in (l['special_entry'],l['out']);assert 0<=to<len(g)
    before=g[at]['next'];g[at]['next']=to;patches.append([paid,p,at,before,to,False]);mutation=['next',at,before,to,False]
   elif t=='PatchConst':
    at=r[n['node']];to=r[n['value']];assert at==guard;before=g[at]['value'];suppress=arm['off'] and p==l['activation']
    if suppress:kind='suppressed'
    else:g[at]['value']=to
    patches.append([paid,p,at,before,to,suppress]);mutation=['const',at,before,to,suppress]
   else:raise AssertionError((name,p,t))
   occupied=len(g)+1+len(d);peak=max(peak,occupied);assert len(g)<=200 and len(d)<=16 and occupied<=220 and credit>=0
   expected=dict(step=paid,pc=p,next_pc=pc,registers=r,credits=credit,cells=[d.get(k) for k in keys],nodes=len(g),guard=g[guard]['value'] if guard is not None else None,out_edge=g[l['out']]['next'],occupied=occupied,kind=kind,mutation=mutation)
   assert obs==expected,(name,paid,obs,expected)
   if produced and prod<case['n']:
    for at,x,y in case.get('changes',[]):
     if at==prod:assert json.loads(next(f))=={'change':[paid,prod,x,y]};d[50]=x;d[51]=y
   if prod==case['n']:assert not f.readline();break
 assert prod==case['n']
 row=dict(id=name,case=case['id'],cohort=case['cohort'],arm=arm['id'],paid=paid,production=prod,initial=initial,nodes=len(g),peak_occupied=peak,clones=clones,patches=patches,trials=trials,active=g[guard]['value'] if guard is not None else None,clone_trial_steps=trial_steps,clone_production_steps=production_steps)
 assert row==next(x for x in native['rows'] if x['id']==name),(name,row)
 if arm['source'] in ('generated','fault'):
  x,y=case['xy'];assert len(clones)==x-(arm['source']=='fault') and len(trials)==2
  assert trials[0]['output']==x*y
  assert trials[1]['output']==(x-(arm['source']=='fault'))*y
  assert trials[0]['cost']==15+3*x and trials[1]['cost']==15+x-(arm['source']=='fault')
  assert trial_steps==len(clones)
  if arm['source']=='fault' or arm['off']:assert row['active']==0 and production_steps==0
  else:assert row['active']==x and production_steps>0
 rows.append(row)
assert len(rows)==67
pairs=[]
for case in m['cases']:
 a={x['arm']:x for x in rows if x['case']==case['id']};on=a['on'];off=a['off'];first=None
 with (folder/(on['id']+'.jsonl')).open() as lf,(folder/(off['id']+'.jsonl')).open() as rf:
  for x,y in zip(lf,rf):
   x=json.loads(x);y=json.loads(y)
   if x==y:continue
   assert x['pc']==m['sources']['generated']['l']['activation']
   assert {k:v for k,v in x.items() if k not in ('kind','guard','mutation')}=={k:v for k,v in y.items() if k not in ('kind','guard','mutation')}
   assert x['kind']=='executed' and y['kind']=='suppressed' and x['guard']==case['xy'][0] and y['guard']==0
   assert x['mutation'][:-1]==y['mutation'][:-1] and x['mutation'][-1]==False and y['mutation'][-1]==True
   first=x['step'];break
 assert first is not None
 pairs.append(dict(case=case['id'],cohort=case['cohort'],first_physical_exposure=first,physical_saving=off['paid']-on['paid'],economic_saving=a['plain']['paid']-on['paid'],guard_saving=a['guard']['paid']-on['paid'],on=on['paid'],off=off['paid'],plain=a['plain']['paid'],guard=a['guard']['paid'],clone_production_steps=on['clone_production_steps']))
print(json.dumps(dict(study=m['study'],replayed=len(rows),first_divergence=None,pairs=pairs),indent=2))
