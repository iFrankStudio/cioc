"""CIOC V0.8: distributed, bounded recovery of complementary maintenance roles.

All organism policies are data-driven instruction tuples. The host provides a
small interpreter, deterministic scheduler, test oracle and resource accounting.
No organism receives a complete backup of any other role.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from itertools import product

ROLE_SPEC = {
    "O": ("SCAN", "REQUEST"),
    "G": ("ENUMERATE", "PROPOSE"),
    "V": ("TEST", "CERTIFY"),
    "M": ("CHECK", "COMMIT"),
}
ALPHABET = tuple(sorted({op for ops in ROLE_SPEC.values() for op in ops}))
ROLES = tuple(ROLE_SPEC)

@dataclass
class World:
    budget: int = 500
    max_length: int = 2
    alphabet: tuple[str, ...] = ALPHABET
    roles: dict[str, tuple[str, ...] | None] = field(default_factory=lambda: dict(ROLE_SPEC))
    fragments: dict[str, dict[str, tuple[str, ...]]] = field(default_factory=dict)
    queue: list[tuple[str, str, str]] = field(default_factory=list)
    candidates: dict[str, tuple[str, ...]] = field(default_factory=dict)
    approved: dict[str, tuple[str, ...]] = field(default_factory=dict)
    failures: set[str] = field(default_factory=set)
    log: list[dict] = field(default_factory=list)
    internal_repairs: int = 0
    external_repairs: int = 0
    probes: int = 0

    def __post_init__(self):
        if not self.fragments:
            # Every role holds one instruction of every *other* role, never
            # a full copy. The fragments are designed, not emergent.
            self.fragments = {
                holder: {
                    target: (spec[ROLES.index(holder) % len(spec)],)
                    for target, spec in ROLE_SPEC.items() if holder != target
                }
                for holder in ROLES
            }

    def damage(self, *names: str):
        for name in names:
            self.roles[name] = None
            self.log.append({"event": "damage", "role": name})

    def spend(self, amount=1):
        if self.budget < amount:
            return False
        self.budget -= amount
        return True

    def send(self, dest: str, kind: str, target: str):
        msg = (dest, kind, target)
        if msg not in self.queue:
            self.queue.append(msg)

    def pop(self, dest: str, kind: str):
        for index, msg in enumerate(self.queue):
            if msg[0] == dest and msg[1] == kind:
                self.queue.pop(index)
                return msg[2]
        return None

    def compatible(self, target: str, candidate: tuple[str, ...]):
        # The organism can compare distributed fragments; it cannot read ROLE_SPEC.
        constraints = [(i, frag[0]) for holder, data in self.fragments.items()
                       if self.roles[holder] is not None
                       for frag in [data.get(target)]
                       if frag is not None
                       for i in [ROLES.index(holder) % len(candidate)]]
        if len({i for i, _ in constraints}) < 2:
            return False
        return all(candidate[i] == op for i, op in constraints)

    def contract(self, target: str, candidate: tuple[str, ...]):
        # Host-owned oracle; fixed exact-code equality remains an explicit limit.
        return candidate == ROLE_SPEC[target]

    def run(self, cycles=100):
        for _ in range(cycles):
            if all(v is not None for v in self.roles.values()):
                break
            if not self.tick():
                break
        return all(v is not None for v in self.roles.values())

    def tick(self):
        progress = False
        for actor in ROLES:
            code = self.roles[actor]
            if code is None:
                continue
            if not self.spend():
                return False
            for op in code:
                if op == "SCAN":
                    for target in ROLES:
                        if self.roles[target] is None and target not in self.failures:
                            self.send("G", "BUILD", target)
                            progress = True
                elif op == "REQUEST":
                    # Fallback: when G is missing, O initiates bounded synthesis.
                    if self.roles["G"] is None and "G" not in self.failures:
                        progress = self.synthesize("G", "O") or progress
                elif op == "ENUMERATE":
                    target = self.pop("G", "BUILD")
                    if target is not None:
                        progress = self.synthesize(target, "G") or progress
                elif op == "PROPOSE":
                    for target in self.candidates:
                        self.send("V", "VERIFY", target)
                elif op == "TEST":
                    target = self.pop("V", "VERIFY")
                    if target is not None and target in self.candidates:
                        candidate = self.candidates[target]
                        if self.spend(2) and self.contract(target, candidate):
                            self.approved[target] = candidate
                            self.log.append({"event": "verified", "role": target})
                        else:
                            self.failures.add(target)
                        progress = True
                elif op == "CERTIFY":
                    for target in self.approved:
                        self.send("M", "INSTALL", target)
                elif op == "CHECK":
                    pass
                elif op == "COMMIT":
                    target = self.pop("M", "INSTALL")
                    if target is not None and target in self.approved:
                        candidate = self.approved[target]
                        if self.spend(3) and self.contract(target, candidate):
                            self.roles[target] = candidate
                            self.internal_repairs += 1
                            self.log.append({"event": "internal_repair", "role": target})
                            self.candidates.pop(target, None)
                            self.approved.pop(target, None)
                        progress = True
        return progress

    def synthesize(self, target, actor):
        if target in self.candidates or target in self.failures:
            return False
        if target == "G" and actor != "O":
            return False
        for candidate in product(self.alphabet, repeat=self.max_length):
            if not self.spend():
                return False
            self.probes += 1
            if self.compatible(target, candidate):
                self.candidates[target] = candidate
                self.send("V", "VERIFY", target)
                self.log.append({"event": "candidate", "role": target})
                return True
        self.failures.add(target)
        return True

    def external_restore(self, target):
        self.roles[target] = ROLE_SPEC[target]
        self.external_repairs += 1
        self.log.append({"event": "external_repair", "role": target})

def scenarios():
    output = {}
    for role in ROLES:
        w = World(); w.damage(role)
        output["loss_" + role] = {"recovered": w.run(), "internal": w.internal_repairs}
    for pair in (("O", "G"), ("G", "M")):
        w = World(); w.damage(*pair)
        output["loss_" + "_".join(pair)] = {"recovered": w.run(), "internal": w.internal_repairs}
    w = World(budget=4); w.damage("G")
    output["low_budget"] = {"recovered": w.run(), "internal": w.internal_repairs}
    w = World(); w.damage("G"); w.external_restore("G")
    output["external"] = {"recovered": w.run(), "internal": w.internal_repairs, "external": w.external_repairs}
    return output
