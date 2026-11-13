"""V0.14 native-resource ecology: replicating mutable instruction tapes.

No evaluator provides fitness or reproduction decisions. The host runs a fixed
minimal interpreter and bounded resources. Observed persistence or adaptation
is not a claim of artificial life or open-ended evolution.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from collections import Counter
import random

OPS=('EAT_A','EAT_B','DIVIDE','WAIT')

@dataclass
class Individual:
    uid: int
    code: tuple[str,...]
    energy: int=16
    parent: int|None=None
    born: int=0

@dataclass
class Ecology:
    seed: int=42
    capacity: int=80
    mutation_rate: float=0.12
    a_supply: int=120
    b_supply: int=8
    price: int=1
    transfer: int=12
    alive: dict[int,Individual]=field(default_factory=dict)
    tick_no: int=0
    next_uid: int=0
    births: int=0
    deaths: int=0
    mutations: int=0
    external_insertions: int=0
    stats: list[dict]=field(default_factory=list)

    def __post_init__(self): self.rng=random.Random(self.seed)

    def insert(self, code: tuple[str,...], energy=16, *, external=True, parent=None):
        if len(self.alive)>=self.capacity or not code or any(op not in OPS for op in code):
            return None
        uid=self.next_uid;self.next_uid+=1
        self.alive[uid]=Individual(uid,code,energy,parent,self.tick_no)
        if external:self.external_insertions+=1
        return uid

    def mutate(self, code):
        if self.rng.random()>=self.mutation_rate:return code
        i=self.rng.randrange(len(code))
        choices=tuple(op for op in OPS if op!=code[i])
        altered=code[:i]+(self.rng.choice(choices),)+code[i+1:]
        self.mutations+=1
        return altered

    def step(self):
        self.tick_no+=1
        resources={'A':self.a_supply,'B':self.b_supply}
        ids=list(self.alive)
        self.rng.shuffle(ids)
        newborn=[]
        for uid in ids:
            agent=self.alive.get(uid)
            if agent is None:continue
            for op in agent.code:
                agent.energy-=self.price  # computation consumes energy, including waiting
                if agent.energy<=0:break
                if op in ('EAT_A','EAT_B'):
                    key=op[-1];take=min(5,resources[key]);resources[key]-=take;agent.energy+=take
                elif op=='DIVIDE' and agent.energy>=2*self.transfer and len(self.alive)+len(newborn)<self.capacity:
                    agent.energy-=self.transfer
                    child=Individual(self.next_uid,self.mutate(agent.code),self.transfer,uid,self.tick_no)
                    self.next_uid+=1;newborn.append(child)
            if agent.energy<=0:
                del self.alive[uid];self.deaths+=1
        for child in newborn:
            self.alive[child.uid]=child;self.births+=1
        pop=len(self.alive)
        types=Counter('A' if x.code.count('EAT_A')>x.code.count('EAT_B') else
                      'B' if x.code.count('EAT_B')>x.code.count('EAT_A') else 'mixed'
                      for x in self.alive.values())
        datum={'tick':self.tick_no,'alive':pop,'births':self.births,'deaths':self.deaths,
               'types':dict(types),'energy':sum(a.energy for a in self.alive.values()),
               'resources_left':resources.copy()}
        self.stats.append(datum)
        return datum

    def run(self,steps):
        for _ in range(steps):self.step()
        return self.stats

    def switch(self,a,b):
        self.a_supply=a;self.b_supply=b


def scenarios():
    out={}
    w=Ecology(mutation_rate=0)
    w.insert(('EAT_A','EAT_A','DIVIDE'))
    w.run(90);out['replication']={'population':len(w.alive),'births':w.births,'external_insertions':w.external_insertions}
    w=Ecology(mutation_rate=0)
    w.insert(('WAIT','WAIT','DIVIDE'))
    w.run(60);out['no_food']={'population':len(w.alive),'births':w.births}
    w=Ecology(mutation_rate=0,seed=3)
    w.insert(('EAT_A','EAT_A','DIVIDE'))
    w.run(30);w.switch(0,120);w.run(50)
    out['switch_no_mutation']={'alive':len(w.alive),'deaths':w.deaths}
    w=Ecology(mutation_rate=0.25,seed=3)
    w.insert(('EAT_A','EAT_A','DIVIDE'))
    w.run(30);w.switch(0,120);w.run(50)
    out['switch_with_mutation']={'alive':len(w.alive),'births':w.births,'mutations':w.mutations,
                                 'B_type':w.stats[-1]['types'].get('B',0)}
    w=Ecology(mutation_rate=0,seed=3)
    w.insert(('EAT_A','EAT_A','DIVIDE'))
    w.insert(('EAT_B','EAT_B','DIVIDE'))
    w.run(30);w.switch(0,120);w.run(50)
    out['switch_with_initial_diversity']={'alive':len(w.alive),'B_type':w.stats[-1]['types'].get('B',0)}
    return out
