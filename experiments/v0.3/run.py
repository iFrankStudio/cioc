import json
from pathlib import Path
from cioc import SCENARIOS, run_case

def main():
    results = [run_case(s) for s in SCENARIOS]
    output = Path(__file__).parent / 'results' / 'experiments.json'
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(results, ensure_ascii=False, indent=2), encoding='utf-8')
    for r in results:
        print(f"{r['scenario']:22} viable={str(r['viable']):5} internal={r['internal_commits']} "
              f"external={r['external_repairs']} checks={r['candidate_checks']} budget={r['remaining_budget']} "
              f"programs={r['synthesized_programs']}")
    print('saved:', output)

if __name__ == '__main__': main()
