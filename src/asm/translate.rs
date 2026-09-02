use crate::{
    asm::{self, Register},
    tacky,
};

pub fn translate_program(program: tacky::Program) -> asm::Program {
    program.into()
}

impl From<tacky::Program> for asm::Program {
    fn from(program: tacky::Program) -> asm::Program {
        asm::Program {
            functions: program
                .functions
                .into_iter()
                .map(asm::Function::from)
                .collect(),
        }
    }
}

impl From<tacky::Function> for asm::Function {
    fn from(function: tacky::Function) -> asm::Function {
        asm::Function {
            name: function.identifier,
            stack_size: 0,
            instructions: function
                .params
                .into_iter()
                .enumerate()
                .map(|(i, parameter)| translate_param(i, parameter))
                .chain(function.body.into_iter().flat_map(translate_instruction))
                .collect(),
        }
    }
}

fn translate_param(index: usize, parameter: String) -> asm::Instruction {
    asm::Instruction::Move {
        src: get_register_for_param(index),
        dst: asm::Operand::Pseudo(parameter),
    }
}

fn get_register_for_param(index: usize) -> asm::Operand {
    match index {
        0 => asm::Operand::Register(asm::Register::DI),
        1 => asm::Operand::Register(asm::Register::SI),
        2 => asm::Operand::Register(asm::Register::DX),
        3 => asm::Operand::Register(asm::Register::CX),
        4 => asm::Operand::Register(asm::Register::R8),
        5 => asm::Operand::Register(asm::Register::R9),
        index => {
            let offset = 16 + ((index as i32 - 6) * 8);
            asm::Operand::Stack(offset)
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
                    op,
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
        tacky::Instruction::FunCall {
            fun_name,
            args,
            dst,
        } => {
            let (register_args, stack_args) = args.split_at(args.len().min(6));
            let stack_padding = if stack_args.len() % 2 == 0 { 8 } else { 0 };

            if stack_padding != 0 {
                asm_instructions.push(asm::Instruction::AllocateStack(stack_padding))
            };

            register_args
                .into_iter()
                .enumerate()
                .for_each(|(i, parameter)| {
                    asm_instructions.push(asm::Instruction::Move {
                        src: asm::Operand::from(parameter.clone()),
                        dst: get_register_for_param(i),
                    })
                });

            stack_args.into_iter().rev().for_each(|stack_arg| {
                let stack_arg = asm::Operand::from(stack_arg.clone());
                match stack_arg {
                    asm::Operand::Register(_) | asm::Operand::Imm(_) => {
                        asm_instructions.push(asm::Instruction::Push(stack_arg))
                    }
                    _ => {
                        asm_instructions.push(asm::Instruction::Move {
                            src: stack_arg,
                            dst: asm::Operand::Register(Register::AX),
                        });
                        asm_instructions
                            .push(asm::Instruction::Push(asm::Operand::Register(Register::AX)));
                    }
                }
            });

            asm_instructions.push(asm::Instruction::Call(fun_name));

            let bytes_to_remove = 8 * stack_args.len() as i32 + stack_padding;

            if bytes_to_remove != 0 {
                asm_instructions.push(asm::Instruction::DeallocateStack(bytes_to_remove));
            }

            asm_instructions.push(asm::Instruction::Move {
                src: asm::Operand::Register(Register::AX),
                dst: asm::Operand::from(dst),
            });
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
            _ => panic!("Not a comparison operand"),
        }
    }
}

impl From<tacky::UnaryOp> for asm::UnaryOp {
    fn from(unary_op: tacky::UnaryOp) -> asm::UnaryOp {
        match unary_op {
            tacky::UnaryOp::Complement => asm::UnaryOp::Not,
            tacky::UnaryOp::Negate => asm::UnaryOp::Neg,
            tacky::UnaryOp::Not => {
                panic!("Tacky unary logical NOT is not converted into asm unary structure")
            }
        }
    }
}
