"""Run all experiments and export machine-readable reproducible results."""
import csv, json
from pathlib import Path
from simulator import scenario, intervene
from analyzer import SCENARIOS, discover, classify, probe

ROOT = Path(__file__).resolve().parent
OUT = ROOT / 'results'
OUT.mkdir(exist_ok=True)


def main():
    reports = []
    csvrows = []
    for name in SCENARIOS:
        findings = discover(name, repetitions=50, max_order=2)
        conclusions = classify(name, findings)
        reports.append({**findings, 'summary': conclusions})
        for entry in findings['minimal_disabling_sets']:
            csvrows.append({'scenario': name, 'target': entry['target'],
                            'disabled': '+'.join(entry['disabled']),
                            'baseline': entry['baseline_rate'],
                            'intervention': entry['intervention_rate'],
                            'loss': entry['loss_of_recovery']})
    # Controlled resource scarcity: same organizational closure edges cannot
    # repair if compute budget is too small to pay housekeeping + repair.
    scarce = []
    for b in (1, 2, 3, 4, 14):
        t = intervene(scenario('internal_closure'), 'B', budget=b)
        scarce.append({'budget': b, 'recovered': t.recovered,
                       'remaining': t.remaining_budget, 'repairs': len(t.repairs)})
    # Noisy check tests robustness to imperfect repair, with paired seed draws.
    stochastic = []
    for name, target, disabled in (
        ('internal_closure', 'B', ('A',)),
        ('external_repair', 'B', ('F',)),
        ('redundant_maintenance', 'C', ('A',)),
        ('redundant_maintenance', 'C', ('A', 'B')),
    ):
        est = probe(name, target, disabled, repetitions=200,
                    repair_success=.45, budget=14, horizon=2)
        stochastic.append(est.__dict__)

    payload = {'protocol': {'repetitions_deterministic': 50,
                            'repetitions_stochastic': 200,
                            'max_ablation_order': 2,
                            'horizon_events': 4,
                            'initial_budget': 14,
                            'repair_cost': 2,
                            'maintenance_per_event': 1,
                            'detect_loss_threshold': .25},
               'scenarios': reports, 'resource_scarcity': scarce,
               'stochastic_checks': stochastic}
    (OUT / 'experiments.json').write_text(json.dumps(payload, indent=2, ensure_ascii=False), encoding='utf-8')
    with (OUT/'detected_cutsets.csv').open('w', encoding='utf-8', newline='') as f:
        writer = csv.DictWriter(f, fieldnames=['scenario','target','disabled','baseline','intervention','loss'])
        writer.writeheader(); writer.writerows(csvrows)
    print(json.dumps({'cutsets': csvrows, 'resource_scarcity': scarce,
                      'stochastic_checks': stochastic}, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
