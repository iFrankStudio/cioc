"""Black-box identification from interventions, not scenario repair rules."""
from __future__ import annotations
from dataclasses import dataclass, asdict
from itertools import combinations
from math import sqrt
from typing import Iterable
from simulator import scenario, intervene, CORE

SCENARIOS = ('call_cycle', 'internal_closure', 'external_repair', 'redundant_maintenance')
PROBES = ('A', 'B', 'C', 'D')
SOURCES = ('A', 'B', 'C', 'D', 'E', 'F')

@dataclass
class Estimate:
    target: str
    disabled: tuple[str, ...]
    baseline_rate: float
    intervention_rate: float
    loss_of_recovery: float
    baseline_wilson: tuple[float, float]
    intervention_wilson: tuple[float, float]


def wilson(k: int, n: int, z: float = 1.96) -> tuple[float, float]:
    # For sampling uncertainty. With deterministic trials, a confidence interval
    # is not a measure of hidden environmental uncertainty.
    p = k / n
    denominator = 1 + z*z/n
    center = (p + z*z/(2*n)) / denominator
    half = z * sqrt((p*(1-p) + z*z/(4*n)) / n) / denominator
    return max(0., center-half), min(1., center+half)


def probe(scenario_name: str, target: str, disabled: Iterable[str] = (),
          *, repetitions: int = 50, repair_success: float = 1.0,
          budget: int = 14, horizon: int = 4) -> Estimate:
    case = scenario(scenario_name)
    disabled = tuple(sorted(disabled))
    baseline = [intervene(case, target, seed=i, budget=budget, horizon=horizon,
                          repair_success=repair_success).recovered for i in range(repetitions)]
    ablated = [intervene(case, target, frozenset(disabled), seed=i, budget=budget,
                         horizon=horizon, repair_success=repair_success).recovered
               for i in range(repetitions)]
    kb, ka = sum(baseline), sum(ablated)
    return Estimate(target, disabled, kb/repetitions, ka/repetitions,
                    (kb-ka)/repetitions, wilson(kb, repetitions), wilson(ka, repetitions))


def discover(scenario_name: str, *, max_order: int = 2,
             repetitions: int = 50, repair_success: float = 1.,
             threshold: float = .25) -> dict:
    """Discover minimal ablation sets that causally impede recovery.

    A discovered {A,B}->C is a *cut set*, NOT proof that {A,B} must act
    jointly. Multiple redundant repair paths produce this signature too.
    """
    found = []
    base_rates = {}
    for target in PROBES:
        baseline = probe(scenario_name, target, (), repetitions=repetitions,
                         repair_success=repair_success)
        base_rates[target] = baseline.baseline_rate
        positive = []
        for size in range(1, max_order+1):
            for combo in combinations([s for s in SOURCES if s != target], size):
                effect = probe(scenario_name, target, combo, repetitions=repetitions,
                               repair_success=repair_success)
                if effect.loss_of_recovery >= threshold:
                    # We keep only minimal disabling sets with effect. This is
                    # failure-path analysis, not a direct hyperedge inference.
                    if not any(set(existing).issubset(combo) for existing in positive):
                        positive.append(combo)
        for combo in positive:
            eff = probe(scenario_name, target, combo, repetitions=repetitions,
                        repair_success=repair_success)
            found.append(asdict(eff))
    return {'scenario': scenario_name, 'baseline_recovery': base_rates,
            'minimal_disabling_sets': found,
            'rule': 'A disabling set suppresses repair under the tested conditions; it is not necessarily a direct maintenance hyperedge.'}


def classify(scenario_name: str, findings: dict) -> dict:
    """Simple boundary hypothesis, explicitly not a life classifier.

    Reciprocal single-source maintenance on A/B/C supports an *internal
    maintenance core candidate*. A strict closure claim requires additional
    tests of maintaining the maintenance mechanisms themselves.
    """
    pairs = {(tuple(x['disabled']), x['target']) for x in findings['minimal_disabling_sets']}
    sources_from_pairs = {(p[0][0], p[1]) for p in pairs if len(p[0]) == 1 and p[0][0] in CORE and p[1] in CORE}
    # cycle detection over inferred support dependencies (the direction is
    # needed-source -> damaged-target)
    adj = {x: set() for x in CORE}
    for src, dest in sources_from_pairs:
        adj[src].add(dest)
    cyclic = any(a in adj.get(c, set()) and c in adj.get(b, set()) and b in adj.get(a, set())
                 for a in CORE for b in CORE for c in CORE if len({a,b,c}) == 3)
    # A joint disabling cut set {A,B} gives the Boolean support constraint
    # (A OR B), not proof of a joint A-and-B repair mechanism. For candidate
    # screening, all its members are potential providers; downstream closure
    # verification must test actual repair mechanisms and alternatives.
    potential = {x: set() for x in CORE}
    internal_targets = set()
    for entry in findings['minimal_disabling_sets']:
        dest = entry['target']
        providers = set(entry['disabled'])
        if dest in CORE and providers and providers.issubset(CORE):
            internal_targets.add(dest)
            for provider in providers:
                potential[provider].add(dest)

    def reachable(origin):
        visited, stack = {origin}, [origin]
        while stack:
            for neighbor in potential[stack.pop()]:
                if neighbor not in visited:
                    visited.add(neighbor)
                    stack.append(neighbor)
        return visited

    strongly_connected_candidate = (internal_targets == CORE and
                                    all(reachable(src) == CORE for src in CORE))
    external = [x for x in findings['minimal_disabling_sets']
                if x['disabled'] == ('F',)]
    return {'scenario': scenario_name,
            'three_node_internal_maintenance_cycle_detected': cyclic,
            'strongly_connected_internal_support_candidate': strongly_connected_candidate,
            'external_dependency_targets': sorted(x['target'] for x in external),
            'inference_status': 'candidate only: component-level intervention effects are not full organizational closure proof'}
