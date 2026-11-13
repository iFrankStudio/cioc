"""CIOC V0.2: finite, sandboxed algorithm-organization regeneration benchmark.

Not a life simulator. An intrinsic controller enumerates programs using fragments it
owns; the world only executes, charges budget, checks permissions, and commits.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from itertools import permutations
from typing import Optional
import copy

CORE = ('A', 'B', 'C')
OPCODES = ('LOAD', 'WRITE', 'VERIFY')
SUCCESSOR = {'A': 'B', 'B': 'C', 'C': 'A'}
CANONICAL = ('LOAD', 'WRITE', 'VERIFY')

@dataclass
class Module:
    name: str
    program: Optional[tuple[str, ...]] = CANONICAL
    # Local fragment repertoire and a replica of the next module's code.
    fragments: tuple[str, ...] = ('VERIFY', 'LOAD', 'WRITE')
    backup: tuple[str, ...] = CANONICAL
    permitted_target: str = ''

    @property
    def functional(self) -> bool:
        return self.program == CANONICAL

@dataclass
class World:
    modules: dict[str, Module]
    budget: int = 100
    external_repairs: int = 0
    history: list[dict] = field(default_factory=list)
    version: int = 0
    branch: str = 'main'

    @classmethod
    def create(cls, budget: int = 100) -> 'World':
        return cls({n: Module(n, permitted_target=SUCCESSOR[n]) for n in CORE}, budget)

    def fault(self, name: str, kind: str = 'erase_program') -> None:
        if name not in self.modules: raise ValueError('unknown module')
        if kind == 'erase_program': self.modules[name].program = None
        elif kind == 'corrupt_program': self.modules[name].program = ('WRITE', 'LOAD', 'VERIFY')
        else: raise ValueError('unknown fault')
        self.history.append({'event':'fault', 'module':name, 'kind':kind})

    def charge(self, amount: int) -> bool:
        if amount < 0: raise ValueError('negative cost')
        if self.budget < amount: return False
        self.budget -= amount
        return True

    def verify_candidate(self, source: str, target: str, candidate: tuple[str,...]) -> bool:
        """Pure capability/semantic check. No automatic proposal or repair."""
        if source not in self.modules or target not in self.modules: return False
        src = self.modules[source]
        if not src.functional or src.permitted_target != target: return False
        if len(candidate) != 3 or sorted(candidate) != sorted(OPCODES): return False
        # Verification on a disposable simulated slot, never edits target.
        loaded = False; written = False; valid = False
        for op in candidate:
            if op == 'LOAD': loaded = src.backup == CANONICAL
            elif op == 'WRITE': written = loaded
            elif op == 'VERIFY': valid = written and src.backup == CANONICAL
        return valid and written

    def attempt(self, source: str, target: str, candidate: tuple[str,...], *, origin: str='internal') -> bool:
        if not self.charge(2):
            self.history.append({'event':'budget_exhausted', 'source':source, 'target':target})
            return False
        accepted = self.verify_candidate(source, target, candidate)
        self.history.append({'event':'candidate_evaluated','source':source, 'target':target,
                             'candidate':list(candidate),'accepted':accepted,'origin':origin})
        if accepted:
            # The world only executes an accepted internally-proposed modification.
            self.modules[target].program = self.modules[source].backup
            self.version += 1
            self.history.append({'event':'commit','source':source,'target':target,
                                 'revision':self.version,'origin':origin})
        return accepted

    def external_restore(self, target: str) -> bool:
        if not self.charge(5): return False
        self.modules[target].program = CANONICAL
        self.external_repairs += 1
        self.version += 1
        self.history.append({'event':'external_restore','target':target,'revision':self.version})
        return True

    def checkpoint(self):
        return copy.deepcopy(self)

    def viable(self) -> bool:
        return all(m.functional for m in self.modules.values())

@dataclass
class OrganismPolicy:
    """Resident within organization, not part of World; no arbitrary host access.

    Proposed repairs are synthesized by enumerating candidate programs from the
    resident module's fragments. This fixed search mechanism is a bootstrap,
    not open-ended algorithm invention.
    """
    plastic: bool = True
    allow_external: bool = False

    def act(self, world: World, *, disabled: frozenset[str] = frozenset()) -> bool:
        for source in CORE:
            src = world.modules[source]
            if source in disabled or not src.functional: continue
            target = src.permitted_target
            if world.modules[target].functional: continue
            if not self.plastic: continue
            for program in permutations(src.fragments, 3):
                if world.attempt(source,target,program): return True
                if world.budget < 2: return False
        if self.allow_external:
            for target in CORE:
                if not world.modules[target].functional:
                    return world.external_restore(target)
        return False


def run_case(name: str, *, budget: int = 100) -> dict:
    w = World.create(budget)
    policy = OrganismPolicy(plastic=name not in ('fixed','external'),
                            allow_external=name=='external')
    if name == 'internal_regeneration':
        w.fault('B'); policy.act(w)
        first = w.modules['B'].functional
        w.fault('C'); policy.act(w)
        second = w.modules['C'].functional
    elif name == 'fixed':
        w.fault('B'); first = policy.act(w); second = w.modules['B'].functional
    elif name == 'external':
        w.fault('B'); first = policy.act(w); second = w.modules['B'].functional
    elif name == 'maintenance_cut':
        w.fault('B')
        first = policy.act(w, disabled=frozenset({'A'}))
        second = w.modules['B'].functional
    elif name == 'insufficient_resources':
        w.fault('B'); first = policy.act(w); second = w.modules['B'].functional
    elif name == 'broken_blueprint':
        w.modules['A'].backup = ('WRITE','LOAD','VERIFY')
        w.fault('B'); first = policy.act(w); second = w.modules['B'].functional
    else: raise ValueError(name)
    return {'scenario':name,'first_result':first,'second_result':second,
            'viable':w.viable(),'remaining_budget':w.budget,
            'external_repairs':w.external_repairs,'revisions':w.version,
            'candidates_tested':sum(x['event']=='candidate_evaluated' for x in w.history),
            'history':w.history}
