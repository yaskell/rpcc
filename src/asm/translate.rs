use crate::{asm, tacky};

pub fn translate_program(program: tacky::Program) -> asm::Program {
    asm::Program {
        function_definition: translate_function_definition(program.tacky_function_definition),
    }
}

fn translate_function_definition(
    function_definition: tacky::FunctionDefinition,
) -> asm::FunctionDefinition {
    asm::FunctionDefinition {
        name: function_definition.identifier,
        instructions: translate_instruction(function_definition.body),
    }
}

fn translate_instruction(instructions: Vec<tacky::Instruction>) -> Vec<asm::Instruction> {
    let mut asm_instructions = Vec::new();
    for val in instructions.iter() {
        match val {
            tacky::Instruction::Return(val) => {
                asm_instructions.push(asm::Instruction::Move(asm::MoveInstruction {
                    src: translate_val(val),
                    dst: asm::Operand::Register(asm::Register::AX),
                }));
                asm_instructions.push(asm::Instruction::Ret)
            }
            tacky::Instruction::Unary(uop, src, dst) => {
                asm_instructions.push(asm::Instruction::Move(asm::MoveInstruction {
                    src: translate_val(src),
                    dst: translate_val(dst),
                }));
                asm_instructions.push(asm::Instruction::Unary(asm::UnaryInstruction {
                    op: translate_unary_operator(uop),
                    operand: translate_val(dst),
                }))
            }
        }
    }
    asm_instructions
}

//FIXME
fn translate_val(val: &tacky::Val) -> asm::Operand {
    match val {
        tacky::Val::Constant(c) => asm::Operand::Imm(c.clone()),
        tacky::Val::Var(v) => asm::Operand::Pseudo(v.clone()),
    }
}

fn translate_unary_operator(unary_op: &tacky::UnaryOperator) -> asm::UnaryOp {
    match unary_op {
        tacky::UnaryOperator::Complement => asm::UnaryOp::Not,
        tacky::UnaryOperator::Negate => asm::UnaryOp::Neg,
    }
}
