use crate::{asm, tacky};

//NOTE: consider using impl blocks for asm:: structures

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
                let dst = translate_val(dst);
                match op {
                    tacky::UnaryOp::Complement | tacky::UnaryOp::Negate => {
                        asm_instructions.push(asm::Instruction::Move {
                            src: translate_val(src),
                            dst: dst.clone(),
                        });
                        asm_instructions.push(asm::Instruction::Unary {
                            op: translate_unary_op(op),
                            operand: dst.clone(),
                        })
                    }
                    tacky::UnaryOp::Not => {
                        asm_instructions.push(asm::Instruction::Cmp {
                            left: asm::Operand::Imm(0),
                            right: translate_val(src),
                        });
                        asm_instructions.push(asm::Instruction::Move {
                            src: asm::Operand::Imm(0),
                            dst: dst.clone(),
                        });
                        asm_instructions.push(asm::Instruction::SetCC {
                            cond_code: asm::ConditionalCode::E,
                            operand: dst,
                        });
                    }
                }
            }
            tacky::Instruction::Binary {
                op,
                left,
                right,
                dst,
            } => match &op {
                tacky::BinaryOp::Divide => {
                    asm_instructions.push(asm::Instruction::Move {
                        src: translate_val(left),
                        dst: asm::Operand::Register(asm::Register::AX),
                    });
                    asm_instructions.push(asm::Instruction::Cdq);
                    asm_instructions.push(asm::Instruction::Idiv(translate_val(right)));
                    asm_instructions.push(asm::Instruction::Move {
                        src: asm::Operand::Register(asm::Register::AX),
                        dst: translate_val(dst),
                    });
                }
                tacky::BinaryOp::Remainder => {
                    asm_instructions.push(asm::Instruction::Move {
                        src: translate_val(left),
                        dst: asm::Operand::Register(asm::Register::AX),
                    });
                    asm_instructions.push(asm::Instruction::Cdq);
                    asm_instructions.push(asm::Instruction::Idiv(translate_val(right)));
                    asm_instructions.push(asm::Instruction::Move {
                        src: asm::Operand::Register(asm::Register::DX),
                        dst: translate_val(dst),
                    });
                }
                tacky::BinaryOp::Equal
                | tacky::BinaryOp::NotEqual
                | tacky::BinaryOp::LessThan
                | tacky::BinaryOp::LessOrEqual
                | tacky::BinaryOp::GreaterThan
                | tacky::BinaryOp::GreaterOrEqual => {
                    asm_instructions.push(asm::Instruction::Cmp {
                        left: translate_val(right),
                        right: translate_val(left),
                    });
                    asm_instructions.push(asm::Instruction::Move {
                        src: asm::Operand::Imm(0),
                        dst: translate_val(dst.clone()),
                    });
                    asm_instructions.push(asm::Instruction::SetCC {
                        cond_code: translate_condition_code(op),
                        operand: translate_val(dst),
                    });
                }
                _ => {
                    let op = match &op {
                        tacky::BinaryOp::Add => asm::BinaryOp::Add,
                        tacky::BinaryOp::Subtract => asm::BinaryOp::Sub,
                        tacky::BinaryOp::Multiply => asm::BinaryOp::Mult,
                        _ => {
                            unreachable!("Should've matched super branch")
                        }
                    };

                    asm_instructions.push(asm::Instruction::Move {
                        src: translate_val(left),
                        dst: translate_val(dst.clone()),
                    });
                    asm_instructions.push(asm::Instruction::Binary {
                        op: op,
                        left: translate_val(right),
                        right: translate_val(dst),
                    });
                }
            },
            tacky::Instruction::Copy { src, dst } => {
                asm_instructions.push(asm::Instruction::Move {
                    src: translate_val(src),
                    dst: translate_val(dst),
                })
            }
            tacky::Instruction::Jump { target } => {
                asm_instructions.push(asm::Instruction::Jmp(target))
            }
            tacky::Instruction::JumpIfZero { condition, target } => {
                asm_instructions.push(asm::Instruction::Cmp {
                    left: asm::Operand::Imm(0),
                    right: translate_val(condition),
                });
                asm_instructions.push(asm::Instruction::JmpCC {
                    cond_code: asm::ConditionalCode::E,
                    target,
                });
            }
            tacky::Instruction::JumpIfNotZero { condition, target } => {
                asm_instructions.push(asm::Instruction::Cmp {
                    left: asm::Operand::Imm(0),
                    right: translate_val(condition),
                });
                asm_instructions.push(asm::Instruction::JmpCC {
                    cond_code: asm::ConditionalCode::NE,
                    target,
                });
            }
            tacky::Instruction::Label(identifier) => {
                asm_instructions.push(asm::Instruction::Label(identifier))
            }
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

fn translate_condition_code(code: tacky::BinaryOp) -> asm::ConditionalCode {
    match code {
        tacky::BinaryOp::Equal => asm::ConditionalCode::E,
        tacky::BinaryOp::NotEqual => asm::ConditionalCode::NE,
        tacky::BinaryOp::LessThan => asm::ConditionalCode::L,
        tacky::BinaryOp::LessOrEqual => asm::ConditionalCode::LE,
        tacky::BinaryOp::GreaterThan => asm::ConditionalCode::G,
        tacky::BinaryOp::GreaterOrEqual => asm::ConditionalCode::GE,
        _ => unreachable!("Not a comparison operand"),
    }
}

fn translate_unary_op(unary_op: tacky::UnaryOp) -> asm::UnaryOp {
    match unary_op {
        tacky::UnaryOp::Complement => asm::UnaryOp::Not,
        tacky::UnaryOp::Negate => asm::UnaryOp::Neg,
        tacky::UnaryOp::Not => todo!(),
    }
}
