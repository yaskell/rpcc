use crate::{asm, tacky};

pub fn translate_program(program: tacky::Program) -> asm::Program {
    asm::Program {
        function: translate_function(program.function),
    }
}

fn translate_function(function: tacky::Function) -> asm::Function {
    asm::Function {
        name: function.identifier,
        instructions: translate_instruction(function.body),
    }
}

fn translate_instruction(instructions: Vec<tacky::Instruction>) -> Vec<asm::Instruction> {
    let mut asm_instructions = Vec::new();
    for instruction in instructions {
        match instruction {
            tacky::Instruction::Return(val) => {
                asm_instructions.push(asm::Instruction::Move {
                    src: translate_val(val),
                    dst: asm::Operand::Register(asm::Register::AX),
                });
                asm_instructions.push(asm::Instruction::Ret)
            }
            tacky::Instruction::Unary { op, src, dst } => {
                asm_instructions.push(asm::Instruction::Move {
                    src: translate_val(src),
                    dst: translate_val(dst.clone()),
                });
                asm_instructions.push(asm::Instruction::Unary {
                    op: translate_unary_op(op),
                    operand: translate_val(dst),
                })
            }
            tacky::Instruction::Binary {
                op,
                left,
                right,
                dst,
            } => todo!(),
        }
    }
    asm_instructions
}

fn translate_val(val: tacky::Val) -> asm::Operand {
    match val {
        tacky::Val::Constant(c) => asm::Operand::Imm(c),
        tacky::Val::Var(v) => asm::Operand::Pseudo(v),
    }
}

fn translate_unary_op(unary_op: tacky::UnaryOp) -> asm::UnaryOp {
    match unary_op {
        tacky::UnaryOp::Complement => asm::UnaryOp::Not,
        tacky::UnaryOp::Negate => asm::UnaryOp::Neg,
    }
}
