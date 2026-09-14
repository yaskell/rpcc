use crate::{
    asm::{self, Register},
    ir,
};

pub fn translate_program(program: ir::Program) -> asm::Program {
    program.into()
}

impl From<ir::Program> for asm::Program {
    fn from(program: ir::Program) -> Self {
        Self {
            functions: program
                .functions
                .into_iter()
                .map(asm::Function::from)
                .collect(),
        }
    }
}

impl From<ir::Function> for asm::Function {
    fn from(function: ir::Function) -> Self {
        Self {
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

fn translate_instruction(instruction: ir::Instruction) -> Vec<asm::Instruction> {
    let mut asm_instructions = Vec::new();
    match instruction {
        ir::Instruction::Return(val) => {
            asm_instructions.push(asm::Instruction::Move {
                src: val.into(),
                dst: asm::Operand::Register(asm::Register::AX),
            });
            asm_instructions.push(asm::Instruction::Ret);
        }
        ir::Instruction::Unary { op, src, dst } => {
            let dst: asm::Operand = dst.into();
            match op {
                ir::UnaryOp::Complement | ir::UnaryOp::Negate => {
                    asm_instructions.push(asm::Instruction::Move {
                        src: src.into(),
                        dst: dst.clone(),
                    });
                    asm_instructions.push(asm::Instruction::Unary {
                        op: op.into(),
                        operand: dst,
                    });
                }
                ir::UnaryOp::Not => {
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
        ir::Instruction::Binary {
            op,
            left,
            right,
            dst,
        } => match &op {
            ir::BinaryOp::Divide => {
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
            ir::BinaryOp::Remainder => {
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
            ir::BinaryOp::Equal
            | ir::BinaryOp::NotEqual
            | ir::BinaryOp::LessThan
            | ir::BinaryOp::LessOrEqual
            | ir::BinaryOp::GreaterThan
            | ir::BinaryOp::GreaterOrEqual => {
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
                    ir::BinaryOp::Add => asm::BinaryOp::Add,
                    ir::BinaryOp::Subtract => asm::BinaryOp::Sub,
                    ir::BinaryOp::Multiply => asm::BinaryOp::Mult,
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
        ir::Instruction::Copy { src, dst } => asm_instructions.push(asm::Instruction::Move {
            src: src.into(),
            dst: dst.into(),
        }),
        ir::Instruction::Jump { target } => asm_instructions.push(asm::Instruction::Jmp(target)),
        ir::Instruction::JumpIfZero { condition, target } => {
            asm_instructions.push(asm::Instruction::Cmp {
                left: asm::Operand::Imm(0),
                right: condition.into(),
            });
            asm_instructions.push(asm::Instruction::JmpCC {
                cond_code: asm::ConditionalCode::E,
                target,
            });
        }
        ir::Instruction::JumpIfNotZero { condition, target } => {
            asm_instructions.push(asm::Instruction::Cmp {
                left: asm::Operand::Imm(0),
                right: condition.into(),
            });
            asm_instructions.push(asm::Instruction::JmpCC {
                cond_code: asm::ConditionalCode::NE,
                target,
            });
        }
        ir::Instruction::Label(identifier) => {
            asm_instructions.push(asm::Instruction::Label(identifier));
        }
        ir::Instruction::FunCall {
            fun_name,
            args,
            dst,
        } => {
            let (register_args, stack_args) = args.split_at(args.len().min(6));
            let stack_padding = if stack_args.len() % 2 == 0 { 0 } else { 8 };

            if stack_padding != 0 {
                asm_instructions.push(asm::Instruction::AllocateStack(stack_padding));
            }

            register_args.iter().enumerate().for_each(|(i, parameter)| {
                asm_instructions.push(asm::Instruction::Move {
                    src: asm::Operand::from(parameter.clone()),
                    dst: get_register_for_param(i),
                });
            });

            stack_args.iter().rev().for_each(|stack_arg| {
                let stack_arg = asm::Operand::from(stack_arg.clone());
                match stack_arg {
                    asm::Operand::Register(_) | asm::Operand::Imm(_) => {
                        asm_instructions.push(asm::Instruction::Push(stack_arg));
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

impl From<ir::Val> for asm::Operand {
    fn from(val: ir::Val) -> Self {
        match val {
            ir::Val::Constant(c) => Self::Imm(c),
            ir::Val::Var(v) => Self::Pseudo(v),
        }
    }
}

impl From<ir::BinaryOp> for asm::ConditionalCode {
    fn from(code: ir::BinaryOp) -> Self {
        match code {
            ir::BinaryOp::Equal => Self::E,
            ir::BinaryOp::NotEqual => Self::NE,
            ir::BinaryOp::LessThan => Self::L,
            ir::BinaryOp::LessOrEqual => Self::LE,
            ir::BinaryOp::GreaterThan => Self::G,
            ir::BinaryOp::GreaterOrEqual => Self::GE,
            _ => panic!("Not a comparison operand"),
        }
    }
}

impl From<ir::UnaryOp> for asm::UnaryOp {
    fn from(unary_op: ir::UnaryOp) -> Self {
        match unary_op {
            ir::UnaryOp::Complement => Self::Not,
            ir::UnaryOp::Negate => Self::Neg,
            ir::UnaryOp::Not => {
                panic!("IR unary logical NOT is not converted into asm unary structure")
            }
        }
    }
}
