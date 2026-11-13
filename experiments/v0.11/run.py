from pathlib import Path
import json
from cioc import scenarios


def main():
    result = scenarios()
    target = Path(__file__).parent / 'results' / 'experiments.json'
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == '__main__':
    main()
