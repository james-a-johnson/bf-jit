use crate::instruction::Instruction;
use crate::native::{InputFunc, OutputFunc};
use dynasmrt::mmap::ExecutableBuffer;
use dynasmrt::{dynasm, AssemblyOffset, DynasmApi, DynasmLabelApi};

pub fn assemble<const N: usize, R, W>(
    instructions: &[Instruction],
    output: OutputFunc<N, R, W>,
    input: InputFunc<N, R, W>,
) -> (ExecutableBuffer, AssemblyOffset) {
    let mut program = dynasmrt::aarch64::Assembler::new().unwrap();
    let mut labels = Vec::new();
    let output_label = program.new_dynamic_label();
    let input_label = program.new_dynamic_label();
    dynasm!(
        program
        ; => output_label
        ; .bytes (output as usize).to_ne_bytes()
        ; => input_label
        ; .bytes (input as usize).to_ne_bytes()
    );
    let start = program.offset();
    dynasm!(
        program
        ; .arch aarch64
        ; sub sp, sp, 8 * 4
        ; str x30, [sp]
        ; mov x2, 0
    );
    for instr in instructions {
        match instr {
            Instruction::Alu(v) => {
                if *v == 0 {
                    continue;
                }
                let value = v.unsigned_abs();
                if *v > 0 {
                    dynasm!(
                        program
                        ; ldr x3, [x1, x2]
                        ; add x3, x3, value as u32
                        ; str x3, [x1, x2]
                    )
                } else {
                    dynasm!(
                        program
                        ; ldr x3, [x1, x2]
                        ; sub x3, x3, value as u32
                        ; str x3, [x1, x2]
                    )
                }
            }
            Instruction::Ptr(v) => {
                if *v == 0 {
                    continue;
                }
                let value: u32 = (v.unsigned_abs() * 8).try_into().unwrap();
                if *v > 0 {
                    dynasm!(
                        program
                        ; add x2, x2, #value
                    )
                } else {
                    dynasm!(
                        program
                        ; sub x2, x2, #value
                    )
                }
            }
            Instruction::Nop => {}
            Instruction::LoopStart => {
                let forward_label = program.new_dynamic_label();
                let backward_label = program.new_dynamic_label();
                labels.push((forward_label, backward_label));
                dynasm!(
                    program
                    ; ldr x3, [x1, x2]
                    ; cmp x3, 0
                    ; b.eq => forward_label
                    ; => backward_label
                );
            }
            Instruction::LoopEnd => {
                let (forward, backward) = labels.pop().unwrap();
                dynasm!(
                    program
                    ; ldr x3, [x1, x2]
                    ; cmp x3, 0
                    ; b.ne => backward
                    ; => forward
                );
            }
            Instruction::Input => {
                dynasm!(
                    program
                    ; stp x0, x1, [sp, 8]
                    ; str x2, [sp, 24]
                    ; ldr x4, => input_label
                    ; blr x4
                    ; ldp x1, x2, [sp, 16]
                    ; strb w0, [x1, x2]
                    ; ldr x0, [sp, 8]
                );
            }
            Instruction::Output => {
                dynasm!(
                    program
                    ; stp x0, x1, [sp, 8]
                    ; str x2, [sp, 24]
                    ; ldrb w1, [x1, x2]
                    ; ldr x4, => output_label
                    ; blr x4
                    ; ldp x1, x2, [sp, 16]
                    ; ldr x0, [sp, 8]
                )
            }
        }
    }
    dynasm!(
        program
        ; ldr x30, [sp]
        ; add sp, sp, 8 * 4
        ; ret
    );
    let exec = program.finalize().unwrap();
    (exec, start)
}
