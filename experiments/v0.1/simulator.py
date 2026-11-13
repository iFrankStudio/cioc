"""CIOC proof-of-concept: event-based component organization simulator.

The simulator knows the *ground-truth* rules. The analyzer only accesses an
intervention API, not these rules. This is deliberately a small synthetic
benchmark, not evidence that any digital system is alive.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from typing import FrozenSet, Tuple
import random

CORE = frozenset({'A', 'B', 'C'})
ORGANISM = frozenset({'A', 'B', 'C', 'D'})
COMPONENTS = ('A', 'B', 'C', 'D', 'E', 'F')

@dataclass(frozen=True)
class RepairRule:
    sources: FrozenSet[str]
    target: str
    external: bool = False

@dataclass(frozen=True)
class Scenario:
    name: str
    rules: Tuple[RepairRule, ...]
    calls: Tuple[Tuple[str,str], ...] = ()
    description: str = ''


def scenario(name: str) -> Scenario:
    if name == 'call_cycle':
        return Scenario(name, (), (('A','B'),('B','C'),('C','A')),
                        'A/B/C call one another but do not repair one another.')
    if name == 'internal_closure':
        return Scenario(name, (
            RepairRule(frozenset({'C'}), 'A'),
            RepairRule(frozenset({'A'}), 'B'),
            RepairRule(frozenset({'B'}), 'C'),
            RepairRule(frozenset({'A'}), 'D'),
        ), description='A/B/C provide reciprocal maintenance; D is a maintained peripheral component.')
    if name == 'external_repair':
        return Scenario(name, tuple(RepairRule(frozenset({'F'}), t, external=True)
                                    for t in ('A','B','C','D')),
                        description='Only external controller F repairs A/B/C/D.')
    if name == 'redundant_maintenance':
        return Scenario(name, (
            RepairRule(frozenset({'C'}), 'A'),
            RepairRule(frozenset({'C'}), 'B'),
            RepairRule(frozenset({'A'}), 'C'),
            RepairRule(frozenset({'B'}), 'C'),
            RepairRule(frozenset({'A'}), 'D'),
        ), description='Either A or B can repair C, masking single-ablation effects.')
    raise ValueError(name)


@dataclass
class Trial:
    scenario: str
    target: str
    disabled: FrozenSet[str]
    initial_budget: int
    recovered: bool
    core_viable: bool
    remaining_budget: int
    external_budget_remaining: int
    repairs: list[dict] = field(default_factory=list)
    calls_executed: int = 0


def intervene(case: Scenario, target: str, disabled: FrozenSet[str] = frozenset(),
              *, budget: int = 14, external_budget: int = 14,
              horizon: int = 4, repair_success: float = 1.0, seed: int = 0) -> Trial:
    """Damage target at step 0 and disable selected sources through recovery.

    A source can repair only if functional, not disabled, and resourced.
    Disabling is a *temporary intervention* (not interpreted as physical damage).
    No code or secret repair rules are given to the causal analyzer.
    """
    if target not in ORGANISM or (disabled - set(COMPONENTS)):
        raise ValueError('Unknown target or disabled component')
    if target in disabled:
        raise ValueError('Target cannot also be disabled')
    if budget < 0 or external_budget < 0 or horizon < 1 or not 0 <= repair_success <= 1:
        raise ValueError('Invalid resources/parameters')
    rng = random.Random(seed)
    functional = {c: True for c in COMPONENTS}
    functional[target] = False
    b, ext = budget, external_budget
    repairs: list[dict] = []
    calls_executed = 0

    for tick in range(horizon):
        # Minimal world housekeeping cost. If exhausted, the system cannot act.
        if b < 1:
            break
        b -= 1
        for source, dest in case.calls:
            if source not in disabled and dest not in disabled and functional[source] and functional[dest]:
                calls_executed += 1
        # Repair policies only operate through the sandbox world API.
        # One successful repair per tick; no instant cascade in a single event.
        for rule in case.rules:
            if functional[rule.target] or rule.target in disabled:
                continue
            if any(not functional[src] or src in disabled for src in rule.sources):
                continue
            if rule.external:
                if ext < 2:
                    continue
                ext -= 2
            else:
                if b < 2:
                    continue
                b -= 2
            success = rng.random() < repair_success
            repairs.append({'tick': tick, 'sources': sorted(rule.sources),
                            'target': rule.target, 'external': rule.external,
                            'success': success})
            if success:
                functional[rule.target] = True
                break

    recovered = functional[target]
    # Viability is assessed after intervention releases; disabling a maintenance
    # source isn't itself equated to damage. Core viability means all core modules
    # were functional in the base (non-disabled) projection; for single target
    # damage this is the same as target recovery if target in CORE.
    viable = recovered if target in CORE else all(functional[c] for c in CORE)
    return Trial(case.name, target, disabled, budget, recovered, viable, b, ext,
                 repairs, calls_executed)
