use crate::tacky::{self, UnaryOperator};

pub type Identifier = String;

pub type Int = i32;

#[derive(Debug)]
pub struct ASMProgram {
    pub function_definition: ASMFunctionDefinition,
}

#[derive(Debug)]
pub struct ASMFunctionDefinition {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Unary(UnaryInstruction),
    AllocateStack(Int),
    Move(MoveInstruction),
    Ret,
}

#[derive(Debug)]
pub struct MoveInstruction {
    pub src: Operand,
    pub dst: Operand,
}

#[derive(Debug)]
pub struct UnaryInstruction {
    pub op: UnaryOp,
    pub operand: Operand,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum Operand {
    Imm(i32),
    Register(Register),
    Pseudo(Identifier),
    Stack(Int),
}

#[derive(Debug)]
pub enum Register {
    AX,
    R10,
}

pub fn translate_program(program: tacky::TackyProgram) -> ASMProgram {
    ASMProgram {
        function_definition: translate_function_definition(program.tacky_function_definition),
    }
}

fn translate_function_definition(
    function_definition: tacky::TackyFunctionDefinition,
) -> ASMFunctionDefinition {
    ASMFunctionDefinition {
        name: function_definition.identifier,
        instructions: translate_instruction(function_definition.body),
    }
}

fn translate_instruction(instructions: Vec<tacky::Instruction>) -> Vec<Instruction> {
    let mut asm_instructions = Vec::new();
    for val in instructions.iter() {
        match val {
            tacky::Instruction::Return(val) => {
                asm_instructions.push(Instruction::Move(MoveInstruction {
                    src: translate_val(val),
                    dst: Operand::Register(Register::AX),
                }));
                asm_instructions.push(Instruction::Ret)
            }
            tacky::Instruction::Unary(uop, src, dst) => {
                asm_instructions.push(Instruction::Move(MoveInstruction {
                    src: translate_val(src),
                    dst: translate_val(dst),
                }));
                asm_instructions.push(Instruction::Unary(UnaryInstruction {
                    op: translate_unary_operator(uop),
                    operand: translate_val(dst),
                }))
            }
        }
    }
    asm_instructions
}

//FIXME
fn translate_val(val: &tacky::Val) -> Operand {
    match val {
        tacky::Val::Constant(c) => Operand::Imm(c.clone()),
        tacky::Val::Var(v) => Operand::Pseudo(v.clone()),
    }
}

fn translate_unary_operator(unary_op: &UnaryOperator) -> UnaryOp {
    match unary_op {
        UnaryOperator::Complement => UnaryOp::Not,
        UnaryOperator::Negate => UnaryOp::Neg,
    }
}
