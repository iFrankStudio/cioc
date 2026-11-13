"""V0.15: executable ecology with costly, damageable maintenance programs.

Generic substrate interprets opcodes and accounts resources. Organisms own two
mutable tapes: metabolism/reproduction and diagnosis/reconfiguration. Repair
search evaluates net resource yield, NOT an experimenter-provided canonical gene.
The audit/rebuild primitives, candidate alphabet and reproduction rules remain
host-supplied. Thus this is not full autonomy or organizational closure.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from collections import Counter
from itertools import product
import random

BODY_OPS=('EAT_A','EAT_B','WAIT','DIVIDE')
CONTROL_OPS=('AUDIT','REBUILD','WAIT')
BODY_INIT=('EAT_A','EAT_A','DIVIDE')
CONTROL_INIT=('AUDIT','REBUILD')

@dataclass
class Organism:
    uid: int
    body: tuple[str,...]=BODY_INIT
    controller: tuple[str,...]=CONTROL_INIT
    anchor: str|None='DIVIDE'  # a partial fragment, never a complete body backup
    energy: int=45
    parent: int|None=None
    flagged: bool=False
    repair_count: int=0

@dataclass
class World:
    seed:int=7
    capacity:int=100
    supply_a:int=150
    supply_b:int=8
    mutation_rate:float=0.02
    controller_mutation_rate:float=0.002
    damage_rate:float=0.01
    repair_enabled:bool=True
    organisms:dict[int,Organism]=field(default_factory=dict)
    next_uid:int=0
    ticks:int=0
    repairs:int=0
    births:int=0
    deaths:int=0
    faults:int=0
    external_repairs:int=0
    records:list[dict]=field(default_factory=list)

    def __post_init__(self):self.rng=random.Random(self.seed)

    def insert(self,body=BODY_INIT,controller=CONTROL_INIT,energy=45,anchor='DIVIDE'):
        if len(self.organisms)>=self.capacity: return None
        if any(op not in BODY_OPS for op in body) or any(op not in CONTROL_OPS for op in controller):
            raise ValueError('unrecognized opcode')
        uid=self.next_uid;self.next_uid+=1
        self.organisms[uid]=Organism(uid,tuple(body),tuple(controller),anchor,energy)
        return uid

    def mutate(self,code,alphabet,rate):
        if self.rng.random()>=rate:return code
        pos=self.rng.randrange(len(code))
        options=tuple(op for op in alphabet if op != code[pos])
        return code[:pos]+(self.rng.choice(options),)+code[pos+1:]

    def corrupt(self,uid,*,body=None,controller=None):
        o=self.organisms[uid]
        if body is not None:o.body=tuple(body)
        if controller is not None:o.controller=tuple(controller)

    @staticmethod
    def preview(code,available):
        """Finite trial of metabolic effect. Does not consume actual substrate."""
        if not code:return -99
        return sum(min(5,available[op[-1]]) if op in ('EAT_A','EAT_B') else 0 for op in code)-len(code)

    def recover(self,o,available):
        if not self.repair_enabled or o.anchor != 'DIVIDE':return False
        # Search is expressed by organ-owned REBUILD, but enumeration and
        # hypothetical execution are host-provided fixed primitive semantics.
        best=None;best_value=0
        for prefix in product(('EAT_A','EAT_B','WAIT'),repeat=2):
            o.energy-=1  # trial computation is expensive
            if o.energy<=0:return False
            candidate=prefix+(o.anchor,)
            value=self.preview(candidate,available)
            if value>best_value:
                best_value=value;best=candidate
        if best is None:return False
        o.energy-=3 # atomic write charge
        if o.energy<=0:return False
        o.body=best;o.repair_count+=1;self.repairs+=1;o.flagged=False
        return True

    def step(self):
        self.ticks+=1
        resources={'A':self.supply_a,'B':self.supply_b}
        ids=list(self.organisms);self.rng.shuffle(ids)
        newborn=[]
        for uid in ids:
            o=self.organisms.get(uid)
            if o is None:continue
            if self.rng.random()<self.damage_rate:
                # Untrusted memory mutation not delivered as a fault flag.
                pos=self.rng.randrange(len(o.body));o.body=o.body[:pos]+('WAIT',)+o.body[pos+1:]
                self.faults+=1
            # Organism-owned maintenance tape is executed by generic VM.
            o.flagged=False
            for op in o.controller:
                o.energy-=1
                if o.energy<=0:break
                if op=='AUDIT':
                    if (self.preview(o.body,resources)<=0 or 'DIVIDE' not in o.body):o.flagged=True
                elif op=='REBUILD' and o.flagged:
                    self.recover(o,resources)
            if o.energy<=0:
                del self.organisms[uid];self.deaths+=1;continue
            for op in o.body:
                o.energy-=1
                if o.energy<=0:break
                if op in ('EAT_A','EAT_B'):
                    key=op[-1];yielded=min(5,resources[key]);resources[key]-=yielded;o.energy+=yielded
                elif op=='DIVIDE' and o.energy>=36 and len(self.organisms)+len(newborn)<self.capacity:
                    o.energy-=17
                    child=Organism(self.next_uid,
                        self.mutate(o.body,BODY_OPS,self.mutation_rate),
                        self.mutate(o.controller,CONTROL_OPS,self.controller_mutation_rate),
                        o.anchor,17,uid)
                    self.next_uid+=1;newborn.append(child)
            if o.energy<=0:
                del self.organisms[uid];self.deaths+=1
        for child in newborn:self.organisms[child.uid]=child;self.births+=1
        counts=Counter('repair_capable' if 'AUDIT' in o.controller and 'REBUILD' in o.controller else 'other'
                       for o in self.organisms.values())
        datum={'tick':self.ticks,'alive':len(self.organisms),'births':self.births,
               'deaths':self.deaths,'repairs':self.repairs,'faults':self.faults,
               'repair_capable':counts.get('repair_capable',0),
               'other':counts.get('other',0),
               'resources_left':resources}
        self.records.append(datum)
        return datum

    def run(self,n):
        for _ in range(n):self.step()
        return self.records

    def switch(self,a,b):self.supply_a=a;self.supply_b=b


def scenarios():
    result={}
    w=World(damage_rate=0,mutation_rate=0);uid=w.insert()
    w.corrupt(uid,body=('WAIT','WAIT','DIVIDE'));w.step()
    result['single_body_repair']={'repaired':w.repairs,'body':list(w.organisms[uid].body),
                                  'energy':w.organisms[uid].energy}
    w=World(damage_rate=0,repair_enabled=False);uid=w.insert();w.corrupt(uid,body=('WAIT','WAIT','DIVIDE'))
    w.run(25);result['no_repair']={'alive':len(w.organisms),'repairs':w.repairs}
    w=World(damage_rate=0);uid=w.insert();w.corrupt(uid,body=('WAIT','WAIT','DIVIDE'),controller=('WAIT','WAIT'))
    w.run(25);result['damaged_controller']={'alive':len(w.organisms),'repairs':w.repairs}
    w=World(damage_rate=0);uid=w.insert(anchor=None);w.corrupt(uid,body=('WAIT','WAIT','DIVIDE'))
    w.run(25);result['missing_anchor']={'alive':len(w.organisms),'repairs':w.repairs}
    w=World(damage_rate=0,mutation_rate=0);uid=w.insert();w.run(12);w.switch(0,150);w.run(20)
    result['resource_switch']={'alive':len(w.organisms),'repairs':w.repairs,
                               'B_bodies':sum('EAT_B' in o.body for o in w.organisms.values())}
    for label,rate in [('without_damage',0),('damage_with_repair',0.07)]:
        w=World(seed=11,damage_rate=rate);w.insert();w.run(80)
        result[label]={'alive':len(w.organisms),'repairs':w.repairs,'births':w.births,'faults':w.faults}
    w=World(seed=11,damage_rate=0.07,repair_enabled=False);w.insert();w.run(80)
    result['damage_no_repair']={'alive':len(w.organisms),'repairs':w.repairs,'births':w.births,'faults':w.faults}
    return result
