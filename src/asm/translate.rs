use crate::{asm, tacky};

pub fn translate_program(program: tacky::Program) -> asm::Program {
    asm::Program {
        function: program.function.into(),
    }
}

impl From<tacky::Program> for asm::Program {
    fn from(program: tacky::Program) -> asm::Program {
        asm::Program {
            function: program.function.into(),
        }
    }
}

impl From<tacky::Function> for asm::Function {
    fn from(function: tacky::Function) -> asm::Function {
        asm::Function {
            name: function.identifier,
            instructions: function
                .body
                .into_iter()
                .flat_map(|i| translate_instruction(i))
                .collect(),
        }
    }
}

fn translate_instruction(instruction: tacky::Instruction) -> Vec<asm::Instruction> {
    let mut asm_instructions = Vec::new();
    match instruction {
        tacky::Instruction::Return(val) => {
            asm_instructions.push(asm::Instruction::Move {
                src: val.into(),
                dst: asm::Operand::Register(asm::Register::AX),
            });
            asm_instructions.push(asm::Instruction::Ret)
        }
        tacky::Instruction::Unary { op, src, dst } => {
            let dst: asm::Operand = dst.into();
            match op {
                tacky::UnaryOp::Complement | tacky::UnaryOp::Negate => {
                    asm_instructions.push(asm::Instruction::Move {
                        src: src.into(),
                        dst: dst.clone(),
                    });
                    asm_instructions.push(asm::Instruction::Unary {
                        op: op.into(),
                        operand: dst.clone(),
                    })
                }
                tacky::UnaryOp::Not => {
                    asm_instructions.push(asm::Instruction::Cmp {
                        left: asm::Operand::Imm(0),
                        right: src.into(),
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
                    src: left.into(),
                    dst: asm::Operand::Register(asm::Register::AX),
                });
                asm_instructions.push(asm::Instruction::Cdq);
                asm_instructions.push(asm::Instruction::Idiv(right.into()));
                asm_instructions.push(asm::Instruction::Move {
                    src: asm::Operand::Register(asm::Register::AX),
                    dst: dst.into(),
                });
            }
            tacky::BinaryOp::Remainder => {
                asm_instructions.push(asm::Instruction::Move {
                    src: left.into(),
                    dst: asm::Operand::Register(asm::Register::AX),
                });
                asm_instructions.push(asm::Instruction::Cdq);
                asm_instructions.push(asm::Instruction::Idiv(right.into()));
                asm_instructions.push(asm::Instruction::Move {
                    src: asm::Operand::Register(asm::Register::DX),
                    dst: dst.into(),
                });
            }
            tacky::BinaryOp::Equal
            | tacky::BinaryOp::NotEqual
            | tacky::BinaryOp::LessThan
            | tacky::BinaryOp::LessOrEqual
            | tacky::BinaryOp::GreaterThan
            | tacky::BinaryOp::GreaterOrEqual => {
                asm_instructions.push(asm::Instruction::Cmp {
                    left: right.into(),
                    right: left.into(),
                });
                asm_instructions.push(asm::Instruction::Move {
                    src: asm::Operand::Imm(0),
                    dst: dst.clone().into(),
                });
                asm_instructions.push(asm::Instruction::SetCC {
                    cond_code: op.into(),
                    operand: dst.into(),
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
                    src: left.into(),
                    dst: dst.clone().into(),
                });
                asm_instructions.push(asm::Instruction::Binary {
                    op: op,
                    left: right.into(),
                    right: dst.into(),
                });
            }
        },
        tacky::Instruction::Copy { src, dst } => asm_instructions.push(asm::Instruction::Move {
            src: src.into(),
            dst: dst.into(),
        }),
        tacky::Instruction::Jump { target } => asm_instructions.push(asm::Instruction::Jmp(target)),
        tacky::Instruction::JumpIfZero { condition, target } => {
            asm_instructions.push(asm::Instruction::Cmp {
                left: asm::Operand::Imm(0),
                right: condition.into(),
            });
            asm_instructions.push(asm::Instruction::JmpCC {
                cond_code: asm::ConditionalCode::E,
                target,
            });
        }
        tacky::Instruction::JumpIfNotZero { condition, target } => {
            asm_instructions.push(asm::Instruction::Cmp {
                left: asm::Operand::Imm(0),
                right: condition.into(),
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
    asm_instructions
}

impl From<tacky::Val> for asm::Operand {
    fn from(val: tacky::Val) -> asm::Operand {
        match val {
            tacky::Val::Constant(c) => asm::Operand::Imm(c),
            tacky::Val::Var(v) => asm::Operand::Pseudo(v),
        }
    }
}

impl From<tacky::BinaryOp> for asm::ConditionalCode {
    fn from(code: tacky::BinaryOp) -> asm::ConditionalCode {
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
}

impl From<tacky::UnaryOp> for asm::UnaryOp {
    fn from(unary_op: tacky::UnaryOp) -> asm::UnaryOp {
        match unary_op {
            tacky::UnaryOp::Complement => asm::UnaryOp::Not,
            tacky::UnaryOp::Negate => asm::UnaryOp::Neg,
            tacky::UnaryOp::Not => {
                unreachable!("Tacky unary logical NOT is not converted into asm unary structure")
            }
        }
    }
}
