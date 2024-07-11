use crate::err::BFError;
use crate::instruction::Instruction;
use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::slice::from_mut;

pub struct Interpreter<R, W> {
    instr: usize,
    pointer: usize,
    data: Vec<usize>,
    pub(crate) program: Vec<Instruction>,
    stack: HashMap<usize, usize>,
    pub(crate) stdin: R,
    pub(crate) stdout: W,
}

impl<R, W> std::fmt::Debug for Interpreter<R, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.program.iter()).finish()
    }
}

impl<R: Read, W: Write> Interpreter<R, W> {
    pub fn new(prog: &str, size: usize, input: R, output: W) -> Result<Self, BFError> {
        let (program, stack) = Self::parse(prog)?;
        Ok(Self {
            instr: 0,
            pointer: 0,
            data: vec![0usize; size],
            program,
            stack,
            stdin: input,
            stdout: output,
        })
    }

    fn parse(prog: &str) -> Result<(Vec<Instruction>, HashMap<usize, usize>), BFError> {
        let mut stack = HashMap::new();
        let mut program = Vec::with_capacity(prog.len());
        let mut brackets = VecDeque::new();

        for (i, instr) in prog.chars().enumerate() {
            let parsed: Instruction = instr.into();
            program.push(parsed);
            if parsed == Instruction::LoopStart {
                brackets.push_back(i);
            } else if parsed == Instruction::LoopEnd {
                if let Some(matching) = brackets.pop_back() {
                    stack.insert(i, matching);
                    stack.insert(matching, i);
                } else {
                    return Err(BFError::Unmatched);
                }
            }
        }
        if brackets.is_empty() {
            return Err(BFError::Unmatched);
        }

        Ok((program, stack))
    }

    pub fn optimize(&mut self) {
        // Find a way to do better optimizations than this
        let mut index: usize = 0;
        let mut new_instructions = Vec::with_capacity(self.program.len());
        new_instructions.push(Instruction::Nop);
        for instr in &self.program {
            if let Some(combined) = new_instructions[index].combine(instr) {
                new_instructions[index] = combined;
            } else {
                new_instructions.push(*instr);
                index += 1;
            }
        }
        self.program = new_instructions;
        // Now need to recreate the stack of brackets
        self.stack.clear();
        let mut stack = VecDeque::new();
        for (index, instr) in self.program.iter().enumerate() {
            match instr {
                Instruction::LoopStart => stack.push_back(index),
                Instruction::LoopEnd => {
                    let jump_index = stack.pop_back().expect("This should be valid");
                    self.stack.insert(index, jump_index);
                    self.stack.insert(jump_index, index);
                }
                _ => {}
            }
        }
    }

    pub fn step(&mut self) -> Result<bool, BFError> {
        if self.instr == self.program.len() {
            return Ok(true);
        }
        let data_ptr = &mut self.data[self.pointer];
        let mut input: u8 = 0;
        match self.program[self.instr] {
            Instruction::Alu(v) => *data_ptr = data_ptr.wrapping_add_signed(v),
            Instruction::Ptr(v) => self.pointer = self.pointer.wrapping_add_signed(v),
            Instruction::LoopStart => {
                let jump = self
                    .stack
                    .get(&self.instr)
                    .expect("Parsing should ensure this");
                if *data_ptr == 0 {
                    self.instr = *jump;
                }
            }
            Instruction::LoopEnd => {
                let jump = self
                    .stack
                    .get(&self.instr)
                    .expect("Parsing should ensure this");
                if *data_ptr != 0 {
                    self.instr = *jump;
                }
            }
            Instruction::Input => {
                self.stdin.read_exact(from_mut(&mut input))?;
                *data_ptr = input as usize;
            }
            Instruction::Output => {
                input = *data_ptr as u8;
                self.stdout.write_all(from_mut(&mut input))?;
            }
            Instruction::Nop => {}
        }
        self.instr += 1;
        Ok(false)
    }

    pub fn interpret(&mut self) -> Result<(), BFError> {
        loop {
            let finished = self.step()?;
            if finished {
                break;
            }
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.instr = 0;
        self.data.fill(0);
    }
}
