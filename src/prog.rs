use crate::err::BFError;
use crate::instruction::{JitInstr, Instruction};
use std::io::{Read, Write};
use std::collections::{HashMap, VecDeque};
use std::slice::from_mut;

#[derive(Debug, Clone)]
pub struct JitProg(Vec<JitInstr>);

impl JitProg {
    pub fn optimize(&mut self) {
        // Find a way to do better optimizations than this
        let mut index: usize = 0;
        loop {
            if index >= self.0.len() - 1 {
                break;
            }
            let new_inst = self.0[index].combine(&self.0[index+1]);
            if let Some(instr) = new_inst {
                self.0[index] = instr;
                self.0.remove(index+1);
            } else {
                index += 1;
            }
        }
    }
}

impl std::convert::From<Vec<Instruction>> for JitProg {
    fn from(value: Vec<Instruction>) -> Self {
        let prog = value.into_iter().map(|inst| inst.into()).collect::<Vec<JitInstr>>();
        Self(prog)
    }
}

impl std::convert::From<Box<[Instruction]>> for JitProg {
    fn from(value: Box<[Instruction]>) -> Self {
        let prog = value.into_iter().map(|inst| (*inst).into()).collect::<Vec<JitInstr>>();
        Self(prog)
    }
}

pub struct JitRunnner<'io> {
    instr: usize,
    pointer: usize,
    data: Box<[u8]>,
    program: Box<[JitInstr]>,
    stack: HashMap<usize, usize>,
    stdin: &'io mut dyn Read,
    stdout: &'io mut dyn Write,
}

impl<'io> JitRunnner<'io> {
    pub fn new(prog: JitProg, mem_size: usize, input: &'io mut dyn Read, output: &'io mut dyn Write) -> Result<Self, BFError> {
        let mut brackets = VecDeque::new();
        let mut stack = HashMap::new();
        let program = prog.0.into_boxed_slice();
        let data = vec![0u8; mem_size].into_boxed_slice();
        for (i, instr) in program.iter().enumerate() {
            if *instr == JitInstr::LoopStart {
                brackets.push_back(i);
            } else if *instr == JitInstr::LoopEnd {
                if let Some(b) = brackets.pop_back() {
                    stack.insert(i, b);
                    stack.insert(b, i);
                } else {
                    return Err(BFError::Unmatched);
                }
            }
        }
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

    pub fn interpret(&mut self) -> Result<(), BFError> {
        loop {
            if self.instr == self.program.len() {
                return Ok(())
            }
            let data_ptr = &mut self.data[self.pointer];
            match self.program[self.instr] {
                JitInstr::ALU(v) => *data_ptr = data_ptr.wrapping_add_signed(v as i8),
                JitInstr::Ptr(v) => self.pointer = self.pointer.wrapping_add_signed(v),
                JitInstr::LoopStart => {
                    let jump = self.stack.get(&self.instr).expect("Parsing should ensure this");
                    if *data_ptr == 0 {
                        self.instr = *jump;
                    }
                },
                JitInstr::LoopEnd => {
                    let jump = self.stack.get(&self.instr).expect("Parsing should ensure this");
                    if *data_ptr != 0 {
                        self.instr = *jump;
                    }
                },
                JitInstr::Input => {
                    self.stdin.read_exact(from_mut(data_ptr))?;
                },
                JitInstr::Output => {
                    self.stdout.write_all(from_mut(data_ptr))?;
                },

            }
            self.instr += 1;
        }
    }
}