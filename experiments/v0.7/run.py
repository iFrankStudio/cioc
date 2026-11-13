import json
from cioc import experiment
if __name__=='__main__':
    print(json.dumps(experiment(),ensure_ascii=False,indent=2))
