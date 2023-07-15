#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Ptr(isize),
    Alu(isize),
    Input,
    Output,
    LoopStart,
    LoopEnd,
    Nop,
}

impl Instruction {
    pub fn combine(&self, other: &Self) -> Option<Self> {
        match (*self, *other) {
            (Instruction::Ptr(m1), Instruction::Ptr(m2)) => Some(Instruction::Ptr(m1 + m2)),
            (Instruction::Alu(a1), Instruction::Alu(a2)) => Some(Instruction::Alu(a1 + a2)),
            (_, _) => None,
        }
    }

    pub fn to_str(&self) -> String {
        match *self {
            Self::Ptr(v) => {
                if v < 0 {
                    format!("< {v}")
                } else {
                    format!("> {v}")
                }
            },
            Self::Alu(v) => {
                if v < 0 {
                    format!("- {v}")
                } else {
                    format!("+ {v}")
                }
            },
            Self::Input => ",".into(),
            Self::Output => ".".into(),
            Self::LoopStart => "[".into(),
            Self::LoopEnd => "]".into(),
            Self::Nop => " ".into(),
        }
    }
}

impl std::convert::From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Alu(1),
            '-' => Self::Alu(-1),
            '>' => Self::Ptr(1),
            '<' => Self::Ptr(-1),
            '.' => Self::Output,
            ',' => Self::Input,
            '[' => Self::LoopStart,
            ']' => Self::LoopEnd,
            _ => Self::Nop,
        }
    }
}
