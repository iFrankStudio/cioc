"""Matched-seed study of maintenance costs and severe damage."""
from cioc import World

def evaluate(seeds=range(30)):
    out={}
    for damage in (0.0,0.07,0.15):
        for repair in (True,False):
            cases=[]
            for seed in seeds:
                w=World(seed=seed,damage_rate=damage,repair_enabled=repair)
                w.insert();w.run(80)
                cases.append({'seed':seed,'population':len(w.organisms),
                              'births':w.births,'reconfigurations':w.repairs,
                              'faults':w.faults})
            key=f'damage={damage};repair={repair}'
            out[key]={'mean_population':sum(c['population'] for c in cases)/len(cases),
                      'mean_births':sum(c['births'] for c in cases)/len(cases),
                      'runs':cases}
    return out
if __name__=='__main__':
    import json
    from pathlib import Path
    path=Path(__file__).resolve().parent/'results'/'multiseed.json'
    path.write_text(json.dumps(evaluate(),indent=2)+'\n')
