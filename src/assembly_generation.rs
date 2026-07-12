use crate::parser;

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
    Move(Move),
    Ret,
}

#[derive(Debug)]
pub struct Move {
    pub src: Operand,
    pub dst: Operand,
}

#[derive(Debug)]
pub enum Operand {
    Imm(i32),
    Register(Register),
}

#[derive(Debug)]
pub enum Register {
    EAX,
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

fn translate_identifier(identifier: parser::Identifier) -> String {
    identifier
}

fn translate_statement(statement: parser::Statement) -> Vec<Instruction> {
    match statement {
        parser::Statement::Return(expression) => vec![
            Instruction::Move(Move {
                src: translate_expression(expression),
                dst: Operand::Register(Register::EAX),
            }),
            Instruction::Ret,
        ],
    }
}

fn translate_expression(expression: parser::Expression) -> Operand {
    match expression {
        parser::Expression::Constant(int) => translate_int(int),
        _ => todo!(),
    }
}

fn translate_int(int: parser::Int) -> Operand {
    Operand::Imm(int)
}
