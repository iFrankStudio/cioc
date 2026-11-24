"""Replay every original native C29 instruction; independently check paired inputs and outputs."""
import copy,json,sys
from pathlib import Path
m=json.loads(Path(sys.argv[1]).read_text()); folder=Path(sys.argv[2])
summary=json.loads((folder/'summary.json').read_text()); rows=[]
keys=[11,40,41,42,43,44,45,46,50,51,60]
for case in m['cases']:
 for arm in m['arms']:
  name=case['id']+'_'+arm['id']; src=m['sources'][arm['source']]
  g=copy.deepcopy(src['g']); l=src['labels']; dispatch=l['dispatch']
  r=[0]*4; d={50:case['xy'][0],51:case['xy'][1]}; pc=0;credit=m['capacity']['credits']
  stage=prod=trials=paid=0;probes=[];edits=[];measurements=[];inputs=[None,None];peak=len(g)+3;mid=False
  with (folder/(name+'.jsonl')).open() as f:
   for line in f:
    obs=json.loads(line);assert 'change' not in obs
    p=pc;n=g[p];paid+=1;credit-=1;pc=n.get('next');kind='executed';stage_write=False;trial_a=False
    if p==l['eval']:probes.append(stage)
    if p==l['loop_a'] and inputs[0] is None:inputs[0]=[r[1],r[2]]
    if p==l['loop_b'] and inputs[1] is None:inputs[1]=[r[2],r[1]]
    if p==l['compare']:
     a,b=inputs;inputs=[None,None]
     assert d[45]==11+3*a[0] and d[46]==11+3*b[1]
     assert d[43]==a[0]*a[1] and d[44]==b[0]*b[1]
     if arm['source']!='live':assert a==b==[d[40],d[41]]
     measurements.append(dict(stage=stage,a_input=a,b_input=b,a=d[45],b=d[46],outputs=[d[43],d[44]],live=[d[50],d[51]]))
    t=n['kind']
    if t=='SenseOwnCredits':r[n['dst']]=credit
    elif t=='Const':r[n['dst']]=n['value']
    elif t=='Read':r[n['dst']]=d.get(r[n['address']],0);kind='read'
    elif t=='Write':
     k=r[n['address']];v=r[n['value']];d[k]=v;kind='write'
     if k==11:assert v==d[50]*d[51];prod+=1
     if k in (43,44):trials+=1;trial_a=k==43
     if k==60:stage+=1;assert stage==prod==v;stage_write=True
    elif t=='Sub':r[n['dst']]=r[n['lhs']]-r[n['rhs']]
    elif t=='Add':r[n['dst']]=r[n['lhs']]+r[n['rhs']]
    elif t=='Branch':pc=n['nonzero'] if r[n['test']]!=0 else n['zero']
    elif t=='ReadNext':r[n['dst']]=g[r[n['node']]]['next']
    elif t=='PatchNext':
     assert r[n['node']]==dispatch and r[n['target']] in (l['prod_a'],l['prod_b'])
     before=g[dispatch]['next'];target=r[n['target']];assert before!=target
     if arm['off']:kind='suppressed'
     else:g[dispatch]['next']=target
     edits.append([stage,before,target,g[dispatch]['next'],arm['off']])
    else:raise AssertionError((name,t))
    occupied=len(g)+1+len(d);peak=max(peak,occupied)
    expected=dict(step=paid,pc=p,next_pc=pc,registers=r,credits=credit,cells=[d.get(k) for k in keys],edge=g[dispatch]['next'],occupied=occupied,kind=kind)
    assert obs==expected,(name,paid,obs,expected)
    if trial_a and not mid and 'midTrial' in case:
     mid=True;x,y=case['midTrial'];assert json.loads(next(f))=={'change':[paid,stage,x,y,'mid_trial']};d[50]=x;d[51]=y
    if stage_write and stage<case['n']:
     for at,x,y in case.get('changes',[]):
      if stage==at:assert json.loads(next(f))=={'change':[paid,stage,x,y,'stage']};d[50]=x;d[51]=y
    if stage==case['n']:
     assert not f.readline();break
  assert stage==prod==case['n'] and trials==2*len(measurements)
  row=dict(id=name,case=case['id'],arm=arm['id'],paid=paid,production=prod,trials=trials,probes=probes,edits=edits,measurements=measurements,nodes=len(g),peak_occupied=peak,cutoff=True)
  assert row==next(x for x in summary['rows'] if x['id']==name),(name,row)
  rows.append(row)
assert len(rows)==80
pairs=[]
for case in m['cases']:
 a={x['arm']:x for x in rows if x['case']==case['id']}
 for policy in ('snapshot','live'):
  on=a[policy+'_on'];off=a[policy+'_off'];first=None
  with (folder/(on['id']+'.jsonl')).open() as lf,(folder/(off['id']+'.jsonl')).open() as rf:
   for x,y in zip(lf,rf):
    x=json.loads(x);y=json.loads(y)
    if x==y:continue
    assert {k:v for k,v in x.items() if k not in ('kind','edge')}=={k:v for k,v in y.items() if k not in ('kind','edge')}
    assert x['kind']=='executed' and y['kind']=='suppressed'
    if x['edge']!=y['edge']:first=x['step'];break
  pairs.append(dict(case=case['id'],cohort=case['cohort'],policy=policy,physical_saving=off['paid']-on['paid'],economic_saving=a['once']['paid']-on['paid'],first_physical_exposure=first,on=on['paid'],off=off['paid'],once=a['once']['paid'],peak_occupied=on['peak_occupied'],mismatched_inputs=sum(q['a_input']!=q['b_input'] for q in on['measurements']),equal_output_mismatched_inputs=sum(q['a_input']!=q['b_input'] and q['outputs'][0]==q['outputs'][1] for q in on['measurements'])))
print(json.dumps(dict(study=m['study'],replayed=len(rows),first_divergence=None,pairs=pairs),indent=2))
