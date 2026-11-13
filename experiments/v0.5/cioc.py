"""CIOC V0.5: autonomous repair decisions in mutable graph programs.

A bounded, deliberately artificial benchmark. The host VM schedules graph programs,
executes generic opcodes, enforces budgets and atomic writes, and evaluates programs
in a sandbox. The VM DOES NOT contain a function that selects targets or repairs them.
Graph controllers scan damage, enumerate code points, call sandbox probes, decide
whether to install, and write to the organism's store themselves.

The grammar, contracts and interpreter are immutable *environment assumptions*;
this is not full autopoiesis or proof of life.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from typing import Any
import copy
import json

# A graph program uses instruction opcodes, explicit jumps and mutable data.
# A genome selects alternative graph edges and tests, NOT an existing source backup.
# Correct repair programs can be synthesized from this finite DSL without stored copies.
OPS = ("READ", "FIRST", "IF_NONE", "ZERO", "DECODE", "EVAL", "IF_VALID", "INC", "IF_MORE", "PUT", "HALT")
GENOME_SIZE = 16


@dataclass(frozen=True)
class Node:
    op: str
    yes: int | None = None
    no: int | None = None


@dataclass
class Graph:
    nodes: dict[int, Node]
    entry: int = 0
    genome: int = -1


def decode_graph(code: int) -> Graph:
    """Bounded graph encoding; no serialized target controller is stored.

    4 bits choose whether this policy scans, uses its test result, increments
    the enumerator and installs. Only some combinations can actually repair.
    Their relative order and branch structure are encoded as graph edges.
    """
    if not 0 <= code < GENOME_SIZE:
        raise ValueError("genome outside finite language")
    scan = (code & 1) != 0
    verify = (code & 2) != 0
    advance = (code & 4) != 0
    install = (code & 8) != 0
    # Instructions are real graph nodes and must be traversed by the VM.
    return Graph({
        0: Node("READ" if scan else "HALT", 11),
        11: Node("FIRST", 1),
        1: Node("IF_NONE", yes=10, no=2),
        2: Node("ZERO", 3),
        3: Node("DECODE", 4),
        4: Node("EVAL" if verify else "HALT", 5),
        5: Node("IF_VALID", yes=8 if install else 10, no=6),
        6: Node("INC" if advance else "HALT", 7),
        7: Node("IF_MORE", yes=3, no=10),
        8: Node("PUT", 10),
        10: Node("HALT"),
    }, genome=code)


@dataclass
class Organism:
    programs: dict[str, Graph | None]
    compute: int = 1400
    version: int = 0
    external_repairs: int = 0
    history: list[dict[str, Any]] = field(default_factory=list)
    contracts_enabled: bool = True
    # World-enforced action rights, not mutable by the organism.
    mutate_enabled: bool = True
    # Programmable limits exposed as environmental budgets.
    max_candidates: int = GENOME_SIZE

    @classmethod
    def create(cls, compute: int = 1400) -> "Organism":
        # Same functional behavior, nonidentical executable state objects.
        # All controllers are damaged/replaced in the same way as other assets.
        return cls({k: decode_graph(15) for k in ("A", "B", "C")}, compute=compute)

    def damage(self, name: str) -> None:
        if name not in self.programs:
            raise ValueError(name)
        self.programs[name] = None
        self.history.append({"event": "damage", "target": name})

    def viable(self) -> bool:
        return all(p is not None and self._has_viable_code(p) for p in self.programs.values())

    @staticmethod
    def _has_viable_code(graph: Graph) -> bool:
        return graph.genome == 15  # functional equivalence for this tiny grammar


@dataclass
class Frame:
    pc: int
    candidate_index: int = 0
    candidate: Graph | None = None
    target: str | None = None
    observed_faults: tuple[str, ...] = ()
    valid: bool = False
    done: bool = False


class VM:
    """Fixed mechanism: generic graph interpreter, state query and guarded writes.

    No external `regenerate(target)` method. Calling `run()` only allocates turns
    to intact resident graph programs. Every repair decision flows through nodes.
    """
    def __init__(self, org: Organism, oracle_code: int | None = None):
        self.o = org
        self.oracle_code = oracle_code
        self.steps = 0
        self._frames: dict[str, Frame] = {}

    def _charge(self, program: str, instruction: str, units: int = 1) -> bool:
        if self.o.compute < units:
            self.o.history.append({"event": "out_of_budget", "program": program, "op": instruction})
            return False
        self.o.compute -= units
        self.steps += 1
        return True

    def _sandbox_test(self, graph: Graph) -> bool:
        """Run candidate graph in an isolated simulated damaged system.

        The behavioral contract asks the candidate to find genome 15 using
        basic VM instructions, then install it. A probe-only EVAL oracle is
        required to avoid infinite recursion while examining the controller's
        control flow. It is a fixed declared test contract, not source backup.
        """
        if not self.o.contracts_enabled:
            return False
        probe = Organism({"T": None, "R": graph}, compute=350,
                         max_candidates=GENOME_SIZE)
        interpreter = VM(probe, oracle_code=15)
        interpreter.run(max_steps=300)
        return (probe.programs["T"] is not None
                and probe.programs["T"].genome == 15)

    def _execute_one(self, actor: str, graph: Graph, frame: Frame,
                     allow_sandbox: bool) -> None:
        node = graph.nodes.get(frame.pc)
        if node is None or node.op not in OPS:
            frame.done = True
            return
        if not self._charge(actor, node.op):
            frame.done = True
            return
        self.o.history.append({"event": "step", "actor": actor, "op": node.op, "pc": frame.pc})
        op = node.op
        if op == "HALT":
            frame.done = True
        elif op == "READ":
            # Generic introspection returns data. It does not select/repair.
            frame.observed_faults = tuple(k for k, v in self.o.programs.items() if v is None)
            frame.pc = node.yes
        elif op == "FIRST":
            frame.target = frame.observed_faults[0] if frame.observed_faults else None
            frame.pc = node.yes
        elif op == "IF_NONE":
            frame.pc = node.yes if frame.target is None else node.no
        elif op == "ZERO":
            frame.candidate_index = 0
            frame.pc = node.yes
        elif op == "DECODE":
            frame.candidate = decode_graph(frame.candidate_index)
            frame.pc = node.yes
        elif op == "EVAL":
            # Sandbox execution is a generic VM operation, not a hidden
            # target-specific repair policy. It charges a fixed evaluation cost.
            if self.oracle_code is not None:
                # Explicit sandbox probe oracle: behavioral test feedback only.
                frame.valid = frame.candidate_index == self.oracle_code
            else:
                frame.valid = (allow_sandbox and self.o.contracts_enabled
                               and self._charge(actor, "sandbox_budget", 4)
                               and frame.candidate is not None
                               and self._sandbox_test(frame.candidate))
            frame.pc = node.yes
        elif op == "IF_VALID":
            frame.pc = node.yes if frame.valid else node.no
        elif op == "INC":
            frame.candidate_index += 1
            frame.pc = node.yes
        elif op == "IF_MORE":
            frame.pc = node.yes if frame.candidate_index < min(GENOME_SIZE, self.o.max_candidates) else node.no
        elif op == "PUT":
            if self.o.mutate_enabled and frame.target is not None and frame.candidate is not None and frame.valid:
                if self.o.programs[frame.target] is None:
                    self.o.programs[frame.target] = copy.deepcopy(frame.candidate)
                    self.o.version += 1
                    self.o.history.append({"event": "internal_commit", "actor": actor,
                        "target": frame.target, "genome": frame.candidate.genome,
                        "revision": self.o.version})
            frame.pc = node.yes

    def run(self, max_steps: int = 1000, *, allow_sandbox: bool = True) -> Organism:
        # Scheduler only executes available graphs; it has no repair target policy.
        # Under a fixed event bound, damaged actors do not execute.
        order = list(self.o.programs.keys())
        while self.steps < max_steps:
            progress = False
            for actor in order:
                graph = self.o.programs[actor]
                if graph is None:
                    continue
                # Each actor is activated once each round; dormant actors restart
                # only if a new fault appears after a previous completed round.
                if actor not in self._frames:
                    self._frames[actor] = Frame(pc=graph.entry)
                frame = self._frames[actor]
                if frame.done:
                    continue
                self._execute_one(actor, graph, frame, allow_sandbox)
                progress = True
                if self.steps >= max_steps:
                    break
            if not progress:
                break
        return self.o


def run_case(name: str) -> dict[str, Any]:
    o = Organism.create(compute=28 if name == "low_budget" else 1400)
    if name == "single_fault":
        o.damage("A")
    elif name == "sequential_faults":
        for target in ("A", "B", "C"):
            o.damage(target)
            VM(o).run()
        return summarize(name, o)
    elif name == "double_fault":
        o.damage("A"); o.damage("B")
    elif name == "all_fault":
        for target in ("A", "B", "C"): o.damage(target)
    elif name == "no_contract":
        o.damage("A"); o.contracts_enabled = False
    elif name == "no_mutation":
        o.damage("A"); o.mutate_enabled = False
    elif name == "low_budget":
        o.damage("A")
    elif name == "no_valid_genome":
        o.damage("A"); o.max_candidates = 15
    elif name == "external_baseline":
        o.damage("A")
        # Explicit experimental external intervention, never counted internal.
        o.programs["A"] = decode_graph(15)
        o.external_repairs += 1
        o.history.append({"event": "external_restore", "target": "A"})
        return summarize(name, o)
    elif name == "idle":
        pass
    else:
        raise ValueError(name)
    VM(o).run()
    return summarize(name, o)


def summarize(case: str, o: Organism) -> dict[str, Any]:
    commits = [h for h in o.history if h["event"] == "internal_commit"]
    return {"scenario": case, "viable": o.viable(), "remaining_compute": o.compute,
            "internal_commits": len(commits), "external_repairs": o.external_repairs,
            "actors": [e["actor"] for e in commits],
            "repaired": {e["target"]: e["genome"] for e in commits},
            "events": o.history}

SCENARIOS = ("single_fault", "sequential_faults", "double_fault", "all_fault",
             "no_contract", "no_mutation", "low_budget", "no_valid_genome",
             "external_baseline", "idle")
