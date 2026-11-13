from pathlib import Path
import json
from cioc import scenarios

if __name__ == '__main__':
    data = scenarios()
    path = Path(__file__).parent / 'results' / 'experiments.json'
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(data, ensure_ascii=False, indent=2))
