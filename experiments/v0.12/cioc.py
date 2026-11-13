"""CIOC V0.12: two distinct, damageable diagnostic policies cross-check each other.

This is a finite experiment, not complete organizational closure. The host provides
canary fixtures, finite program interpretation, scheduling, budget, and atomic
installation. A peer's code is assessed by *executing* it on canaries; the host
does not compare to the original diagnostic program during repair decisions.
"""
from __future__ import annotations

from dataclasses import dataclass, field
import importlib.util
from pathlib import Path
import sys
from itertools import product

BASE = Path(__file__).resolve().parent.parent / 'v0.11' / 'cioc.py'
if not BASE.is_file():
    raise RuntimeError('V0.12 requires experiments/v0.11/cioc.py and v0.10/cioc.py')
spec = importlib.util.spec_from_file_location('cioc_v11_base', BASE)
assert spec and spec.loader
base = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = base
spec.loader.exec_module(base)

DIAGNOSTICS = ('DR', 'DP')  # DR monitors roles, DP monitors protocols
DEFAULT = {'DR': ('SCAN_ROLE', 'REPORT'), 'DP': ('SCAN_PROTOCOL', 'REPORT')}
ALPHABET = ('NOOP', 'REPORT_ALT', 'REPORT', 'DROP', 'SCAN_ROLE', 'SCAN_PROTOCOL')
FAMILY = {'DR': 'role', 'DP': 'protocol'}


def interpret(code: tuple[str, ...] | None, family: str, faulty: bool) -> tuple[str, ...]:
    """Host VM primitive; no role identity or canonical source comparison.

    Canary is a bool indicating a controlled fixture's *observable* fault.
    A diagnostic must report once on the faulty canary and never on healthy.
    """
    if code is None:
        return ()
    evidence = False
    output = []
    for op in code:
        if op == 'DROP':
            evidence = False
            output.clear()
        elif op == 'SCAN_ROLE' and family == 'role':
            evidence = bool(faulty)
        elif op == 'SCAN_PROTOCOL' and family == 'protocol':
            evidence = bool(faulty)
        elif op in ('REPORT', 'REPORT_ALT') and evidence:
            output.append('alert')
    return tuple(output)


def challenges(code: tuple[str, ...] | None, family: str) -> bool:
    """Two finite black-box challenge fixtures, not source equality."""
    return interpret(code, family, True) == ('alert',) and interpret(code, family, False) == ()


@dataclass
class TwinWorld(base.DiagnosticWorld):
    diagnostics: dict[str, tuple[str, ...] | None] = field(default_factory=lambda: dict(DEFAULT))
    # O knows the first opcode only, not a complete diagnostic program.
    diagnostic_fragments: dict[str, str] = field(default_factory=lambda: {'DR': 'SCAN_ROLE', 'DP': 'SCAN_PROTOCOL'})
    repairs_diagnostics: int = 0
    challenges_run: int = 0
    cross_detections: int = 0
    allow_peer_repair: bool = True
    # D inherited from V0.11 is disabled; twin policies replace it.
    diagnostic_enabled: bool = False

    def damage(self, *names: str):
        for name in names:
            if name in DIAGNOSTICS:
                self.diagnostics[name] = None
                self.record('diagnostic_loss', target=name)
            else:
                super().damage(name)

    def corrupt(self, target: str, code: tuple[str, ...]):
        if target in DIAGNOSTICS:
            self.diagnostics[target] = code
            self.record('diagnostic_corruption', target=target, code=list(code))
        else:
            super().corrupt(target, code)

    def _infrastructure_available(self) -> bool:
        # These capabilities are needed to propose, verify and commit a new
        # diagnostic program. Explicit dependency, not autonomous discovery.
        return (self.roles.get('O') is not None and
                self.roles.get('G') is not None and
                self.roles.get('V') is not None and
                self.roles.get('M') is not None and
                all(base.probe(r, self.roles[r], True) == base.expected(r, True)
                    for r in ('O', 'G', 'V', 'M')))

    def _peer_checks(self, name: str) -> bool | None:
        peer = 'DP' if name == 'DR' else 'DR'
        peer_code = self.diagnostics[peer]
        if peer_code is None or not self.spend(4):
            return None
        self.challenges_run += 2
        # Only a functioning peer may judge the target. No host-level oracle
        # diagnoses the target in tick(): it measures the peer's canary effects.
        if not challenges(peer_code, FAMILY[peer]):
            return None
        good = challenges(self.diagnostics[name], FAMILY[name])
        self.record('cross_check', observer=peer, target=name, passed=good)
        if not good:
            self.cross_detections += 1
        return good

    def _restore_diagnostic(self, target: str) -> bool:
        if not self.allow_peer_repair or not self._infrastructure_available():
            return False
        prefix = self.diagnostic_fragments.get(target)
        if prefix is None:
            return False
        for tail in ALPHABET:
            if not self.spend(1):
                return False
            code = (prefix, tail)
            # Surviving peer executes challenges; V verifies their outcomes;
            # M authorizes an atomic replacement. This is still a predesigned
            # host workflow, explicitly outside the strong closure claim.
            if challenges(code, FAMILY[target]):
                if not self.spend(5):
                    return False
                self.diagnostics[target] = code
                self.repairs_diagnostics += 1
                self.record('internal_diagnostic_install', target=target, program=list(code))
                return True
        return False

    def audit_twins(self) -> bool:
        progress = False
        for target in DIAGNOSTICS:
            state = self._peer_checks(target)
            if state is False:
                self.diagnostics[target] = None
                self.record('diagnostic_quarantine', target=target)
                progress = True
                # Do not allow untrusted target code to participate after isolation.
                if self._restore_diagnostic(target):
                    progress = True
        for name, family, targets in (
            ('DR', 'role', base.ROLES), ('DP', 'protocol', base.PROTOCOLS)
        ):
            code = self.diagnostics[name]
            if code is None or not challenges(code, family):
                continue
            for target in targets:
                if not self.spend(2):
                    return progress
                self.audits += 1
                current = self.get_code(target)
                if current is not None and not self.valid(target, current):
                    if self.spend(1):
                        if target in base.ROLES:
                            self.roles[target] = None
                        else:
                            self.protocols[target] = None
                        self.detections += 1
                        self.record('functional_quarantine', auditor=name, target=target)
                        progress = True
        return progress

    def tick(self):
        detected = self.audit_twins()
        # V0.11 inherited tick would audit D; D has been disabled above.
        return base.DiagnosticWorld.tick(self) or detected

    def healthy(self):
        return (all(challenges(self.diagnostics[d], FAMILY[d]) for d in DIAGNOSTICS)
                and all(self.valid(n, self.get_code(n)) for n in base.ROLES + base.PROTOCOLS))

    def run(self, cycles=100):
        for _ in range(cycles):
            if self.healthy():
                self.audit_twins()  # healthy false-alarm control
                return self.healthy()
            if not self.tick():
                break
        return self.healthy()


def scenarios():
    results = {}
    for name in DIAGNOSTICS:
        for fault in ('loss', 'corrupt'):
            w = TwinWorld()
            if fault == 'loss':
                w.damage(name)
            else:
                w.corrupt(name, ('NOOP', 'NOOP'))
            results[f'{fault}_{name}'] = dict(recovered=w.run(), internal=w.repairs_diagnostics,
                                              cross_detections=w.cross_detections)
    for name in ('G', 'V', 'M', 'PV', 'PI', 'O'):
        w = TwinWorld(); w.corrupt(name, ('NOOP',) * 3)
        results['corrupt_' + name] = dict(recovered=w.run(), internal=w.internal_repairs,
                                         detections=w.detections)
    w = TwinWorld(); w.damage(*DIAGNOSTICS)
    results['loss_both'] = dict(recovered=w.run(), internal=w.repairs_diagnostics)
    w = TwinWorld(); w.damage('DR'); w.corrupt('DP', ('NOOP', 'NOOP'))
    results['corrupt_peer'] = dict(recovered=w.run(), internal=w.repairs_diagnostics)
    w = TwinWorld(); w.damage('DR'); w.diagnostic_fragments.pop('DR')
    results['missing_fragment'] = dict(recovered=w.run(), internal=w.repairs_diagnostics)
    w = TwinWorld(budget=4); w.damage('DR')
    results['low_budget'] = dict(recovered=w.run(), internal=w.repairs_diagnostics)
    w = TwinWorld(allow_peer_repair=False); w.damage('DP')
    results['disable_peer_repair'] = dict(recovered=w.run(), internal=w.repairs_diagnostics)
    w = TwinWorld(); w.corrupt('DR', ('SCAN_ROLE', 'REPORT_ALT'))
    results['equivalent'] = dict(healthy=w.run(), internal=w.repairs_diagnostics,
                                  detections=w.cross_detections)
    w = TwinWorld(); w.damage('DR'); first = w.run(); w.damage('DP'); second = w.run()
    results['sequential_DR_DP'] = dict(first=first, second=second, internal=w.repairs_diagnostics)
    w = TwinWorld(); w.damage('DR'); w.external_restore('G')
    results['external_unrelated'] = dict(recovered=w.run(), diagnostic_repairs=w.repairs_diagnostics,
                                          external=w.external_repairs)
    return results
