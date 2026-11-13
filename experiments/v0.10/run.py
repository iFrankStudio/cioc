"""Reproducible V0.10 benchmark. Run from the experiment directory."""
from pathlib import Path
import json
from cioc import scenarios

if __name__ == '__main__':
    output = Path(__file__).parent / 'results' / 'experiments.json'
    output.parent.mkdir(exist_ok=True)
    result = scenarios()
    output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(result, ensure_ascii=False, indent=2))
