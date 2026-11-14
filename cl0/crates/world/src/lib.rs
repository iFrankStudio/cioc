//! CL-0 Stage 1: deterministic mechanics only. No hardcoded organism, repair or fitness.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub type ProcessId = u64;
pub type CellKey = i64;
pub const REGISTERS: usize = 4;
pub const WORLD_FORMAT_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Op {
    Const {
        dst: usize,
        value: i64,
        next: usize,
    },
    Add {
        dst: usize,
        lhs: usize,
        rhs: usize,
        next: usize,
    },
    Sub {
        dst: usize,
        lhs: usize,
        rhs: usize,
        next: usize,
    },
    Read {
        dst: usize,
        address: usize,
        next: usize,
    },
    Write {
        address: usize,
        value: usize,
        next: usize,
    },
    Branch {
        test: usize,
        nonzero: usize,
        zero: usize,
    },
    PatchNext {
        node: usize,
        target: usize,
        next: usize,
    },
    ReadNext {
        dst: usize,
        node: usize,
        next: usize,
    },
    CloneNode {
        dst: usize,
        source: usize,
        next: usize,
    },
    PatchConst {
        node: usize,
        value: usize,
        next: usize,
    },
    Delete {
        address: usize,
        next: usize,
    },
    GraphNew {
        dst: usize,
        next: usize,
    },
    GraphCopyNode {
        graph: usize,
        source: usize,
        dst: usize,
        next: usize,
    },
    /// Generic local-address transport: copied control edges are normalized
    /// into the destination graph (environmental language affordance, NOT repair).
    GraphCopyNodeLocal {
        graph: usize,
        source: usize,
        dst: usize,
        next: usize,
    },
    GraphSetNext {
        graph: usize,
        node: usize,
        target: usize,
        next: usize,
    },
    GraphSetConst {
        graph: usize,
        node: usize,
        value: usize,
        next: usize,
    },
    GraphReadNext {
        dst: usize,
        graph: usize,
        node: usize,
        next: usize,
    },
    GraphDrop {
        graph: usize,
        next: usize,
    },
    GraphLaunch {
        graph: usize,
        credits: usize,
        dst: usize,
        next: usize,
    },
    GraphEncode {
        graph: usize,
        address: usize,
        dst: usize,
        next: usize,
    },
    GraphGrantRead {
        graph: usize,
        reader: usize,
        next: usize,
    },
    GraphRevokeRead {
        graph: usize,
        reader: usize,
        next: usize,
    },
    GraphReadTag {
        dst: usize,
        graph: usize,
        node: usize,
        next: usize,
    },
    DrawCompute {
        amount: usize,
        next: usize,
    },
    ReturnCompute {
        amount: usize,
        next: usize,
    },
    TransferCompute {
        recipient: usize,
        amount: usize,
        next: usize,
    },
    GraphReadField {
        graph: usize,
        node: usize,
        field: usize,
        next: usize,
    },
    GraphReplace {
        graph: usize,
        node: usize,
        address: usize,
        next: usize,
    },
    /// Copy code across graph objects with explicit read/owner boundaries.
    GraphCopyFrom {
        target: usize,
        source: usize,
        node: usize,
        next: usize,
    },
    /// Transfer graph ownership; recipient must be an existing process.
    GraphTransfer {
        graph: usize,
        recipient: usize,
        next: usize,
    },
    Halt,
}

impl Op {
    /// Numeric description of an existing instruction, preserving signed literals.
    /// Five cells match the GraphEncode descriptor contract.
    fn encode_instruction(&self) -> Result<[i64; 5], String> {
        match self {
            Op::Const { dst, value, next } => Ok([
                0,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                *value,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
            ]),
            Op::Add {
                dst,
                lhs,
                rhs,
                next,
            } => Ok([
                1,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*lhs).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*rhs).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
            Op::Sub {
                dst,
                lhs,
                rhs,
                next,
            } => Ok([
                2,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*lhs).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*rhs).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
            Op::Read { dst, address, next } => Ok([
                3,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*address).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
            ]),
            Op::Write {
                address,
                value,
                next,
            } => Ok([
                4,
                i64::try_from(*address).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*value).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
            ]),
            Op::Branch {
                test,
                nonzero,
                zero,
            } => Ok([
                5,
                i64::try_from(*test).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*nonzero).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*zero).map_err(|_| "operand exceeds i64".to_string())?,
                0,
            ]),
            Op::PatchNext { node, target, next } => Ok([
                6,
                i64::try_from(*node).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*target).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
            ]),
            Op::ReadNext { dst, node, next } => Ok([
                7,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*node).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
            ]),
            Op::CloneNode { dst, source, next } => Ok([
                8,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*source).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
            ]),
            Op::PatchConst { node, value, next } => Ok([
                9,
                i64::try_from(*node).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*value).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
            ]),
            Op::Delete { address, next } => Ok([
                10,
                i64::try_from(*address).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
                0,
            ]),
            Op::GraphNew { dst, next } => Ok([
                11,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
                0,
            ]),
            Op::GraphCopyNode {
                graph,
                source,
                dst,
                next,
            } => Ok([
                12,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*source).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
            Op::GraphSetNext {
                graph,
                node,
                target,
                next,
            } => Ok([
                13,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*node).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*target).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
            Op::GraphSetConst {
                graph,
                node,
                value,
                next,
            } => Ok([
                14,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*node).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*value).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
            Op::GraphReadNext {
                dst,
                graph,
                node,
                next,
            } => Ok([
                15,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*node).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
            Op::GraphDrop { graph, next } => Ok([
                16,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
                0,
            ]),
            Op::GraphLaunch {
                graph,
                credits,
                dst,
                next,
            } => Ok([
                17,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*credits).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
            Op::GraphEncode {
                graph,
                address,
                dst,
                next,
            } => Ok([
                18,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*address).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
            Op::Halt => Ok([19, 0, 0, 0, 0]),
            Op::GraphGrantRead {
                graph,
                reader,
                next,
            } => Ok([
                20,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*reader).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
            ]),
            Op::GraphRevokeRead {
                graph,
                reader,
                next,
            } => Ok([
                21,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*reader).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
            ]),
            Op::GraphReadTag {
                dst,
                graph,
                node,
                next,
            } => Ok([
                22,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*node).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
            Op::DrawCompute { amount, next } => Ok([
                23,
                i64::try_from(*amount).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
                0,
            ]),
            Op::ReturnCompute { amount, next } => Ok([
                24,
                i64::try_from(*amount).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
                0,
            ]),
            Op::TransferCompute {
                recipient,
                amount,
                next,
            } => Ok([
                25,
                i64::try_from(*recipient).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*amount).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
            ]),
            Op::GraphReadField {
                graph,
                node,
                field,
                next,
            } => Ok([
                26,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*node).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*field).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
            Op::GraphReplace {
                graph,
                node,
                address,
                next,
            } => Ok([
                27,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*node).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*address).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
            Op::GraphCopyFrom {
                target,
                source,
                node,
                next,
            } => Ok([
                28,
                i64::try_from(*target).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*source).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*node).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
            Op::GraphTransfer {
                graph,
                recipient,
                next,
            } => Ok([
                29,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*recipient).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
                0,
            ]),
            Op::GraphCopyNodeLocal {
                graph,
                source,
                dst,
                next,
            } => Ok([
                30,
                i64::try_from(*graph).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*source).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*dst).map_err(|_| "operand exceeds i64".to_string())?,
                i64::try_from(*next).map_err(|_| "operand exceeds i64".to_string())?,
            ]),
        }
    }

    fn opcode_tag(&self) -> i64 {
        match self {
            Op::Const { .. } => 0,
            Op::Add { .. } => 1,
            Op::Sub { .. } => 2,
            Op::Read { .. } => 3,
            Op::Write { .. } => 4,
            Op::Branch { .. } => 5,
            Op::PatchNext { .. } => 6,
            Op::ReadNext { .. } => 7,
            Op::CloneNode { .. } => 8,
            Op::PatchConst { .. } => 9,
            Op::Delete { .. } => 10,
            Op::GraphNew { .. } => 11,
            Op::GraphCopyNode { .. } => 12,
            Op::GraphSetNext { .. } => 13,
            Op::GraphSetConst { .. } => 14,
            Op::GraphReadNext { .. } => 15,
            Op::GraphDrop { .. } => 16,
            Op::GraphLaunch { .. } => 17,
            Op::GraphEncode { .. } => 18,
            Op::Halt => 19,
            Op::GraphGrantRead { .. } => 20,
            Op::GraphRevokeRead { .. } => 21,
            Op::GraphReadTag { .. } => 22,
            Op::DrawCompute { .. } => 23,
            Op::ReturnCompute { .. } => 24,
            Op::TransferCompute { .. } => 25,
            Op::GraphReadField { .. } => 26,
            Op::GraphReplace { .. } => 27,
            Op::GraphCopyFrom { .. } => 28,
            Op::GraphTransfer { .. } => 29,
            Op::GraphCopyNodeLocal { .. } => 30,
        }
    }

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
            | Op::Delete { next, .. }
            | Op::GraphNew { next, .. }
            | Op::GraphCopyNode { next, .. }
            | Op::GraphCopyNodeLocal { next, .. }
            | Op::GraphSetNext { next, .. }
            | Op::GraphSetConst { next, .. }
            | Op::GraphReadNext { next, .. }
            | Op::GraphDrop { next, .. }
            | Op::GraphLaunch { next, .. }
            | Op::GraphEncode { next, .. }
            | Op::GraphGrantRead { next, .. }
            | Op::GraphRevokeRead { next, .. }
            | Op::GraphReadTag { next, .. }
            | Op::DrawCompute { next, .. }
            | Op::ReturnCompute { next, .. }
            | Op::TransferCompute { next, .. }
            | Op::GraphReadField { next, .. }
            | Op::GraphReplace { next, .. }
            | Op::GraphCopyFrom { next, .. }
            | Op::GraphTransfer { next, .. } => Some(next),
            Op::Branch { .. } | Op::Halt => None,
        }
    }

    /// Explicit, generic destination-graph edge addressing for the opt-in
    /// GraphCopyNodeLocal language primitive. Closing every edge here is a
    /// *VM-supplied affordance*, never evidence of autonomous node repair.
    fn relocate_edges(&mut self, source: usize, destination: usize) {
        let modulus = i128::try_from(destination + 1).expect("bounded graph index");
        let move_edge = |edge: &mut usize| {
            let relative = i128::try_from(*edge).expect("bounded source edge")
                - i128::try_from(source).expect("bounded source node");
            let new_edge = (i128::try_from(destination).expect("bounded graph index") + relative)
                .rem_euclid(modulus);
            *edge = usize::try_from(new_edge).expect("positive bounded edge");
        };
        match self {
            Op::Branch { nonzero, zero, .. } => {
                move_edge(nonzero);
                move_edge(zero);
            }
            _ => {
                if let Some(edge) = self.next_mut() {
                    move_edge(edge);
                }
            }
        }
    }
}

impl Op {
    /// Stable Stage-3.3 ABI: five integer data cells [tag, arg0, arg1, arg2, arg3].
    /// The descriptor is ordinary world data; no semantic goal oracle is involved.
    pub fn decode_instruction(words: [i64; 5]) -> Result<Op, String> {
        let slot = |index: usize| -> Result<usize, String> {
            usize::try_from(words[index])
                .map_err(|_| "negative or overflowing instruction operand".to_string())
        };
        match words[0] {
            0 => Ok(Op::Const {
                dst: slot(1)?,
                value: words[2],
                next: slot(3)?,
            }),
            1 => Ok(Op::Add {
                dst: slot(1)?,
                lhs: slot(2)?,
                rhs: slot(3)?,
                next: slot(4)?,
            }),
            2 => Ok(Op::Sub {
                dst: slot(1)?,
                lhs: slot(2)?,
                rhs: slot(3)?,
                next: slot(4)?,
            }),
            3 => Ok(Op::Read {
                dst: slot(1)?,
                address: slot(2)?,
                next: slot(3)?,
            }),
            4 => Ok(Op::Write {
                address: slot(1)?,
                value: slot(2)?,
                next: slot(3)?,
            }),
            5 => Ok(Op::Branch {
                test: slot(1)?,
                nonzero: slot(2)?,
                zero: slot(3)?,
            }),
            6 => Ok(Op::PatchNext {
                node: slot(1)?,
                target: slot(2)?,
                next: slot(3)?,
            }),
            7 => Ok(Op::ReadNext {
                dst: slot(1)?,
                node: slot(2)?,
                next: slot(3)?,
            }),
            8 => Ok(Op::CloneNode {
                dst: slot(1)?,
                source: slot(2)?,
                next: slot(3)?,
            }),
            9 => Ok(Op::PatchConst {
                node: slot(1)?,
                value: slot(2)?,
                next: slot(3)?,
            }),
            10 => Ok(Op::Delete {
                address: slot(1)?,
                next: slot(2)?,
            }),
            11 => Ok(Op::GraphNew {
                dst: slot(1)?,
                next: slot(2)?,
            }),
            12 => Ok(Op::GraphCopyNode {
                graph: slot(1)?,
                source: slot(2)?,
                dst: slot(3)?,
                next: slot(4)?,
            }),
            13 => Ok(Op::GraphSetNext {
                graph: slot(1)?,
                node: slot(2)?,
                target: slot(3)?,
                next: slot(4)?,
            }),
            14 => Ok(Op::GraphSetConst {
                graph: slot(1)?,
                node: slot(2)?,
                value: slot(3)?,
                next: slot(4)?,
            }),
            15 => Ok(Op::GraphReadNext {
                dst: slot(1)?,
                graph: slot(2)?,
                node: slot(3)?,
                next: slot(4)?,
            }),
            16 => Ok(Op::GraphDrop {
                graph: slot(1)?,
                next: slot(2)?,
            }),
            17 => Ok(Op::GraphLaunch {
                graph: slot(1)?,
                credits: slot(2)?,
                dst: slot(3)?,
                next: slot(4)?,
            }),
            18 => Ok(Op::GraphEncode {
                graph: slot(1)?,
                address: slot(2)?,
                dst: slot(3)?,
                next: slot(4)?,
            }),
            19 => Ok(Op::Halt),
            20 => Ok(Op::GraphGrantRead {
                graph: slot(1)?,
                reader: slot(2)?,
                next: slot(3)?,
            }),
            21 => Ok(Op::GraphRevokeRead {
                graph: slot(1)?,
                reader: slot(2)?,
                next: slot(3)?,
            }),
            22 => Ok(Op::GraphReadTag {
                dst: slot(1)?,
                graph: slot(2)?,
                node: slot(3)?,
                next: slot(4)?,
            }),
            23 => Ok(Op::DrawCompute {
                amount: slot(1)?,
                next: slot(2)?,
            }),
            24 => Ok(Op::ReturnCompute {
                amount: slot(1)?,
                next: slot(2)?,
            }),
            25 => Ok(Op::TransferCompute {
                recipient: slot(1)?,
                amount: slot(2)?,
                next: slot(3)?,
            }),
            26 => Ok(Op::GraphReadField {
                graph: slot(1)?,
                node: slot(2)?,
                field: slot(3)?,
                next: slot(4)?,
            }),
            27 => Ok(Op::GraphReplace {
                graph: slot(1)?,
                node: slot(2)?,
                address: slot(3)?,
                next: slot(4)?,
            }),
            28 => Ok(Op::GraphCopyFrom {
                target: slot(1)?,
                source: slot(2)?,
                node: slot(3)?,
                next: slot(4)?,
            }),
            29 => Ok(Op::GraphTransfer {
                graph: slot(1)?,
                recipient: slot(2)?,
                next: slot(3)?,
            }),
            30 => Ok(Op::GraphCopyNodeLocal {
                graph: slot(1)?,
                source: slot(2)?,
                dst: slot(3)?,
                next: slot(4)?,
            }),
            _ => Err("unknown opcode tag".into()),
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
    /// Charged but unfulfilled native Draw, only with opt-in recoverable world law.
    DrawDeferred {
        requested: u64,
        available: u64,
    },
    Halted,
    Trapped(String),
    Intervened(String),
    InterventionRejected(String),
    GraphCopied {
        source_graph: u64,
        source_owner: ProcessId,
        source_node: usize,
        target_graph: u64,
        target_owner: ProcessId,
        new_node: usize,
    },
    GraphTransferred {
        graph: u64,
        former_owner: ProcessId,
        new_owner: ProcessId,
    },
    /// Observer-only successful native shared-cell read.
    CellRead {
        key: CellKey,
        value: i64,
    },
    /// Observer-only successful native shared-cell write.
    CellWritten {
        key: CellKey,
        value: i64,
    },
    /// Observer-only exact result of a successful native graph opcode-tag read.
    GraphTagRead {
        graph: u64,
        owner: ProcessId,
        node: usize,
        tag: i64,
    },
    /// Observer-only exact result of a successful native graph descriptor-field read.
    GraphFieldRead {
        graph: u64,
        owner: ProcessId,
        node: usize,
        field: usize,
        word: i64,
    },
    /// Successful resource-accounted movement of graph code into an executable child.
    GraphLaunched {
        source_graph: u64,
        parent_process: ProcessId,
        child_process: ProcessId,
        transferred_credits: u64,
        moved_nodes: usize,
    },
    /// Successful generic decoding of an existing shared-cell descriptor into a new node.
    GraphEncoded {
        graph: u64,
        owner: ProcessId,
        new_node: usize,
        descriptor_base: i64,
        descriptor: [i64; 5],
    },
    /// Successful generic replacement using five shared-cell descriptor words.
    GraphReplaced {
        graph: u64,
        owner: ProcessId,
        node: usize,
        descriptor_base: i64,
        descriptor: [i64; 5],
    },
    /// A native edit opcode consumed its ordinary instruction charge but had no code effect
    /// under the researcher-selected mutation-disabled control world.
    NativeCodeEditSuppressed {
        opcode_tag: i64,
    },
    /// Opt-in observer-only native edit trace; native actor PC is on the event.
    ExecutableEdgePatched {
        node: usize,
        before: usize,
        after: usize,
    },
    /// Observer-only successful graph allocation.
    GraphCreated {
        graph: u64,
        owner: ProcessId,
    },
    /// Successful node copy from an existing process's executable code.
    GraphNodeCopied {
        graph: u64,
        owner: ProcessId,
        source_process: ProcessId,
        source_node: usize,
        new_node: usize,
    },
    /// Explicit native local-address-copy event: VM automatically relocates
    /// copied edges; this is not a self-repair or an independent organism.
    GraphNodeCopiedLocal {
        graph: u64,
        owner: ProcessId,
        source_process: ProcessId,
        source_node: usize,
        new_node: usize,
    },
    /// Successful generic graph node edit; does not imply effective novelty.
    GraphNodeEdited {
        graph: u64,
        owner: ProcessId,
        node: usize,
        edit: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub tick: u64,
    pub origin: Origin,
    pub process: ProcessId,
    pub pc: usize,
    pub kind: EventKind,
    pub credits_after: u64,
    /// True only when the world's opt-in shared-pool rule paid this native Draw attempt.
    #[serde(default, skip_serializing_if = "is_false")]
    pub pool_paid_attempt: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphObject {
    pub owner: ProcessId,
    pub nodes: Vec<Op>,
    #[serde(default)]
    pub readers: BTreeSet<ProcessId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct World {
    #[serde(default = "default_world_format_version")]
    format_version: u32,
    processes: BTreeMap<ProcessId, Process>,
    cells: BTreeMap<CellKey, i64>,
    storage_limit: usize,
    max_nodes_per_process: usize,
    /// Optional common allocation cap: all executable nodes plus shared cells.
    total_storage_limit: Option<usize>,
    /// Versioned opt-in accounting for process headers and graph reader entries.
    #[serde(default)]
    strict_accounting: bool,
    /// Environment-held computation credits; transfers preserve the total inventory.
    #[serde(default)]
    compute_pool: u64,
    /// Generic, opt-in substrate rule: a zero-credit actor may attempt its own
    /// next DrawCompute using one world-pool credit as the instruction charge.
    /// Not equivalent to an actor-targeted host refill or autonomous energy creation.
    #[serde(default, skip_serializing_if = "is_false")]
    pool_paid_draw: bool,
    /// Generic opt-in world physics: insufficient valid native Draw waits without stopping.
    /// No credits are created; each attempted Draw is still charged.
    #[serde(default, skip_serializing_if = "is_false")]
    recoverable_draw_shortage: bool,
    /// Opt-in matched experimental law: charge native executable-code edit attempts
    /// without allowing their code/edge changes. Never enabled in legacy worlds.
    #[serde(default, skip_serializing_if = "is_false")]
    native_code_edits_disabled: bool,
    /// Observer-only event detail for native PatchNext; default off to preserve
    /// the original hash-serialized world semantics in older studies.
    #[serde(default, skip_serializing_if = "is_false")]
    trace_native_executable_edits: bool,
    #[serde(default)]
    graph_objects: BTreeMap<u64, GraphObject>,
    #[serde(default = "default_graph_id")]
    next_graph_id: u64,
    #[serde(default = "default_process_id")]
    next_process_id: u64,
    cursor: Option<ProcessId>,
    tick: u64,
    events: Vec<Event>,
    /// Transient provenance for the current successful instruction only.
    #[serde(skip)]
    pending_transition: Option<EventKind>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intervention {
    Disable {
        process: ProcessId,
    },
    SetCredits {
        process: ProcessId,
        credits: u64,
    },
    SetCell {
        key: CellKey,
        value: i64,
    },
    SetComputePool {
        credits: u64,
    },
    PatchEdge {
        process: ProcessId,
        node: usize,
        next: usize,
    },
    /// Explicit researcher-side structural ablation, never native CIL.
    DropGraph {
        graph: u64,
    },
    /// External injection of an already allocated but currently absent graph.
    /// This is a laboratory positive control, never a native CIL primitive.
    PutGraph {
        graph: u64,
        object: GraphObject,
    },
}

fn is_false(value: &bool) -> bool {
    !*value
}

fn default_world_format_version() -> u32 {
    WORLD_FORMAT_VERSION
}
fn default_graph_id() -> u64 {
    1
}
fn default_process_id() -> u64 {
    1
}

impl World {
    pub fn new(storage_limit: usize, max_nodes_per_process: usize) -> Self {
        Self {
            format_version: WORLD_FORMAT_VERSION,
            processes: BTreeMap::new(),
            cells: BTreeMap::new(),
            storage_limit,
            max_nodes_per_process,
            total_storage_limit: None,
            strict_accounting: false,
            compute_pool: 0,
            pool_paid_draw: false,
            recoverable_draw_shortage: false,
            native_code_edits_disabled: false,
            trace_native_executable_edits: false,
            graph_objects: BTreeMap::new(),
            next_graph_id: 1,
            next_process_id: 1,
            cursor: None,
            tick: 0,
            events: Vec::new(),
            pending_transition: None,
        }
    }

    /// New Stage-3 worlds can enforce a shared allocation cap for code nodes and data cells.
    /// Legacy `new` keeps independent Stage-1 limits for experiment compatibility.
    pub fn with_total_capacity(
        storage_limit: usize,
        max_nodes_per_process: usize,
        total: usize,
    ) -> Self {
        let mut world = Self::new(storage_limit, max_nodes_per_process);
        world.total_storage_limit = Some(total);
        world
    }

    pub fn occupied_units(&self) -> usize {
        let used = self.processes.values().fold(self.cells.len(), |used, p| {
            used.saturating_add(p.code.nodes.len())
        });
        let used = self.graph_objects.values().fold(used, |used, g| {
            used.saturating_add(1).saturating_add(g.nodes.len())
        });
        if self.strict_accounting {
            let with_processes = used.saturating_add(self.processes.len());
            self.graph_objects
                .values()
                .fold(with_processes, |count, g| {
                    count.saturating_add(g.readers.len())
                })
        } else {
            used
        }
    }

    /// CL-0 acceptance baseline: price each process header and graph read grant.
    /// Existing `with_total_capacity` snapshots retain their earlier semantics.
    pub fn with_strict_capacity(
        storage_limit: usize,
        max_nodes_per_process: usize,
        total: usize,
    ) -> Self {
        let mut world = Self::with_total_capacity(storage_limit, max_nodes_per_process, total);
        world.strict_accounting = true;
        world
    }

    /// Researcher chooses an opt-in generic environment law before running.
    /// It remains visible in the serialized world and never revives stopped actors.
    pub fn with_pool_paid_draw(mut self) -> Self {
        self.pool_paid_draw = true;
        self
    }

    pub fn pool_paid_draw_enabled(&self) -> bool {
        self.pool_paid_draw
    }

    /// Generic retry physics, not an organism-authored action or code repair.
    pub fn with_recoverable_draw_shortage(mut self) -> Self {
        self.recoverable_draw_shortage = true;
        self
    }

    pub fn recoverable_draw_shortage_enabled(&self) -> bool {
        self.recoverable_draw_shortage
    }

    /// Opt-in observational control, not a new CIL instruction or a repair mechanism.
    /// The code bytes remain identical at initialization in paired worlds.
    pub fn with_native_code_edits_disabled(mut self) -> Self {
        self.native_code_edits_disabled = true;
        self
    }

    pub fn native_code_edits_disabled(&self) -> bool {
        self.native_code_edits_disabled
    }

    /// Instrument only real native PatchNext edge changes, never a virtual goal
    /// checker, fitness oracle, injected edit or CIL-visible new instruction.
    pub fn with_native_executable_edit_trace(mut self) -> Self {
        self.trace_native_executable_edits = true;
        self
    }

    pub fn total_capacity(&self) -> Option<usize> {
        self.total_storage_limit
    }

    pub fn format_version(&self) -> u32 {
        self.format_version
    }

    pub fn strict_accounting(&self) -> bool {
        self.strict_accounting
    }

    /// Read-only visibility for analysis. Graph capabilities remain local to the creator.
    pub fn graph_object(&self, id: u64) -> Option<&GraphObject> {
        self.graph_objects.get(&id)
    }
    pub fn graph_count(&self) -> usize {
        self.graph_objects.len()
    }

    fn can_allocate(&self, additional: usize) -> bool {
        self.total_storage_limit.is_none_or(|limit| {
            self.occupied_units()
                .checked_add(additional)
                .is_some_and(|new_total| new_total <= limit)
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
        let needed = process
            .code
            .nodes
            .len()
            .checked_add(usize::from(self.strict_accounting))
            .ok_or("total storage capacity exhausted")?;
        if !self.can_allocate(needed) {
            return Err("total storage capacity exhausted");
        }
        self.total_compute_inventory()
            .and_then(|total| total.checked_add(process.credits))
            .ok_or("total compute inventory overflow")?;
        let next_id = process.id.checked_add(1).ok_or("process id exhausted")?;
        let id = process.id;
        self.next_process_id = self.next_process_id.max(next_id);
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
            pool_paid_attempt: false,
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

    pub fn available_compute_pool(&self) -> u64 {
        self.compute_pool
    }

    /// Checked aggregate inventory for scientific resource accounting.
    pub fn total_compute_inventory(&self) -> Option<u64> {
        self.processes
            .values()
            .try_fold(self.compute_pool, |sum, p| sum.checked_add(p.credits))
    }

    /// Applies a labeled experimenter action, never attributed to internal CIL.
    /// Rejected interventions are also observable. There is no host-side repair policy.
    pub fn intervene(&mut self, intervention: Intervention) -> Result<(), String> {
        let description = format!("{intervention:?}");
        let process = match &intervention {
            Intervention::Disable { process }
            | Intervention::SetCredits { process, .. }
            | Intervention::PatchEdge { process, .. } => *process,
            Intervention::SetCell { .. }
            | Intervention::SetComputePool { .. }
            | Intervention::DropGraph { .. }
            | Intervention::PutGraph { .. } => 0,
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
            pool_paid_attempt: false,
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
                let previous = self
                    .processes
                    .get(&process)
                    .ok_or("unknown process")?
                    .credits;
                self.total_compute_inventory()
                    .and_then(|total| total.checked_sub(previous))
                    .and_then(|other| other.checked_add(credits))
                    .ok_or("total compute inventory overflow")?;
                self.processes
                    .get_mut(&process)
                    .expect("checked process")
                    .credits = credits;
            }
            Intervention::SetComputePool { credits } => {
                let inventory_without_pool = self
                    .processes
                    .values()
                    .try_fold(0_u64, |sum, p| sum.checked_add(p.credits))
                    .ok_or("total compute inventory overflow")?;
                inventory_without_pool
                    .checked_add(credits)
                    .ok_or("total compute inventory overflow")?;
                self.compute_pool = credits;
            }
            Intervention::SetCell { key, value } => {
                if !self.cells.contains_key(&key)
                    && (self.cells.len() >= self.storage_limit || !self.can_allocate(1))
                {
                    return Err("storage capacity exhausted".into());
                }
                self.cells.insert(key, value);
            }
            Intervention::PatchEdge {
                process,
                node,
                next,
            } => {
                let p = self.processes.get_mut(&process).ok_or("unknown process")?;
                if node >= p.code.nodes.len() || next >= p.code.nodes.len() {
                    return Err("invalid graph node or edge".into());
                }
                let dest = p.code.nodes[node].next_mut().ok_or("no patchable edge")?;
                *dest = next;
            }
            Intervention::DropGraph { graph } => {
                if self.graph_objects.remove(&graph).is_none() {
                    return Err("unknown graph handle".into());
                }
            }
            Intervention::PutGraph { graph, ref object } => {
                if graph == 0 || graph >= self.next_graph_id {
                    return Err("graph handle was never allocated".into());
                }
                if self.graph_objects.contains_key(&graph) {
                    return Err("graph handle already exists".into());
                }
                if !self.processes.contains_key(&object.owner)
                    || object
                        .readers
                        .iter()
                        .any(|id| !self.processes.contains_key(id))
                {
                    return Err("unknown graph owner or reader".into());
                }
                if object.nodes.is_empty() || object.nodes.len() > self.max_nodes_per_process {
                    return Err("invalid injected graph size".into());
                }
                let additional = object
                    .nodes
                    .len()
                    .checked_add(1)
                    .and_then(|n| {
                        if self.strict_accounting {
                            n.checked_add(object.readers.len())
                        } else {
                            Some(n)
                        }
                    })
                    .ok_or("injected graph storage overflow")?;
                if !self.can_allocate(additional) {
                    return Err("total storage capacity exhausted".into());
                }
                self.graph_objects.insert(graph, object.clone());
            }
        }
        Ok(())
    }

    pub fn step(&mut self) -> bool {
        // The opt-in shared-pool rule never gives a process credits directly.
        // At zero local credits only an actual pending native DrawCompute may
        // execute, with its single attempt charge taken from the shared pool.
        let eligible = |p: &Process| {
            !p.stopped
                && (p.credits > 0
                    || (self.pool_paid_draw
                        && self.compute_pool > 0
                        && matches!(p.code.nodes.get(p.pc), Some(Op::DrawCompute { .. }))
                        && (!self.recoverable_draw_shortage
                            || matches!(
                                p.code.nodes.get(p.pc),
                                Some(Op::DrawCompute { amount, .. })
                                    if p.registers.get(*amount)
                                        .and_then(|raw| u64::try_from(*raw).ok())
                                        .and_then(|requested| requested.checked_add(1))
                                        .is_some_and(|required| self.compute_pool >= required)
                            ))))
        };
        let next_id = self
            .processes
            .iter()
            .filter(|(&id, p)| eligible(p) && self.cursor.is_none_or(|last| id > last))
            .map(|(&id, _)| id)
            .next()
            .or_else(|| {
                self.processes
                    .iter()
                    .find(|(_, p)| eligible(p))
                    .map(|(&id, _)| id)
            });
        let Some(id) = next_id else {
            return false;
        };
        self.cursor = Some(id);
        self.pending_transition = None;
        let pool_paid_attempt = self.processes.get(&id).is_some_and(|p| p.credits == 0);
        if pool_paid_attempt {
            // Provenance marker below distinguishes this environmental fee
            // from ordinary process-funded native instruction execution.
            self.compute_pool -= 1;
        }
        let (pc, op) = {
            let p = self
                .processes
                .get_mut(&id)
                .expect("selected process exists");
            if !pool_paid_attempt {
                p.credits -= 1; // all attempts, including traps, consume a credit
            }
            (p.pc, p.code.nodes.get(p.pc).cloned())
        };
        let outcome = match op {
            Some(op) => self.execute(id, op),
            None => Err("invalid program counter".to_string()),
        };
        let pending = self.pending_transition.take();
        let p = self
            .processes
            .get_mut(&id)
            .expect("selected process exists");
        let kind = match outcome {
            Ok(true) => {
                p.stopped = true;
                EventKind::Halted
            }
            Ok(false) => pending.unwrap_or(EventKind::Executed),
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
            pool_paid_attempt,
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
        // Study80 control physics: consume the same charged attempt and advance the
        // original next edge, while suppressing ONLY mutations of executable code.
        // Invalid next edges still trap; no actor is refilled, rescued or cloned.
        if self.native_code_edits_disabled
            && matches!(
                &op,
                Op::PatchNext { .. }
                    | Op::PatchConst { .. }
                    | Op::CloneNode { .. }
                    | Op::GraphCopyNode { .. }
                    | Op::GraphCopyNodeLocal { .. }
                    | Op::GraphSetNext { .. }
                    | Op::GraphSetConst { .. }
                    | Op::GraphEncode { .. }
                    | Op::GraphReplace { .. }
                    | Op::GraphCopyFrom { .. }
            )
        {
            // Validate against the real VM operation on an ephemeral shadow world.
            // This preserves invalid registers, access denial, storage exhaustion and
            // illegal next-edge traps; the shadow's code changes never enter reality.
            let mut shadow = self.clone();
            shadow.native_code_edits_disabled = false;
            let halted = shadow.execute(id, op.clone())?;
            debug_assert!(!halted, "native code edits cannot be Halt");
            let next_pc = shadow.process(id).expect("shadow actor exists").pc;
            self.processes
                .get_mut(&id)
                .expect("scheduled process exists")
                .pc = next_pc;
            self.pending_transition = Some(EventKind::NativeCodeEditSuppressed {
                opcode_tag: op.opcode_tag(),
            });
            return Ok(false);
        }
        if matches!(
            op,
            Op::DrawCompute { .. } | Op::ReturnCompute { .. } | Op::TransferCompute { .. }
        ) {
            return self.execute_compute(id, op);
        }
        if matches!(
            op,
            Op::GraphNew { .. }
                | Op::GraphCopyNode { .. }
                | Op::GraphCopyNodeLocal { .. }
                | Op::GraphSetNext { .. }
                | Op::GraphSetConst { .. }
                | Op::GraphReadNext { .. }
                | Op::GraphDrop { .. }
                | Op::GraphLaunch { .. }
                | Op::GraphEncode { .. }
                | Op::GraphGrantRead { .. }
                | Op::GraphRevokeRead { .. }
                | Op::GraphReadTag { .. }
                | Op::GraphReadField { .. }
                | Op::GraphReplace { .. }
                | Op::GraphCopyFrom { .. }
                | Op::GraphTransfer { .. }
        ) {
            return self.execute_graph(id, op);
        }
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
                if dst >= REGISTERS {
                    return Err("invalid register".into());
                }
                p.registers[dst] = value;
                p.pc = next;
            }
            Op::Add {
                dst,
                lhs,
                rhs,
                next,
            } => {
                edge(next)?;
                let a = reg(lhs)?;
                let b = reg(rhs)?;
                if dst >= REGISTERS {
                    return Err("invalid register".into());
                }
                p.registers[dst] = a.wrapping_add(b);
                p.pc = next;
            }
            Op::Sub {
                dst,
                lhs,
                rhs,
                next,
            } => {
                edge(next)?;
                let a = reg(lhs)?;
                let b = reg(rhs)?;
                if dst >= REGISTERS {
                    return Err("invalid register".into());
                }
                p.registers[dst] = a.wrapping_sub(b);
                p.pc = next;
            }
            Op::Read { dst, address, next } => {
                edge(next)?;
                let key = reg(address)?;
                if dst >= REGISTERS {
                    return Err("invalid register".into());
                }
                let value = cells.get(&key).copied().unwrap_or(0);
                p.registers[dst] = value;
                p.pc = next;
                self.pending_transition = Some(EventKind::CellRead { key, value });
            }
            Op::Write {
                address,
                value,
                next,
            } => {
                edge(next)?;
                let key = reg(address)?;
                let value = reg(value)?;
                if !cells.contains_key(&key) && (cells.len() >= self.storage_limit || !can_add_unit)
                {
                    return Err("storage capacity exhausted".into());
                }
                cells.insert(key, value);
                p.pc = next;
                self.pending_transition = Some(EventKind::CellWritten { key, value });
            }
            Op::Branch {
                test,
                nonzero,
                zero,
            } => {
                let condition = reg(test)?;
                p.pc = edge(if condition != 0 { nonzero } else { zero })?;
            }
            Op::PatchNext { node, target, next } => {
                edge(next)?;
                let target_node =
                    usize::try_from(reg(node)?).map_err(|_| "invalid node".to_string())?;
                let target_edge =
                    usize::try_from(reg(target)?).map_err(|_| "invalid edge".to_string())?;
                edge(target_node)?;
                edge(target_edge)?;
                let destination = p.code.nodes[target_node]
                    .next_mut()
                    .ok_or_else(|| "node has no patchable edge".to_string())?;
                let original_edge = *destination;
                *destination = target_edge;
                p.pc = next;
                if self.trace_native_executable_edits && original_edge != target_edge {
                    self.pending_transition = Some(EventKind::ExecutableEdgePatched {
                        node: target_node,
                        before: original_edge,
                        after: target_edge,
                    });
                }
            }
            Op::ReadNext { dst, node, next } => {
                edge(next)?;
                let index = usize::try_from(reg(node)?).map_err(|_| "invalid node".to_string())?;
                edge(index)?;
                if dst >= REGISTERS {
                    return Err("invalid register".into());
                }
                let edge_value = match &p.code.nodes[index] {
                    Op::Const { next, .. }
                    | Op::Add { next, .. }
                    | Op::Sub { next, .. }
                    | Op::Read { next, .. }
                    | Op::Write { next, .. }
                    | Op::PatchNext { next, .. }
                    | Op::ReadNext { next, .. }
                    | Op::CloneNode { next, .. }
                    | Op::PatchConst { next, .. }
                    | Op::Delete { next, .. } => *next,
                    Op::GraphNew { next, .. }
                    | Op::GraphCopyNode { next, .. }
                    | Op::GraphCopyNodeLocal { next, .. }
                    | Op::GraphSetNext { next, .. }
                    | Op::GraphSetConst { next, .. }
                    | Op::GraphReadNext { next, .. }
                    | Op::GraphDrop { next, .. }
                    | Op::GraphLaunch { next, .. }
                    | Op::GraphEncode { next, .. }
                    | Op::GraphGrantRead { next, .. }
                    | Op::GraphRevokeRead { next, .. }
                    | Op::GraphReadTag { next, .. }
                    | Op::DrawCompute { next, .. }
                    | Op::ReturnCompute { next, .. }
                    | Op::TransferCompute { next, .. }
                    | Op::GraphReadField { next, .. }
                    | Op::GraphReplace { next, .. }
                    | Op::GraphCopyFrom { next, .. }
                    | Op::GraphTransfer { next, .. } => *next,
                    Op::Branch { .. } | Op::Halt => {
                        return Err("node has no single next edge".into())
                    }
                };
                p.registers[dst] = i64::try_from(edge_value)
                    .map_err(|_| "edge does not fit register".to_string())?;
                p.pc = next;
            }
            Op::CloneNode { dst, source, next } => {
                edge(next)?;
                let index =
                    usize::try_from(reg(source)?).map_err(|_| "invalid node".to_string())?;
                edge(index)?;
                if dst >= REGISTERS {
                    return Err("invalid register".into());
                }
                if p.code.nodes.len() >= self.max_nodes_per_process || !can_add_unit {
                    return Err("code storage capacity exhausted".into());
                }
                let new_index = p.code.nodes.len();
                let encoded =
                    i64::try_from(new_index).map_err(|_| "node index overflow".to_string())?;
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
            Op::GraphNew { .. }
            | Op::GraphCopyNode { .. }
            | Op::GraphCopyNodeLocal { .. }
            | Op::GraphSetNext { .. }
            | Op::GraphSetConst { .. }
            | Op::GraphReadNext { .. }
            | Op::GraphDrop { .. }
            | Op::GraphLaunch { .. }
            | Op::GraphEncode { .. }
            | Op::GraphGrantRead { .. }
            | Op::GraphRevokeRead { .. }
            | Op::GraphReadTag { .. }
            | Op::DrawCompute { .. }
            | Op::ReturnCompute { .. }
            | Op::TransferCompute { .. }
            | Op::GraphReadField { .. }
            | Op::GraphReplace { .. }
            | Op::GraphCopyFrom { .. }
            | Op::GraphTransfer { .. } => unreachable!("dispatched above"),
        }
        Ok(false)
    }

    /// All compute transfers are ordinary one-credit CIL operations. No host fitness policy.
    fn execute_compute(&mut self, id: ProcessId, op: Op) -> Result<bool, String> {
        let p = self.processes.get(&id).expect("scheduled process exists");
        let reg = |i: usize| -> Result<u64, String> {
            let raw = *p
                .registers
                .get(i)
                .ok_or("invalid compute operand register")?;
            u64::try_from(raw).map_err(|_| "negative compute amount or process id".into())
        };
        let next_ok = |next: usize| -> Result<(), String> {
            if next < p.code.nodes.len() {
                Ok(())
            } else {
                Err("invalid execution edge".into())
            }
        };
        match op {
            Op::DrawCompute { amount, next } => {
                next_ok(next)?;
                let quantity = reg(amount)?;
                if quantity > self.compute_pool {
                    if self.recoverable_draw_shortage {
                        self.pending_transition = Some(EventKind::DrawDeferred {
                            requested: quantity,
                            available: self.compute_pool,
                        });
                        return Ok(false);
                    }
                    return Err("insufficient world compute pool".into());
                }
                p.credits
                    .checked_add(quantity)
                    .ok_or("process compute balance overflow")?;
                self.compute_pool -= quantity;
                let actor = self.processes.get_mut(&id).expect("actor exists");
                actor.credits += quantity;
                actor.pc = next;
            }
            Op::ReturnCompute { amount, next } => {
                next_ok(next)?;
                let quantity = reg(amount)?;
                if quantity > p.credits {
                    return Err("insufficient process compute".into());
                }
                self.compute_pool
                    .checked_add(quantity)
                    .ok_or("world compute pool overflow")?;
                self.compute_pool += quantity;
                let actor = self.processes.get_mut(&id).expect("actor exists");
                actor.credits -= quantity;
                actor.pc = next;
            }
            Op::TransferCompute {
                recipient,
                amount,
                next,
            } => {
                next_ok(next)?;
                let target_id = reg(recipient)?;
                let quantity = reg(amount)?;
                if quantity > p.credits {
                    return Err("insufficient process compute".into());
                }
                let target = self
                    .processes
                    .get(&target_id)
                    .ok_or("unknown recipient process")?;
                if target_id != id {
                    target
                        .credits
                        .checked_add(quantity)
                        .ok_or("recipient compute balance overflow")?;
                    self.processes.get_mut(&id).expect("actor exists").credits -= quantity;
                    self.processes
                        .get_mut(&target_id)
                        .expect("recipient exists")
                        .credits += quantity;
                }
                self.processes.get_mut(&id).expect("actor exists").pc = next;
            }
            _ => unreachable!("compute-only dispatcher"),
        }
        Ok(false)
    }

    fn execute_graph(&mut self, id: ProcessId, op: Op) -> Result<bool, String> {
        if self.total_storage_limit.is_none() {
            return Err("graph object operations require a bounded world".into());
        }
        let p = self.processes.get(&id).ok_or("unknown process")?;
        let len = p.code.nodes.len();
        let reg = |i: usize| -> Result<i64, String> {
            p.registers
                .get(i)
                .copied()
                .ok_or_else(|| "invalid register".into())
        };
        let edge = |i: usize| -> Result<(), String> {
            if i < len {
                Ok(())
            } else {
                Err("invalid execution edge".into())
            }
        };
        // Resolve and validate register values before mutating any state.
        let is_local_copy = matches!(&op, Op::GraphCopyNodeLocal { .. });
        match op {
            Op::GraphNew { dst, next } => {
                edge(next)?;
                if dst >= REGISTERS {
                    return Err("invalid destination register".into());
                }
                if !self.can_allocate(1) {
                    return Err("storage capacity exhausted".into());
                }
                let object_id = self.next_graph_id;
                let new_id = object_id.checked_add(1).ok_or("graph id exhausted")?;
                let encoded = i64::try_from(object_id).map_err(|_| "graph id overflow")?;
                self.graph_objects.insert(
                    object_id,
                    GraphObject {
                        owner: id,
                        nodes: Vec::new(),
                        readers: BTreeSet::new(),
                    },
                );
                self.next_graph_id = new_id;
                let p = self.processes.get_mut(&id).expect("owner exists");
                p.registers[dst] = encoded;
                p.pc = next;
                self.pending_transition = Some(EventKind::GraphCreated {
                    graph: object_id,
                    owner: id,
                });
            }
            Op::GraphCopyNode {
                graph,
                source,
                dst,
                next,
            }
            | Op::GraphCopyNodeLocal {
                graph,
                source,
                dst,
                next,
            } => {
                edge(next)?;
                if dst >= REGISTERS {
                    return Err("invalid destination register".into());
                }
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let index = usize::try_from(reg(source)?).map_err(|_| "invalid source node")?;
                let mut source_node = p
                    .code
                    .nodes
                    .get(index)
                    .ok_or("invalid source node")?
                    .clone();
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id {
                    return Err("access denied".into());
                }
                if object.nodes.len() >= self.max_nodes_per_process || !self.can_allocate(1) {
                    return Err("code storage capacity exhausted".into());
                }
                let new_index = object.nodes.len();
                let encoded = i64::try_from(new_index).map_err(|_| "node index overflow")?;
                if is_local_copy {
                    source_node.relocate_edges(index, new_index);
                }
                self.graph_objects
                    .get_mut(&object_id)
                    .expect("checked graph")
                    .nodes
                    .push(source_node);
                let p = self.processes.get_mut(&id).expect("owner exists");
                p.registers[dst] = encoded;
                p.pc = next;
                self.pending_transition = Some(if is_local_copy {
                    EventKind::GraphNodeCopiedLocal {
                        graph: object_id,
                        owner: id,
                        source_process: id,
                        source_node: index,
                        new_node: new_index,
                    }
                } else {
                    EventKind::GraphNodeCopied {
                        graph: object_id,
                        owner: id,
                        source_process: id,
                        source_node: index,
                        new_node: new_index,
                    }
                });
            }
            Op::GraphSetNext {
                graph,
                node,
                target,
                next,
            } => {
                edge(next)?;
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let index = usize::try_from(reg(node)?).map_err(|_| "invalid node")?;
                let target_index =
                    usize::try_from(reg(target)?).map_err(|_| "invalid target edge")?;
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id {
                    return Err("access denied".into());
                }
                if target_index >= object.nodes.len() {
                    return Err("invalid target edge".into());
                }
                let item = object.nodes.get(index).ok_or("invalid node")?;
                if item.clone().next_mut().is_none() {
                    return Err("node has no patchable edge".into());
                }
                *self
                    .graph_objects
                    .get_mut(&object_id)
                    .expect("checked graph")
                    .nodes[index]
                    .next_mut()
                    .expect("checked edge") = target_index;
                self.processes.get_mut(&id).expect("owner exists").pc = next;
                self.pending_transition = Some(EventKind::GraphNodeEdited {
                    graph: object_id,
                    owner: id,
                    node: index,
                    edit: "next".to_owned(),
                });
            }
            Op::GraphSetConst {
                graph,
                node,
                value,
                next,
            } => {
                edge(next)?;
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let index = usize::try_from(reg(node)?).map_err(|_| "invalid node")?;
                let new_value = reg(value)?;
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id {
                    return Err("access denied".into());
                }
                if !matches!(object.nodes.get(index), Some(Op::Const { .. })) {
                    return Err("node is not a const".into());
                }
                if let Op::Const { value, .. } = &mut self
                    .graph_objects
                    .get_mut(&object_id)
                    .expect("checked graph")
                    .nodes[index]
                {
                    *value = new_value;
                }
                self.processes.get_mut(&id).expect("owner exists").pc = next;
                self.pending_transition = Some(EventKind::GraphNodeEdited {
                    graph: object_id,
                    owner: id,
                    node: index,
                    edit: "const".to_owned(),
                });
            }
            Op::GraphReadNext {
                dst,
                graph,
                node,
                next,
            } => {
                edge(next)?;
                if dst >= REGISTERS {
                    return Err("invalid destination register".into());
                }
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let index = usize::try_from(reg(node)?).map_err(|_| "invalid node")?;
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id && !object.readers.contains(&id) {
                    return Err("access denied".into());
                }
                let mut item = object.nodes.get(index).ok_or("invalid node")?.clone();
                let outgoing = *item.next_mut().ok_or("node has no single next edge")?;
                let encoded = i64::try_from(outgoing).map_err(|_| "edge index overflow")?;
                let p = self.processes.get_mut(&id).expect("owner exists");
                p.registers[dst] = encoded;
                p.pc = next;
            }
            Op::GraphDrop { graph, next } => {
                edge(next)?;
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id {
                    return Err("access denied".into());
                }
                self.graph_objects.remove(&object_id);
                self.processes.get_mut(&id).expect("owner exists").pc = next;
            }
            Op::GraphLaunch {
                graph,
                credits,
                dst,
                next,
            } => {
                edge(next)?;
                if dst >= REGISTERS {
                    return Err("invalid destination register".into());
                }
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let requested =
                    u64::try_from(reg(credits)?).map_err(|_| "invalid credit amount")?;
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id {
                    return Err("access denied".into());
                }
                if object.nodes.is_empty() || object.nodes.len() > self.max_nodes_per_process {
                    return Err("invalid executable graph size".into());
                }
                if requested > p.credits {
                    return Err("insufficient transferable credit".into());
                }
                let child_id = self.next_process_id;
                let next_id = child_id.checked_add(1).ok_or("process id exhausted")?;
                let encoded = i64::try_from(child_id).map_err(|_| "process id overflow")?;
                // Move, rather than duplicate, graph instruction nodes.
                let object = self
                    .graph_objects
                    .remove(&object_id)
                    .expect("checked object");
                let moved_nodes = object.nodes.len();
                let parent = self.processes.get_mut(&id).expect("owner exists");
                parent.credits -= requested;
                parent.registers[dst] = encoded;
                parent.pc = next;
                self.processes
                    .insert(child_id, Process::new(child_id, object.nodes, requested));
                self.next_process_id = next_id;
                self.pending_transition = Some(EventKind::GraphLaunched {
                    source_graph: object_id,
                    parent_process: id,
                    child_process: child_id,
                    transferred_credits: requested,
                    moved_nodes,
                });
            }

            Op::GraphEncode {
                graph,
                address,
                dst,
                next,
            } => {
                edge(next)?;
                if dst >= REGISTERS {
                    return Err("invalid destination register".into());
                }
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let base = reg(address)?;
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id {
                    return Err("access denied".into());
                }
                if object.nodes.len() >= self.max_nodes_per_process || !self.can_allocate(1) {
                    return Err("code storage capacity exhausted".into());
                }
                let mut words = [0_i64; 5];
                for (offset, word) in words.iter_mut().enumerate() {
                    let offset = i64::try_from(offset).map_err(|_| "descriptor offset overflow")?;
                    let address = base
                        .checked_add(offset)
                        .ok_or("descriptor address overflow")?;
                    *word = *self
                        .cells
                        .get(&address)
                        .ok_or("incomplete instruction descriptor")?;
                }
                let candidate = Op::decode_instruction(words)?;
                let position = object.nodes.len();
                let position_reg = i64::try_from(position).map_err(|_| "node index overflow")?;
                self.graph_objects
                    .get_mut(&object_id)
                    .expect("validated handle")
                    .nodes
                    .push(candidate);
                let p = self.processes.get_mut(&id).expect("owner exists");
                p.registers[dst] = position_reg;
                p.pc = next;
                self.pending_transition = Some(EventKind::GraphEncoded {
                    graph: object_id,
                    owner: id,
                    new_node: position,
                    descriptor_base: base,
                    descriptor: words,
                });
            }

            Op::GraphGrantRead {
                graph,
                reader,
                next,
            } => {
                edge(next)?;
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let reader_id = u64::try_from(reg(reader)?).map_err(|_| "invalid reader id")?;
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id {
                    return Err("access denied".into());
                }
                if !self.processes.contains_key(&reader_id) {
                    return Err("unknown reader process".into());
                }
                if self.strict_accounting
                    && !object.readers.contains(&reader_id)
                    && !self.can_allocate(1)
                {
                    return Err("read grant storage capacity exhausted".into());
                }
                self.graph_objects
                    .get_mut(&object_id)
                    .expect("checked object")
                    .readers
                    .insert(reader_id);
                self.processes.get_mut(&id).expect("owner exists").pc = next;
            }
            Op::GraphRevokeRead {
                graph,
                reader,
                next,
            } => {
                edge(next)?;
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let reader_id = u64::try_from(reg(reader)?).map_err(|_| "invalid reader id")?;
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id {
                    return Err("access denied".into());
                }
                self.graph_objects
                    .get_mut(&object_id)
                    .expect("checked object")
                    .readers
                    .remove(&reader_id);
                self.processes.get_mut(&id).expect("owner exists").pc = next;
            }
            Op::GraphReadTag {
                dst,
                graph,
                node,
                next,
            } => {
                edge(next)?;
                if dst >= REGISTERS {
                    return Err("invalid destination register".into());
                }
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let index = usize::try_from(reg(node)?).map_err(|_| "invalid node")?;
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id && !object.readers.contains(&id) {
                    return Err("access denied".into());
                }
                let item = object.nodes.get(index).ok_or("invalid node")?;
                let tag = item.opcode_tag();
                let owner = object.owner;
                let p = self.processes.get_mut(&id).expect("reader exists");
                p.registers[dst] = tag;
                p.pc = next;
                self.pending_transition = Some(EventKind::GraphTagRead {
                    graph: object_id,
                    owner,
                    node: index,
                    tag,
                });
            }
            Op::GraphReadField {
                graph,
                node,
                field,
                next,
            } => {
                edge(next)?;
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let index = usize::try_from(reg(node)?).map_err(|_| "invalid node")?;
                let field_index =
                    usize::try_from(reg(field)?).map_err(|_| "invalid descriptor field")?;
                if field_index >= 5 {
                    return Err("invalid descriptor field".into());
                }
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id && !object.readers.contains(&id) {
                    return Err("access denied".into());
                }
                let instruction = object.nodes.get(index).ok_or("invalid node")?;
                let description = instruction.encode_instruction()?;
                let owner = object.owner;
                let word = description[field_index];
                let actor = self.processes.get_mut(&id).expect("owner exists");
                actor.registers[field] = word;
                actor.pc = next;
                self.pending_transition = Some(EventKind::GraphFieldRead {
                    graph: object_id,
                    owner,
                    node: index,
                    field: field_index,
                    word,
                });
            }
            Op::GraphReplace {
                graph,
                node,
                address,
                next,
            } => {
                edge(next)?;
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let index = usize::try_from(reg(node)?).map_err(|_| "invalid node")?;
                let base = reg(address)?;
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id {
                    return Err("access denied".into());
                }
                if index >= object.nodes.len() {
                    return Err("invalid node".into());
                }
                let mut description = [0_i64; 5];
                for (offset, word) in description.iter_mut().enumerate() {
                    let offset = i64::try_from(offset).map_err(|_| "descriptor offset overflow")?;
                    let key = base
                        .checked_add(offset)
                        .ok_or("descriptor address overflow")?;
                    *word = *self
                        .cells
                        .get(&key)
                        .ok_or("incomplete instruction descriptor")?;
                }
                let replacement = Op::decode_instruction(description)?;
                self.graph_objects
                    .get_mut(&object_id)
                    .expect("checked graph")
                    .nodes[index] = replacement;
                self.processes.get_mut(&id).expect("actor exists").pc = next;
                self.pending_transition = Some(EventKind::GraphReplaced {
                    graph: object_id,
                    owner: id,
                    node: index,
                    descriptor_base: base,
                    descriptor: description,
                });
            }
            Op::GraphCopyFrom {
                target,
                source,
                node,
                next,
            } => {
                edge(next)?;
                let target_id =
                    u64::try_from(reg(target)?).map_err(|_| "invalid target graph handle")?;
                let source_id =
                    u64::try_from(reg(source)?).map_err(|_| "invalid source graph handle")?;
                let index = usize::try_from(reg(node)?).map_err(|_| "invalid source node index")?;
                let from = self
                    .graph_objects
                    .get(&source_id)
                    .ok_or("unknown source graph handle")?;
                if from.owner != id && !from.readers.contains(&id) {
                    return Err("source graph access denied".into());
                }
                let source_owner = from.owner;
                let copied = from.nodes.get(index).ok_or("invalid source node")?.clone();
                let to = self
                    .graph_objects
                    .get(&target_id)
                    .ok_or("unknown target graph handle")?;
                if to.owner != id {
                    return Err("target graph access denied".into());
                }
                if to.nodes.len() >= self.max_nodes_per_process || !self.can_allocate(1) {
                    return Err("code storage capacity exhausted".into());
                }
                let new_node = to.nodes.len();
                let new_index = i64::try_from(new_node).map_err(|_| "node index overflow")?;
                let target_owner = to.owner;
                self.graph_objects
                    .get_mut(&target_id)
                    .expect("target validated")
                    .nodes
                    .push(copied);
                let process = self.processes.get_mut(&id).expect("actor exists");
                process.registers[node] = new_index;
                process.pc = next;
                self.pending_transition = Some(EventKind::GraphCopied {
                    source_graph: source_id,
                    source_owner,
                    source_node: index,
                    target_graph: target_id,
                    target_owner,
                    new_node,
                });
            }
            Op::GraphTransfer {
                graph,
                recipient,
                next,
            } => {
                edge(next)?;
                let object_id = u64::try_from(reg(graph)?).map_err(|_| "invalid graph handle")?;
                let recipient_id =
                    u64::try_from(reg(recipient)?).map_err(|_| "invalid recipient")?;
                let object = self
                    .graph_objects
                    .get(&object_id)
                    .ok_or("unknown graph handle")?;
                if object.owner != id {
                    return Err("access denied".into());
                }
                if !self.processes.contains_key(&recipient_id) {
                    return Err("unknown recipient process".into());
                }
                self.graph_objects
                    .get_mut(&object_id)
                    .expect("checked graph")
                    .owner = recipient_id;
                self.processes.get_mut(&id).expect("actor exists").pc = next;
                self.pending_transition = Some(EventKind::GraphTransferred {
                    graph: object_id,
                    former_owner: id,
                    new_owner: recipient_id,
                });
            }
            _ => unreachable!("only graph operations are dispatched here"),
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn writer(credits: u64) -> Process {
        Process::new(
            1,
            vec![
                Op::Const {
                    dst: 0,
                    value: 7,
                    next: 1,
                },
                Op::Const {
                    dst: 1,
                    value: 42,
                    next: 2,
                },
                Op::Write {
                    address: 0,
                    value: 1,
                    next: 3,
                },
                Op::Halt,
            ],
            credits,
        )
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
        assert!(matches!(
            world.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
        assert_eq!(world.remaining_credits(), 7);
    }

    #[test]
    fn invalid_edge_is_charged_and_stops() {
        let mut world = World::new(1, 16);
        world
            .load(Process::new(
                3,
                vec![Op::Const {
                    dst: 0,
                    value: 3,
                    next: 9,
                }],
                2,
            ))
            .unwrap();
        assert_eq!(world.run(100), 1);
        assert_eq!(world.process(3).unwrap().registers[0], 0);
        assert_eq!(world.remaining_credits(), 1);
        assert!(matches!(
            world.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
    }

    #[test]
    fn generic_graph_mutation_changes_control_flow() {
        let mut world = World::new(1, 16);
        world
            .load(Process::new(
                1,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 3,
                        next: 1,
                    }, // node to patch
                    Op::Const {
                        dst: 1,
                        value: 5,
                        next: 2,
                    }, // new edge
                    Op::PatchNext {
                        node: 0,
                        target: 1,
                        next: 3,
                    },
                    Op::Const {
                        dst: 2,
                        value: 99,
                        next: 4,
                    }, // old edge
                    Op::Halt,
                    Op::Const {
                        dst: 2,
                        value: 17,
                        next: 4,
                    }, // new edge
                ],
                10,
            ))
            .unwrap();
        assert_eq!(world.run(10), 6);
        assert_eq!(world.process(1).unwrap().registers[2], 17);
        assert!(matches!(
            world.process(1).unwrap().code.nodes[3],
            Op::Const { next: 5, .. }
        ));
    }

    #[test]
    fn matched_external_intervention_is_attributed_and_causal() {
        let mut base = World::new(2, 16);
        base.load(writer(8)).unwrap();
        let mut normal = base.clone();
        let mut disabled = base;
        disabled
            .intervene(Intervention::Disable { process: 1 })
            .unwrap();
        normal.run(20);
        disabled.run(20);
        assert_eq!(normal.cell(7), Some(42));
        assert_eq!(disabled.cell(7), None);
        assert!(disabled
            .events()
            .iter()
            .any(|e| e.origin == Origin::External && matches!(e.kind, EventKind::Intervened(_))));
    }

    #[test]
    fn rejected_intervention_is_atomic_and_logged() {
        let mut world = World::new(0, 16);
        world.load(writer(8)).unwrap();
        assert!(world
            .intervene(Intervention::SetCell { key: 3, value: 9 })
            .is_err());
        assert_eq!(world.cell(3), None);
        assert!(matches!(
            world.events().last().unwrap().kind,
            EventKind::InterventionRejected(_)
        ));
    }

    #[test]
    fn total_capacity_accounts_for_code_and_data_without_side_effects() {
        let mut world = World::with_total_capacity(4, 16, 4);
        world.load(writer(10)).unwrap();
        assert_eq!(world.occupied_units(), 4);
        assert_eq!(world.run(10), 3);
        assert_eq!(world.occupied_units(), 4);
        assert_eq!(world.cell(7), None);
        assert!(matches!(
            world.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
        assert!(world.load(Process::new(2, vec![Op::Halt], 1)).is_err());
    }

    #[test]
    fn clone_node_is_generic_and_resource_priced() {
        let mut world = World::with_total_capacity(2, 12, 8);
        world
            .load(Process::new(
                1,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 1,
                        next: 1,
                    },
                    Op::CloneNode {
                        dst: 1,
                        source: 0,
                        next: 2,
                    },
                    Op::ReadNext {
                        dst: 2,
                        node: 1,
                        next: 3,
                    },
                    Op::Halt,
                ],
                8,
            ))
            .unwrap();
        assert_eq!(world.occupied_units(), 4);
        assert_eq!(world.run(10), 4);
        assert_eq!(world.occupied_units(), 5);
        assert_eq!(world.process(1).unwrap().registers[1], 4);
        assert_eq!(world.process(1).unwrap().registers[2], 2);
        assert_eq!(
            world.process(1).unwrap().code.nodes[4],
            world.process(1).unwrap().code.nodes[1]
        );
        assert_eq!(world.remaining_credits(), 4);
    }

    #[test]
    fn no_allocation_space_blocks_clone_and_still_charges_instruction() {
        let mut world = World::with_total_capacity(2, 12, 3);
        world
            .load(Process::new(
                1,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 0,
                        next: 1,
                    },
                    Op::CloneNode {
                        dst: 1,
                        source: 0,
                        next: 2,
                    },
                    Op::Halt,
                ],
                5,
            ))
            .unwrap();
        assert_eq!(world.run(10), 2);
        assert_eq!(world.occupied_units(), 3);
        assert_eq!(world.process(1).unwrap().code.nodes.len(), 3);
        assert_eq!(world.remaining_credits(), 3);
        assert!(matches!(
            world.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
    }

    #[test]
    fn delete_reclaims_cell_allocation() {
        let mut world = World::with_total_capacity(2, 12, 7);
        world
            .load(Process::new(
                1,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 9,
                        next: 1,
                    },
                    Op::Const {
                        dst: 1,
                        value: 3,
                        next: 2,
                    },
                    Op::Write {
                        address: 0,
                        value: 1,
                        next: 3,
                    },
                    Op::Delete {
                        address: 0,
                        next: 4,
                    },
                    Op::Halt,
                ],
                6,
            ))
            .unwrap();
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
        assert!(world
            .intervene(Intervention::SetCell { key: 1, value: 2 })
            .is_err());
        assert_eq!(world.occupied_units(), 1);
    }

    #[test]
    fn constructed_graph_launch_transfers_nodes_and_credits_without_creation() {
        let mut world = World::with_total_capacity(2, 32, 20);
        world
            .load(Process::new(
                1,
                vec![
                    Op::GraphNew { dst: 0, next: 1 },
                    Op::Const {
                        dst: 1,
                        value: 8,
                        next: 2,
                    },
                    Op::Const {
                        dst: 2,
                        value: 2,
                        next: 3,
                    },
                    Op::GraphCopyNode {
                        graph: 0,
                        source: 2,
                        dst: 3,
                        next: 4,
                    },
                    Op::GraphCopyNode {
                        graph: 0,
                        source: 2,
                        dst: 3,
                        next: 5,
                    },
                    Op::GraphCopyNode {
                        graph: 0,
                        source: 2,
                        dst: 3,
                        next: 6,
                    },
                    Op::GraphCopyNode {
                        graph: 0,
                        source: 2,
                        dst: 3,
                        next: 7,
                    },
                    Op::GraphLaunch {
                        graph: 0,
                        credits: 1,
                        dst: 3,
                        next: 8,
                    },
                    Op::Halt,
                ],
                20,
            ))
            .unwrap();
        let initial = world.occupied_units();
        assert_eq!(world.run(7), 7);
        assert_eq!(world.graph_count(), 1);
        assert_eq!(world.occupied_units(), initial + 5);
        let object = world.graph_object(1).unwrap();
        assert_eq!(object.owner, 1);
        assert_eq!(object.nodes.len(), 4);
        assert_eq!(world.run(1), 1);
        assert_eq!(world.graph_count(), 0);
        assert_eq!(world.occupied_units(), initial + 4);
        assert_eq!(world.process(2).unwrap().credits, 8);
        assert_eq!(world.process(1).unwrap().credits, 4);
        assert_eq!(world.remaining_credits(), 12);
        assert!(world.events().iter().any(|e| matches!(
            e.kind,
            EventKind::GraphLaunched {
                source_graph: 1,
                parent_process: 1,
                child_process: 2,
                transferred_credits: 8,
                moved_nodes: 4
            }
        )));
    }

    #[test]
    fn graph_object_access_denied_without_mutating_target() {
        let mut world = World::with_total_capacity(1, 20, 20);
        world
            .load(Process::new(
                1,
                vec![Op::GraphNew { dst: 0, next: 1 }, Op::Halt],
                5,
            ))
            .unwrap();
        world
            .load(Process::new(
                2,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 1,
                        next: 1,
                    },
                    Op::GraphDrop { graph: 0, next: 2 },
                    Op::Halt,
                ],
                5,
            ))
            .unwrap();
        assert_eq!(world.run(4), 4);
        assert!(world.graph_object(1).is_some());
        assert!(matches!(
            world.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
    }

    #[test]
    fn empty_graph_capacity_and_invalid_handle_are_atomic() {
        let mut world = World::with_total_capacity(0, 8, 2);
        world
            .load(Process::new(
                1,
                vec![Op::GraphNew { dst: 0, next: 1 }, Op::Halt],
                2,
            ))
            .unwrap();
        assert_eq!(world.run(1), 1);
        assert_eq!(world.graph_count(), 0);
        assert_eq!(world.occupied_units(), 2);
        assert!(matches!(
            world.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
    }

    #[test]
    fn graph_drop_reclaims_header_and_nodes() {
        let mut world = World::with_total_capacity(0, 12, 10);
        world
            .load(Process::new(
                1,
                vec![
                    Op::GraphNew { dst: 0, next: 1 },
                    Op::Const {
                        dst: 1,
                        value: 3,
                        next: 2,
                    },
                    Op::GraphCopyNode {
                        graph: 0,
                        source: 1,
                        dst: 2,
                        next: 3,
                    },
                    Op::GraphDrop { graph: 0, next: 4 },
                    Op::Halt,
                ],
                10,
            ))
            .unwrap();
        let baseline = world.occupied_units();
        world.run(3);
        assert_eq!(world.occupied_units(), baseline + 2);
        world.run(1);
        assert_eq!(world.occupied_units(), baseline);
    }

    #[test]
    fn invalid_graph_edges_and_literals_are_not_written() {
        let mut world = World::with_total_capacity(0, 16, 20);
        world
            .load(Process::new(
                1,
                vec![
                    Op::GraphNew { dst: 0, next: 1 },
                    Op::Const {
                        dst: 1,
                        value: 1,
                        next: 2,
                    },
                    Op::GraphCopyNode {
                        graph: 0,
                        source: 1,
                        dst: 2,
                        next: 3,
                    },
                    Op::GraphSetNext {
                        graph: 0,
                        node: 2,
                        target: 1,
                        next: 4,
                    },
                    Op::Halt,
                ],
                9,
            ))
            .unwrap();
        world.run(4);
        assert_eq!(
            world.graph_object(1).unwrap().nodes[0],
            Op::Const {
                dst: 1,
                value: 1,
                next: 2
            }
        );
        assert!(matches!(
            world.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
    }

    #[test]
    fn graph_encode_constructs_new_opcode_from_world_data() {
        let mut w = World::with_total_capacity(8, 16, 24);
        w.load(Process::new(
            1,
            vec![
                Op::GraphNew { dst: 0, next: 1 },
                Op::Const {
                    dst: 1,
                    value: 100,
                    next: 2,
                },
                Op::GraphEncode {
                    graph: 0,
                    address: 1,
                    dst: 2,
                    next: 3,
                },
                Op::Const {
                    dst: 1,
                    value: 2,
                    next: 4,
                },
                Op::GraphLaunch {
                    graph: 0,
                    credits: 1,
                    dst: 2,
                    next: 5,
                },
                Op::Halt,
            ],
            12,
        ))
        .unwrap();
        // The descriptor is investigator-supplied data, NOT spontaneous discovery.
        for (offset, value) in [19, 0, 0, 0, 0].into_iter().enumerate() {
            w.intervene(Intervention::SetCell {
                key: 100 + offset as i64,
                value,
            })
            .unwrap();
        }
        assert_eq!(w.run(3), 3);
        assert_eq!(w.graph_object(1).unwrap().nodes, vec![Op::Halt]);
        assert_eq!(w.run(4), 4);
        assert!(w.process(2).unwrap().stopped);
        assert_eq!(w.graph_count(), 0);
        assert_eq!(w.occupied_units(), 12);
        assert_eq!(w.remaining_credits(), 5);
    }

    #[test]
    fn malformed_descriptor_does_not_allocate_and_is_charged() {
        let mut w = World::with_total_capacity(8, 12, 20);
        w.load(Process::new(
            1,
            vec![
                Op::GraphNew { dst: 0, next: 1 },
                Op::Const {
                    dst: 1,
                    value: 30,
                    next: 2,
                },
                Op::GraphEncode {
                    graph: 0,
                    address: 1,
                    dst: 2,
                    next: 3,
                },
                Op::Halt,
            ],
            8,
        ))
        .unwrap();
        for (offset, value) in [200, 0, 0, 0, 0].into_iter().enumerate() {
            w.intervene(Intervention::SetCell {
                key: 30 + offset as i64,
                value,
            })
            .unwrap();
        }
        let before = w.occupied_units();
        assert_eq!(w.run(3), 3);
        assert_eq!(w.occupied_units(), before + 1);
        assert!(w.graph_object(1).unwrap().nodes.is_empty());
        assert_eq!(w.remaining_credits(), 5);
        assert!(matches!(
            w.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
    }

    #[test]
    fn internal_cil_writes_descriptor_and_constructs_opcode() {
        let mut w = World::with_total_capacity(5, 24, 30);
        w.load(Process::new(
            1,
            vec![
                Op::GraphNew { dst: 0, next: 1 },
                Op::Const {
                    dst: 1,
                    value: 100,
                    next: 2,
                },
                Op::Const {
                    dst: 2,
                    value: 19,
                    next: 3,
                },
                Op::Write {
                    address: 1,
                    value: 2,
                    next: 4,
                },
                Op::Const {
                    dst: 2,
                    value: 0,
                    next: 5,
                },
                Op::Const {
                    dst: 1,
                    value: 101,
                    next: 6,
                },
                Op::Write {
                    address: 1,
                    value: 2,
                    next: 7,
                },
                Op::Const {
                    dst: 1,
                    value: 102,
                    next: 8,
                },
                Op::Write {
                    address: 1,
                    value: 2,
                    next: 9,
                },
                Op::Const {
                    dst: 1,
                    value: 103,
                    next: 10,
                },
                Op::Write {
                    address: 1,
                    value: 2,
                    next: 11,
                },
                Op::Const {
                    dst: 1,
                    value: 104,
                    next: 12,
                },
                Op::Write {
                    address: 1,
                    value: 2,
                    next: 13,
                },
                Op::Const {
                    dst: 1,
                    value: 100,
                    next: 14,
                },
                Op::GraphEncode {
                    graph: 0,
                    address: 1,
                    dst: 3,
                    next: 15,
                },
                Op::Halt,
            ],
            20,
        ))
        .unwrap();
        assert_eq!(w.used_cells(), 0);
        assert_eq!(w.run(16), 16);
        assert_eq!(w.graph_object(1).unwrap().nodes, vec![Op::Halt]);
        assert_eq!(w.used_cells(), 5);
        assert_eq!(w.occupied_units(), 23);
        assert_eq!(w.remaining_credits(), 4);
        assert!(w.events().iter().any(|e| matches!(
            e.kind,
            EventKind::GraphEncoded {
                graph: 1,
                owner: 1,
                new_node: 0,
                descriptor_base: 100,
                descriptor: [19, 0, 0, 0, 0]
            }
        )));
        assert!(w
            .events()
            .iter()
            .all(|e| e.origin != Origin::External || matches!(e.kind, EventKind::Loaded)));
    }

    #[test]
    fn decoding_supports_branches_and_self_reference_without_templates() {
        assert_eq!(
            Op::decode_instruction([5, 0, 2, 3, 0]).unwrap(),
            Op::Branch {
                test: 0,
                nonzero: 2,
                zero: 3
            }
        );
        assert_eq!(
            Op::decode_instruction([18, 0, 1, 2, 3]).unwrap(),
            Op::GraphEncode {
                graph: 0,
                address: 1,
                dst: 2,
                next: 3
            }
        );
        assert!(Op::decode_instruction([0, -1, 12, 0, 0]).is_err());
        assert!(Op::decode_instruction([999, 0, 0, 0, 0]).is_err());
    }

    #[test]
    fn graph_read_authorization_and_revocation_follow_scheduler() {
        let mut w = World::with_total_capacity(0, 16, 20);
        w.load(Process::new(
            1,
            vec![
                Op::GraphNew { dst: 0, next: 1 },
                Op::Const {
                    dst: 1,
                    value: 3,
                    next: 2,
                },
                Op::GraphCopyNode {
                    graph: 0,
                    source: 1,
                    dst: 2,
                    next: 3,
                },
                Op::Const {
                    dst: 1,
                    value: 2,
                    next: 4,
                },
                Op::GraphGrantRead {
                    graph: 0,
                    reader: 1,
                    next: 5,
                },
                Op::GraphRevokeRead {
                    graph: 0,
                    reader: 1,
                    next: 6,
                },
                Op::Halt,
            ],
            12,
        ))
        .unwrap();
        w.load(Process::new(
            2,
            vec![
                Op::Const {
                    dst: 0,
                    value: 1,
                    next: 1,
                },
                Op::Const {
                    dst: 1,
                    value: 0,
                    next: 2,
                },
                Op::GraphReadTag {
                    dst: 2,
                    graph: 0,
                    node: 1,
                    next: 3,
                },
                Op::Halt,
            ],
            0,
        ))
        .unwrap();
        // Owner creates and explicitly grants read-only access; receiver has no compute yet.
        assert_eq!(w.run(5), 5);
        assert!(w.graph_object(1).unwrap().readers.contains(&2));
        w.intervene(Intervention::SetCredits {
            process: 1,
            credits: 0,
        })
        .unwrap();
        w.intervene(Intervention::SetCredits {
            process: 2,
            credits: 3,
        })
        .unwrap();
        assert_eq!(w.run(3), 3);
        assert!(matches!(
            w.events().last().unwrap().kind,
            EventKind::GraphTagRead {
                graph: 1,
                owner: 1,
                node: 0,
                tag: 0
            }
        ));
        assert_eq!(w.events().last().unwrap().origin, Origin::Internal(2));
        assert_eq!(w.process(2).unwrap().registers[2], 0);
        w.intervene(Intervention::SetCredits {
            process: 1,
            credits: 1,
        })
        .unwrap();
        assert_eq!(w.run(1), 1);
        assert!(w.graph_object(1).unwrap().readers.is_empty());
    }

    #[test]
    fn graph_field_read_records_exact_source_owner_and_word() {
        let mut world = World::with_total_capacity(0, 16, 40);
        world
            .load(Process::new(
                1,
                vec![
                    Op::GraphNew { dst: 0, next: 1 },
                    Op::Const {
                        dst: 2,
                        value: 3,
                        next: 2,
                    },
                    Op::GraphCopyNode {
                        graph: 0,
                        source: 2,
                        dst: 3,
                        next: 3,
                    },
                    Op::Const {
                        dst: 1,
                        value: 42,
                        next: 4,
                    },
                    Op::Const {
                        dst: 1,
                        value: 2,
                        next: 5,
                    },
                    Op::GraphGrantRead {
                        graph: 0,
                        reader: 1,
                        next: 6,
                    },
                    Op::Halt,
                ],
                6,
            ))
            .unwrap();
        world
            .load(Process::new(
                2,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 1,
                        next: 1,
                    },
                    Op::Const {
                        dst: 1,
                        value: 0,
                        next: 2,
                    },
                    Op::Const {
                        dst: 2,
                        value: 2,
                        next: 3,
                    },
                    Op::GraphReadField {
                        graph: 0,
                        node: 1,
                        field: 2,
                        next: 4,
                    },
                    Op::Halt,
                ],
                0,
            ))
            .unwrap();
        assert_eq!(world.run(6), 6);
        world
            .intervene(Intervention::SetCredits {
                process: 2,
                credits: 4,
            })
            .unwrap();
        assert_eq!(world.run(4), 4);
        let event = world.events().last().unwrap();
        assert_eq!(event.origin, Origin::Internal(2));
        assert_eq!(event.process, 2);
        assert_eq!(event.pc, 3);
        assert!(matches!(
            event.kind,
            EventKind::GraphFieldRead {
                graph: 1,
                owner: 1,
                node: 0,
                field: 2,
                word: 42
            }
        ));
        assert_eq!(world.process(2).unwrap().registers[2], 42);
        assert_eq!(world.remaining_credits(), 0);
    }

    #[test]
    fn guessed_graph_handle_cannot_read_without_grant() {
        let mut w = World::with_total_capacity(0, 10, 14);
        w.load(Process::new(
            1,
            vec![Op::GraphNew { dst: 0, next: 1 }, Op::Halt],
            3,
        ))
        .unwrap();
        w.load(Process::new(
            2,
            vec![
                Op::Const {
                    dst: 0,
                    value: 1,
                    next: 1,
                },
                Op::Const {
                    dst: 1,
                    value: 0,
                    next: 2,
                },
                Op::GraphReadTag {
                    dst: 2,
                    graph: 0,
                    node: 1,
                    next: 3,
                },
                Op::Halt,
            ],
            5,
        ))
        .unwrap();
        w.run(7);
        assert_eq!(w.process(2).unwrap().registers[2], 0);
        assert!(matches!(
            w.events()
                .iter()
                .rev()
                .find(|e| e.process == 2 && e.origin == Origin::Internal(2))
                .unwrap()
                .kind,
            EventKind::Trapped(_)
        ));
    }

    #[test]
    fn strict_capacity_prices_process_headers_and_read_grants() {
        let mut world = World::with_strict_capacity(0, 16, 10);
        world
            .load(Process::new(
                1,
                vec![
                    Op::GraphNew { dst: 0, next: 1 },
                    Op::Const {
                        dst: 1,
                        value: 2,
                        next: 2,
                    },
                    Op::GraphGrantRead {
                        graph: 0,
                        reader: 1,
                        next: 3,
                    },
                    Op::GraphRevokeRead {
                        graph: 0,
                        reader: 1,
                        next: 4,
                    },
                    Op::Halt,
                ],
                8,
            ))
            .unwrap();
        world.load(Process::new(2, vec![Op::Halt], 0)).unwrap();
        assert!(world.strict_accounting());
        assert_eq!(world.occupied_units(), 8);
        assert_eq!(world.run(1), 1);
        assert_eq!(world.occupied_units(), 9);
        assert_eq!(world.run(2), 2);
        assert_eq!(world.occupied_units(), 10);
        assert!(world.graph_object(1).unwrap().readers.contains(&2));
        assert_eq!(world.run(1), 1);
        assert_eq!(world.occupied_units(), 9);
        assert!(!world.graph_object(1).unwrap().readers.contains(&2));
    }

    #[test]
    fn strict_capacity_rejects_grant_without_leaking_storage() {
        let mut world = World::with_strict_capacity(0, 16, 9);
        world
            .load(Process::new(
                1,
                vec![
                    Op::GraphNew { dst: 0, next: 1 },
                    Op::Const {
                        dst: 1,
                        value: 2,
                        next: 2,
                    },
                    Op::GraphGrantRead {
                        graph: 0,
                        reader: 1,
                        next: 3,
                    },
                    Op::Halt,
                ],
                6,
            ))
            .unwrap();
        world.load(Process::new(2, vec![Op::Halt], 0)).unwrap();
        assert_eq!(world.occupied_units(), 7);
        world.run(2);
        assert_eq!(world.occupied_units(), 8);
        // One free slot allows a grant, proving that it is actually charged.
        assert_eq!(world.run(1), 1);
        assert_eq!(world.occupied_units(), 9);
        assert!(world.graph_object(1).unwrap().readers.contains(&2));
        let mut smaller = World::with_strict_capacity(0, 16, 8);
        smaller
            .load(Process::new(
                1,
                vec![
                    Op::GraphNew { dst: 0, next: 1 },
                    Op::Const {
                        dst: 1,
                        value: 2,
                        next: 2,
                    },
                    Op::GraphGrantRead {
                        graph: 0,
                        reader: 1,
                        next: 3,
                    },
                    Op::Halt,
                ],
                6,
            ))
            .unwrap();
        smaller.load(Process::new(2, vec![Op::Halt], 0)).unwrap();
        smaller.run(3);
        assert!(smaller.graph_object(1).unwrap().readers.is_empty());
        assert_eq!(smaller.occupied_units(), 8);
        assert!(matches!(
            smaller.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
    }

    #[test]
    fn compute_pool_draw_and_return_preserve_inventory_except_execution_cost() {
        let mut w = World::with_strict_capacity(0, 10, 8);
        w.load(Process::new(
            1,
            vec![
                Op::Const {
                    dst: 0,
                    value: 3,
                    next: 1,
                },
                Op::DrawCompute { amount: 0, next: 2 },
                Op::ReturnCompute { amount: 0, next: 3 },
                Op::Halt,
            ],
            5,
        ))
        .unwrap();
        w.intervene(Intervention::SetComputePool { credits: 10 })
            .unwrap();
        assert_eq!(w.total_compute_inventory(), Some(15));
        assert_eq!(w.run(4), 4);
        assert_eq!(w.total_compute_inventory(), Some(11));
        assert_eq!(w.available_compute_pool(), 10);
        assert_eq!(w.remaining_credits(), 1);
    }

    #[test]
    fn internal_credit_transfer_cannot_mint_credits() {
        let mut w = World::with_strict_capacity(0, 12, 10);
        w.load(Process::new(
            1,
            vec![
                Op::Const {
                    dst: 0,
                    value: 2,
                    next: 1,
                },
                Op::Const {
                    dst: 1,
                    value: 4,
                    next: 2,
                },
                Op::TransferCompute {
                    recipient: 0,
                    amount: 1,
                    next: 3,
                },
                Op::Halt,
            ],
            10,
        ))
        .unwrap();
        w.load(Process::new(2, vec![Op::Halt], 0)).unwrap();
        assert_eq!(w.run(3), 3);
        assert_eq!(w.process(1).unwrap().credits, 3);
        assert_eq!(w.process(2).unwrap().credits, 4);
        assert_eq!(w.total_compute_inventory(), Some(7));
    }

    #[test]
    fn compute_pool_rejection_is_atomic_and_charged() {
        let mut w = World::with_strict_capacity(0, 10, 10);
        w.load(Process::new(
            1,
            vec![
                Op::Const {
                    dst: 0,
                    value: 6,
                    next: 1,
                },
                Op::DrawCompute { amount: 0, next: 2 },
                Op::Halt,
            ],
            4,
        ))
        .unwrap();
        w.intervene(Intervention::SetComputePool { credits: 2 })
            .unwrap();
        w.run(2);
        assert_eq!(w.available_compute_pool(), 2);
        assert_eq!(w.remaining_credits(), 2);
        assert!(matches!(
            w.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
    }

    #[test]
    fn external_credit_overflow_is_rejected_without_mutating_balances() {
        let mut w = World::with_strict_capacity(0, 10, 4);
        w.load(Process::new(1, vec![Op::Halt], 1)).unwrap();
        w.intervene(Intervention::SetComputePool {
            credits: u64::MAX - 1,
        })
        .unwrap();
        assert!(w
            .intervene(Intervention::SetCredits {
                process: 1,
                credits: 2
            })
            .is_err());
        assert_eq!(w.process(1).unwrap().credits, 1);
        assert_eq!(w.total_compute_inventory(), Some(u64::MAX));
        assert!(w.load(Process::new(2, vec![Op::Halt], 1)).is_err());
        assert!(w.process(2).is_none());
    }

    #[test]
    fn instruction_descriptor_is_round_trip_for_all_opcodes() {
        let examples = [
            Op::Const {
                dst: 0,
                value: -17,
                next: 1,
            },
            Op::Branch {
                test: 0,
                nonzero: 2,
                zero: 3,
            },
            Op::GraphGrantRead {
                graph: 0,
                reader: 1,
                next: 2,
            },
            Op::DrawCompute { amount: 0, next: 1 },
            Op::GraphReadField {
                graph: 0,
                node: 1,
                field: 2,
                next: 3,
            },
            Op::GraphReplace {
                graph: 0,
                node: 1,
                address: 2,
                next: 3,
            },
            Op::Halt,
        ];
        for op in examples {
            assert_eq!(
                Op::decode_instruction(op.encode_instruction().unwrap()).unwrap(),
                op
            );
        }
    }

    #[test]
    fn graph_replace_can_edit_branch_structure_without_special_semantics() {
        let mut w = World::with_strict_capacity(5, 14, 25);
        w.load(Process::new(
            1,
            vec![
                Op::GraphNew { dst: 0, next: 1 },
                Op::Const {
                    dst: 1,
                    value: 100,
                    next: 2,
                },
                Op::Const {
                    dst: 2,
                    value: 2,
                    next: 3,
                },
                Op::GraphCopyNode {
                    graph: 0,
                    source: 2,
                    dst: 3,
                    next: 4,
                },
                Op::Const {
                    dst: 2,
                    value: 0,
                    next: 5,
                },
                Op::GraphReplace {
                    graph: 0,
                    node: 2,
                    address: 1,
                    next: 6,
                },
                Op::Halt,
            ],
            12,
        ))
        .unwrap();
        for (offset, word) in [5, 0, 0, 0, 0].into_iter().enumerate() {
            w.intervene(Intervention::SetCell {
                key: 100 + offset as i64,
                value: word,
            })
            .unwrap();
        }
        assert_eq!(w.run(6), 6);
        assert_eq!(
            w.graph_object(1).unwrap().nodes[0],
            Op::Branch {
                test: 0,
                nonzero: 0,
                zero: 0
            }
        );
        // Replacement overwrites exactly one node and does not allocate any extra node.
        assert_eq!(w.graph_object(1).unwrap().nodes.len(), 1);
        assert!(w.events().iter().any(|e| matches!(
            e.kind,
            EventKind::GraphReplaced {
                graph: 1,
                owner: 1,
                node: 0,
                descriptor_base: 100,
                descriptor: [5, 0, 0, 0, 0]
            }
        )));
    }

    #[test]
    fn external_graph_ablation_is_logged_and_refunds_strict_storage() {
        let mut w = World::with_strict_capacity(0, 12, 16);
        w.load(Process::new(
            1,
            vec![Op::GraphNew { dst: 0, next: 1 }, Op::Halt],
            3,
        ))
        .unwrap();
        assert_eq!(w.occupied_units(), 3);
        assert_eq!(w.run(1), 1);
        assert_eq!(w.occupied_units(), 4);
        assert_eq!(w.graph_count(), 1);
        let credits = w.remaining_credits();
        w.intervene(Intervention::DropGraph { graph: 1 }).unwrap();
        assert_eq!(w.occupied_units(), 3);
        assert_eq!(w.graph_count(), 0);
        assert_eq!(w.remaining_credits(), credits);
        assert_eq!(w.events().last().unwrap().origin, Origin::External);
        let prior = w.occupied_units();
        assert!(w.intervene(Intervention::DropGraph { graph: 1 }).is_err());
        assert_eq!(w.occupied_units(), prior);
        assert!(matches!(
            &w.events().last().unwrap().kind,
            EventKind::InterventionRejected(_)
        ));
    }

    #[test]
    fn cross_owner_copy_requires_explicit_grant_and_transfer_changes_owner() {
        let mut w = World::with_strict_capacity(0, 24, 32);
        w.load(Process::new(
            1,
            vec![
                Op::GraphNew { dst: 0, next: 1 },
                Op::Const {
                    dst: 1,
                    value: 1,
                    next: 2,
                },
                Op::GraphCopyNode {
                    graph: 0,
                    source: 1,
                    dst: 2,
                    next: 3,
                },
                Op::Const {
                    dst: 1,
                    value: 2,
                    next: 4,
                },
                Op::GraphGrantRead {
                    graph: 0,
                    reader: 1,
                    next: 5,
                },
                Op::Halt,
            ],
            10,
        ))
        .unwrap();
        w.load(Process::new(
            2,
            vec![
                Op::Const {
                    dst: 0,
                    value: 1,
                    next: 1,
                },
                Op::GraphNew { dst: 1, next: 2 },
                Op::Const {
                    dst: 2,
                    value: 0,
                    next: 3,
                },
                Op::GraphCopyFrom {
                    target: 1,
                    source: 0,
                    node: 2,
                    next: 4,
                },
                Op::Const {
                    dst: 3,
                    value: 1,
                    next: 5,
                },
                Op::GraphTransfer {
                    graph: 1,
                    recipient: 3,
                    next: 6,
                },
                Op::Halt,
            ],
            0,
        ))
        .unwrap();
        // Freeze the recipient while the original owner creates and grants source access.
        assert_eq!(w.run(5), 5);
        assert_eq!(w.graph_object(1).unwrap().nodes.len(), 1);
        assert!(w.graph_object(1).unwrap().readers.contains(&2));
        w.intervene(Intervention::SetCredits {
            process: 1,
            credits: 0,
        })
        .unwrap();
        w.intervene(Intervention::SetCredits {
            process: 2,
            credits: 8,
        })
        .unwrap();
        assert_eq!(w.run(6), 6);
        let assembled = w.graph_object(2).unwrap();
        assert_eq!(assembled.owner, 1);
        assert_eq!(
            assembled.nodes,
            vec![Op::Const {
                dst: 1,
                value: 1,
                next: 2
            }]
        );
        assert_eq!(w.process(2).unwrap().registers[2], 0);
        assert_eq!(w.occupied_units(), 20);
        assert!(w.events().iter().any(|e| matches!(
            &e.kind,
            EventKind::GraphCopied {
                source_graph: 1,
                source_owner: 1,
                source_node: 0,
                target_graph: 2,
                target_owner: 2,
                new_node: 0
            }
        )));
        assert!(w.events().iter().any(|e| matches!(
            &e.kind,
            EventKind::GraphTransferred {
                graph: 2,
                former_owner: 2,
                new_owner: 1
            }
        )));
    }

    #[test]
    fn ownership_transfer_is_not_a_grant_or_a_code_clone() {
        let mut w = World::with_strict_capacity(0, 12, 22);
        w.load(Process::new(
            1,
            vec![
                Op::GraphNew { dst: 0, next: 1 },
                Op::Const {
                    dst: 1,
                    value: 2,
                    next: 2,
                },
                Op::GraphTransfer {
                    graph: 0,
                    recipient: 1,
                    next: 3,
                },
                Op::GraphDrop { graph: 0, next: 4 },
                Op::Halt,
            ],
            6,
        ))
        .unwrap();
        w.load(Process::new(2, vec![Op::Halt], 0)).unwrap();
        assert_eq!(w.run(3), 3);
        assert_eq!(w.graph_object(1).unwrap().owner, 2);
        assert_eq!(w.run(1), 1);
        assert!(w.process(1).unwrap().stopped);
        assert_eq!(w.graph_object(1).unwrap().owner, 2);
        assert!(matches!(
            w.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
    }

    #[test]
    fn graph_copy_from_inaccessible_source_traps_without_allocation() {
        let mut w = World::with_strict_capacity(0, 12, 24);
        w.load(Process::new(
            1,
            vec![
                Op::GraphNew { dst: 0, next: 1 },
                Op::GraphCopyNode {
                    graph: 0,
                    source: 0,
                    dst: 1,
                    next: 2,
                },
                Op::Halt,
            ],
            4,
        ))
        .unwrap();
        w.load(Process::new(
            2,
            vec![
                Op::Const {
                    dst: 0,
                    value: 1,
                    next: 1,
                },
                Op::GraphNew { dst: 1, next: 2 },
                Op::Const {
                    dst: 2,
                    value: 0,
                    next: 3,
                },
                Op::GraphCopyFrom {
                    target: 1,
                    source: 0,
                    node: 2,
                    next: 4,
                },
                Op::Halt,
            ],
            0,
        ))
        .unwrap();
        assert_eq!(w.run(2), 2);
        w.intervene(Intervention::SetCredits {
            process: 1,
            credits: 0,
        })
        .unwrap();
        w.intervene(Intervention::SetCredits {
            process: 2,
            credits: 5,
        })
        .unwrap();
        assert_eq!(w.run(4), 4);
        assert!(w.graph_object(2).unwrap().nodes.is_empty());
        assert!(!w
            .events()
            .iter()
            .any(|e| matches!(e.kind, EventKind::GraphCopied { .. })));
        assert!(matches!(
            &w.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
    }

    #[test]
    fn new_graph_instruction_tags_roundtrip() {
        for op in [
            Op::GraphCopyFrom {
                target: 1,
                source: 2,
                node: 3,
                next: 4,
            },
            Op::GraphTransfer {
                graph: 0,
                recipient: 3,
                next: 2,
            },
        ] {
            assert_eq!(
                Op::decode_instruction(op.encode_instruction().unwrap()).unwrap(),
                op
            );
        }
    }

    #[test]
    fn structured_internal_graph_lineage_is_charged_and_exact() {
        let mut w = World::with_strict_capacity(0, 16, 28);
        w.load(Process::new(
            1,
            vec![
                Op::GraphNew { dst: 0, next: 1 },
                Op::Const {
                    dst: 2,
                    value: 3,
                    next: 2,
                },
                Op::GraphCopyNode {
                    graph: 0,
                    source: 2,
                    dst: 1,
                    next: 3,
                },
                Op::Const {
                    dst: 2,
                    value: 0,
                    next: 4,
                },
                Op::GraphSetConst {
                    graph: 0,
                    node: 2,
                    value: 2,
                    next: 5,
                },
                Op::GraphSetNext {
                    graph: 0,
                    node: 2,
                    target: 2,
                    next: 6,
                },
                Op::Halt,
            ],
            8,
        ))
        .unwrap();
        assert_eq!(w.run(6), 6);
        assert!(matches!(
            &w.events()[1].kind,
            EventKind::GraphCreated { graph: 1, owner: 1 }
        ));
        assert!(matches!(
            &w.events()[3].kind,
            EventKind::GraphNodeCopied {
                graph: 1,
                owner: 1,
                source_process: 1,
                source_node: 3,
                new_node: 0
            }
        ));
        // Both edits succeed on the copied Const node at index 0.
        assert!(matches!(&w.events()[5].kind, EventKind::GraphNodeEdited {
            graph: 1, owner: 1, node: 0, edit
        } if edit == "const"));
        assert!(matches!(&w.events()[6].kind, EventKind::GraphNodeEdited {
            graph: 1, owner: 1, node: 0, edit
        } if edit == "next"));
        assert_eq!(w.remaining_credits(), 2);
        assert_eq!(
            w.events()
                .iter()
                .filter(|e| matches!(e.kind, EventKind::GraphCreated { .. }))
                .count(),
            1
        );
        assert_eq!(w.graph_object(1).unwrap().nodes.len(), 1);
    }

    #[test]
    fn cell_io_events_are_success_only_and_do_not_mint_resources() {
        let mut world = World::with_strict_capacity(1, 8, 12);
        world
            .load(Process::new(
                1,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 7,
                        next: 1,
                    },
                    Op::Read {
                        dst: 1,
                        address: 0,
                        next: 2,
                    },
                    Op::Write {
                        address: 0,
                        value: 1,
                        next: 3,
                    },
                    Op::Read {
                        dst: 2,
                        address: 0,
                        next: 4,
                    },
                    Op::Const {
                        dst: 0,
                        value: 8,
                        next: 5,
                    },
                    Op::Write {
                        address: 0,
                        value: 1,
                        next: 6,
                    },
                    Op::Halt,
                ],
                7,
            ))
            .unwrap();
        let initial_inventory = world.total_compute_inventory().unwrap();
        assert_eq!(world.run(6), 6);
        assert!(matches!(
            world.events()[2].kind,
            EventKind::CellRead { key: 7, value: 0 }
        ));
        assert!(matches!(
            world.events()[3].kind,
            EventKind::CellWritten { key: 7, value: 0 }
        ));
        assert!(matches!(
            world.events()[4].kind,
            EventKind::CellRead { key: 7, value: 0 }
        ));
        assert!(matches!(world.events()[6].kind, EventKind::Trapped(_)));
        assert_eq!(
            world
                .events()
                .iter()
                .filter(|e| matches!(e.kind, EventKind::CellWritten { .. }))
                .count(),
            1
        );
        assert_eq!(world.remaining_credits(), initial_inventory - 6);
        assert_eq!(world.cell(7), Some(0));
        assert_eq!(world.cell(8), None);
    }

    #[test]
    fn external_graph_injection_restores_exact_object_without_compute_mint() {
        let mut world = World::with_strict_capacity(0, 10, 30);
        world
            .load(Process::new(
                1,
                vec![
                    Op::GraphNew { dst: 0, next: 1 },
                    Op::Const {
                        dst: 1,
                        value: 2,
                        next: 2,
                    },
                    Op::GraphGrantRead {
                        graph: 0,
                        reader: 1,
                        next: 3,
                    },
                    Op::GraphCopyNode {
                        graph: 0,
                        source: 2,
                        dst: 2,
                        next: 4,
                    },
                    Op::Halt,
                ],
                7,
            ))
            .unwrap();
        world.load(Process::new(2, vec![Op::Halt], 0)).unwrap();
        assert_eq!(world.run(4), 4);
        let before = world.graph_object(1).unwrap().clone();
        assert_eq!(before.readers.len(), 1);
        let occupancy = world.occupied_units();
        let credits = world.total_compute_inventory().unwrap();
        world
            .intervene(Intervention::DropGraph { graph: 1 })
            .unwrap();
        assert!(world.graph_object(1).is_none());
        world
            .intervene(Intervention::PutGraph {
                graph: 1,
                object: before.clone(),
            })
            .unwrap();
        assert_eq!(world.graph_object(1), Some(&before));
        assert_eq!(world.occupied_units(), occupancy);
        assert_eq!(world.total_compute_inventory(), Some(credits));
        assert_eq!(world.events().last().unwrap().origin, Origin::External);
        assert!(matches!(
            &world.events().last().unwrap().kind,
            EventKind::Intervened(text) if text.contains("PutGraph")
        ));
    }

    #[test]
    fn external_graph_injection_invalid_cases_are_atomic() {
        let mut world = World::with_strict_capacity(1, 8, 6);
        world
            .load(Process::new(
                1,
                vec![
                    Op::GraphNew { dst: 0, next: 1 },
                    Op::GraphCopyNode {
                        graph: 0,
                        source: 0,
                        dst: 1,
                        next: 2,
                    },
                    Op::Halt,
                ],
                4,
            ))
            .unwrap();
        assert_eq!(world.run(2), 2);
        let saved = world.graph_object(1).unwrap().clone();
        // An occupied handle cannot be injected over, and no data changes.
        let occupied = world.occupied_units();
        assert!(world
            .intervene(Intervention::PutGraph {
                graph: 1,
                object: saved.clone(),
            })
            .is_err());
        assert_eq!(world.occupied_units(), occupied);
        world
            .intervene(Intervention::DropGraph { graph: 1 })
            .unwrap();
        for graph in [0, 2] {
            assert!(world
                .intervene(Intervention::PutGraph {
                    graph,
                    object: saved.clone(),
                })
                .is_err());
        }
        let mut invalid_owner = saved.clone();
        invalid_owner.owner = 99;
        assert!(world
            .intervene(Intervention::PutGraph {
                graph: 1,
                object: invalid_owner,
            })
            .is_err());
        let mut invalid_reader = saved.clone();
        invalid_reader.readers.insert(99);
        assert!(world
            .intervene(Intervention::PutGraph {
                graph: 1,
                object: invalid_reader,
            })
            .is_err());
        let mut empty = saved.clone();
        empty.nodes.clear();
        assert!(world
            .intervene(Intervention::PutGraph {
                graph: 1,
                object: empty,
            })
            .is_err());
        let mut oversized = saved.clone();
        oversized.nodes = vec![Op::Halt; 9];
        assert!(world
            .intervene(Intervention::PutGraph {
                graph: 1,
                object: oversized,
            })
            .is_err());
        // Storage refunded by DropGraph is now partially occupied by a cell.
        world
            .intervene(Intervention::SetCell { key: 8, value: 1 })
            .unwrap();
        let before_rejection = world.occupied_units();
        assert!(world
            .intervene(Intervention::PutGraph {
                graph: 1,
                object: saved,
            })
            .is_err());
        assert_eq!(world.occupied_units(), before_rejection);
        assert!(world.graph_object(1).is_none());
    }

    #[test]
    fn stable_round_robin_by_id() {
        let mut world = World::new(0, 2);
        world
            .load(Process::new(
                8,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 1,
                        next: 1,
                    },
                    Op::Halt,
                ],
                2,
            ))
            .unwrap();
        world
            .load(Process::new(
                2,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 1,
                        next: 1,
                    },
                    Op::Halt,
                ],
                2,
            ))
            .unwrap();
        assert_eq!(world.run(4), 4);
        let ids: Vec<_> = world
            .events()
            .iter()
            .filter(|e| e.origin != Origin::External)
            .map(|e| e.process)
            .collect();
        assert_eq!(ids, vec![2, 8, 2, 8]);
    }
    #[test]
    fn graph_copy_node_local_transports_all_edges_without_changing_absolute_copy() {
        for local in [false, true] {
            let copy = if local {
                Op::GraphCopyNodeLocal {
                    graph: 0,
                    source: 1,
                    dst: 2,
                    next: 3,
                }
            } else {
                Op::GraphCopyNode {
                    graph: 0,
                    source: 1,
                    dst: 2,
                    next: 3,
                }
            };
            let mut world = World::with_strict_capacity(2, 16, 64);
            world
                .load(Process::new(
                    1,
                    vec![
                        Op::GraphNew { dst: 0, next: 1 },
                        Op::Const {
                            dst: 1,
                            value: 4,
                            next: 2,
                        },
                        copy,
                        Op::Halt,
                        Op::Write {
                            address: 0,
                            value: 2,
                            next: 5,
                        },
                        Op::Halt,
                    ],
                    5,
                ))
                .unwrap();
            assert_eq!(world.run(3), 3);
            assert_eq!(
                world.graph_object(1).unwrap().nodes[0],
                Op::Write {
                    address: 0,
                    value: 2,
                    next: if local { 0 } else { 5 },
                }
            );
            assert!(
                matches!(
                    world.events().last().unwrap().kind,
                    EventKind::GraphNodeCopiedLocal { .. }
                ) == local
            );
            assert_eq!(world.total_compute_inventory(), Some(2));
        }
    }

    #[test]
    fn generic_graph_local_copy_relates_both_branch_edges_and_preserves_opcode_abi() {
        let mut graph_branch = Op::Branch {
            test: 1,
            nonzero: 4,
            zero: 9,
        };
        graph_branch.relocate_edges(7, 1);
        assert_eq!(
            graph_branch,
            Op::Branch {
                test: 1,
                nonzero: 0,
                zero: 1,
            }
        );
        let local = Op::GraphCopyNodeLocal {
            graph: 3,
            source: 2,
            dst: 0,
            next: 17,
        };
        let descriptor = local.encode_instruction().unwrap();
        assert_eq!(descriptor[0], 30);
        assert_eq!(Op::decode_instruction(descriptor).unwrap(), local);
        assert_eq!(
            Op::GraphCopyNode {
                graph: 3,
                source: 2,
                dst: 0,
                next: 17,
            }
            .encode_instruction()
            .unwrap()[0],
            12
        );
    }

    #[test]
    fn pool_paid_native_draw_is_opt_in_and_energy_conserving() {
        let code = vec![
            Op::Const {
                dst: 1,
                value: 3,
                next: 1,
            },
            Op::DrawCompute { amount: 1, next: 2 },
            Op::Const {
                dst: 0,
                value: 7,
                next: 3,
            },
            Op::Halt,
        ];
        let mut original = World::new(2, 8);
        original.load(Process::new(1, code, 1)).unwrap();
        original
            .intervene(Intervention::SetComputePool { credits: 6 })
            .unwrap();
        assert_eq!(original.run(1), 1); // native Const set amount=3, actor now zero
        assert_eq!(original.process(1).unwrap().credits, 0);
        assert_eq!(original.available_compute_pool(), 6);
        let mut baseline = original.clone();
        assert_eq!(baseline.run(4), 0); // legacy scheduler must remain inert
        let mut permitted = original.with_pool_paid_draw();
        assert!(permitted.pool_paid_draw_enabled());
        assert_eq!(permitted.run(1), 1); // one pool-paid native Draw attempt
        assert_eq!(permitted.available_compute_pool(), 2); // -1 fee, -3 Draw
        assert_eq!(permitted.process(1).unwrap().credits, 3);
        assert_eq!(permitted.total_compute_inventory(), Some(5)); // 6 - 1 charged attempt
        let last = permitted.events().last().unwrap();
        assert!(last.pool_paid_attempt);
        assert_eq!(last.origin, Origin::Internal(1));
        assert!(matches!(last.kind, EventKind::Executed));
        assert_eq!(permitted.run(1), 1); // subsequent Const uses own credits
        assert!(!permitted.events().last().unwrap().pool_paid_attempt);
        assert_eq!(permitted.total_compute_inventory(), Some(4));
        assert_eq!(permitted.process(1).unwrap().registers[0], 7);
        // Default-world JSON and old events must not acquire new fields.
        let legacy = serde_json::to_string(&baseline).unwrap();
        assert!(!legacy.contains("pool_paid_draw"));
        assert!(!legacy.contains("pool_paid_attempt"));
        let enabled = serde_json::to_string(&permitted).unwrap();
        assert!(enabled.contains("\"pool_paid_draw\":true"));
        assert!(enabled.contains("\"pool_paid_attempt\":true"));
    }

    #[test]
    fn pool_paid_draw_does_not_execute_others_recover_traps_or_mint_credits() {
        let pending = vec![
            Op::Const {
                dst: 1,
                value: 9,
                next: 1,
            },
            Op::DrawCompute { amount: 1, next: 2 },
            Op::Halt,
        ];
        let mut failure = World::new(0, 8).with_pool_paid_draw();
        failure.load(Process::new(1, pending.clone(), 1)).unwrap();
        failure
            .intervene(Intervention::SetComputePool { credits: 5 })
            .unwrap();
        assert_eq!(failure.run(1), 1);
        assert_eq!(failure.run(1), 1); // pool fee leaves only 4, amount9 traps
        assert_eq!(failure.available_compute_pool(), 4);
        assert_eq!(failure.total_compute_inventory(), Some(4));
        assert!(failure.events().last().unwrap().pool_paid_attempt);
        assert!(matches!(
            failure.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
        assert!(failure.process(1).unwrap().stopped);
        assert_eq!(failure.run(5), 0); // even with remaining pool: no resurrection

        let mut no_pool = World::new(0, 8).with_pool_paid_draw();
        no_pool.load(Process::new(1, pending.clone(), 1)).unwrap();
        assert_eq!(no_pool.run(1), 1);
        assert_eq!(no_pool.run(1), 0);

        let mut wrong_opcode = World::new(0, 8).with_pool_paid_draw();
        wrong_opcode
            .load(Process::new(
                1,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 1,
                        next: 1,
                    },
                    Op::Const {
                        dst: 1,
                        value: 3,
                        next: 1,
                    },
                ],
                1,
            ))
            .unwrap();
        wrong_opcode
            .intervene(Intervention::SetComputePool { credits: 5 })
            .unwrap();
        assert_eq!(wrong_opcode.run(1), 1);
        assert_eq!(wrong_opcode.run(1), 0);

        let mut disabled = World::new(0, 8).with_pool_paid_draw();
        disabled.load(Process::new(1, pending, 1)).unwrap();
        disabled
            .intervene(Intervention::SetComputePool { credits: 5 })
            .unwrap();
        assert_eq!(disabled.run(1), 1);
        disabled
            .intervene(Intervention::Disable { process: 1 })
            .unwrap();
        assert_eq!(disabled.run(3), 0);
    }

    #[test]
    fn opt_in_recoverable_draw_defers_charged_attempt_and_continues_when_pool_refills() {
        let mut p = Process::new(
            1,
            vec![
                Op::Const {
                    dst: 1,
                    value: 8,
                    next: 1,
                },
                Op::DrawCompute { amount: 1, next: 2 },
                Op::Const {
                    dst: 0,
                    value: 7,
                    next: 3,
                },
                Op::Write {
                    address: 2,
                    value: 0,
                    next: 4,
                },
                Op::Halt,
            ],
            2,
        );
        p.registers[2] = 9;
        let mut world = World::new(16, 16)
            .with_pool_paid_draw()
            .with_recoverable_draw_shortage();
        world.load(p).unwrap();
        world
            .intervene(Intervention::SetComputePool { credits: 4 })
            .unwrap();
        assert_eq!(world.run(2), 2);
        let actor = world.process(1).unwrap();
        assert!(!actor.stopped);
        assert_eq!(actor.pc, 1);
        assert_eq!(actor.credits, 0);
        assert_eq!(world.available_compute_pool(), 4);
        assert!(matches!(
            world.events().last().unwrap().kind,
            EventKind::DrawDeferred {
                requested: 8,
                available: 4
            }
        ));
        assert!(!world.events().last().unwrap().pool_paid_attempt);
        // Ambient resource arrives to the WORLD only, never SetCredits to the actor.
        world
            .intervene(Intervention::SetComputePool { credits: 16 })
            .unwrap();
        assert_eq!(world.run(3), 3);
        assert_eq!(world.process(1).unwrap().credits, 6);
        assert_eq!(world.available_compute_pool(), 7);
        assert!(world
            .events()
            .iter()
            .any(|e| e.pool_paid_attempt && matches!(e.kind, EventKind::Executed)));
        assert!(world
            .events()
            .iter()
            .any(|e| matches!(e.kind, EventKind::CellWritten { key: 9, value: 7 })));
        assert_eq!(world.total_compute_inventory(), Some(13));
    }

    #[test]
    fn recoverable_pool_paid_draw_waits_for_atomic_fee_plus_requested_energy() {
        let mut actor = Process::new(1, vec![Op::DrawCompute { amount: 1, next: 1 }, Op::Halt], 0);
        actor.registers[1] = 8;
        let mut world = World::new(0, 8)
            .with_pool_paid_draw()
            .with_recoverable_draw_shortage();
        world.load(actor.clone()).unwrap();
        world
            .intervene(Intervention::SetComputePool { credits: 8 })
            .unwrap();
        assert_eq!(world.run(4), 0);
        assert!(!world.process(1).unwrap().stopped);
        assert_eq!(world.total_compute_inventory(), Some(8));
        world
            .intervene(Intervention::SetComputePool { credits: 16 })
            .unwrap();
        assert_eq!(world.run(1), 1);
        assert!(world.events().last().unwrap().pool_paid_attempt);
        assert_eq!(world.available_compute_pool(), 7);
        assert_eq!(world.process(1).unwrap().credits, 8);
        assert_eq!(world.total_compute_inventory(), Some(15));

        let mut legacy = World::new(0, 8).with_pool_paid_draw();
        legacy.load(actor).unwrap();
        legacy
            .intervene(Intervention::SetComputePool { credits: 8 })
            .unwrap();
        assert_eq!(legacy.run(1), 1);
        assert!(legacy.process(1).unwrap().stopped);
        assert!(matches!(
            legacy.events().last().unwrap().kind,
            EventKind::Trapped(_)
        ));
        assert!(!legacy.recoverable_draw_shortage_enabled());
    }
}
