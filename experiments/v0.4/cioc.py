"""CIOC V0.4: bounded regeneration of mutable maintenance *control programs*.

This is a designed toy benchmark, NOT evidence of life or open-ended emergence.
The world supplies immutable ISA semantics, behavioral contracts, permissions,
resource accounting and scheduling. A resident control module supplies each
repair action (proposal, verification, commit). With one damaged module,
secondary capabilities carried by other intact modules may take its role.
Unlike simply re-enabling a backup, a new target program is synthesized from
primitive tokens and transactionally installed without reading its old code.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from itertools import product
from typing import Optional
import copy

# Untrusted programs can only contain this tiny, inert finite instruction set.
TOKENS = ('ENUM', 'REVERSE', 'LIMIT3', 'CHECK_TRAIN', 'CHECK_HOLDOUT',
          'CHECK_PERMIT', 'WRITE_ATOMIC', 'NOP')
ROLES = ('search', 'verify', 'commit')
# Declared contracts (specifications), not source/target implementation templates.
ROLE_REQUIREMENTS = {
    'search': frozenset({'ENUM', 'LIMIT3'}),
    'verify': frozenset({'CHECK_TRAIN', 'CHECK_HOLDOUT'}),
    'commit': frozenset({'CHECK_PERMIT', 'WRITE_ATOMIC'}),
}
PROBES_TRAIN = (-2, 0, 3)
PROBES_HOLDOUT = (-7, -1, 2, 8)


def execute_role(program: tuple[str, ...] | None, role: str, x: int, *, phase: str) -> Optional[bool]:
    """Toy role interpreter: role contract is evaluated on all test inputs.

    Search: returns ability to enumerate under bound (not an exhaustive
    internal search runtime). Verify: must check two disjoint probe sets.
    Commit: checks authorization and supports atomic write. This checks
    primitive capability coverage, not arbitrary semantic equivalence.
    """
    if program is None or len(program) == 0 or len(program) > 4:
        return None
    if any(token not in TOKENS for token in program):
        return None
    requirements = ROLE_REQUIREMENTS[role]
    if not requirements.issubset(program):
        return False
    # Disallow duplicate side-effecting instructions; inert NOP is legal.
    for t in requirements:
        if program.count(t) != 1:
            return False
    return True


@dataclass
class Module:
    name: str
    role: str
    program: Optional[tuple[str, ...]]
    # Secondary role programs are mutable assets, not privileged world helpers.
    backups: dict[str, Optional[tuple[str, ...]]] = field(default_factory=dict)

    def capable(self, role: str) -> bool:
        prog = self.program if self.role == role else self.backups.get(role)
        return execute_role(prog, role, 0, phase='train') is True


@dataclass
class World:
    nodes: dict[str, Module]
    budget: int = 1100
    revision: int = 0
    external_repairs: int = 0
    history: list[dict] = field(default_factory=list)
    # Spec tests are independent of code and known to resident validators.
    contract_access: bool = True
    external_rescuer_enabled: bool = False

    @classmethod
    def create(cls, budget: int = 1100):
        return cls({
            'S': Module('S', 'search', ('REVERSE', 'LIMIT3', 'ENUM'),
                        {'commit': ('WRITE_ATOMIC', 'CHECK_PERMIT')}),
            'V': Module('V', 'verify', ('CHECK_HOLDOUT', 'CHECK_TRAIN'),
                        {'search': ('LIMIT3', 'ENUM')}),
            'M': Module('M', 'commit', ('WRITE_ATOMIC', 'CHECK_PERMIT'),
                        {'verify': ('CHECK_HOLDOUT', 'CHECK_TRAIN')}),
        }, budget=budget)

    def clone(self): return copy.deepcopy(self)

    def damage(self, target: str, *, role: str | None = None):
        node = self.nodes[target]
        if role is None or role == node.role:
            node.program = None
            affected = node.role
        else:
            if role not in node.backups: raise ValueError('no such secondary capability')
            node.backups[role] = None
            affected = role
        self.history.append({'event':'damage', 'node':target, 'role':affected})

    def functional(self) -> bool:
        return all(node.capable(node.role) for node in self.nodes.values())

    def charge(self, amount: int, event: str, **details) -> bool:
        if self.budget < amount:
            self.history.append({'event':'insufficient_budget', 'attempt':event, **details})
            return False
        self.budget -= amount
        self.history.append({'event':event, 'cost':amount, **details})
        return True

    def resident(self, role: str, *, excluded: frozenset[str] = frozenset()) -> str | None:
        # Prefer a module whose primary job matches the requested role; backups next.
        return next((name for name, node in self.nodes.items()
                     if name not in excluded and node.role == role and node.capable(role)), None) or next(
            (name for name, node in self.nodes.items()
             if name not in excluded and node.capable(role)), None)

    def _resident_candidate_stream(self, operator: str, target_role: str, *, primitives: tuple[str,...], max_len: int):
        """The actual candidate enumeration is enabled by a live resident search code.

        The host Python loop is interpreter implementation, but no target
        source/template is supplied to it. Role-specific contracts are public.
        """
        if not self.nodes[operator].capable('search'):
            return
        for length in range(1, max_len + 1):
            for candidate in product(primitives, repeat=length):
                yield candidate

    def _role_probe(self, verifier: str, target_role: str, candidate: tuple[str,...], phase: str) -> bool | None:
        if not self.nodes[verifier].capable('verify') or not self.contract_access:
            return None
        if not self.charge(1, 'probe', verifier=verifier, target_role=target_role, phase=phase,
                           candidate=list(candidate)):
            return None
        probes = PROBES_TRAIN if phase == 'train' else PROBES_HOLDOUT
        return all(execute_role(candidate, target_role, x, phase=phase) is True for x in probes)

    def _install(self, committer: str, target: str, role: str, candidate: tuple[str,...],
                 *, verified_train: bool, verified_holdout: bool) -> bool:
        if not self.nodes[committer].capable('commit') or not (verified_train and verified_holdout):
            self.history.append({'event':'commit_rejected', 'target':target, 'reason':'gate'})
            return False
        if not self.contract_access or not self.charge(3, 'commit_cost', target=target):
            return False
        # World-enforced atomicity / permissions: a deliberate unmodifiable boundary.
        node = self.nodes[target]
        if role == node.role:
            if node.program is not None: return False
            node.program = tuple(candidate)
        elif role in node.backups:
            if node.backups[role] is not None: return False
            node.backups[role] = tuple(candidate)
        else: return False
        self.revision += 1
        self.history.append({'event':'internal_commit','operator':committer,'target':target,
                             'role':role,'program':list(candidate),'revision':self.revision})
        return True

    def regenerate(self, target: str, *, role: str | None = None,
                   primitives: tuple[str,...] = TOKENS, max_len: int = 3,
                   excluded: frozenset[str] = frozenset()) -> bool:
        """Experimental scheduler invokes resident components; not itself a life policy."""
        node = self.nodes[target]
        role = role or node.role
        if node.capable(role): return False
        searcher = self.resident('search', excluded=excluded | frozenset({target}))
        verifier = self.resident('verify', excluded=excluded | frozenset({target}))
        committer = self.resident('commit', excluded=excluded | frozenset({target}))
        if not all((searcher, verifier, committer)):
            self.history.append({'event':'repair_blocked', 'target':target,'role':role,
                                 'reason':'missing_resident_capability',
                                 'roles':{'search':searcher,'verify':verifier,'commit':committer}})
            return False
        self.history.append({'event':'repair_started','target':target,'role':role,
                             'roles':{'search':searcher,'verify':verifier,'commit':committer}})
        for candidate in self._resident_candidate_stream(searcher, role, primitives=primitives, max_len=max_len):
            train = self._role_probe(verifier, role, candidate, 'train')
            if train is None: return False
            if not train: continue
            holdout = self._role_probe(verifier, role, candidate, 'holdout')
            if holdout is None: return False
            if holdout and self._install(committer, target, role, candidate,
                                        verified_train=train, verified_holdout=holdout):
                return True
        self.history.append({'event':'search_exhausted','target':target,'role':role})
        return False

    def external_restore(self, target: str) -> bool:
        """Explicit external control: never counted as internal regeneration."""
        if not self.external_rescuer_enabled or not self.charge(5,'external_cost',target=target):
            return False
        node = self.nodes[target]
        node.program = tuple(sorted(ROLE_REQUIREMENTS[node.role]))
        self.external_repairs += 1
        self.revision += 1
        self.history.append({'event':'external_restore','target':target})
        return True


def summarize(name: str, w: World) -> dict:
    commits = [h for h in w.history if h['event']=='internal_commit']
    return {'scenario':name, 'viable':w.functional(), 'remaining_budget':w.budget,
            'internal_commits':len(commits), 'external_repairs':w.external_repairs,
            'probes':sum(h['event']=='probe' for h in w.history),
            'regenerated':{f"{h['target']}.{h['role']}":h['program'] for h in commits},
            'history':w.history}


def run_case(name: str) -> dict:
    w = World.create(budget=14 if name=='low_budget' else 1100)
    if name=='search_regeneration':
        w.damage('S'); w.regenerate('S')
    elif name=='verify_regeneration':
        w.damage('V'); w.regenerate('V')
    elif name=='commit_regeneration':
        w.damage('M'); w.regenerate('M')
    elif name=='sequential_regeneration':
        for n in ('S','V','M'):
            w.damage(n); w.regenerate(n)
    elif name=='joint_failure':
        w.damage('S'); w.damage('V'); w.damage('V',role='search'); w.regenerate('S'); w.regenerate('V')
    elif name=='no_backup_search':
        w.damage('S'); w.damage('V',role='search'); w.regenerate('S')
    elif name=='no_contract':
        w.damage('S'); w.contract_access=False; w.regenerate('S')
    elif name=='low_budget':
        w.damage('S'); w.regenerate('S')
    elif name=='missing_primitive':
        w.damage('V'); w.regenerate('V',primitives=('ENUM','REVERSE','LIMIT3','CHECK_TRAIN','CHECK_PERMIT','WRITE_ATOMIC','NOP'))
    elif name=='external_baseline':
        w.damage('S'); w.external_rescuer_enabled=True; w.external_restore('S')
    elif name=='destroy_secondary':
        w.damage('V',role='search'); w.regenerate('V',role='search')
    else: raise ValueError(name)
    return summarize(name,w)

SCENARIOS = ('search_regeneration','verify_regeneration','commit_regeneration',
             'sequential_regeneration','joint_failure','no_backup_search','no_contract',
             'low_budget','missing_primitive','external_baseline','destroy_secondary')
