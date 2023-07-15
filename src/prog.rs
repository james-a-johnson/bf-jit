use crate::err::BFError;
use crate::instruction::Instruction;
use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::slice::from_mut;

pub struct Interpreter<'io> {
    pub(crate) instr: usize,
    pub(crate) pointer: usize,
    pub(crate) data: Vec<usize>,
    pub(crate) program: Vec<Instruction>,
    stack: HashMap<usize, usize>,
    stdin: &'io mut dyn Read,
    stdout: &'io mut dyn Write,
}

impl<'io> Interpreter<'io> {
    pub fn new(
        prog: &str,
        size: usize,
        input: &'io mut dyn Read,
        output: &'io mut dyn Write,
    ) -> Result<Self, BFError> {
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
        if brackets.len() != 0 {
            return Err(BFError::Unmatched);
        }

        Ok((program, stack))
    }

    pub fn optimize(&mut self) {
        // Find a way to do better optimizations than this
        let mut index: usize = 0;
        loop {
            if index >= self.program.len() - 1 {
                break;
            }
            let new_inst = self.program[index].combine(&self.program[index + 1]);
            if let Some(instr) = new_inst {
                self.program[index] = instr;
                self.program.remove(index + 1);
            } else {
                index += 1;
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
}
