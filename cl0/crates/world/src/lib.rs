//! CL-0 Stage 1: deterministic mechanics only. No hardcoded organism, repair or fitness.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type ProcessId = u64;
pub type CellKey = i64;
pub const REGISTERS: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Op {
    Const { dst: usize, value: i64, next: usize },
    Add { dst: usize, lhs: usize, rhs: usize, next: usize },
    Sub { dst: usize, lhs: usize, rhs: usize, next: usize },
    Read { dst: usize, address: usize, next: usize },
    Write { address: usize, value: usize, next: usize },
    Branch { test: usize, nonzero: usize, zero: usize },
    PatchNext { node: usize, target: usize, next: usize },
    ReadNext { dst: usize, node: usize, next: usize },
    CloneNode { dst: usize, source: usize, next: usize },
    PatchConst { node: usize, value: usize, next: usize },
    Delete { address: usize, next: usize },
    Halt,
}

impl Op {
    fn next_mut(&mut self) -> Option<&mut usize> {
        match self {
            Op::Const { next, .. }
            | Op::Add { next, .. }
            | Op::Sub { next, .. }
            | Op::Read { next, .. }
            | Op::Write { next, .. }
            | Op::PatchNext { next, .. }
            | Op::ReadNext { next, .. }
            | Op::CloneNode { next, .. }
            | Op::PatchConst { next, .. }
            | Op::Delete { next, .. } => Some(next),
            Op::Branch { .. } | Op::Halt => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    pub nodes: Vec<Op>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Process {
    pub id: ProcessId,
    pub code: Program,
    pub pc: usize,
    pub registers: [i64; REGISTERS],
    pub credits: u64,
    pub stopped: bool,
}

impl Process {
    pub fn new(id: ProcessId, nodes: Vec<Op>, credits: u64) -> Self {
        Self {
            id,
            code: Program { nodes },
            pc: 0,
            registers: [0; REGISTERS],
            credits,
            stopped: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Origin {
    External,
    Internal(ProcessId),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    Loaded,
    Executed,
    Halted,
    Trapped(String),
    Intervened(String),
    InterventionRejected(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub tick: u64,
    pub origin: Origin,
    pub process: ProcessId,
    pub pc: usize,
    pub kind: EventKind,
    pub credits_after: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct World {
    processes: BTreeMap<ProcessId, Process>,
    cells: BTreeMap<CellKey, i64>,
    storage_limit: usize,
    max_nodes_per_process: usize,
    /// Optional common allocation cap: all executable nodes plus shared cells.
    total_storage_limit: Option<usize>,
    cursor: Option<ProcessId>,
    tick: u64,
    events: Vec<Event>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intervention {
    Disable { process: ProcessId },
    SetCredits { process: ProcessId, credits: u64 },
    SetCell { key: CellKey, value: i64 },
    PatchEdge { process: ProcessId, node: usize, next: usize },
}

impl World {
    pub fn new(storage_limit: usize, max_nodes_per_process: usize) -> Self {
        Self {
            processes: BTreeMap::new(),
            cells: BTreeMap::new(),
            storage_limit,
            max_nodes_per_process,
            total_storage_limit: None,
            cursor: None,
            tick: 0,
            events: Vec::new(),
        }
    }

    /// New Stage-3 worlds can enforce a shared allocation cap for code nodes and data cells.
    /// Legacy `new` keeps independent Stage-1 limits for experiment compatibility.
    pub fn with_total_capacity(storage_limit: usize, max_nodes_per_process: usize, total: usize) -> Self {
        let mut world = Self::new(storage_limit, max_nodes_per_process);
        world.total_storage_limit = Some(total);
        world
    }

    pub fn occupied_units(&self) -> usize {
        self.processes.values().fold(self.cells.len(), |used, p| used.saturating_add(p.code.nodes.len()))
    }

    pub fn total_capacity(&self) -> Option<usize> { self.total_storage_limit }

    fn can_allocate(&self, additional: usize) -> bool {
        self.total_storage_limit.is_none_or(|limit| {
            self.occupied_units().checked_add(additional).is_some_and(|new_total| new_total <= limit)
        })
    }

    /// External initial loading, intentionally distinct from internally executed CIL.
    pub fn load(&mut self, process: Process) -> Result<(), &'static str> {
        if self.processes.contains_key(&process.id) {
            return Err("duplicate process id");
        }
        if process.code.nodes.is_empty() || process.code.nodes.len() > self.max_nodes_per_process {
            return Err("invalid initial graph size");
        }
        if process.pc >= process.code.nodes.len() {
            return Err("invalid initial pc");
        }
        if !self.can_allocate(process.code.nodes.len()) {
            return Err("total storage capacity exhausted");
        }
        let id = process.id;
        let pc = process.pc;
        let credits = process.credits;
        self.processes.insert(id, process);
        self.events.push(Event {
            tick: self.tick,
            origin: Origin::External,
            process: id,
            pc,
            kind: EventKind::Loaded,
            credits_after: credits,
        });
        Ok(())
    }

    pub fn process(&self, id: ProcessId) -> Option<&Process> {
        self.processes.get(&id)
    }

    pub fn cell(&self, key: CellKey) -> Option<i64> {
        self.cells.get(&key).copied()
    }

    pub fn used_cells(&self) -> usize {
        self.cells.len()
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }

    pub fn remaining_credits(&self) -> u64 {
        self.processes.values().map(|p| p.credits).sum()
    }


    /// Applies a labeled experimenter action, never attributed to internal CIL.
    /// Rejected interventions are also observable. There is no host-side repair policy.
    pub fn intervene(&mut self, intervention: Intervention) -> Result<(), String> {
        let description = format!("{intervention:?}");
        let process = match &intervention {
            Intervention::Disable { process }
            | Intervention::SetCredits { process, .. }
            | Intervention::PatchEdge { process, .. } => *process,
            Intervention::SetCell { .. } => 0,
        };
        let result = self.intervene_inner(&intervention);
        self.tick += 1;
        self.events.push(Event {
            tick: self.tick,
            origin: Origin::External,
            process,
            pc: self.processes.get(&process).map_or(0, |p| p.pc),
            kind: match &result {
                Ok(()) => EventKind::Intervened(description),
                Err(reason) => EventKind::InterventionRejected(format!("{description}: {reason}")),
            },
            credits_after: self.processes.get(&process).map_or(0, |p| p.credits),
        });
        result
    }

    fn intervene_inner(&mut self, intervention: &Intervention) -> Result<(), String> {
        match *intervention {
            Intervention::Disable { process } => {
                let p = self.processes.get_mut(&process).ok_or("unknown process")?;
                p.stopped = true;
            }
            Intervention::SetCredits { process, credits } => {
                self.processes.get_mut(&process).ok_or("unknown process")?.credits = credits;
            }
            Intervention::SetCell { key, value } => {
                if !self.cells.contains_key(&key)
                    && (self.cells.len() >= self.storage_limit || !self.can_allocate(1))
                {
                    return Err("storage capacity exhausted".into());
                }
                self.cells.insert(key, value);
            }
            Intervention::PatchEdge { process, node, next } => {
                let p = self.processes.get_mut(&process).ok_or("unknown process")?;
                if node >= p.code.nodes.len() || next >= p.code.nodes.len() {
                    return Err("invalid graph node or edge".into());
                }
                let dest = p.code.nodes[node].next_mut().ok_or("no patchable edge")?;
                *dest = next;
            }
        }
        Ok(())
    }

    pub fn step(&mut self) -> bool {
        let next_id = self
            .processes
            .iter()
            .filter(|(&id, p)| !p.stopped && p.credits > 0 && self.cursor.is_none_or(|last| id > last))
            .map(|(&id, _)| id)
            .next()
            .or_else(|| {
                self.processes
                    .iter()
                    .find(|(_, p)| !p.stopped && p.credits > 0)
                    .map(|(&id, _)| id)
            });
        let Some(id) = next_id else {
            return false;
        };
        self.cursor = Some(id);
        let (pc, op) = {
            let p = self.processes.get_mut(&id).expect("selected process exists");
            p.credits -= 1; // every attempted instruction costs one credit, including traps
            (p.pc, p.code.nodes.get(p.pc).cloned())
        };
        let outcome = match op {
            Some(op) => self.execute(id, op),
            None => Err("invalid program counter".to_string()),
        };
        let p = self.processes.get_mut(&id).expect("selected process exists");
        let kind = match outcome {
            Ok(true) => {
                p.stopped = true;
                EventKind::Halted
            }
            Ok(false) => EventKind::Executed,
            Err(message) => {
                p.stopped = true;
                EventKind::Trapped(message)
            }
        };
        self.tick += 1;
        self.events.push(Event {
            tick: self.tick,
            origin: Origin::Internal(id),
            process: id,
            pc,
            kind,
            credits_after: p.credits,
        });
        true
    }

    /// Execute at most `limit` charged instructions. Returns the number of attempts.
    pub fn run(&mut self, limit: usize) -> usize {
        let mut steps = 0;
        while steps < limit && self.step() {
            steps += 1;
        }
        steps
    }

    /// Returns Ok(true) on halt; no instruction expresses semantic notions of repair.
    fn execute(&mut self, id: ProcessId, op: Op) -> Result<bool, String> {
        let can_add_unit = self.can_allocate(1);
        let (processes, cells) = (&mut self.processes, &mut self.cells);
        let p = processes.get_mut(&id).expect("selected process exists");
        let len = p.code.nodes.len();
        let reg = |i: usize| -> Result<i64, String> {
            p.registers
                .get(i)
                .copied()
                .ok_or_else(|| "invalid register".to_string())
        };
        let edge = |i: usize| -> Result<usize, String> {
            if i < len {
                Ok(i)
            } else {
                Err("invalid edge".to_string())
            }
        };
        // Validate before mutating any storage or program edge.
        match op {
            Op::Const { dst, value, next } => {
                edge(next)?;
                if dst >= REGISTERS { return Err("invalid register".into()); }
                p.registers[dst] = value;
                p.pc = next;
            }
            Op::Add { dst, lhs, rhs, next } => {
                edge(next)?;
                let a = reg(lhs)?;
                let b = reg(rhs)?;
                if dst >= REGISTERS { return Err("invalid register".into()); }
                p.registers[dst] = a.wrapping_add(b);
                p.pc = next;
            }
            Op::Sub { dst, lhs, rhs, next } => {
                edge(next)?;
                let a = reg(lhs)?;
                let b = reg(rhs)?;
                if dst >= REGISTERS { return Err("invalid register".into()); }
                p.registers[dst] = a.wrapping_sub(b);
                p.pc = next;
            }
            Op::Read { dst, address, next } => {
                edge(next)?;
                let key = reg(address)?;
                if dst >= REGISTERS { return Err("invalid register".into()); }
                p.registers[dst] = cells.get(&key).copied().unwrap_or(0);
                p.pc = next;
            }
            Op::Write { address, value, next } => {
                edge(next)?;
                let key = reg(address)?;
                let value = reg(value)?;
                if !cells.contains_key(&key) && (cells.len() >= self.storage_limit || !can_add_unit) {
                    return Err("storage capacity exhausted".into());
                }
                cells.insert(key, value);
                p.pc = next;
            }
            Op::Branch { test, nonzero, zero } => {
                let condition = reg(test)?;
                p.pc = edge(if condition != 0 { nonzero } else { zero })?;
            }
            Op::PatchNext { node, target, next } => {
                edge(next)?;
                let target_node = usize::try_from(reg(node)?).map_err(|_| "invalid node".to_string())?;
                let target_edge = usize::try_from(reg(target)?).map_err(|_| "invalid edge".to_string())?;
                edge(target_node)?;
                edge(target_edge)?;
                let destination = p.code.nodes[target_node]
                    .next_mut()
                    .ok_or_else(|| "node has no patchable edge".to_string())?;
                *destination = target_edge;
                p.pc = next;
            }
            Op::ReadNext { dst, node, next } => {
                edge(next)?;
                let index = usize::try_from(reg(node)?).map_err(|_| "invalid node".to_string())?;
                edge(index)?;
                if dst >= REGISTERS { return Err("invalid register".into()); }
                let edge_value = match &p.code.nodes[index] {
                    Op::Const { next, .. } | Op::Add { next, .. } | Op::Sub { next, .. }
                    | Op::Read { next, .. } | Op::Write { next, .. } | Op::PatchNext { next, .. }
                    | Op::ReadNext { next, .. } | Op::CloneNode { next, .. }
                    | Op::PatchConst { next, .. } | Op::Delete { next, .. } => *next,
                    Op::Branch { .. } | Op::Halt => return Err("node has no single next edge".into()),
                };
                p.registers[dst] = i64::try_from(edge_value).map_err(|_| "edge does not fit register".to_string())?;
                p.pc = next;
            }
            Op::CloneNode { dst, source, next } => {
                edge(next)?;
                let index = usize::try_from(reg(source)?).map_err(|_| "invalid node".to_string())?;
                edge(index)?;
                if dst >= REGISTERS { return Err("invalid register".into()); }
                if p.code.nodes.len() >= self.max_nodes_per_process || !can_add_unit {
                    return Err("code storage capacity exhausted".into());
                }
                let new_index = p.code.nodes.len();
                let encoded = i64::try_from(new_index).map_err(|_| "node index overflow".to_string())?;
                let new_node = p.code.nodes[index].clone();
                p.code.nodes.push(new_node);
                p.registers[dst] = encoded;
                p.pc = next;
            }
            Op::PatchConst { node, value, next } => {
                edge(next)?;
                let index = usize::try_from(reg(node)?).map_err(|_| "invalid node".to_string())?;
                edge(index)?;
                let new_value = reg(value)?;
                if let Op::Const { value: literal, .. } = &mut p.code.nodes[index] {
                    *literal = new_value;
                } else {
                    return Err("node is not a const".into());
                }
                p.pc = next;
            }
            Op::Delete { address, next } => {
                edge(next)?;
                let key = reg(address)?;
                cells.remove(&key);
                p.pc = next;
            }
            Op::Halt => return Ok(true),
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn writer(credits: u64) -> Process {
        Process::new(1, vec![
            Op::Const { dst: 0, value: 7, next: 1 },
            Op::Const { dst: 1, value: 42, next: 2 },
            Op::Write { address: 0, value: 1, next: 3 },
            Op::Halt,
        ], credits)
    }

    #[test]
    fn deterministic_trace_and_state() {
        let mut a = World::new(2, 16);
        a.load(writer(10)).unwrap();
        let mut b = a.clone();
        assert_eq!(a.run(100), 4);
        assert_eq!(b.run(100), 4);
        assert_eq!(a, b);
        assert_eq!(a.cell(7), Some(42));
        assert_eq!(a.remaining_credits(), 6);
    }

    #[test]
    fn zero_credit_and_inert_negative_controls() {
        let mut zero = World::new(1, 16);
        zero.load(writer(0)).unwrap();
        assert_eq!(zero.run(100), 0);
        assert_eq!(zero.used_cells(), 0);
        let mut inert = World::new(1, 16);
        inert.load(Process::new(5, vec![Op::Halt], 2)).unwrap();
        assert_eq!(inert.run(100), 1);
        assert_eq!(inert.used_cells(), 0);
        assert_eq!(inert.remaining_credits(), 1);
    }

    #[test]
    fn out_of_capacity_traps_without_write() {
        let mut world = World::new(0, 16);
        world.load(writer(10)).unwrap();
        assert_eq!(world.run(100), 3);
        assert_eq!(world.used_cells(), 0);
        assert!(matches!(world.events().last().unwrap().kind, EventKind::Trapped(_)));
        assert_eq!(world.remaining_credits(), 7);
    }

    #[test]
    fn invalid_edge_is_charged_and_stops() {
        let mut world = World::new(1, 16);
        world.load(Process::new(3, vec![Op::Const { dst: 0, value: 3, next: 9 }], 2)).unwrap();
        assert_eq!(world.run(100), 1);
        assert_eq!(world.process(3).unwrap().registers[0], 0);
        assert_eq!(world.remaining_credits(), 1);
        assert!(matches!(world.events().last().unwrap().kind, EventKind::Trapped(_)));
    }

    #[test]
    fn generic_graph_mutation_changes_control_flow() {
        let mut world = World::new(1, 16);
        world.load(Process::new(1, vec![
            Op::Const { dst: 0, value: 3, next: 1 }, // node to patch
            Op::Const { dst: 1, value: 5, next: 2 }, // new edge
            Op::PatchNext { node: 0, target: 1, next: 3 },
            Op::Const { dst: 2, value: 99, next: 4 }, // old edge
            Op::Halt,
            Op::Const { dst: 2, value: 17, next: 4 }, // new edge
        ], 10)).unwrap();
        assert_eq!(world.run(10), 6);
        assert_eq!(world.process(1).unwrap().registers[2], 17);
        assert!(matches!(world.process(1).unwrap().code.nodes[3], Op::Const { next: 5, .. }));
    }

    #[test]
    fn matched_external_intervention_is_attributed_and_causal() {
        let mut base = World::new(2, 16);
        base.load(writer(8)).unwrap();
        let mut normal = base.clone();
        let mut disabled = base;
        disabled.intervene(Intervention::Disable { process: 1 }).unwrap();
        normal.run(20);
        disabled.run(20);
        assert_eq!(normal.cell(7), Some(42));
        assert_eq!(disabled.cell(7), None);
        assert!(disabled.events().iter().any(|e| e.origin == Origin::External
            && matches!(e.kind, EventKind::Intervened(_))));
    }

    #[test]
    fn rejected_intervention_is_atomic_and_logged() {
        let mut world = World::new(0, 16);
        world.load(writer(8)).unwrap();
        assert!(world.intervene(Intervention::SetCell { key: 3, value: 9 }).is_err());
        assert_eq!(world.cell(3), None);
        assert!(matches!(world.events().last().unwrap().kind, EventKind::InterventionRejected(_)));
    }

    #[test]
    fn total_capacity_accounts_for_code_and_data_without_side_effects() {
        let mut world = World::with_total_capacity(4, 16, 4);
        world.load(writer(10)).unwrap();
        assert_eq!(world.occupied_units(), 4);
        assert_eq!(world.run(10), 3);
        assert_eq!(world.occupied_units(), 4);
        assert_eq!(world.cell(7), None);
        assert!(matches!(world.events().last().unwrap().kind, EventKind::Trapped(_)));
        assert!(world.load(Process::new(2, vec![Op::Halt], 1)).is_err());
    }

    #[test]
    fn clone_node_is_generic_and_resource_priced() {
        let mut world = World::with_total_capacity(2, 12, 8);
        world.load(Process::new(1, vec![
            Op::Const { dst: 0, value: 1, next: 1 },
            Op::CloneNode { dst: 1, source: 0, next: 2 },
            Op::ReadNext { dst: 2, node: 1, next: 3 },
            Op::Halt,
        ], 8)).unwrap();
        assert_eq!(world.occupied_units(), 4);
        assert_eq!(world.run(10), 4);
        assert_eq!(world.occupied_units(), 5);
        assert_eq!(world.process(1).unwrap().registers[1], 4);
        assert_eq!(world.process(1).unwrap().registers[2], 2);
        assert_eq!(world.process(1).unwrap().code.nodes[4], world.process(1).unwrap().code.nodes[1]);
        assert_eq!(world.remaining_credits(), 4);
    }

    #[test]
    fn no_allocation_space_blocks_clone_and_still_charges_instruction() {
        let mut world = World::with_total_capacity(2, 12, 3);
        world.load(Process::new(1, vec![
            Op::Const { dst: 0, value: 0, next: 1 },
            Op::CloneNode { dst: 1, source: 0, next: 2 },
            Op::Halt,
        ], 5)).unwrap();
        assert_eq!(world.run(10), 2);
        assert_eq!(world.occupied_units(), 3);
        assert_eq!(world.process(1).unwrap().code.nodes.len(), 3);
        assert_eq!(world.remaining_credits(), 3);
        assert!(matches!(world.events().last().unwrap().kind, EventKind::Trapped(_)));
    }

    #[test]
    fn delete_reclaims_cell_allocation() {
        let mut world = World::with_total_capacity(2, 12, 7);
        world.load(Process::new(1, vec![
            Op::Const { dst: 0, value: 9, next: 1 },
            Op::Const { dst: 1, value: 3, next: 2 },
            Op::Write { address: 0, value: 1, next: 3 },
            Op::Delete { address: 0, next: 4 },
            Op::Halt,
        ], 6)).unwrap();
        assert_eq!(world.occupied_units(), 5);
        assert_eq!(world.run(3), 3);
        assert_eq!(world.occupied_units(), 6);
        assert_eq!(world.run(3), 2);
        assert_eq!(world.occupied_units(), 5);
        assert_eq!(world.cell(9), None);
    }

    #[test]
    fn external_cell_intervention_respects_total_capacity() {
        let mut world = World::with_total_capacity(5, 10, 1);
        world.load(Process::new(1, vec![Op::Halt], 1)).unwrap();
        assert!(world.intervene(Intervention::SetCell { key: 1, value: 2 }).is_err());
        assert_eq!(world.occupied_units(), 1);
    }

    #[test]
    fn stable_round_robin_by_id() {
        let mut world = World::new(0, 2);
        world.load(Process::new(8, vec![Op::Const { dst: 0, value: 1, next: 1 }, Op::Halt], 2)).unwrap();
        world.load(Process::new(2, vec![Op::Const { dst: 0, value: 1, next: 1 }, Op::Halt], 2)).unwrap();
        assert_eq!(world.run(4), 4);
        let ids: Vec<_> = world.events().iter().filter(|e| e.origin != Origin::External).map(|e| e.process).collect();
        assert_eq!(ids, vec![2, 8, 2, 8]);
    }
}
