from cioc import SCENARIOS, run_case
from pathlib import Path
import json

out = Path(__file__).parent / 'results'
out.mkdir(exist_ok=True)
results = [run_case(x) for x in SCENARIOS]
(out / 'experiments.json').write_text(json.dumps(results, ensure_ascii=False, indent=2), encoding='utf-8')
for r in results:
    print(f"{r['scenario']:21} viable={str(r['viable']):5} internal={r['internal_commits']} external={r['external_repairs']} remaining={r['remaining_compute']}")
