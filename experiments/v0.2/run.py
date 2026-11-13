from pathlib import Path
import json
from dynamic import run_case

ROOT=Path(__file__).resolve().parent
SCENARIOS=('internal_regeneration','fixed','external','maintenance_cut','insufficient_resources','broken_blueprint')
def main():
    data=[]
    for name in SCENARIOS:
        data.append(run_case(name,budget=3 if name=='insufficient_resources' else 100))
    (ROOT/'results').mkdir(exist_ok=True)
    (ROOT/'results'/'experiments.json').write_text(json.dumps(data,ensure_ascii=False,indent=2),encoding='utf-8')
    for x in data:
        print(f"{x['scenario']:26} viable={str(x['viable']):5} internal_commits={x['revisions']-x['external_repairs']} external={x['external_repairs']} candidates={x['candidates_tested']} budget_left={x['remaining_budget']}")
if __name__=='__main__': main()
