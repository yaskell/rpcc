use crate::parser;

#[derive(Debug)]
pub struct ASMProgram {
    pub function_definition: ASMFunctionDefinition,
}

#[derive(Debug)]
pub struct ASMFunctionDefinition {
    pub name: Identifier,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Move(Move),
    Ret,
}

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub struct Imm(pub i32);

#[derive(Debug)]
pub struct Register(pub String);

#[derive(Debug)]
pub struct Move {
    pub src: Operand,
    pub dst: Operand,
}

#[derive(Debug)]
pub enum Operand {
    Imm(Imm),
    Register(Register),
}

pub fn translate_program(program: parser::Program) -> ASMProgram {
    ASMProgram {
        function_definition: translate_function_definition(program.function_definition),
    }
}

fn translate_function_definition(
    function_definition: parser::FunctionDefinition,
) -> ASMFunctionDefinition {
    ASMFunctionDefinition {
        name: translate_identifier(function_definition.name),
        instructions: translate_statement(function_definition.body),
    }
}

fn translate_identifier(identifier: parser::Identifier) -> Identifier {
    Identifier(identifier.0)
}

fn translate_statement(statement: parser::Statement) -> Vec<Instruction> {
    match statement {
        parser::Statement::Return(expression) => vec![
            Instruction::Move(Move {
                src: Operand::Imm(translate_expression(expression)),
                dst: Operand::Register(Register("EAX".to_string())),
            }),
            Instruction::Ret,
        ],
    }
}

fn translate_expression(expression: parser::Expression) -> Imm {
    match expression {
        parser::Expression::Constant(int) => translate_int(int),
    }
}

fn translate_int(int: parser::Int) -> Imm {
    Imm(int.0)
}
