"""V0.16: combine native resource ecology and mutual maintenance of two controllers.

One controller can recover the other from partial shared fragment and bounded
behavioral search. Both controllers damaged is a demonstrated cut set. The host
still supplies canary probes, scheduling, interpreter, copying and atomic write.
"""
from __future__ import annotations
import importlib.util
from pathlib import Path
import sys
from dataclasses import dataclass, field

basefile=Path(__file__).resolve().parent.parent/'v0.15'/'cioc.py'
if not basefile.is_file():raise RuntimeError('V0.16 requires experiments/v0.15/cioc.py')
spec=importlib.util.spec_from_file_location('cioc_v15_ecology',basefile)
assert spec and spec.loader
base=importlib.util.module_from_spec(spec);sys.modules[spec.name]=base;spec.loader.exec_module(base)


def controller_probe(code:tuple[str,...]|None,broken:bool)->bool:
    """Finite two-stage check: audit (observation), then rebuild (reaction)."""
    if code is None:return False
    flag=False
    for opcode in code:
        if opcode=='AUDIT':flag=broken
        if opcode=='REBUILD' and flag:return True
    return False


def controller_valid(code:tuple[str,...]|None)->bool:
    return controller_probe(code,True) and not controller_probe(code,False)

@dataclass
class MutualWorld(base.World):
    secondary:dict[int,tuple[str,...]|None]=field(default_factory=dict)
    fragments:dict[int,str|None]=field(default_factory=dict)
    controller_repairs:int=0
    peer_checks:int=0
    disable_peer_repair:bool=False

    def insert(self,body=base.BODY_INIT,controller=base.CONTROL_INIT,energy=45,anchor='DIVIDE',secondary=('AUDIT','REBUILD','WAIT'),fragment='AUDIT'):
        uid=super().insert(body,controller,energy,anchor)
        if uid is not None:
            self.secondary[uid]=tuple(secondary) if secondary is not None else None
            self.fragments[uid]=fragment
        return uid

    def corrupt_controller(self,uid,which):
        if which=='primary':self.organisms[uid].controller=('WAIT','WAIT')
        elif which=='secondary':self.secondary[uid]=('WAIT','WAIT')
        else:raise ValueError(which)

    def _peer_repair(self,agent,target,checker):
        if self.disable_peer_repair or not controller_valid(checker):return False
        fragment=self.fragments.get(agent.uid)
        if fragment!='AUDIT':return False
        # Bounded finite search; consumes substrate energy for each trial.
        for tail in base.CONTROL_OPS:
            agent.energy-=1
            if agent.energy<=0:return False
            candidate=(fragment,tail)
            if controller_valid(candidate):
                agent.energy-=4
                if agent.energy<=0:return False
                if target=='primary':agent.controller=candidate
                else:self.secondary[agent.uid]=candidate
                self.controller_repairs+=1
                return True
        return False

    def step(self):
        # World schedules executable peer checks. Neither faults nor target
        # identity are given to the candidate programs as privileged input.
        for uid,agent in list(self.organisms.items()):
            peer=self.secondary.get(uid)
            primary=agent.controller
            self.peer_checks+=2
            if not controller_valid(primary) and controller_valid(peer):
                self._peer_repair(agent,'primary',peer)
            elif not controller_valid(peer) and controller_valid(primary):
                self._peer_repair(agent,'secondary',primary)
        before=set(self.organisms)
        datum=super().step()
        # Secondary program is transmitted on reproduction through ancestor.
        for uid in set(self.organisms)-before:
            child=self.organisms[uid]
            parent=child.parent
            self.secondary[uid]=self.secondary.get(parent,('AUDIT','REBUILD','WAIT'))
            self.fragments[uid]=self.fragments.get(parent,'AUDIT')
        for uid in before-set(self.organisms):
            self.secondary.pop(uid,None);self.fragments.pop(uid,None)
        datum['controller_repairs']=self.controller_repairs
        datum['peer_checks']=self.peer_checks
        return datum


def scenarios():
    result={}
    for which in ('primary','secondary'):
        w=MutualWorld(damage_rate=0,mutation_rate=0);uid=w.insert()
        w.corrupt_controller(uid,which);w.step()
        result[f'loss_{which}']={'controller_repairs':w.controller_repairs,
                                  'healthy':controller_valid(w.organisms[uid].controller) and controller_valid(w.secondary[uid])}
    w=MutualWorld(damage_rate=0,mutation_rate=0);uid=w.insert()
    w.corrupt_controller(uid,'primary');w.corrupt_controller(uid,'secondary');w.run(30)
    result['both_loss']={'controller_repairs':w.controller_repairs,'alive':len(w.organisms)}
    w=MutualWorld(damage_rate=0);uid=w.insert();w.corrupt(uid,body=('WAIT','WAIT','DIVIDE'))
    w.corrupt_controller(uid,'primary');w.step()
    result['primary_then_body']={'controller_repairs':w.controller_repairs,'body_repairs':w.repairs}
    w=MutualWorld(damage_rate=0,disable_peer_repair=True);uid=w.insert();w.corrupt_controller(uid,'primary');w.step()
    result['peer_disabled']={'controller_repairs':w.controller_repairs}
    w=MutualWorld(damage_rate=0);uid=w.insert(fragment=None);w.corrupt_controller(uid,'primary');w.step()
    result['fragment_missing']={'controller_repairs':w.controller_repairs}
    w=MutualWorld(damage_rate=0);uid=w.insert(energy=3);w.corrupt_controller(uid,'primary');w.step()
    result['low_energy']={'controller_repairs':w.controller_repairs}
    return result
