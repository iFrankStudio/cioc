"""Write reproducible V0.9 experimental outcomes (standard library only)."""
import json
from pathlib import Path
from cioc import World, scenarios


def generate():
    result = scenarios()
    for first, second in (("V", "M"), ("M", "V"), ("G", "V")):
        w = World()
        w.damage(first)
        recovered_first = w.run()
        w.damage(second)
        recovered_second = w.run()
        result[f"sequential_{first}_{second}"] = {
            "first_recovered": recovered_first,
            "second_recovered": recovered_second,
            "internal": w.internal_repairs,
            "external": w.external_repairs,
        }
    return result


if __name__ == "__main__":
    path = Path(__file__).parent / "results" / "experiments.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    data = generate()
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(data, ensure_ascii=False, indent=2))
