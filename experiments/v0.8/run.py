"""Run the deterministic intervention matrix and write its result artifact."""
import json
from pathlib import Path
from cioc import scenarios


def main():
    result = scenarios()
    out = Path(__file__).parent / "results" / "experiments.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
