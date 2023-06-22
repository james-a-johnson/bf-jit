use crate::err::BFError;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Bracket {
    pub start: usize,
    pub end: usize,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Instruction {
    Right,
    Left,
    Add,
    Sub,
    Input,
    Output,
    Start,
    Stop,
}

impl std::convert::TryFrom<char> for Instruction {
    type Error = BFError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Instruction::Right),
            '<' => Ok(Instruction::Left),
            '.' => Ok(Instruction::Output),
            ',' => Ok(Instruction::Input),
            '+' => Ok(Instruction::Add),
            '-' => Ok(Instruction::Sub),
            '[' => Ok(Instruction::Start),
            ']' => Ok(Instruction::Stop),
            _ => Err(BFError::Instruction(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JitInstr {
    Ptr(isize),
    ALU(isize),
    Input,
    Output,
    LoopStart,
    LoopEnd,
}

impl JitInstr {
    pub fn combine(&self, other: &Self) -> Option<Self> {
        match (*self, *other) {
            (JitInstr::Ptr(m1), JitInstr::Ptr(m2)) => Some(JitInstr::Ptr(m1 + m2)),
            (JitInstr::ALU(a1), JitInstr::ALU(a2)) => Some(JitInstr::ALU(a1 + a2)),
            (_ ,_) => None,
        }
    }
}

impl std::convert::From<Instruction> for JitInstr {
    fn from(value: Instruction) -> Self {
        match value {
            Instruction::Add => JitInstr::ALU(1),
            Instruction::Sub => JitInstr::ALU(-1),
            Instruction::Left => JitInstr::Ptr(-1),
            Instruction::Right => JitInstr::Ptr(1),
            Instruction::Start => JitInstr::LoopStart,
            Instruction::Stop => JitInstr::LoopEnd,
            Instruction::Input => JitInstr::Input,
            Instruction::Output => JitInstr::Output,
        }
    }
}