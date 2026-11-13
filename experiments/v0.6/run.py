import json
from pathlib import Path
from cioc import Organism


def scenario(name, damage, **kwargs):
    o = Organism.create(compute=kwargs.get('compute', 1500), available_ops=kwargs.get('ops', ('INC','DOUBLE','SQUARE','DEC')))
    for target in damage: o.damage(target)
    if kwargs.get('disable_detection'): o.detection_enabled=False
    if kwargs.get('external'): o.restore_external(kwargs['external'])
    result=o.run(ticks=kwargs.get('ticks',300))
    return {'name':name,'damage':damage,'summary':result,'history':o.history}


def main():
    scenarios = [
      scenario('task_algorithm_reconstruction', ['TASK']),
      scenario('contract_data_reconstruction', ['CONTRACT']),
      scenario('contract_then_algorithm_reconstruction', ['TASK','CONTRACT']),
      scenario('observer_disabled', ['TASK','O']),
      scenario('generator_disabled', ['TASK','G']),
      scenario('verifier_disabled', ['TASK','V']),
      scenario('modifier_disabled', ['TASK','M']),
      scenario('contract_share_O_missing', ['CONTRACT','SHARD_O']),
      scenario('contract_share_V_missing', ['CONTRACT','SHARD_V']),
      scenario('resource_exhaustion', ['TASK'], compute=12),
      scenario('required_primitive_missing', ['TASK'], ops=('INC','DEC','SQUARE')),
      scenario('external_restore_control', ['TASK'], external='TASK'),
      scenario('no_internal_detection', ['TASK'], disable_detection=True),
    ]
    folder=Path(__file__).parent/'results'; folder.mkdir(exist_ok=True)
    (folder/'experiments.json').write_text(json.dumps(scenarios,ensure_ascii=False,indent=2), encoding='utf-8')
    for s in scenarios:
        z=s['summary']
        print(f"{s['name']:<37} task={str(z['task_restored']):<5} contract={str(z['contract_restored']):<5} internal={z['internal_task_repairs']}/{z['internal_contract_repairs']} heldout={z['heldout_passed']} attempts={z['attempted']}")
    return scenarios

if __name__=='__main__': main()
