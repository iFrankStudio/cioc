"""Matched-seed comparison; no p-values are inferred from this small toy sample."""
from cioc import Ecology

def evaluate(seeds=range(30)):
    output={}
    for mutation in (0.0,0.05,0.12,0.25):
        survivors=[]
        for seed in seeds:
            w=Ecology(seed=seed,mutation_rate=mutation)
            w.insert(('EAT_A','EAT_A','DIVIDE'))
            w.run(30);w.switch(0,120);w.run(50)
            survivors.append({'seed':seed,'survived':bool(w.alive),'population':len(w.alive)})
        output[str(mutation)]={'surviving_runs':sum(x['survived'] for x in survivors),
                               'runs':survivors}
    return output
if __name__=='__main__':
    import json
    from pathlib import Path
    path=Path(__file__).resolve().parent/'results'/'multiseed.json'
    path.write_text(json.dumps(evaluate(),indent=2)+'\n')
