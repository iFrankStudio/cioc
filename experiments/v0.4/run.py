from cioc import SCENARIOS, run_case
from pathlib import Path
import json

root = Path(__file__).resolve().parent
out = root / 'results'
out.mkdir(exist_ok=True)
results = [run_case(s) for s in SCENARIOS]
(out/'experiments.json').write_text(json.dumps(results,ensure_ascii=False,indent=2),encoding='utf-8')
lines=[]
for r in results:
    line=(f"{r['scenario']:26s} viable={str(r['viable']):5s} "
          f"internal={r['internal_commits']} external={r['external_repairs']} "
          f"probes={r['probes']} budget={r['remaining_budget']}")
    print(line); lines.append(line)
(out/'console.txt').write_text('\n'.join(lines)+'\n', encoding='utf-8')
