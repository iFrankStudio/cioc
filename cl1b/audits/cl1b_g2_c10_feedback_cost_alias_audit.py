#!/usr/bin/env python3
"""Read-only C10 original-native feedback/cost-channel alias audit.

This is a post hoc reanalysis of prior verified Rust-native evidence, not a
new experiment nor an actor-visible value oracle. See the fixed protocol.
"""
import argparse
import hashlib
import json
from collections import defaultdict
from pathlib import Path
from zipfile import ZipFile

ZIP_SHA = '9b468cd8ce3c9ddc8c07ebe092decf297a7a9e77e65c7ebfe62d0f53e4f0d276'
MEMBER_SHA = {
    'cl1b_g2_state_rollback_c10_native_274_worlds.json': 'feeafdfe9c805c7bd7079c5854cf21cacb8089b6534b20764b45795d37236129',
    'cl1b_g2_state_rollback_c10_independent_replay.json': '76d67596752490687b67c6ecc882213da08a44ae32176499a6cff45dffad180e',
}
ARM_ORDER = (
    'G2_rollback', 'G2_rollback_pc102_suppressed',
    'G2_rollback_pc28_suppressed', 'G2_rollback_placebo180',
    'G2_C9_checkpoint_no_rollback', 'G2_C9_checkpoint_abstain',
)

def load_original(path):
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    assert digest == ZIP_SHA, f'Git archive byte hash differs: {digest}'
    assert path.stat().st_size == 8868234
    with ZipFile(path) as z:
        assert z.testzip() is None
        names = z.namelist()
        assert len(names) == 278 and len(names) == len(set(names))
        assert sum(n.endswith('.jsonl') for n in names) == 274
        results = {}
        for basename, expected in MEMBER_SHA.items():
            matches = [n for n in names if n == basename or n.endswith('/' + basename)]
            assert len(matches) == 1, (basename, matches)
            raw = z.read(matches[0])
            assert hashlib.sha256(raw).hexdigest() == expected, basename
            results[basename] = json.loads(raw)
    return (results['cl1b_g2_state_rollback_c10_native_274_worlds.json'],
            results['cl1b_g2_state_rollback_c10_independent_replay.json'])

def service_sequence(record):
    return [(x['stage'], x['guess'], x['feedback']) for x in record['services']]

def counters(record):
    events = record['services']
    assert all(x['feedback'] == x['truth'] for x in events)
    return len(events), sum(x['feedback'] == 0 for x in events)

def audit(native, independent):
    assert native['study'] == independent['study'] == 'CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10'
    assert native['native_worlds'] == independent['native_original_worlds'] == 274
    assert native['total_private_paid_cil'] == independent['original_native_private_paid_CIL_replayed'] == 887330
    assert independent['first_divergence'] is None
    assert len(native['truthful_records']) == 270
    assert len(native['adversarial_records']) == 4
    assert len(native['per_case']) == 45
    assert sum(x['steps'] for x in native['truthful_records'] + native['adversarial_records']) == 887330
    assert independent['fees'] == native['costs']
    assert native['costs']['G2_rollback_pc102_suppressed'] - native['costs']['G2_rollback'] == 1840

    result = []
    family = defaultdict(lambda: {'cases': 0, 'cost_delta_total': 0, 'cost_win': 0,
                                  'cost_loss': 0, 'cost_tie': 0,
                                  'exact_feedback_alias': 0,
                                  'attempt_failure_count_alias': 0})
    for index in range(45):
        group = native['truthful_records'][index * 6:(index + 1) * 6]
        assert tuple(x['source'] for x in group) == ARM_ORDER
        assert all(x['feedback'] == 'truth' and x['completed_stages'] == 48 and
                   x['truthful_matches'] == 48 and x['claim'] == 1 and
                   x['budget_exhausted'] is False for x in group)
        assert len({tuple(x['goals']) for x in group}) == 1
        live, suppressed, _, placebo, _, _ = group
        item = native['per_case'][index]
        assert item['case'] == live['name'].removesuffix('_G2_rollback')
        assert item['regime'] == live['regime'] == suppressed['regime']
        assert live['steps'] == placebo['steps']
        assert service_sequence(live) == service_sequence(placebo)
        assert live['rollback_attempt_records'] == placebo['rollback_attempt_records']
        assert suppressed['genuine_paid_physical_parent_restores'] == 0
        assert live['genuine_paid_physical_parent_restores'] == item['actual_physical_parent_restores']
        assert live['steps'] == item['live_cost']
        assert suppressed['steps'] == item['same_source_suppress_PC102_cost']

        delta = suppressed['steps'] - live['steps']
        live_attempts, live_failures = counters(live)
        sup_attempts, sup_failures = counters(suppressed)
        eq_feedback = service_sequence(live) == service_sequence(suppressed)
        eq_counts = (live_attempts, live_failures) == (sup_attempts, sup_failures)
        assert not eq_feedback or eq_counts
        row = {
            'case': item['case'], 'regime': item['regime'],
            'delta_cil_suppressed_minus_live': delta,
            'live_paid_cil': live['steps'], 'suppressed_paid_cil': suppressed['steps'],
            'live_attempts': live_attempts, 'suppressed_attempts': sup_attempts,
            'live_failed_feedback': live_failures, 'suppressed_failed_feedback': sup_failures,
            'identical_guess_and_feedback_sequence': eq_feedback,
            'identical_attempt_and_failure_counts': eq_counts,
            'exact_feedback_cost_alias': eq_feedback and delta != 0,
            'coarse_count_cost_alias': eq_counts and delta != 0,
            'physical_parent_restorations_live': live['genuine_paid_physical_parent_restores'],
            'later_original_source_readbacks_live': len(live['later_actual_rollback_readbacks']),
        }
        result.append(row)
        f = family[item['regime']]
        f['cases'] += 1
        f['cost_delta_total'] += delta
        f['cost_win' if delta > 0 else 'cost_loss' if delta < 0 else 'cost_tie'] += 1
        f['exact_feedback_alias'] += int(row['exact_feedback_cost_alias'])
        f['attempt_failure_count_alias'] += int(row['coarse_count_cost_alias'])

    assert sum(v['cost_delta_total'] for v in family.values()) == 1840
    assert all(v['cases'] == 9 for v in family.values()) and len(family) == 5
    assert sum(r['live_paid_cil'] for r in result) == native['costs']['G2_rollback'] == 153000
    assert sum(r['suppressed_paid_cil'] for r in result) == native['costs']['G2_rollback_pc102_suppressed'] == 154840
    return {
        'study': 'CL1B-C10-FEEDBACK-COST-ALIAS',
        'status': 'RETROSPECTIVE_SOURCE_PINNED_READ_ONLY_ANALYSIS',
        'preregistration': 'cl1b/studies/CL1B-C10-FEEDBACK-COST-ALIAS-preregistration.md',
        'tested_original_native_source_sha': '353e8fdc914dd9f52f66d22802ebb3afcd4ebb1e',
        'original_run_id': 45041352673,
        'original_zip_sha256': ZIP_SHA,
        'observed_channel': 'ordered actor-paid Write11 (stage,guess) and returned truth-mode feedback only; omits host instruction counts, actor internal state and counterfactual futures',
        'total_original_native_worlds_referenced': 274,
        'truthful_comparable_pairs': 45,
        'all_six_truth_arms': 270,
        'original_native_paid_cil_crosschecked': 887330,
        'strict_same_initial_source_delta_cil': 1840,
        'live_paid_cil': 153000,
        'suppressed_paid_cil': 154840,
        'effect_positive_cases': sum(x['delta_cil_suppressed_minus_live'] > 0 for x in result),
        'effect_negative_cases': sum(x['delta_cil_suppressed_minus_live'] < 0 for x in result),
        'effect_zero_cases': sum(x['delta_cil_suppressed_minus_live'] == 0 for x in result),
        'exact_feedback_cost_alias_pairs': sum(x['exact_feedback_cost_alias'] for x in result),
        'coarse_attempt_failure_count_cost_alias_pairs': sum(x['coarse_count_cost_alias'] for x in result),
        'per_family': dict(sorted(family.items())),
        'per_case': result,
        'limits': [
            'Retrospective analysis of preexisting original-native output, not a new Rust run.',
            'An alias excludes only an estimator restricted to exactly this service-feedback channel, not complete actor-visible private history.',
            'No actor-computed paid fee counter, online treatment comparison, ROI learner, external fitness oracle, CL-1B B3 or B4 is demonstrated.',
            'Different-initial-CIL C9 controls are deliberately excluded from same-source causal pairs.',
        ],
    }

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('original_git_zip', type=Path)
    ap.add_argument('output_json', type=Path)
    args = ap.parse_args()
    native, independent = load_original(args.original_git_zip)
    d = audit(native, independent)
    args.output_json.parent.mkdir(parents=True, exist_ok=True)
    args.output_json.write_text(json.dumps(d, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')
    print('SOURCE_PINNED_AUDIT ' + json.dumps({k: d[k] for k in (
        'truthful_comparable_pairs', 'strict_same_initial_source_delta_cil',
        'effect_positive_cases', 'effect_negative_cases', 'effect_zero_cases',
        'exact_feedback_cost_alias_pairs', 'coarse_attempt_failure_count_cost_alias_pairs',
        'per_family')}, ensure_ascii=False, sort_keys=True))

if __name__ == '__main__':
    main()
