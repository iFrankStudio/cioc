import json
from cioc import scenarios

if __name__ == "__main__":
    print(json.dumps(scenarios(), ensure_ascii=False, indent=2))
