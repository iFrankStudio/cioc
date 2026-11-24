"""Independent ordinary CIL replay of every native paid event. No model cost predictions for prospective cases."""
import copy,json,sys
from pathlib import Path
m=json.loads(Path(sys.argv[1]).read_text())
folder=Path(sys.argv[2])
summary=json.loads((folder/'summary.json').read_text())
rows=[]
keys=[11,41,44,45,46,47]
for case in m['cases']:
 for arm in m['arms']:
  name=case['id']+'_'+arm['id']
  src=m['sources'][arm['source']]
  g=copy.deepcopy(src['nodes']);old=src['old_pc'];dispatch=src['dispatch']
  r=[0]*4;d={50:case['mode']};pc=0;credit=m['capacity']['credits']
  stage=prod=trial=paid=0;probes=[];edits=[];peak=len(g)+2
  with (folder/(name+'.jsonl')).open() as f:
   for line in f:
    obs=json.loads(line)
    assert 'change' not in obs,(name,'unexpected intervention',obs)
    p=pc;n=g[p];op=old[p];paid+=1;credit-=1;pc=n.get('next');kind='executed';stage_write=False
    if op==0:probes.append(stage)
    t=n['kind']
    if t=='SenseOwnCredits':r[n['dst']]=credit
    elif t=='Const':r[n['dst']]=n['value']
    elif t=='Read':r[n['dst']]=d.get(r[n['address']],0);kind='read'
    elif t=='Write':
     key=r[n['address']];value=r[n['value']];d[key]=value;kind='write'
     if key==11:
      assert value==1
      if op in (12,42):trial+=1
      else:prod+=1
     if key==45:
      stage+=1;assert stage==value==prod;stage_write=True
    elif t=='Sub':r[n['dst']]=r[n['lhs']]-r[n['rhs']]
    elif t=='Add':r[n['dst']]=r[n['lhs']]+r[n['rhs']]
    elif t=='Branch':pc=n['nonzero'] if r[n['test']]!=0 else n['zero']
    elif t=='ReadNext':r[n['dst']]=g[r[n['node']]]['next']
    elif t=='PatchNext':
     assert r[n['node']]==dispatch and r[n['target']] in src['targets']
     before=g[dispatch]['next'];target=r[n['target']]
     if arm['off']:kind='suppressed'
     else:g[dispatch]['next']=target
     edits.append([stage,old[before],old[target],old[g[dispatch]['next']],arm['off']])
    else:raise AssertionError((name,t))
    occupied=len(g)+1+len(d);peak=max(peak,occupied)
    expected=dict(step=paid,pc=p,next_pc=pc,registers=r,credits=credit,cells=[d.get(k) for k in keys],mode=d[50],edge=g[dispatch]['next'],occupied=occupied,kind=kind)
    assert obs==expected,(name,paid,obs,expected)
    if stage_write and stage<case['n']:
     for at,value in case['changes']:
      if stage==at:
       assert json.loads(next(f))=={'change':[paid,stage,value]}
       d[50]=value
    if stage==case['n']:
     assert not f.readline(),(name,'extra rows')
     break
  assert stage==prod==case['n']
  row=dict(id=name,case=case['id'],arm=arm['id'],paid=paid,production=prod,trials=trial,probes=probes,edits=edits,nodes=len(g),peak_occupied=peak,cutoff=True)
  assert row==next(x for x in summary['rows'] if x['id']==name),(name,row)
  rows.append(row)
assert len(rows)==60
pairs=[]
for case in m['cases']:
 a={x['arm']:x for x in rows if x['case']==case['id']}
 for policy in ['duplicate','shared']:
  on=a[policy+'_on'];off=a[policy+'_off']
  # Causal contrast checks exact original prefix until first physical edit.
  paths=[folder/(x['id']+'.jsonl') for x in (on,off)]
  first=None
  with paths[0].open() as l,paths[1].open() as r:
   for i,(x,y) in enumerate(zip(l,r)):
    x=json.loads(x);y=json.loads(y)
    if x==y:continue
    # Suppression changes event label even for a redundant edit. Normalize it;
    # physical exposure is first unequal actual executable edge.
    assert x.get('pc')==y.get('pc')
    assert {k:v for k,v in x.items() if k not in ('kind','edge')}=={k:v for k,v in y.items() if k not in ('kind','edge')}
    assert x['kind']=='executed' and y['kind']=='suppressed'
    if x['edge']!=y['edge']:first=x['step'];break
  pairs.append(dict(case=case['id'],cohort=case['cohort'],policy=policy,physical_saving=off['paid']-on['paid'],economic_saving=a['once']['paid']-on['paid'],first_physical_exposure=first,on=on['paid'],off=off['paid'],once=a['once']['paid'],peak_occupied=on['peak_occupied']))
print(json.dumps(dict(study=m['study'],replayed=len(rows),first_divergence=None,pairs=pairs),indent=2))
