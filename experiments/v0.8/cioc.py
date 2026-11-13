"""CIOC V0.8: distributed fragments + bounded *behavioral* regeneration.

The world supplies opcode interpretation and deterministic scheduling. Organism
policies are instruction sequences; fragments and messages are organism state.
A host-provided finite behavioral probe is explicitly NOT proof of general
program equivalence, organizational closure, or digital life.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from itertools import product
from typing import Optional

ROLES = ("O", "G", "V", "M")
# PROPOSE_ALT intentionally has the same effect as PROPOSE in our tiny world.
# It can establish that recovery need not equal original source bytes.
ALPHABET = (
    "IDLE", "DROP", "ENUMERATE", "PROPOSE_ALT", "PROPOSE",
    "SCAN", "REQUEST", "TEST", "CERTIFY", "CHECK", "COMMIT",
)
CANONICAL = {
    "O": ("SCAN", "REQUEST", "IDLE"),
    "G": ("ENUMERATE", "PROPOSE", "IDLE"),
    "V": ("TEST", "CERTIFY", "IDLE"),
    "M": ("CHECK", "COMMIT", "IDLE"),
}


def probe(role: str, code: tuple[str, ...], stimulus: bool) -> tuple[str, ...]:
    """Execute real opcode semantics in a small isolated role-behavior probe.

    The fixture expects an observable *effect*, not a canonical instruction
    sequence. It does not consult CANONICAL during candidate validation.
    """
    state: set[str] = set()
    effects: list[str] = []
    for opcode in code:
        if opcode == "DROP":
            state.clear()
            effects.clear()
        elif opcode == "IDLE":
            pass
        elif role == "O":
            if opcode == "SCAN" and stimulus:
                state.add("missing")
            elif opcode == "REQUEST" and "missing" in state:
                effects.append("request")
        elif role == "G":
            if opcode == "ENUMERATE" and stimulus:
                state.add("candidate")
            elif opcode in {"PROPOSE", "PROPOSE_ALT"} and "candidate" in state:
                effects.append("proposal")
        elif role == "V":
            if opcode == "TEST" and stimulus:
                state.add("accepted")
            elif opcode == "CERTIFY" and "accepted" in state:
                effects.append("certification")
        elif role == "M":
            if opcode == "CHECK" and stimulus:
                state.add("approved")
            elif opcode == "COMMIT" and "approved" in state:
                effects.append("installation")
    return tuple(effects)


EXPECTED_EFFECT = {"O": "request", "G": "proposal", "V": "certification", "M": "installation"}


def contract(role: str, code: tuple[str, ...]) -> bool:
    """Finite, host-owned behavioral tests: positive and negative stimulus."""
    return (probe(role, code, True) == (EXPECTED_EFFECT[role],)
            and probe(role, code, False) == ())


@dataclass
class World:
    budget: int = 400
    alphabet: tuple[str, ...] = ALPHABET
    max_program_length: int = 3
    roles: dict[str, Optional[tuple[str, ...]]] = field(
        default_factory=lambda: dict(CANONICAL))
    # Each share discloses ONE position; no holder has full target code.
    fragments: dict[str, dict[str, tuple[int, str]]] = field(default_factory=dict)
    requests: list[str] = field(default_factory=list)
    staged: dict[str, tuple[str, ...]] = field(default_factory=dict)
    proposals: dict[str, tuple[str, ...]] = field(default_factory=dict)
    approved: dict[str, tuple[str, ...]] = field(default_factory=dict)
    rejected: set[str] = field(default_factory=set)
    attempted: dict[str, set[tuple[str, ...]]] = field(default_factory=dict)
    events: list[dict] = field(default_factory=list)
    internal_repairs: int = 0
    external_repairs: int = 0
    probes: int = 0

    def __post_init__(self):
        if not self.fragments:
            # For G the surviving O,V,M supply positions 0,2,0, respectively;
            # position 1 remains unknown and must be behaviorally inferred.
            self.fragments = {
                "O": {"G": (0, "ENUMERATE"), "V": (0, "TEST"), "M": (0, "CHECK")},
                "G": {"O": (0, "SCAN"), "V": (2, "IDLE"), "M": (1, "COMMIT")},
                "V": {"G": (2, "IDLE"), "O": (2, "IDLE"), "M": (2, "IDLE")},
                "M": {"G": (0, "ENUMERATE"), "O": (1, "REQUEST"), "V": (1, "CERTIFY")},
            }

    def damage(self, *names: str):
        for role in names:
            if role not in ROLES:
                raise ValueError(role)
            self.roles[role] = None
            self.events.append({"event": "damage", "role": role})

    def spend(self, amount: int = 1) -> bool:
        if self.budget < amount:
            self.events.append({"event": "budget_exhausted"})
            return False
        self.budget -= amount
        return True

    def fragments_match(self, role: str, candidate: tuple[str, ...]) -> bool:
        surviving = [fragment[role] for holder, fragment in self.fragments.items()
                     if self.roles[holder] is not None and role in fragment]
        # Require TWO independent fragment holders AND coverage of both ends.
        if len(surviving) < 2 or not {0, 2}.issubset({pos for pos, _ in surviving}):
            return False
        return all(candidate[pos] == opcode for pos, opcode in surviving)

    def synthesize(self, role: str, actor: str) -> bool:
        if role in self.proposals or role in self.staged or role in self.rejected:
            return False
        # V0.8 only grants O fallback synthesis for missing G; G can synthesize
        # candidates for other roles, which is not sufficient to repair them.
        if actor != "G" and not (actor == "O" and role == "G"):
            return False
        if actor == "G" and self.roles["G"] is None:
            return False
        if self.max_program_length != 3:
            self.rejected.add(role)
            return False
        for candidate in product(self.alphabet, repeat=self.max_program_length):
            if not self.fragments_match(role, candidate):
                continue
            if candidate in self.attempted.get(role, set()):
                continue
            if not self.spend():
                return False
            self.probes += 1
            if self.fragments_match(role, candidate):
                if actor == "O":
                    # Observer fallback publishes on behalf of the absent G.
                    self.proposals[role] = candidate
                    self.events.append({"event": "fallback_proposal", "role": role})
                else:
                    self.staged[role] = candidate
                self.events.append({"event": "candidate", "role": role, "code": list(candidate)})
                return True
        self.rejected.add(role)
        self.events.append({"event": "no_candidate", "role": role})
        return True

    def tick(self) -> bool:
        progress = False
        for actor in ROLES:
            code = self.roles[actor]
            if code is None or not self.spend():
                continue
            for opcode in code:
                if actor == "O" and opcode == "SCAN":
                    for role in ROLES:
                        if self.roles[role] is None and role not in self.rejected and role not in self.requests:
                            self.requests.append(role)
                            progress = True
                elif actor == "O" and opcode == "REQUEST":
                    if "G" in self.requests and self.roles["G"] is None:
                        progress = self.synthesize("G", "O") or progress
                elif actor == "G" and opcode == "ENUMERATE":
                    for role in list(self.requests):
                        if self.roles[role] is None and role not in self.proposals:
                            progress = self.synthesize(role, "G") or progress
                elif actor == "G" and opcode in {"PROPOSE", "PROPOSE_ALT"}:
                    for role, candidate in list(self.staged.items()):
                        self.proposals[role] = candidate
                        self.staged.pop(role)
                        self.events.append({"event": "proposal_emitted", "role": role, "opcode": opcode})
                        progress = True
                elif actor == "V" and opcode == "TEST":
                    for role, candidate in list(self.proposals.items()):
                        if role not in self.approved and self.spend(2):
                            if contract(role, candidate):
                                self.approved[role] = candidate
                                self.events.append({"event": "verified", "role": role})
                            else:
                                # Rejection of the first fragment-compatible candidate
                                # is NOT terminal: more candidates must be searched.
                                self.events.append({"event": "rejected_candidate", "role": role})
                                self.attempted.setdefault(role, set()).add(candidate)
                                self.proposals.pop(role, None)
                            progress = True
                elif actor == "M" and opcode == "COMMIT":
                    for role, candidate in list(self.approved.items()):
                        if self.spend(3):
                            self.roles[role] = candidate
                            self.internal_repairs += 1
                            self.events.append({"event": "internal_repair", "role": role,
                                                "code": list(candidate)})
                            self.approved.pop(role, None)
                            self.proposals.pop(role, None)
                            self.staged.pop(role, None)
                            if role in self.requests:
                                self.requests.remove(role)
                            progress = True
        return progress

    def run(self, cycles: int = 100) -> bool:
        for _ in range(cycles):
            if all(program is not None for program in self.roles.values()):
                return True
            if not self.tick():
                break
        return all(program is not None for program in self.roles.values())

    def external_restore(self, role: str):
        self.roles[role] = CANONICAL[role]
        self.external_repairs += 1
        self.events.append({"event": "external_repair", "role": role})


def scenarios() -> dict:
    result = {}
    for name in ROLES:
        w = World()
        w.damage(name)
        result["damage_" + name] = {"recovered": w.run(), "internal": w.internal_repairs,
                                   "probes": w.probes}
    for name, losses in (("damage_G_M", ("G", "M")), ("damage_G_V", ("G", "V"))):
        w = World()
        w.damage(*losses)
        result[name] = {"recovered": w.run(), "internal": w.internal_repairs}
    w = World(budget=8)
    w.damage("G")
    result["low_budget"] = {"recovered": w.run(), "internal": w.internal_repairs}
    w = World()
    w.damage("G")
    w.fragments["V"].pop("G")
    result["lost_fragment"] = {"recovered": w.run(), "internal": w.internal_repairs}
    w = World()
    w.damage("G")
    w.external_restore("G")
    result["external"] = {"recovered": w.run(), "internal": w.internal_repairs,
                          "external": w.external_repairs}
    return result
