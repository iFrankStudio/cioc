from cioc import experiment

if __name__ == "__main__":
    import json
    print(json.dumps(experiment(), ensure_ascii=False, indent=2))
