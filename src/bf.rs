use std::collections::HashMap;
use std::io::{Read, Write};
use std::slice::from_mut;

use crate::instruction::{Instruction, Bracket};
use crate::err::BFError;

pub struct BFTerp<'io> {
    instr: usize,
    pointer: usize,
    data: Box<[u8]>,
    program: Box<[Instruction]>,
    stack: HashMap<usize, Bracket>,
    stdin: &'io mut dyn Read,
    stdout: &'io mut dyn Write,
}

impl<'io> BFTerp<'io> {
    pub fn new(
        mem_size: usize,
        prog: &str,
        input: &'io mut dyn Read,
        output: &'io mut dyn Write,
    ) -> Result<Self, BFError> {
        let data = vec![0u8; mem_size].into_boxed_slice();
        let (program, stack) = BFTerp::parse_program(prog)?;
        let program = program.into_boxed_slice();
        Ok(Self {
            instr: 0,
            pointer: 0,
            data,
            program,
            stack,
            stdin: input,
            stdout: output,
        })
    }

    pub fn run(&mut self) -> Result<(), BFError> {
        loop {
            if self.instr == self.program.len() {
                return Ok(());
            }
            let data_ptr = &mut self.data[self.pointer];
            match self.program[self.instr] {
                Instruction::Add => *data_ptr = data_ptr.wrapping_add(1),
                Instruction::Sub => *data_ptr = data_ptr.wrapping_sub(1),
                Instruction::Left => self.pointer -= 1,
                Instruction::Right => self.pointer += 1,
                Instruction::Start => {
                    let bracket = self
                        .stack
                        .get(&self.instr)
                        .expect("Parsing should ensure this");
                    if *data_ptr == 0 {
                        self.instr = bracket.end;
                    }
                }
                Instruction::Stop => {
                    let bracket = self
                        .stack
                        .get(&self.instr)
                        .expect("Parsing should ensure this");
                    if *data_ptr != 0 {
                        self.instr = bracket.start;
                    }
                }
                Instruction::Input => {
                    self.stdin.read_exact(from_mut(data_ptr))?;
                }
                Instruction::Output => {
                    self.stdout.write_all(from_mut(data_ptr))?;
                }
            };
            self.instr += 1;
        }
    }

    fn parse_program(prog: &str) -> Result<(Vec<Instruction>, HashMap<usize, Bracket>), BFError> {
        let mut program = Vec::with_capacity(prog.len());
        let mut stack = HashMap::<usize, Bracket>::new();
        let mut brackets = Vec::<usize>::new();
        for (i, instr) in prog.chars().enumerate() {
            let decoded: Instruction = instr.try_into()?;
            program.push(decoded);
            if decoded == Instruction::Start {
                brackets.push(i);
                continue;
            }
            if decoded == Instruction::Stop {
                if let Some(matching) = brackets.pop() {
                    let brack = Bracket {
                        start: matching,
                        end: i,
                    };
                    stack.insert(i, brack);
                    stack.insert(matching, brack);
                } else {
                    return Err(BFError::Unmatched);
                }
            }
        }
        Ok((program, stack))
    }

    pub fn into_prog(self) -> Box<[Instruction]> {
        self.program
    }
}

impl std::fmt::Debug for BFTerp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BFTerp")
            .field("instr_index", &self.instr)
            .field("data_ptr", &self.pointer)
            .finish()
    }
}

impl std::fmt::Display for BFTerp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BFTerp")
    }
}
