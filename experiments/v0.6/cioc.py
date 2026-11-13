"""CIOC V0.6: complementary maintenance roles and repairable contract data.

Controlled *benchmark*, not an implementation of general artificial life. Each role's
policy is defined in Python (immutable in this version); host dispatch is neutral.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from itertools import product
from collections import deque
import hashlib
import json
from typing import Any

OPS = ('INC', 'DOUBLE', 'SQUARE', 'DEC')

def evaluate(program: tuple[str, ...], x: int) -> int:
    y = x
    for op in program:
        if op == 'INC': y += 1
        elif op == 'DOUBLE': y *= 2
        elif op == 'SQUARE': y *= y
        elif op == 'DEC': y -= 1
        else: raise ValueError(op)
        if abs(y) > 1000000: raise ValueError('magnitude exceeded')
    return y

def candidates(ops: tuple[str, ...], max_length: int = 3):
    for length in range(1, max_length + 1):
        yield from product(ops, repeat=length)

def contract_bytes(samples: list[tuple[int, int]]) -> bytes:
    return json.dumps(samples, separators=(',', ':'), ensure_ascii=True).encode()

def make_shares(data: bytes) -> tuple[bytes, bytes]:
    # Deterministic TEST mask for reproducibility; not cryptographic secret-sharing.
    mask = bytes(hashlib.sha256(b'cioc-v06-mask:' + i.to_bytes(4, 'big')).digest()[0]
                 for i in range(len(data)))
    return mask, bytes(a ^ b for a, b in zip(data, mask))

@dataclass
class Message:
    sender: str
    target: str
    kind: str
    payload: Any = None

@dataclass
class Organism:
    compute: int = 1500
    available_ops: tuple[str, ...] = OPS
    enabled: dict[str, bool] = field(default_factory=lambda: {x: True for x in ('O', 'G', 'V', 'M')})
    task_program: tuple[str, ...] | None = None
    contract: list[tuple[int,int]] | None = None
    shard_o: bytes | None = None
    shard_v: bytes | None = None
    contract_digest: str = ''
    inbox: dict[str, deque[Message]] = field(default_factory=lambda: {x: deque() for x in ('O', 'G', 'V', 'M')})
    history: list[dict[str, Any]] = field(default_factory=list)
    verified: set[tuple[str, ...]] = field(default_factory=set)
    external_repairs: int = 0
    revision: int = 0
    attempted: int = 0
    # Role-specific local state is part of organism rather than world scheduler.
    generator_iter: Any = None
    generator_active: bool = False
    search_exhausted: bool = False
    verifier_pending: deque[tuple[str,...]] = field(default_factory=deque)
    modifier_candidate: tuple[str,...] | None = None
    contract_fragments: dict[str, bytes] = field(default_factory=dict)
    detection_enabled: bool = True
    heldout: tuple[int, ...] = (-9, -2, 4, 11, 15, 24, 31)
    target_fn: Any = field(default=lambda x: 2*x+1, repr=False)

    @classmethod
    def create(cls, compute: int = 1500, available_ops: tuple[str,...] = OPS) -> 'Organism':
        o = cls(compute=compute, available_ops=available_ops)
        samples = [(-3, -5), (0, 1), (1, 3), (5, 11)]
        data = contract_bytes(samples)
        o.contract = samples
        o.shard_o, o.shard_v = make_shares(data)
        o.contract_digest = hashlib.sha256(data).hexdigest()
        o.task_program = ('DOUBLE', 'INC')
        return o

    def log(self, kind: str, **fields):
        self.history.append({'step': len(self.history), 'event': kind, **fields})

    def send(self, sender: str, target: str, kind: str, payload=None):
        self.inbox[target].append(Message(sender, target, kind, payload))
        self.log('message', sender=sender, target=target, message_kind=kind)

    def spend(self, amount: int, actor: str) -> bool:
        if self.compute < amount:
            self.log('insufficient_compute', actor=actor, need=amount, have=self.compute)
            return False
        self.compute -= amount
        return True

    def damage(self, what: str):
        if what == 'TASK': self.task_program = None
        elif what == 'CONTRACT': self.contract = None
        elif what in self.enabled: self.enabled[what] = False
        elif what == 'SHARD_O': self.shard_o = None
        elif what == 'SHARD_V': self.shard_v = None
        else: raise ValueError(what)
        self.log('damage', what=what)

    def restore_external(self, what: str):
        if what == 'TASK':
            self.task_program = ('DOUBLE','INC')
        elif what == 'CONTRACT':
            self.contract = [(-3,-5),(0,1),(1,3),(5,11)]
        else: raise ValueError(what)
        self.external_repairs += 1
        self.log('external_restore', what=what)

    def task_valid(self, heldout: bool = False) -> bool:
        if self.task_program is None: return False
        xs = self.heldout if heldout else [x for x,_ in (self.contract or [])]
        return all(evaluate(self.task_program, x) == self.target_fn(x) for x in xs)

    def _observer(self):
        if not self.detection_enabled: return False
        did = False
        if self.contract is None:
            # Broadcast requests; no host-side reconstruct call.
            if self.shard_o is not None and 'O' not in self.contract_fragments:
                self.send('O', 'M', 'FRAGMENT', ('O', self.shard_o))
                self.contract_fragments['O'] = self.shard_o
                did = True
            self.send('O', 'V', 'NEED_FRAGMENT')
            did = True
        elif self.task_program is None and not self.generator_active and not self.search_exhausted:
            self.send('O', 'G', 'REPAIR_TASK')
            self.generator_active = True
            did = True
        return did

    def _generator(self):
        inbox = self.inbox['G']
        if inbox:
            m = inbox.popleft()
            if m.kind == 'REPAIR_TASK':
                self.generator_iter = iter(candidates(self.available_ops))
                self.log('search_started', actor='G', opcount=len(self.available_ops))
                return True
            if m.kind == 'STOP_SEARCH':
                self.generator_iter = None
                self.generator_active = False
                return True
        if self.generator_iter is None: return False
        # Bounded mailboxes: generator cannot flood a disabled verifier.
        if len(self.inbox['V']) >= 1: return False
        if not self.spend(1, 'G'): return False
        try:
            program = next(self.generator_iter)
        except StopIteration:
            self.generator_iter = None
            self.generator_active = False
            self.search_exhausted = True
            self.log('search_exhausted')
            return True
        self.attempted += 1
        self.send('G','V','CANDIDATE',program)
        return True

    def _verifier(self):
        inbox = self.inbox['V']
        if not inbox: return False
        msg = inbox.popleft()
        if msg.kind == 'NEED_FRAGMENT':
            if self.shard_v is not None and self.contract is None:
                self.send('V', 'M', 'FRAGMENT', ('V', self.shard_v))
            return True
        if msg.kind != 'CANDIDATE': return True
        if self.contract is None:
            self.log('verify_blocked_no_contract')
            return True
        if not self.spend(len(self.contract), 'V'): return False
        prog = tuple(msg.payload)
        try:
            passes = all(evaluate(prog,x)==y for x,y in self.contract)
        except ValueError:
            passes = False
        self.log('verification', candidate=list(prog), passed=passes)
        if passes:
            self.send('V','M','VERIFIED_CANDIDATE',prog)
        return True

    def _modifier(self):
        inbox = self.inbox['M']
        if not inbox: return False
        msg = inbox.popleft()
        if msg.kind == 'FRAGMENT':
            name, fragment = msg.payload
            self.contract_fragments[name] = fragment
            if self.contract is None and 'O' in self.contract_fragments and 'V' in self.contract_fragments:
                if not self.spend(3, 'M'): return False
                a,b=self.contract_fragments['O'], self.contract_fragments['V']
                if len(a)!=len(b): self.log('contract_reconstruction_rejected', reason='length');return True
                raw=bytes(x^y for x,y in zip(a,b))
                if hashlib.sha256(raw).hexdigest()!=self.contract_digest:
                    self.log('contract_reconstruction_rejected', reason='digest');return True
                try:
                    samples=json.loads(raw.decode('utf-8'))
                    if not all(isinstance(x,int) and isinstance(y,int) for x,y in samples): raise ValueError()
                except (UnicodeError, ValueError, TypeError):
                    self.log('contract_reconstruction_rejected', reason='format');return True
                self.contract=[tuple(x) for x in samples]
                self.revision += 1
                self.log('contract_reconstructed', actor='M', samples=len(samples), internal=True)
            return True
        if msg.kind == 'VERIFIED_CANDIDATE':
            if not self.spend(2, 'M'): return False
            if self.contract is None:
                self.log('commit_rejected', reason='missing_contract');return True
            prog=tuple(msg.payload)
            # Re-verification at commit avoids implicit trust in message.
            if not all(evaluate(prog,x)==y for x,y in self.contract):
                self.log('commit_rejected', reason='invalid_candidate');return True
            self.task_program=prog
            self.revision+=1
            self.log('task_reconstructed', actor='M', program=list(prog), internal=True)
            self.send('M','G','STOP_SEARCH')
            return True
        return True

    def tick(self):
        # Host only invokes enabled roles in deterministic round robin.
        # Role methods implement high-level behavior in Python; limitation disclosed.
        progressed=False
        for key, method in [('O',self._observer),('G',self._generator),('V',self._verifier),('M',self._modifier)]:
            if self.enabled[key]:
                progressed = method() or progressed
        return progressed

    def run(self, ticks=250):
        for _ in range(ticks):
            self.tick()
            if self.contract is not None and self.task_program is not None:
                # completion is only a measurement, not a host repair decision.
                break
        return self.summary()

    def summary(self):
        return {
            'contract_restored': self.contract is not None,
            'task_restored': self.task_program is not None,
            'heldout_passed': self.task_valid(heldout=True) if self.task_program else False,
            'program': list(self.task_program) if self.task_program else None,
            'internal_task_repairs': sum(e['event']=='task_reconstructed' for e in self.history),
            'internal_contract_repairs': sum(e['event']=='contract_reconstructed' for e in self.history),
            'external_repairs': self.external_repairs,
            'attempted': self.attempted,
            'compute_remaining': self.compute,
            'revision': self.revision,
            'events': len(self.history),
        }
