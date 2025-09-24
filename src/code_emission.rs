use crate::ast;

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


pub fn translate_program(program: ast::Program) -> ASMProgram { 
    ASMProgram { 
        function_definition: translate_function_definition(program.function_definition) 
    }
}

fn translate_function_definition(function_definition: ast::FunctionDefinition) -> ASMFunctionDefinition {
    ASMFunctionDefinition { 
        name: translate_identifier(function_definition.name),
        instructions: translate_statement(function_definition.body)
    }
}

fn translate_identifier(identifier: ast::Identifier) -> Identifier {
    Identifier( identifier.0 )
}

fn translate_statement(statement: ast::Statement) -> Vec<Instruction> {
    match statement {
        ast::Statement::Return(expression) => vec![ 
            Instruction::Move( 
                Move { 
                    src: Operand::Imm(translate_expression(expression)), 
                    dst: Operand::Register(Register { 0: "EAX".to_string() }) 
                }
            ), 
            Instruction::Ret 
        ]
    } 
}

fn translate_expression(expression: ast::Expression) -> Imm {
    match expression {
        ast::Expression::Constant(int) => translate_int(int),
    }
}

fn translate_int(int: ast::Int) -> Imm {
    Imm { 0: int.0}
}
