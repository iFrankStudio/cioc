"""CIOC V0.11: finite live-corruption diagnosis built on the frozen V0.10 world.

Host-owned: scheduler, interpreter, finite contract fixtures, resource accounting,
atomic installation. Organism-owned: damageable diagnostic policy and role assets.
This is a toy causal-intervention benchmark, not an autonomous life claim.
"""
from __future__ import annotations

from dataclasses import dataclass
import importlib.util
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent / 'v0.10' / 'cioc.py'
if not BASE.is_file():
    raise RuntimeError('CIOC V0.11 requires experiments/v0.10/cioc.py')
spec = importlib.util.spec_from_file_location('cioc_v10_base', BASE)
assert spec and spec.loader
base = importlib.util.module_from_spec(spec)
import sys
sys.modules[spec.name] = base
spec.loader.exec_module(base)
ROLES, PROTOCOLS = base.ROLES, base.PROTOCOLS
probe, expected = base.probe, base.expected
protocol_contract = base.protocol_contract
DIAGNOSTIC_CANONICAL = ('AUDIT', 'QUARANTINE')


@dataclass
class DiagnosticWorld(base.World):
    diagnostic_code: tuple[str, ...] | None = DIAGNOSTIC_CANONICAL
    diagnostic_enabled: bool = True
    audits: int = 0
    detections: int = 0
    audit_targets: tuple[str, ...] = ROLES + PROTOCOLS

    def damage(self, *names: str):
        for name in names:
            if name == 'D':
                self.diagnostic_code = None
                self.record('damage', role='D')
            else:
                super().damage(name)

    def corrupt(self, target: str, code: tuple[str, ...]):
        if target == 'D':
            self.diagnostic_code = code
        elif target in PROTOCOLS:
            self.protocols[target] = code
        elif target in ROLES:
            self.roles[target] = code
        else:
            raise ValueError(target)
        self.record('corruption', target=target, code=list(code))

    def get_code(self, target):
        return self.protocols[target] if target in PROTOCOLS else self.roles[target]

    def valid(self, target: str, code: tuple[str, ...] | None) -> bool:
        if code is None:
            return False
        if target in PROTOCOLS:
            return protocol_contract(target, code)
        if target in ROLES:
            return (probe(target, code, True) == expected(target, True)
                    and probe(target, code, False) == expected(target, False))
        raise ValueError(target)

    def diagnostic_functional(self):
        code = self.diagnostic_code
        return (code is not None and 'AUDIT' in code and 'QUARANTINE' in code
                and code.index('AUDIT') < code.index('QUARANTINE')
                and all(op in ('AUDIT', 'QUARANTINE', 'NOOP') for op in code))

    def audit(self):
        """Execute damageable organism diagnostic policy through fixed host probes."""
        if not self.diagnostic_enabled or not self.diagnostic_functional():
            return False
        evidence = []
        progress = False
        for op in self.diagnostic_code:
            if op == 'AUDIT':
                evidence = []
                for target in self.audit_targets:
                    if not self.spend(2):
                        return progress
                    self.audits += 1
                    code = self.get_code(target)
                    if code is not None and not self.valid(target, code):
                        evidence.append(target)
                        self.record('diagnostic_failure', target=target)
            elif op == 'QUARANTINE':
                for target in evidence:
                    if not self.spend():
                        return progress
                    if target in PROTOCOLS:
                        self.protocols[target] = None
                    else:
                        self.roles[target] = None
                    self.detections += 1
                    self.record('quarantine', target=target)
                    progress = True
                evidence = []
        return progress

    def healthy(self):
        # Host-owned experimental success oracle, never used for repair actions.
        return (self.diagnostic_functional() and
                all(self.valid(name, self.get_code(name)) for name in self.audit_targets))

    def tick(self):
        detected = self.audit()
        return super().tick() or detected

    def run(self, cycles=100):
        for _ in range(cycles):
            if self.healthy():
                self.audit()  # positive/false-alarm control on a healthy state
                return self.healthy()
            if not self.tick():
                break
        return self.healthy()


def scenarios():
    out = {}
    for name in ROLES + PROTOCOLS:
        w = DiagnosticWorld(); w.corrupt(name, ('NOOP',) * 3)
        out['corrupt_' + name] = dict(recovered=w.run(), internal=w.internal_repairs,
                                       detections=w.detections, audits=w.audits)
    equivalent = {'G': ('ENUMERATE', 'PROPOSE_ALT', 'DELEGATE'),
                  'V': ('TEST', 'CERTIFY_ALT', 'IDLE'),
                  'M': ('CHECK', 'COMMIT_ALT', 'IDLE'),
                  'PV': ('POS_O', 'NEG_M_ALT', 'CERTIFY_P'),
                  'PI': ('VOTE_V', 'PERMIT_G_ALT', 'INSTALL_P')}
    for name, code in equivalent.items():
        w = DiagnosticWorld(); w.corrupt(name, code)
        out['equivalent_' + name] = dict(healthy=w.run(), internal=w.internal_repairs,
                                           detections=w.detections, audits=w.audits)
    w = DiagnosticWorld(diagnostic_enabled=False); w.corrupt('PV', ('NOOP',) * 3)
    out['without_diagnostics'] = dict(recovered=w.run(), internal=w.internal_repairs)
    w = DiagnosticWorld(budget=3); w.corrupt('V', ('NOOP',) * 3)
    out['low_budget'] = dict(recovered=w.run(), internal=w.internal_repairs)
    w = DiagnosticWorld(); w.corrupt('D', ('NOOP', 'NOOP')); w.corrupt('V', ('NOOP',) * 3)
    out['broken_diagnostics'] = dict(recovered=w.run(), internal=w.internal_repairs)
    w = DiagnosticWorld(); w.corrupt('V', ('NOOP',) * 3); first = w.run()
    w.corrupt('M', ('NOOP',) * 3); second = w.run()
    out['sequential_V_M'] = dict(first=first, second=second,
                                   internal=w.internal_repairs, detections=w.detections)
    w = DiagnosticWorld(); w.corrupt('PV', ('NOOP',) * 3); w.corrupt('V', ('NOOP',) * 3)
    out['joint_PV_V'] = dict(recovered=w.run(), internal=w.internal_repairs)
    w = DiagnosticWorld(); w.corrupt('V', ('NOOP',) * 3); w.external_restore('V')
    out['external'] = dict(recovered=w.run(), internal=w.internal_repairs,
                           external=w.external_repairs)
    return out
