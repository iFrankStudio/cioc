"""CIOC V0.7: bounded recovery of a complementary generator role.

Research toy model only. The host still provides enumeration, contract evaluation
and installation semantics; it never schedules a specialized repair.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from itertools import product
from typing import Optional

ROLE_PROGRAMS = {
    "O": ("SCAN", "REQUEST", "FALLBACK"),
    "G": ("GENERATE", "PROPOSE"),
    "V": ("TEST", "CERTIFY"),
    "M": ("RECHECK", "COMMIT"),
}
OPCODES = tuple(sorted(set(op for p in ROLE_PROGRAMS.values() for op in p)))

@dataclass(frozen=True)
class Message:
    source: str
    dest: str
    kind: str
    payload: str

@dataclass
class Organism:
    budget: int = 2000
    max_program_length: int = 2
    available_ops: tuple[str, ...] = OPCODES
    roles: dict[str, Optional[tuple[str, ...]]] = field(default_factory=lambda: dict(ROLE_PROGRAMS))
    fragments: dict[str, dict[str, tuple[str, ...]]] = field(default_factory=dict)
    inbox: list[Message] = field(default_factory=list)
    proposals: dict[str, tuple[str, ...]] = field(default_factory=dict)
    checked: dict[str, tuple[str, ...]] = field(default_factory=dict)
    history: list[dict] = field(default_factory=list)
    revision: int = 0
    checked_count: int = 0
    external_restores: int = 0
    active_target: Optional[str] = None
    failed: set[str] = field(default_factory=set)

    def __post_init__(self):
        if not self.fragments:
            self.fragments = {
                "O": {r: (p[0],) for r, p in ROLE_PROGRAMS.items() if r != "O"},
                "V": {r: (p[-1],) for r, p in ROLE_PROGRAMS.items() if r != "V"},
                "G": {},
                "M": {},
            }

    def log(self, event: str, **data):
        self.history.append({"event": event, **data})

    def spend(self, n: int = 1) -> bool:
        if self.budget < n:
            self.log("budget_exhausted")
            return False
        self.budget -= n
        return True

    def damage(self, role: str):
        assert role in self.roles
        self.roles[role] = None
        self.log("damage", role=role)

    def send(self, source: str, dest: str, kind: str, payload: str):
        self.inbox.append(Message(source, dest, kind, payload))
        self.log("message", source=source, dest=dest, kind=kind)

    def functional_contract(self, role: str, program: tuple[str, ...]) -> bool:
        # Exact sequence matching is a known limitation, NOT general behavior verification.
        return program == ROLE_PROGRAMS[role]

    def fragment_ok(self, role: str, candidate: tuple[str, ...]) -> bool:
        if role == "O":
            return False
        first = self.fragments.get("O", {}).get(role)
        last = self.fragments.get("V", {}).get(role)
        return bool(
            self.roles["O"] and self.roles["V"] and first and last and candidate
            and candidate[0] == first[0] and candidate[-1] == last[0]
        )

    def tick(self) -> bool:
        progressed = False
        for actor in ("O", "G", "V", "M"):
            program = self.roles[actor]
            if program is None:
                continue
            if not self.spend():
                return progressed
            for instruction in program:
                progressed = self._step(actor, instruction) or progressed
        return progressed

    def _step(self, actor: str, instruction: str) -> bool:
        if actor == "O" and instruction == "SCAN":
            missing = [r for r, p in self.roles.items() if p is None and r not in self.failed]
            if missing and self.active_target is None:
                self.active_target = missing[0]
                return True
        if actor == "O" and instruction == "REQUEST" and self.active_target:
            if not any(m.kind == "BUILD" for m in self.inbox):
                self.send("O", "G", "BUILD", self.active_target)
                return True
        if actor == "O" and instruction == "FALLBACK" and self.active_target == "G" and not self.roles["G"]:
            if "G" not in self.proposals:
                for candidate in product(self.available_ops, repeat=self.max_program_length):
                    if not self.spend():
                        return False
                    self.checked_count += 1
                    if self.fragment_ok("G", candidate):
                        self.proposals["G"] = candidate
                        self.send("O", "V", "VERIFY", "G")
                        self.log("fallback_candidate", candidate=candidate)
                        return True
                self.failed.add("G")
                self.active_target = None
            return False
        if actor == "G" and instruction == "GENERATE":
            msg = next((m for m in self.inbox if m.dest == "G" and m.kind == "BUILD"), None)
            if msg and self.active_target and self.active_target not in self.proposals:
                target = msg.payload
                for candidate in product(self.available_ops, repeat=self.max_program_length):
                    if not self.spend():
                        break
                    self.checked_count += 1
                    if self.fragment_ok(target, candidate):
                        self.proposals[target] = candidate
                        self.log("candidate", target=target, candidate=candidate)
                        return True
                self.failed.add(target)
                self.active_target = None
                self.inbox.remove(msg)
                return True
        if actor == "G" and instruction == "PROPOSE":
            for target in self.proposals:
                if not any(m.kind == "VERIFY" and m.payload == target for m in self.inbox):
                    self.send("G", "V", "VERIFY", target)
                    return True
        if actor == "V" and instruction == "TEST":
            msg = next((m for m in self.inbox if m.dest == "V" and m.kind == "VERIFY"), None)
            if msg:
                self.inbox.remove(msg)
                target = msg.payload
                candidate = self.proposals.get(target)
                if candidate and self.spend(2) and self.functional_contract(target, candidate):
                    self.checked[target] = candidate
                    self.log("verified", target=target)
                    return True
                self.failed.add(target)
                self.active_target = None
                self.log("candidate_rejected", target=target)
                return True
        if actor == "V" and instruction == "CERTIFY":
            for target in self.checked:
                if not any(m.kind == "INSTALL" and m.payload == target for m in self.inbox):
                    self.send("V", "M", "INSTALL", target)
                    return True
        if actor == "M" and instruction == "RECHECK":
            return False
        if actor == "M" and instruction == "COMMIT":
            msg = next((m for m in self.inbox if m.dest == "M" and m.kind == "INSTALL"), None)
            if msg:
                self.inbox.remove(msg)
                target = msg.payload
                candidate = self.checked.get(target)
                if candidate and self.spend(3) and self.functional_contract(target, candidate):
                    self.roles[target] = candidate
                    self.revision += 1
                    self.log("internal_install", target=target, program=candidate)
                    self.active_target = None
                    self.checked.pop(target, None)
                    self.proposals.pop(target, None)
                    self.inbox = [m for m in self.inbox if not (m.kind == "BUILD" and m.payload == target)]
                    return True
                self.failed.add(target)
                self.active_target = None
                return True
        return False

    def run(self, cycles: int = 50) -> bool:
        for _ in range(cycles):
            self.tick()
            if all(p is not None for p in self.roles.values()):
                break
        return all(p is not None for p in self.roles.values())

    def external_restore(self, role: str):
        self.roles[role] = ROLE_PROGRAMS[role]
        self.external_restores += 1
        self.log("external_restore", role=role)

def experiment():
    outcomes = {}
    for role in ROLE_PROGRAMS:
        x = Organism()
        x.damage(role)
        outcomes["damage_" + role] = {"recovered": x.run(), "internal": x.revision}
    x = Organism(); x.damage("G"); x.damage("M")
    outcomes["damage_G_M"] = {"recovered": x.run(), "internal": x.revision}
    x = Organism(budget=10); x.damage("G")
    outcomes["low_budget"] = {"recovered": x.run(), "internal": x.revision}
    x = Organism(); x.damage("G"); x.fragments["V"].pop("G")
    outcomes["missing_fragment"] = {"recovered": x.run(), "internal": x.revision}
    x = Organism(); x.damage("G"); x.external_restore("G")
    outcomes["external"] = {"recovered": x.run(), "internal": x.revision, "external": x.external_restores}
    return outcomes
