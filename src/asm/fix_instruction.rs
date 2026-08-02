use crate::asm;

pub fn fix_program(program: asm::Program, offset: i32) -> asm::Program {
    asm::Program {
        function: fix_function(program.function, offset),
    }
}

pub fn fix_function(function: asm::Function, offset: i32) -> asm::Function {
    asm::Function {
        name: function.name,
        instructions: std::iter::once(asm::Instruction::AllocateStack(offset))
            .chain(
                function
                    .instructions
                    .into_iter()
                    .flat_map(fix_instruction),
            )
            .collect(),
    }
}

pub fn fix_instruction(instruction: asm::Instruction) -> Vec<asm::Instruction> {
    match instruction {
        // Move can't have memory addresses as both src and dst
        asm::Instruction::Move {
            src: src @ asm::Operand::Stack(_),
            dst: dst @ asm::Operand::Stack(_),
        } => {
            vec![
                asm::Instruction::Move {
                    src,
                    dst: asm::Operand::Register(asm::Register::R10),
                },
                asm::Instruction::Move {
                    src: asm::Operand::Register(asm::Register::R10),
                    dst,
                },
            ]
        }
        // add and sub instructions can't operate on two memory addresses
        asm::Instruction::Binary {
            op: op @ (asm::BinaryOp::Add | asm::BinaryOp::Sub),
            left: left @ asm::Operand::Stack(_),
            right: right @ asm::Operand::Stack(_),
        } => {
            vec![
                asm::Instruction::Move {
                    src: left,
                    dst: asm::Operand::Register(asm::Register::R10),
                },
                asm::Instruction::Binary {
                    op,
                    left: asm::Operand::Register(asm::Register::R10),
                    right,
                },
            ]
        }
        // idiv can't operate on immediate values
        asm::Instruction::Idiv(asm::Operand::Imm(i)) => {
            vec![
                asm::Instruction::Move {
                    src: asm::Operand::Imm(i),
                    dst: asm::Operand::Register(asm::Register::R10),
                },
                asm::Instruction::Idiv(asm::Operand::Register(asm::Register::R10)),
            ]
        }
        // imul can't use a memory address as its destination
        asm::Instruction::Binary {
            op: asm::BinaryOp::Mult,
            left,
            right: right @ asm::Operand::Stack(_),
        } => {
            vec![
                asm::Instruction::Move {
                    src: right.clone(),
                    dst: asm::Operand::Register(asm::Register::R11),
                },
                asm::Instruction::Binary {
                    op: asm::BinaryOp::Mult,
                    left,
                    right: asm::Operand::Register(asm::Register::R11),
                },
                asm::Instruction::Move {
                    src: asm::Operand::Register(asm::Register::R11),
                    dst: right,
                },
            ]
        }
        // cmp can't use memory address for both operands
        asm::Instruction::Cmp {
            left: left @ asm::Operand::Stack(_),
            right: right @ asm::Operand::Stack(_),
        } => {
            vec![
                asm::Instruction::Move {
                    src: left,
                    dst: asm::Operand::Register(asm::Register::R10),
                },
                asm::Instruction::Cmp {
                    left: asm::Operand::Register(asm::Register::R10),
                    right,
                },
            ]
        }
        // right of Cmp can't be constant
        asm::Instruction::Cmp {
            left,
            right: right @ asm::Operand::Imm(_),
        } => {
            vec![
                asm::Instruction::Move {
                    src: right,
                    dst: asm::Operand::Register(asm::Register::R11),
                },
                asm::Instruction::Cmp {
                    left,
                    right: asm::Operand::Register(asm::Register::R11),
                },
            ]
        }
        _ => {
            vec![instruction]
        }
    }
}
