"""CIOC V0.3: bounded program synthesis for maintenance-algorithm repair.

No target's working program or source code is stored as a repair template.
An intact peer proposes programs from a shared fixed primitive ISA; training
examples guide search, and disjoint holdout cases gate atomic replacement.
This is a synthetic benchmark, NOT a digital life demonstration.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from itertools import product
from typing import Callable, Optional
import copy

PRIMITIVES = ('INC', 'DEC', 'DOUBLE', 'SQUARE', 'NEG', 'ADD3')
PEERS = {'A': 'B', 'B': 'C', 'C': 'A'}
TRAIN = (-2, -1, 0, 1, 2)
HOLDOUT = (-7, -4, 3, 5, 8)
# Behavioral specifications, not stored implementations of the requested algorithms.
TASKS: dict[str, Callable[[int], int]] = {
    'A': lambda x: x + 2,
    'B': lambda x: 2 * x + 1,
    'C': lambda x: x * x - 1,
}
# Genesis programs instantiate an organism; erased targets retain only task contracts.
GENESIS = {
    'A': ('INC', 'INC'),
    'B': ('INC', 'DOUBLE', 'DEC'),  # 2(x+1)-1 = 2x+1 (non-minimal)
    'C': ('DEC', 'INC', 'SQUARE', 'DEC'),  # x^2-1 (non-minimal)
}


def execute(program: tuple[str, ...], value: int, *, max_abs: int = 1_000_000) -> Optional[int]:
    if not program or len(program) > 4: return None
    acc = value
    for op in program:
        if op == 'INC': acc += 1
        elif op == 'DEC': acc -= 1
        elif op == 'DOUBLE': acc *= 2
        elif op == 'SQUARE': acc *= acc
        elif op == 'NEG': acc = -acc
        elif op == 'ADD3': acc += 3
        else: return None
        if abs(acc) > max_abs: return None
    return acc


def satisfies(program: tuple[str,...], task: str, probes: tuple[int, ...]) -> bool:
    return all(execute(program, x) == TASKS[task](x) for x in probes)


@dataclass
class Component:
    name: str
    program: Optional[tuple[str, ...]]
    permits: str

    def functional(self) -> bool:
        return self.program is not None and satisfies(self.program, self.name, TRAIN + HOLDOUT)


@dataclass
class World:
    components: dict[str, Component]
    budget: int = 600
    revision: int = 0
    external_repairs: int = 0
    history: list[dict] = field(default_factory=list)

    @classmethod
    def create(cls, budget: int = 600):
        return cls({n: Component(n, GENESIS[n], PEERS[n]) for n in PEERS}, budget)

    def clone(self): return copy.deepcopy(self)

    def damage(self, name: str):
        self.components[name].program = None
        self.history.append({'event': 'damage', 'target': name})

    def candidate_check(self, source: str, target: str, program: tuple[str, ...], phase: str):
        # Charge even for rejected proposals. World never generates or suggests programs.
        if self.budget <= 0:
            self.history.append({'event': 'budget_exhausted', 'source': source, 'target': target})
            return None
        self.budget -= 1
        permitted = (source in self.components and target in self.components
                     and self.components[source].functional()
                     and self.components[source].permits == target)
        if not permitted:
            self.history.append({'event': 'permission_denied', 'source': source, 'target': target})
            return False
        probes = TRAIN if phase == 'train' else HOLDOUT
        valid = satisfies(program, target, probes)
        self.history.append({'event': 'candidate_check', 'source': source, 'target': target,
                             'program': list(program), 'phase': phase, 'passed': valid})
        return valid

    def commit(self, source: str, target: str, program: tuple[str, ...]):
        if self.budget < 3: return False
        if not (self.components[source].functional() and self.components[source].permits == target):
            return False
        # Atomic, world-enforced acceptance: no source-program equality check.
        if not satisfies(program, target, TRAIN + HOLDOUT): return False
        self.budget -= 3
        before = self.components[target].program
        self.components[target].program = tuple(program)
        self.revision += 1
        self.history.append({'event': 'internal_commit', 'source': source, 'target': target,
                             'from': list(before) if before else None, 'to': list(program),
                             'revision': self.revision})
        return True

    def external_restore(self, target: str):
        if self.budget < 5: return False
        self.budget -= 5
        # External baseline uses independent oracle choice, not an internal algorithm.
        self.components[target].program = tuple(GENESIS[target])
        self.external_repairs += 1
        self.revision += 1
        self.history.append({'event': 'external_restore', 'target': target,
                             'revision': self.revision})
        return True

    def viable(self): return all(c.functional() for c in self.components.values())


@dataclass
class ResidentSearch:
    """A fixed enumerative search bootstrap owned by the modeled organism.

    Its implementation is not self-regenerating in V0.3. It has no program
    templates or access to GENESIS. World only accepts/tests candidates.
    """
    enabled: bool = True
    primitives: tuple[str,...] = PRIMITIVES
    max_len: int = 3

    def repair(self, world: World, target: str, *, disabled: frozenset[str] = frozenset()):
        sources = [name for name, c in world.components.items()
                   if c.permits == target and name not in disabled and c.functional()]
        if not self.enabled or not sources or world.components[target].functional(): return False
        source = sources[0]
        for length in range(1, self.max_len + 1):
            for p in product(self.primitives, repeat=length):
                candidate = tuple(p)
                train_ok = world.candidate_check(source, target, candidate, 'train')
                if train_ok is None: return False
                if not train_ok: continue
                holdout_ok = world.candidate_check(source, target, candidate, 'holdout')
                if holdout_ok is None: return False
                if holdout_ok and world.commit(source, target, candidate): return True
        return False


def summarize(name: str, world: World) -> dict:
    commits = [x for x in world.history if x['event'] == 'internal_commit']
    return {
        'scenario': name,
        'viable': world.viable(),
        'remaining_budget': world.budget,
        'internal_commits': len(commits),
        'external_repairs': world.external_repairs,
        'candidate_checks': sum(x['event'] == 'candidate_check' for x in world.history),
        'synthesized_programs': {c['target']: c['to'] for c in commits},
        'history': world.history,
    }


def run_case(name: str) -> dict:
    world = World.create(budget=15 if name == 'low_budget' else 600)
    search = ResidentSearch()
    if name == 'chain_regeneration':
        world.damage('B'); search.repair(world, 'B')
        world.damage('C'); search.repair(world, 'C')
    elif name == 'no_search':
        world.damage('B'); search.enabled = False; search.repair(world, 'B')
    elif name == 'external_baseline':
        world.damage('B'); world.external_restore('B')
    elif name == 'disable_source':
        world.damage('B'); search.repair(world, 'B', disabled=frozenset({'A'}))
    elif name == 'low_budget':
        world.damage('B'); search.repair(world, 'B')
    elif name == 'missing_primitive':
        world.damage('C'); search.primitives = ('INC','DEC','DOUBLE','NEG','ADD3'); search.repair(world,'C')
    elif name == 'destroyed_search':
        world.damage('B'); search.enabled = False; search.repair(world, 'B')
        world.history.append({'event': 'limitation', 'message': 'resident search engine cannot regenerate itself'})
    elif name == 'recover_A':
        world.damage('A'); search.repair(world, 'A')
    else: raise ValueError(name)
    return summarize(name, world)

SCENARIOS = ('chain_regeneration','no_search','external_baseline','disable_source',
             'low_budget','missing_primitive','destroyed_search','recover_A')
