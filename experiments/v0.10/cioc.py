"""CIOC V0.10: damageable, executable delegation protocols.

The host supplies scheduling, the opcode interpreter, finite behavioral tests,
resource accounting and atomic writes. Protocol code is organism state, no longer
undamageable Python control flow. This is NOT evidence of full closure or life.
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

PROTOCOLS = ("PV", "PI")  # delegated validation and delegated installation
PROTOCOL_CANONICAL = {
    "PV": ("POS_O", "NEG_M", "CERTIFY_P"),
    "PI": ("VOTE_V", "PERMIT_G", "INSTALL_P"),
}
PROTOCOL_ALPHABET = (
    "NEG_M_ALT", "PERMIT_G_ALT", "NEG_M", "PERMIT_G", "POS_O",
    "VOTE_V", "CERTIFY_P", "INSTALL_P", "NOOP", "ERASE",
)


def protocol_probe(code: tuple[str, ...], environment: dict[str, bool]) -> tuple[str, ...]:
    """Execute mutable protocol instructions through generic interpreter rules.

    These primitives and the probe environment are host-provided: only the
    instruction sequence is an organism asset. Neither target canonical program
    nor a target-specific repair procedure is consulted by this interpreter.
    """
    flags: set[str] = set()
    effect: list[str] = []
    rules = {
        "POS_O": ("O_positive", "pos"),
        "NEG_M": ("M_negative", "neg"),
        "NEG_M_ALT": ("M_negative", "neg"),
        "VOTE_V": ("V_approved", "vote"),
        "PERMIT_G": ("G_delegates", "permit"),
        "PERMIT_G_ALT": ("G_delegates", "permit"),
    }
    emit = {"CERTIFY_P": (frozenset(("pos", "neg")), "certificate"),
            "INSTALL_P": (frozenset(("vote", "permit")), "installation")}
    for opcode in code:
        if opcode == "ERASE":
            flags.clear()
            effect.clear()
        elif opcode in rules:
            condition, flag = rules[opcode]
            if environment.get(condition, False):
                flags.add(flag)
        elif opcode in emit:
            need, output = emit[opcode]
            if need.issubset(flags):
                effect.append(output)
    return tuple(effect)


def protocol_contract(name: str, code: tuple[str, ...]) -> bool:
    """Finite host-owned behavioral fixture; no canonical code equality check."""
    if name == "PV":
        stimuli = (
            ({"O_positive": True, "M_negative": True}, ("certificate",)),
            ({"O_positive": True}, ()),
            ({"M_negative": True}, ()),
        )
    elif name == "PI":
        stimuli = (
            ({"V_approved": True, "G_delegates": True}, ("installation",)),
            ({"V_approved": True}, ()),
            ({"G_delegates": True}, ()),
        )
    else:
        raise ValueError(name)
    return all(protocol_probe(code, ctx) == wanted for ctx, wanted in stimuli)


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
    protocol_alphabet: tuple[str, ...] = PROTOCOL_ALPHABET
    roles: dict[str, tuple[str, ...] | None] = field(default_factory=lambda: dict(CANONICAL))
    protocols: dict[str, tuple[str, ...] | None] = field(default_factory=lambda: dict(PROTOCOL_CANONICAL))
    protocol_fragments: dict[str, dict[str, tuple[int, str]]] = field(default_factory=dict)
    # Whether the host permits the *generic* protocol interpreter to run.
    # Not specialized contingency repair controls.
    enable_protocol_vm: bool = True
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
    protocol_repairs: int = 0

    def __post_init__(self):
        if not self.fragments:
            # No complete role program is stored by another single role.
            self.fragments = {
                "O": {"G": (0, "ENUMERATE"), "V": (0, "TEST"), "M": (0, "CHECK")},
                "G": {"O": (0, "SCAN"), "V": (2, "IDLE"), "M": (2, "IDLE")},
                "V": {"G": (2, "DELEGATE"), "O": (2, "IDLE"), "M": (2, "IDLE")},
                "M": {"G": (2, "DELEGATE"), "O": (1, "REQUEST"), "V": (2, "IDLE")},
            }
        if not self.protocol_fragments:
            # Distributed incomplete fragments; middle instructions unknown.
            self.protocol_fragments = {
                "O": {"PV": (0, "POS_O"), "PI": (0, "VOTE_V")},
                "G": {"PV": (2, "CERTIFY_P"), "PI": (2, "INSTALL_P")},
                "V": {"PV": (2, "CERTIFY_P"), "PI": (0, "VOTE_V")},
                "M": {"PV": (0, "POS_O"), "PI": (2, "INSTALL_P")},
            }

    def record(self, event: str, **details):
        self.events.append({"event": event, **details})

    def damage(self, *roles: str):
        for role in roles:
            if role in ROLES:
                self.roles[role] = None
            elif role in PROTOCOLS:
                self.protocols[role] = None
            else:
                raise ValueError(role)
            self.record("damage", role=role)

    def spend(self, amount=1):
        if self.budget < amount:
            self.record("budget_exhausted", needed=amount, remaining=self.budget)
            return False
        self.budget -= amount
        return True

    def fragments_match(self, target: str, candidate: tuple[str, ...]) -> bool:
        shares = self.protocol_fragments if target in PROTOCOLS else self.fragments
        fragments = [(holder, *data[target]) for holder, data in shares.items()
                     if self.roles[holder] is not None and target in data]
        return (len({holder for holder, _, _ in fragments}) >= 2
                and {0, 2}.issubset({position for _, position, _ in fragments})
                and all(candidate[position] == opcode for _, position, opcode in fragments))

    def search(self, target: str, actor: str):
        if target in self.proposals or target in self.approved or target in self.failed:
            return False
        if actor != "G" and not (actor == "O" and target == "G"):
            return False
        candidates = self.protocol_alphabet if target in PROTOCOLS else self.alphabet
        for candidate in product(candidates, repeat=3):
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
        if target in PROTOCOLS:
            # The host still owns the bounded protocol-contract fixture.
            # The protocol *candidate* lives entirely in mutable organism state.
            observed = ("valid",) if protocol_contract(target, candidate) else ()
            valid = observed == ("valid",)
        else:
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
            # Use an actual executable, damageable protocol program, not a
            # fixed host branch that directly performs both delegated checks.
            if (not self.enable_protocol_vm or self.protocols["PV"] is None
                    or self.roles["O"] is None or self.roles["M"] is None
                    or "REQUEST" not in self.roles["O"]
                    or "CHECK" not in self.roles["M"]):
                return False
            env = {"O_positive": probe(target, candidate, True) == expected(target, True),
                   "M_negative": probe(target, candidate, False) == expected(target, False)}
            if not self.spend(4):
                return False
            self.checks += 2
            if protocol_probe(self.protocols["PV"], env) != ("certificate",):
                self.record("delegated_protocol_reject", protocol="PV", target=target)
                self.attempted.setdefault(target, set()).add(candidate)
                self.proposals.pop(target, None)
                return True
            self.record("protocol_executed", protocol="PV", target=target)
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
              and self.enable_protocol_vm and self.protocols["PI"] is not None
              and self.roles["G"] is not None and "DELEGATE" in self.roles["G"]
              and self.roles["V"] is not None):
            env = {"V_approved": probe("V", self.roles["V"], True) == expected("V", True),
                   "G_delegates": "DELEGATE" in self.roles["G"]}
            if protocol_probe(self.protocols["PI"], env) != ("installation",):
                return False
            self.record("protocol_executed", protocol="PI", target=target)
            actor = "G"
        else:
            return False
        if not self.spend(3):
            return False
        if target in PROTOCOLS:
            self.protocols[target] = candidate
            self.protocol_repairs += 1
        else:
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
            for role in ROLES + PROTOCOLS:
                missing = self.protocols[role] is None if role in PROTOCOLS else self.roles[role] is None
                if missing and role not in self.requests and role not in self.failed:
                    self.requests.append(role)
                    self.record("request", role=role)
                    progress = True
        for role in list(self.requests):
            existing = self.protocols[role] if role in PROTOCOLS else self.roles[role]
            if existing is not None:
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
            if all(program is not None for program in (*self.roles.values(), *self.protocols.values())):
                break
            before = self.budget
            progressed = self.tick()
            if not progressed or before == self.budget:
                break
        return all(program is not None for program in (*self.roles.values(), *self.protocols.values()))

    def external_restore(self, role: str):
        if role in PROTOCOLS:
            self.protocols[role] = PROTOCOL_CANONICAL[role]
        else:
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
    for name in PROTOCOLS:
        w = World(); w.damage(name)
        outcomes["damage_" + name] = {
            "recovered": w.run(), "internal": w.internal_repairs,
            "protocol_repairs": w.protocol_repairs, "checks": w.checks,
        }
    for name, losses in (("damage_PV_V", ("PV", "V")),
                         ("damage_PI_M", ("PI", "M"))):
        w = World(); w.damage(*losses)
        outcomes[name] = {"recovered": w.run(), "internal": w.internal_repairs}
    for name, sequence in (("sequential_PV_V", ("PV", "V")),
                           ("sequential_PI_M", ("PI", "M"))):
        w = World(); w.damage(sequence[0]); first = w.run()
        w.damage(sequence[1]); second = w.run()
        outcomes[name] = {"first": first, "second": second,
                          "internal": w.internal_repairs}
    w = World(); w.damage("PV"); w.protocol_fragments["G"].pop("PV"); w.protocol_fragments["V"].pop("PV")
    outcomes["missing_protocol_fragment"] = {"recovered": w.run(), "internal": w.internal_repairs}
    w = World(budget=5); w.damage("PI")
    outcomes["protocol_budget_exhausted"] = {"recovered": w.run(), "internal": w.internal_repairs}
    w = World(); w.damage("PV"); w.external_restore("PV")
    outcomes["external_protocol_repair"] = {"recovered": w.run(),
        "internal": w.internal_repairs, "external": w.external_repairs}
    return outcomes
