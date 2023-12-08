use crate::AdventErr;
use crate::AdventErr::InputParse;

#[derive(Debug, Copy, Clone)]
pub(super) struct Cycle {
    pub length: u64,
    pub offset: u64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = AdventErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(InputParse(format!(
                "Invalid character for instruction '{value}'"
            ))),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) struct Node {
    pub left: usize,
    pub right: usize,
}

impl Node {
    pub(super) fn empty() -> Self {
        Self { left: 0, right: 0 }
    }
}

#[derive(Debug, Clone)]
pub(super) struct MapSpec {
    pub aaa: usize,
    pub zzz: usize,
    pub start_nodes: Vec<usize>,
    pub end_nodes: Vec<usize>,
}
