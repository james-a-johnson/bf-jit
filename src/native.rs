#[cfg(target_arch = "aarch64")]
use crate::arm64_backend::assemble;
use crate::prog::Interpreter;
use dynasmrt::{AssemblyOffset, ExecutableBuffer};
use std::io::{Read, Write};
use std::ops::Range;
use std::slice::{from_mut, from_ref};

pub type OutputFunc<const N: usize, R, W> = extern "C" fn(*mut Native<N, R, W>, u8);
pub type InputFunc<const N: usize, R, W> = extern "C" fn(*mut Native<N, R, W>) -> u8;

pub struct Native<const N: usize, R, W> {
    blob: ExecutableBuffer,
    start: AssemblyOffset,
    cells: [usize; N],
    output: W,
    input: R,
}

impl<const N: usize, R: Read, W: Write> Native<N, R, W> {
    pub fn run(&mut self) {
        let code: extern "C" fn(state: *mut (), cells: *mut usize) =
            unsafe { std::mem::transmute(self.blob.ptr(self.start)) };
        code(
            self as *mut Native<N, R, W> as *mut (),
            self.cells.as_mut_ptr(),
        );
    }

    extern "C" fn output_data(state: *mut Self, value: u8) {
        let state = unsafe { state.as_mut().unwrap() };
        let _ = state.output.write_all(from_ref(&value));
    }

    extern "C" fn input_data(state: *mut Self) -> u8 {
        let mut data: u8 = 0;
        let state = unsafe { state.as_mut().unwrap() };
        let _ = state.input.read_exact(from_mut(&mut data));
        data
    }
}

impl<const N: usize, R, W> std::ops::Index<usize> for Native<N, R, W> {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl<const N: usize, R, W> std::ops::Index<Range<usize>> for Native<N, R, W> {
    type Output = [usize];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.cells[index]
    }
}

impl<const N: usize, R: Read, W: Write> From<Interpreter<R, W>> for Native<N, R, W> {
    fn from(value: Interpreter<R, W>) -> Self {
        let (blob, offset) = assemble(&value.program, Self::output_data, Self::input_data);
        Self {
            blob,
            start: offset,
            cells: [0; N],
            input: value.stdin,
            output: value.stdout,
        }
    }
}
