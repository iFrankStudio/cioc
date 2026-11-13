"""CIOC V0.7: complementary role-program recovery in a bounded instruction world.

This is NOT general autonomous life. Host primitives still implement enumeration,
contract evaluation and atomic install. Role state machines decide when to act.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from itertools import product
from typing import Optional

# Role programs are executable data. Roles are complementary, not copies.
ROLE_PROGRAMS = {
    'O': ('SCAN', 'REQUEST', 'FALLBACK'),
    'G': ('GENERATE', 'PROPOSE'),
    'V': ('TEST', 'CERTIFY'),
    'M': ('RECHECK', 'COMMIT'),
}
OPCODES = tuple(sorted(set(sum((list(p) for p in ROLE_PROGRAMS.values()), []))))
SAMPLES = (0, 1, 2, 3)

@dataclass(frozen=True)
class Message:
    source: str
    dest: str
    kind: str
    payload: tuple[str, ...] | str

@dataclass
class Organism:
    budget: int = 2000
    max_program_length: int = 2
    # Shared vocab availability is a world allowance, not a hidden template.
    available_ops: tuple[str, ...] = OPCODES
    roles: dict[str, Optional[tuple[str, ...]]] = field(default_factory=lambda: dict(ROLE_PROGRAMS))
    # Each role holds only constraints on the OTHER roles, not a full exact copy.
    fragments: dict[str, dict[str, tuple[str, ...]]] = field(default_factory=dict)
    inbox: list[Message] = field(default_factory=list)
    checked: dict[str, tuple[str, ...]] = field(default_factory=dict)
    proposals: dict[str, tuple[str, ...]] = field(default_factory=dict)
    history: list[dict] = field(default_factory=list)
    external_restores: int = 0
    revision: int = 0
    checked_count: int = 0
    active_target: Optional[str] = None
    failed: set[str] = field(default_factory=set)

    def __post_init__(self):
        if not self.fragments:
            # Fragment distribution: no role owns a complete program of another.
            # O contributes first-op hints, V contributes second-op hints,
            # M stores role contracts. These are predesigned partial constraints.
            self.fragments = {
                'O': {r: (p[0],) for r, p in ROLE_PROGRAMS.items() if r != 'O'},
                'V': {r: (p[1],) for r, p in ROLE_PROGRAMS.items() if r != 'V'},
                'M': {}, 'G': {},
            }

    def log(self, kind: str, **details):
        self.history.append(dict(event=kind, **details))

    def spend(self, amount=1):
        if self.budget < amount:
            self.log('budget_exhausted')
            return False
        self.budget -= amount
        return True

    def damage(self, role: str):
        assert role in self.roles
        self.roles[role] = None
        self.log('damage', role=role)

    def send(self, source: str, dest: str, kind: str, payload):
        self.inbox.append(Message(source, dest, kind, payload))
        self.log('message', source=source, dest=dest, message_kind=kind)

    def functional_contract(self, role: str, program: tuple[str, ...]) -> bool:
        # The host evaluates a role's public behavior contract, NOT hidden source.
        # Strict order is intentional; different task semantics may admit alternatives.
        expected = {
            'O': (('SCAN','REQUEST','FALLBACK'),),
            'G': (('GENERATE','PROPOSE'),),
            'V': (('TEST','CERTIFY'),),
            'M': (('RECHECK','COMMIT'),),
        }
        return program in expected[role]

    def fragment_ok(self, role: str, candidate: tuple[str, ...]) -> bool:
        # At least two independently held fragments must survive for a rebuild.
        if role == 'O':
            # O's source hints are not available in O itself; V and M hold
            # contract-based checks; full recovery of O is intentionally impossible.
            return False
        hinted_first = self.fragments.get('O', {}).get(role)
        hinted_last = self.fragments.get('V', {}).get(role)
        return bool(self.roles['O'] and self.roles['V'] and hinted_first and hinted_last
                    and candidate[0] == hinted_first[0] and candidate[-1] == hinted_last[0])

    def tick(self):
        """World schedules each intact role; role-local opcodes choose all actions."""
        progressed = False
        for actor in ('O','G','V','M'):
            prog = self.roles[actor]
            if prog is None: continue
            if not self.spend(): return progressed
            for instruction in prog:
                progressed = self._step(actor, instruction) or progressed
        return progressed

    def _step(self, actor: str, instruction: str) -> bool:
        if instruction == 'SCAN' and actor == 'O':
            missing = [r for r,v in self.roles.items() if v is None and r not in self.failed]
            if missing and self.active_target is None:
                self.active_target = missing[0]
                return True
        if instruction == 'REQUEST' and actor == 'O' and self.active_target:
            if not any(m.kind == 'BUILD' for m in self.inbox):
                self.send('O','G','BUILD',self.active_target)
                return True
        if instruction == 'FALLBACK' and actor == 'O' and self.active_target == 'G' and self.roles['G'] is None:
            # O contributes detection + bounded fallback enumeration; V and M
            # contribute validation and commit. No role holds an entire G program.
            if 'G' not in self.proposals:
                for cand in product(self.available_ops, repeat=self.max_program_length):
                    if not self.spend(): return False
                    self.checked_count += 1
                    if self.fragment_ok('G', cand):
                        self.proposals['G'] = cand
                        self.send('O', 'V', 'VERIFY', 'G')
                        self.log('fallback_candidate', actor='O', candidate=cand)
                        return True
                self.failed.add('G')
                self.active_target = None
            return False
        if instruction == 'GENERATE' and actor == 'G':
            msg = next((m for m in self.inbox if m.dest == 'G' and m.kind == 'BUILD'), None)
            if msg and self.active_target and self.active_target not in self.proposals:
                target = str(msg.payload)
                # Enumeration is a *host primitive*, not a synthesized searcher.
                for cand in product(self.available_ops, repeat=self.max_program_length):
                    if not self.spend(): break
                    self.checked_count += 1
                    if self.fragment_ok(target,cand):
                        self.proposals[target] = cand
                        self.log('candidate', target=target, candidate=cand)
                        return True
                self.failed.add(target)
                self.active_target = None
                self.inbox.remove(msg)
                return True
        if instruction == 'PROPOSE' and actor == 'G':
            for target,candidate in list(self.proposals.items()):
                if not any(m.kind == 'VERIFY' and m.payload == target for m in self.inbox):
                    self.send('G','V','VERIFY',target)
                    return True
        if instruction == 'TEST' and actor == 'V':
            msg = next((m for m in self.inbox if m.dest == 'V' and m.kind == 'VERIFY'),None)
            if msg:
                self.inbox.remove(msg)
                target = str(msg.payload)
                candidate = self.proposals.get(target)
                if candidate and self.spend(2) and self.functional_contract(target,candidate):
                    self.checked[target] = candidate
                    self.log('verified', target=target)
                    return True
                self.failed.add(target)
                self.active_target = None
                self.log('candidate_rejected', target=target)
                return True
        if instruction == 'CERTIFY' and actor == 'V':
            for target in list(self.checked):
                if not any(m.kind == 'INSTALL' and m.payload == target for m in self.inbox):
                    self.send('V','M','INSTALL',target)
                    return True
        if instruction == 'RECHECK' and actor == 'M':
            # M performs independent host-contract check; no blind trust in V.
            return False
        if instruction == 'COMMIT' and actor == 'M':
            msg = next((m for m in self.inbox if m.dest == 'M' and m.kind == 'INSTALL'),None)
            if msg:
                self.inbox.remove(msg)
                target = str(msg.payload)
                candidate = self.checked.get(target)
                if candidate and self.spend(3) and self.functional_contract(target,candidate):
                    self.roles[target] = candidate
                    self.revision += 1
                    self.log('internal_install', target=target, program=candidate)
                    self.active_target = None
                    self.checked.pop(target,None)
                    self.proposals.pop(target,None)
                    self.inbox = [m for m in self.inbox if not (m.kind == 'BUILD' and m.payload == target)]
                    return True
                self.failed.add(target)
                self.active_target = None
                return True
        return False

    def run(self, cycles=50):
        for _ in range(cycles):
            self.tick()
            if all(p is not None for p in self.roles.values()): break
        return all(p is not None for p in self.roles.values())

    def external_restore(self, role: str):
        self.roles[role] = ROLE_PROGRAMS[role]
        self.external_restores += 1
        self.log('external_restore', role=role)


def experiment():
    outcomes = {}
    for role in ('O','G','V','M'):
        x=Organism();x.damage(role);outcomes['damage_'+role]={
            'recovered':x.run(), 'internal':x.revision, 'checked':x.checked_count}
    x=Organism();x.damage('G');x.damage('M');outcomes['damage_G_M']={'recovered':x.run(),'internal':x.revision}
    x=Organism(budget=10);x.damage('G');outcomes['low_budget']={'recovered':x.run(),'internal':x.revision}
    x=Organism();x.damage('G');x.fragments['V'].pop('G');outcomes['missing_fragment']={'recovered':x.run(),'internal':x.revision}
    x=Organism();x.damage('G');x.external_restore('G');outcomes['external']={'recovered':x.run(),'internal':x.revision,'external':x.external_restores}
    return outcomes
