"""CIOC V0.9: finite, distributed recovery of verifier and modifier roles.

A deterministic *host* runs a small fixed instruction vocabulary and implements
resource accounting, testing primitives, and atomic installation. Recovery
protocols are predefined; this is a causal experiment, not autonomous life.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from itertools import product

ROLES = ("O", "G", "V", "M")
CANONICAL = {
    "O": ("SCAN", "REQUEST", "IDLE"),
    "G": ("ENUMERATE", "PROPOSE", "DELEGATE"),
    "V": ("TEST", "CERTIFY", "IDLE"),
    "M": ("CHECK", "COMMIT", "IDLE"),
}
# Instruction positions 0 and 2 are fragmented. Alternatives are operationally
# equivalent on our bounded probe stimuli; code string equality is not checked.
ALPHABET = (
    "CERTIFY_ALT", "COMMIT_ALT", "PROPOSE_ALT", "CERTIFY", "COMMIT",
    "PROPOSE", "SCAN", "TEST", "CHECK", "IDLE", "ENUMERATE",
    "REQUEST", "DELEGATE", "DROP",
)
OBSERVABLE = {"O": "request", "G": "proposal", "V": "certificate", "M": "installation"}


def probe(role: str, code: tuple[str, ...], stimulus: bool) -> tuple[str, ...]:
    """Interpret instructions in an isolated sandbox; no canonical code lookup."""
    armed = False
    effects: list[str] = []
    for opcode in code:
        if opcode == "DROP":
            armed = False
            effects.clear()
        elif opcode == "IDLE" or opcode == "DELEGATE":
            pass
        elif role == "O":
            if opcode == "SCAN":
                armed = bool(stimulus)
            elif opcode == "REQUEST" and armed:
                effects.append("request")
        elif role == "G":
            if opcode == "ENUMERATE":
                armed = bool(stimulus)
            elif opcode in ("PROPOSE", "PROPOSE_ALT") and armed:
                effects.append("proposal")
        elif role == "V":
            if opcode == "TEST":
                armed = bool(stimulus)
            elif opcode in ("CERTIFY", "CERTIFY_ALT") and armed:
                effects.append("certificate")
        elif role == "M":
            if opcode == "CHECK":
                armed = bool(stimulus)
            elif opcode in ("COMMIT", "COMMIT_ALT") and armed:
                effects.append("installation")
    return tuple(effects)


def expected(role: str, stimulus: bool) -> tuple[str, ...]:
    return (OBSERVABLE[role],) if stimulus else ()


@dataclass
class World:
    budget: int = 1000
    allow_delegated_validation: bool = True
    allow_delegated_installation: bool = True
    alphabet: tuple[str, ...] = ALPHABET
    roles: dict[str, tuple[str, ...] | None] = field(default_factory=lambda: dict(CANONICAL))
    fragments: dict[str, dict[str, tuple[int, str]]] = field(default_factory=dict)
    requests: list[str] = field(default_factory=list)
    proposals: dict[str, tuple[str, ...]] = field(default_factory=dict)
    approved: dict[str, tuple[str, ...]] = field(default_factory=dict)
    partial_checks: dict[str, set[tuple[str, bool]]] = field(default_factory=dict)
    attempted: dict[str, set[tuple[str, ...]]] = field(default_factory=dict)
    failed: set[str] = field(default_factory=set)
    events: list[dict] = field(default_factory=list)
    internal_repairs: int = 0
    external_repairs: int = 0
    searches: int = 0
    checks: int = 0

    def __post_init__(self):
        if not self.fragments:
            # No complete role program is stored by another single role.
            self.fragments = {
                "O": {"G": (0, "ENUMERATE"), "V": (0, "TEST"), "M": (0, "CHECK")},
                "G": {"O": (0, "SCAN"), "V": (2, "IDLE"), "M": (2, "IDLE")},
                "V": {"G": (2, "DELEGATE"), "O": (2, "IDLE"), "M": (2, "IDLE")},
                "M": {"G": (2, "DELEGATE"), "O": (1, "REQUEST"), "V": (2, "IDLE")},
            }

    def record(self, event: str, **details):
        self.events.append({"event": event, **details})

    def damage(self, *roles: str):
        for role in roles:
            if role not in ROLES:
                raise ValueError(role)
            self.roles[role] = None
            self.record("damage", role=role)

    def spend(self, amount=1):
        if self.budget < amount:
            self.record("budget_exhausted", needed=amount, remaining=self.budget)
            return False
        self.budget -= amount
        return True

    def fragments_match(self, target: str, candidate: tuple[str, ...]) -> bool:
        fragments = [(holder, *data[target]) for holder, data in self.fragments.items()
                     if self.roles[holder] is not None and target in data]
        return (len({holder for holder, _, _ in fragments}) >= 2
                and {0, 2}.issubset({position for _, position, _ in fragments})
                and all(candidate[position] == opcode for _, position, opcode in fragments))

    def search(self, target: str, actor: str):
        if target in self.proposals or target in self.approved or target in self.failed:
            return False
        if actor != "G" and not (actor == "O" and target == "G"):
            return False
        for candidate in product(self.alphabet, repeat=3):
            if not self.fragments_match(target, candidate):
                continue
            if candidate in self.attempted.get(target, set()):
                continue
            if not self.spend():
                return False
            self.searches += 1
            self.proposals[target] = candidate
            self.record("candidate", role=target, actor=actor, program=list(candidate))
            return True
        self.failed.add(target)
        self.record("no_more_candidates", role=target)
        return True

    def check_sample(self, target: str, candidate: tuple[str, ...], actor: str, stimulus: bool):
        if not self.spend(2):
            return False
        self.checks += 1
        observed = probe(target, candidate, stimulus)
        valid = observed == expected(target, stimulus)
        self.record("behavior_probe", role=target, checker=actor, stimulus=stimulus,
                    observed=list(observed), accepted=valid)
        if not valid:
            self.record("candidate_rejected", role=target, checker=actor)
            self.attempted.setdefault(target, set()).add(candidate)
            self.proposals.pop(target, None)
            self.partial_checks.pop(target, None)
            return False
        self.partial_checks.setdefault(target, set()).add((actor, stimulus))
        return True

    def validate(self, target: str):
        candidate = self.proposals.get(target)
        if candidate is None or target in self.approved:
            return False
        if self.roles["V"] is not None:
            # The V executable program must actually contain its check operator.
            if probe("V", self.roles["V"], True) != expected("V", True):
                return False
            if not self.check_sample(target, candidate, "V", True):
                return True
            if not self.check_sample(target, candidate, "V", False):
                return True
        elif target == "V" and self.allow_delegated_validation:
            # This fallback requires two DIFFERENT surviving role programs.
            if (self.roles["O"] is None or self.roles["M"] is None
                    or "REQUEST" not in self.roles["O"]
                    or "CHECK" not in self.roles["M"]):
                return False
            if not self.check_sample(target, candidate, "O", True):
                return True
            if not self.check_sample(target, candidate, "M", False):
                return True
        else:
            return False
        self.approved[target] = candidate
        self.record("certified", role=target, path="delegated" if target == "V" and self.roles["V"] is None else "V")
        return True

    def install(self, target: str):
        candidate = self.approved.get(target)
        if candidate is None:
            return False
        if self.roles["M"] is not None:
            if probe("M", self.roles["M"], True) != expected("M", True):
                return False
            actor = "M"
        elif (target == "M" and self.allow_delegated_installation
              and self.roles["G"] is not None and "DELEGATE" in self.roles["G"]
              and self.roles["V"] is not None and probe("V", self.roles["V"], True) == expected("V", True)):
            actor = "G"
        else:
            return False
        if not self.spend(3):
            return False
        self.roles[target] = candidate
        self.internal_repairs += 1
        self.record("internal_install", role=target, actor=actor, program=list(candidate))
        self.requests = [name for name in self.requests if name != target]
        self.proposals.pop(target, None)
        self.approved.pop(target, None)
        self.partial_checks.pop(target, None)
        return True

    def tick(self):
        if not self.spend():
            return False
        progress = False
        # The scheduler activates only pre-existing executable role policies.
        if self.roles["O"] is not None and probe("O", self.roles["O"], True) == expected("O", True):
            for role in ROLES:
                if self.roles[role] is None and role not in self.requests and role not in self.failed:
                    self.requests.append(role)
                    self.record("request", role=role)
                    progress = True
        for role in list(self.requests):
            if self.roles[role] is not None:
                self.requests.remove(role)
                continue
            if role not in self.proposals:
                if self.roles["G"] is not None and probe("G", self.roles["G"], True) == expected("G", True):
                    progress = self.search(role, "G") or progress
                elif role == "G" and self.roles["O"] is not None and "REQUEST" in self.roles["O"]:
                    progress = self.search(role, "O") or progress
            if role in self.proposals:
                progress = self.validate(role) or progress
            if role in self.approved:
                progress = self.install(role) or progress
        return progress

    def run(self, cycles=100):
        for _ in range(cycles):
            if all(program is not None for program in self.roles.values()):
                break
            before = self.budget
            progressed = self.tick()
            if not progressed or before == self.budget:
                break
        return all(program is not None for program in self.roles.values())

    def external_restore(self, role: str):
        self.roles[role] = CANONICAL[role]
        self.external_repairs += 1
        self.record("external_restore", role=role)


def scenarios():
    outcomes = {}
    for role in ROLES:
        w = World(); w.damage(role); recovered = w.run()
        outcomes["damage_" + role] = {"recovered": recovered, "internal": w.internal_repairs,
                                     "checks": w.checks, "searches": w.searches}
    for label, names in (("damage_V_M", ("V", "M")), ("damage_G_V", ("G", "V"))):
        w = World(); w.damage(*names)
        outcomes[label] = {"recovered": w.run(), "internal": w.internal_repairs}
    w = World(budget=4); w.damage("V")
    outcomes["low_budget"] = {"recovered": w.run(), "internal": w.internal_repairs}
    w = World(); w.damage("V"); w.fragments["M"].pop("V"); w.fragments["G"].pop("V")
    outcomes["missing_fragment"] = {"recovered": w.run(), "internal": w.internal_repairs}
    w = World(allow_delegated_validation=False); w.damage("V")
    outcomes["without_validation_delegation"] = {"recovered": w.run(), "internal": w.internal_repairs}
    w = World(allow_delegated_installation=False); w.damage("M")
    outcomes["without_installation_delegation"] = {"recovered": w.run(), "internal": w.internal_repairs}
    w = World(); w.damage("V"); w.external_restore("V")
    outcomes["external"] = {"recovered": w.run(), "internal": w.internal_repairs,
                            "external": w.external_repairs}
    return outcomes
